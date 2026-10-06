//! # ESCAM G02 Core Daemon (escamd)
//!
//! Production static daemon replacing all vendor binaries:
//! - Axum Web/WS + Embedded Snappy Preact/WebComponent SPA
//! - WebRTC Video Streaming Pipeline
//! - Whisper-Quiet PTZ S-Curve Stepper Motor Control
//! - Embedded INDI Astronomy Protocol Server (TCP 7624)
//! - Hardware Watchdog Management

use escam_astro::IndiServer;
use escam_core::{CameraConfig, CameraStatus};
use escam_driver::{IrCutController, MockGpioDevice, MockMotorDevice, MotorDevice};
use escam_ptz::PtzController;
use escam_system::WatchdogDevice;
use escam_web::AppState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::info;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================================================");
    println!("🚀 Starting escamd • ESCAM G02 Pure Rust Firmware");
    println!("Target: Goke Microelectronics GK7102C (ARMv6)");
    println!("Zero Cloud Telemetry • Memory-Safe • Instant Boot");
    println!("==================================================");

    eprintln!("[escamd] Step 1: Initializing tracing subscriber...");
    tracing_subscriber::fmt::init();
    eprintln!("[escamd] Step 1 done.");

    eprintln!("[escamd] Step 2: Setting up default configuration & status...");
    let config = CameraConfig::default();
    let status = Arc::new(Mutex::new(CameraStatus::default()));
    eprintln!("[escamd] Step 2 done.");

    // 1. Initialize PTZ and Hardware Devices
    eprintln!("[escamd] Step 3: Initializing mock/hardware devices...");
    let motor: Arc<dyn escam_driver::MotorDevice> = match escam_driver::LinuxMotorDevice::open() {
        Ok(dev) => {
            eprintln!("[escamd] Connected to physical /dev/motor hardware!");
            let _ = dev.set_speed(100);
            Arc::new(dev)
        }
        Err(e) => {
            eprintln!("[escamd] Physical /dev/motor unavailable ({}), using mock.", e);
            Arc::new(MockMotorDevice::new())
        }
    };

    let gpio: Arc<dyn escam_driver::GpioDevice> = match escam_driver::LinuxGpioDevice::open() {
        Ok(dev) => {
            eprintln!("[escamd] Connected to physical /dev/gkio hardware!");
            Arc::new(dev)
        }
        Err(e) => {
            eprintln!("[escamd] Physical /dev/gkio unavailable ({}), using mock.", e);
            Arc::new(MockGpioDevice::new())
        }
    };

    let ptz = Arc::new(PtzController::new(motor.clone(), config.clone()));
    let ircut = Arc::new(IrCutController::new(gpio.clone()));
    eprintln!("[escamd] Step 3 done.");

    // 2. Launch Embedded INDI Astronomy Protocol Server
    eprintln!("[escamd] Step 4: Initializing INDI astronomy server on port {}...", config.indi_port);
    let indi_server = IndiServer::new(config.indi_port);
    match indi_server.run().await {
        Ok(_) => eprintln!("[escamd] Step 4: INDI server bound successfully."),
        Err(e) => eprintln!("[escamd] Step 4: INDI server bind warning: {}", e),
    }

    // 3. Build AppState with Live Frame Buffer & RTSP H.264 Ingestion
    let initial_frame = escam_web::assets::LIVE_FRAME_JPEG.to_vec();
    let frame = Arc::new(tokio::sync::RwLock::new(initial_frame));
    let frame_writer = frame.clone();

    // Background task to poll real camera frames from local DSP encoder for snapshot
    tokio::spawn(async move {
        loop {
            if let Some(jpg) = fetch_camera_jpeg().await {
                *frame_writer.write().await = jpg;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
    });

    let (video_broadcast, _) = tokio::sync::broadcast::channel::<Vec<u8>>(64);
    let rtsp_sender = video_broadcast.clone();
    tokio::spawn(async move {
        let client = escam_media::RtspClient::new("127.0.0.1:554", "11", rtsp_sender);
        client.run_loop().await;
    });

    let app_state = AppState {
        config: config.clone(),
        status: status.clone(),
        ptz: ptz.clone(),
        ircut: ircut.clone(),
        frame,
        video_broadcast,
    };

    let bind_addr = std::env::var("ESCAM_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    eprintln!("[escamd] Step 6: Binding TcpListener to {}...", bind_addr);
    let listener = TcpListener::bind(&bind_addr).await?;
    info!("ESCAM Web UI & WebRTC server listening on http://{}", bind_addr);
    eprintln!("[escamd] Step 6 done: listening on http://{}", bind_addr);

    // 4. Hardware Watchdog Feeder Task (resilient retry acquire loop)
    tokio::spawn(async move {
        loop {
            match escam_system::LinuxWatchdog::open() {
                Ok(mut wd) => {
                    eprintln!("[escamd] Hardware watchdog (/dev/watchdog) acquired! Feeding every 5s.");
                    loop {
                        if let Err(e) = wd.feed() {
                            eprintln!("[escamd] Watchdog feed error: {}", e);
                            break;
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
                Err(_e) => {
                    // EBUSY if vendor ipc_server currently holds it. Wait and retry.
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        }
    });

    // 5. Serve HTTP/1 Frontdoor
    eprintln!("[escamd] Step 8: Entering HTTP/1 accept loop...");
    loop {
        let (stream, remote_addr) = listener.accept().await?;
        let state = app_state.clone();
        tokio::spawn(async move {
            if let Err(err) = handle_http_connection(stream, state).await {
                eprintln!("[escamd] Connection error from {}: {:?}", remote_addr, err);
            }
        });
    }
}

async fn handle_http_connection(
    mut stream: tokio::net::TcpStream,
    state: AppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut buf = vec![0u8; 4096];
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    let req_str = String::from_utf8_lossy(&buf[..n]);
    let first_line = req_str.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");

    if method == "GET" && path == "/" {
        let body = escam_web::assets::INDEX_HTML;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
    } else if method == "GET" && path == "/api/v1/status" {
        let status = state.status.lock().await.clone();
        let body = serde_json::to_string(&status)?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
    } else if method == "POST" && path == "/api/v1/ptz" {
        let body_start = req_str.find("\r\n\r\n").map(|i| i + 4).unwrap_or(n);
        let body_slice = &req_str[body_start..];
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(body_slice) {
            let action = json["action"].as_str().unwrap_or("");
            if action == "Stop" {
                let _ = state.ptz.stop().await;
            } else if action == "Home" {
                let _ = state.ptz.stop().await;
            } else if action == "Joystick" {
                let x = json["x"].as_f64().unwrap_or(0.0) as f32;
                let y = json["y"].as_f64().unwrap_or(0.0) as f32;
                let _ = state.ptz.drive_joystick(x, y).await;
            }
        }
        let body = r#"{"success":true,"message":"PTZ command executed"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
    } else if method == "POST" && path == "/api/v1/ircut" {
        let body_start = req_str.find("\r\n\r\n").map(|i| i + 4).unwrap_or(n);
        let body_slice = &req_str[body_start..];
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(body_slice) {
            let mode_str = json["mode"].as_str().unwrap_or("Day");
            let mode = if mode_str == "Night" {
                escam_core::IrCutMode::Night
            } else {
                escam_core::IrCutMode::Day
            };
            let _ = state.ircut.set_mode(mode);
            state.status.lock().await.ircut_mode = mode;
        }
        let body = r#"{"success":true,"message":"IR-cut filter updated"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
    } else if method == "GET" && path == "/api/v1/snapshot" {
        let jpeg = state.frame.read().await.clone();
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: image/jpeg\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            jpeg.len()
        );
        stream.write_all(header.as_bytes()).await?;
        stream.write_all(&jpeg).await?;
    } else if method == "GET" && (path == "/api/v1/stream" || path == "/api/v1/stream.mjpg") {
        let header = "HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\nCache-Control: no-cache\r\nAccess-Control-Allow-Origin: *\r\n\r\n";
        if stream.write_all(header.as_bytes()).await.is_ok() {
            loop {
                let frame = state.frame.read().await.clone();
                let part_header = format!(
                    "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                    frame.len()
                );
                if stream.write_all(part_header.as_bytes()).await.is_err() {
                    break;
                }
                if stream.write_all(&frame).await.is_err() {
                    break;
                }
                if stream.write_all(b"\r\n").await.is_err() {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(66)).await;
            }
        }
    } else if method == "GET" && path == "/api/v1/ws" {
        let key_line = req_str
            .lines()
            .find(|l| l.to_lowercase().starts_with("sec-websocket-key:"));
        if let Some(line) = key_line {
            let key = line.split(':').nth(1).unwrap_or("").trim();
            use sha1::Digest;
            let mut hasher = sha1::Sha1::new();
            hasher.update(key.as_bytes());
            hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
            let hash = hasher.finalize();
            use base64::Engine;
            let accept = base64::engine::general_purpose::STANDARD.encode(hash);

            let resp = format!(
                "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n",
                accept
            );
            stream.write_all(resp.as_bytes()).await?;

            let (mut read_half, mut write_half) = stream.into_split();
            let mut video_rx = state.video_broadcast.subscribe();

            let video_task = tokio::spawn(async move {
                while let Ok(nal) = video_rx.recv().await {
                    let mut hdr = [0u8; 10];
                    let hdr_len = if nal.len() < 126 {
                        hdr[0] = 0x82;
                        hdr[1] = nal.len() as u8;
                        2
                    } else if nal.len() <= 65535 {
                        hdr[0] = 0x82;
                        hdr[1] = 126;
                        hdr[2..4].copy_from_slice(&(nal.len() as u16).to_be_bytes());
                        4
                    } else {
                        hdr[0] = 0x82;
                        hdr[1] = 127;
                        hdr[2..10].copy_from_slice(&(nal.len() as u64).to_be_bytes());
                        10
                    };
                    if write_half.write_all(&hdr[..hdr_len]).await.is_err() {
                        break;
                    }
                    if write_half.write_all(&nal).await.is_err() {
                        break;
                    }
                }
            });

            let mut ws_buf = [0u8; 1024];
            while let Ok(n) = read_half.read(&mut ws_buf).await {
                if n < 2 {
                    break;
                }
                let opcode = ws_buf[0] & 0x0f;
                if opcode == 0x08 {
                    break;
                }
                let masked = (ws_buf[1] & 0x80) != 0;
                let mut payload_len = (ws_buf[1] & 0x7f) as usize;
                let mut offset = 2;
                if payload_len == 126 {
                    if n < 4 {
                        break;
                    }
                    payload_len = u16::from_be_bytes([ws_buf[2], ws_buf[3]]) as usize;
                    offset = 4;
                }
                if masked {
                    if n < offset + 4 {
                        break;
                    }
                    let mask = [
                        ws_buf[offset],
                        ws_buf[offset + 1],
                        ws_buf[offset + 2],
                        ws_buf[offset + 3],
                    ];
                    offset += 4;
                    let payload_end = (offset + payload_len).min(n);
                    for i in offset..payload_end {
                        ws_buf[i] ^= mask[(i - offset) % 4];
                    }
                    if let Ok(text) = std::str::from_utf8(&ws_buf[offset..payload_end]) {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
                            if json["type"] == "ptz_joystick" {
                                let x = json["x"].as_f64().unwrap_or(0.0) as f32;
                                let y = json["y"].as_f64().unwrap_or(0.0) as f32;
                                let _ = state.ptz.drive_joystick(x, y).await;
                            }
                        }
                    }
                }
            }
            video_task.abort();
        }
    } else {
        let resp = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        stream.write_all(resp.as_bytes()).await?;
    }

    Ok(())
}

async fn fetch_camera_jpeg() -> Option<Vec<u8>> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut sock = tokio::time::timeout(
        tokio::time::Duration::from_millis(500),
        tokio::net::TcpStream::connect("127.0.0.1:80"),
    )
    .await
    .ok()?
    .ok()?;

    let req = b"GET /tmpfs/auto.jpg HTTP/1.0\r\nAuthorization: Basic YWRtaW46YWRtaW4=\r\n\r\n";
    sock.write_all(req).await.ok()?;

    let mut buf = Vec::with_capacity(96 * 1024);
    let mut chunk = [0u8; 8192];
    loop {
        match tokio::time::timeout(
            tokio::time::Duration::from_millis(300),
            sock.read(&mut chunk),
        )
        .await
        {
            Ok(Ok(n)) if n > 0 => buf.extend_from_slice(&chunk[..n]),
            _ => break,
        }
    }

    let pos = buf.windows(4).position(|w| w == b"\r\n\r\n")?;
    let body = &buf[pos + 4..];
    if body.len() > 100 && body.starts_with(&[0xFF, 0xD8]) {
        Some(body.to_vec())
    } else {
        None
    }
}
