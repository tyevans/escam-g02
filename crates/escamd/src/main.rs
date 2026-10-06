//! # ESCAM G02 Core Daemon (escamd)
//!
//! Production static daemon replacing all vendor binaries:
//! - Axum Web/WS + Embedded Snappy Preact/WebComponent SPA
//! - WebRTC Video Streaming Pipeline
//! - Whisper-Quiet PTZ S-Curve Stepper Motor Control
//! - Embedded INDI Astronomy Protocol Server (TCP 7624)
//! - Hardware Watchdog Management

mod http;

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
    println!("🚀 Starting escamd • ESCAM G02 Pure Rust Firmware (GK7102C)");
    println!("==================================================");
    tracing_subscriber::fmt::init();
    let config = CameraConfig::default();
    let status = Arc::new(Mutex::new(CameraStatus::default()));

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

    // 3. Build AppState with Live Frame Buffer & Dual-Mode Video Ingestion (VPU Socket / RTSP)
    let initial_frame = escam_web::assets::LIVE_FRAME_JPEG.to_vec();
    let frame = Arc::new(tokio::sync::RwLock::new(initial_frame));

    let (video_broadcast, _) = tokio::sync::broadcast::channel::<Vec<u8>>(16);
    let video_sender = video_broadcast.clone();
    tokio::spawn(async move {
        let vpu_sock = std::path::Path::new("/tmp/venc.sock");
        loop {
            if vpu_sock.exists() {
                eprintln!("[escamd] Ingesting video via zero-copy VPU Unix socket (/tmp/venc.sock)...");
                let vpu_reader = escam_media::VpuUnixStreamReader::new(vpu_sock);
                vpu_reader.run_loop(video_sender.clone()).await;
            } else {
                eprintln!("[escamd] /tmp/venc.sock not present; falling back to loopback RTSP (127.0.0.1:554)...");
                let client = escam_media::RtspClient::new("127.0.0.1:554", "11", video_sender.clone());
                tokio::select! {
                    _ = client.run_loop() => {},
                    _ = async {
                        loop {
                            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                            if std::path::Path::new("/tmp/venc.sock").exists() {
                                break;
                            }
                        }
                    } => {
                        eprintln!("[escamd] Detected /tmp/venc.sock! Transitioning to VPU socket ingestion...");
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
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
            if let Err(err) = http::handle_http_connection(stream, state).await {
                eprintln!("[escamd] Connection error from {}: {:?}", remote_addr, err);
            }
        });
    }
}
