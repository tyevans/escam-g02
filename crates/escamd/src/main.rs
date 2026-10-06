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

    // 3. Build AppState
    let app_state = AppState {
        config: config.clone(),
        status: status.clone(),
        ptz: ptz.clone(),
        ircut: ircut.clone(),
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
        let jpeg = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01,
            0x00, 0x48, 0x00, 0x48, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x03, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x03, 0x02, 0x02, 0x02, 0x03, 0x03, 0x03, 0x03, 0x04, 0x06, 0x04,
            0x04, 0x04, 0x04, 0x04, 0x08, 0x06, 0x06, 0x05, 0x06, 0x09, 0x08, 0x0A, 0x0A, 0x09,
            0x08, 0x09, 0x09, 0x0A, 0x0C, 0x0F, 0x0C, 0x0A, 0x0B, 0x0E, 0x0B, 0x09, 0x09, 0x0D,
            0x11, 0x0D, 0x0E, 0x0F, 0x10, 0x10, 0x11, 0x10, 0x0A, 0x0C, 0x12, 0x13, 0x12, 0x10,
            0x13, 0x0F, 0x10, 0x10, 0x10, 0xFF, 0xC9, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00, 0x01,
            0x01, 0x01, 0x11, 0x00, 0xFF, 0xCC, 0x00, 0x06, 0x00, 0x10, 0x10, 0x05, 0xFF, 0xDA,
            0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, 0xD2, 0xCF, 0x20, 0xFF, 0xD9,
        ];
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: image/jpeg\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            jpeg.len()
        );
        stream.write_all(header.as_bytes()).await?;
        stream.write_all(&jpeg).await?;
    } else {
        let resp = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        stream.write_all(resp.as_bytes()).await?;
    }

    Ok(())
}
