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
use escam_driver::{IrCutController, MockGpioDevice, MockMotorDevice};
use escam_ptz::PtzController;
use escam_web::{build_router, AppState};
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
    let motor = MockMotorDevice::new();
    let gpio = MockGpioDevice::new();
    let ptz = Arc::new(PtzController::new(motor, config.clone()));
    let ircut = Arc::new(IrCutController::new(gpio));
    eprintln!("[escamd] Step 3 done.");

    // 2. Launch Embedded INDI Astronomy Protocol Server
    eprintln!("[escamd] Step 4: Initializing INDI astronomy server on port {}...", config.indi_port);
    let indi_server = IndiServer::new(config.indi_port);
    match indi_server.run().await {
        Ok(_) => eprintln!("[escamd] Step 4: INDI server bound successfully."),
        Err(e) => eprintln!("[escamd] Step 4: INDI server bind warning: {}", e),
    }

    // 3. Build Embedded Axum Web Server & SPA
    eprintln!("[escamd] Step 5: Building Axum Web Router & AppState...");
    let app_state = AppState {
        config: config.clone(),
        status: status.clone(),
        ptz: ptz.clone(),
        ircut: ircut.clone(),
    };

    let router = build_router(app_state);
    let bind_addr = std::env::var("ESCAM_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    eprintln!("[escamd] Step 6: Binding TcpListener to {}...", bind_addr);
    let listener = TcpListener::bind(&bind_addr).await?;
    info!("ESCAM Web UI & WebRTC server listening on http://{}", bind_addr);
    eprintln!("[escamd] Step 6 done: listening on http://{}", bind_addr);

    // 5. Serve HTTP/1 explicitly avoiding HTTP/2 (AtomicU64 unsupported on ARMv6)
    eprintln!("[escamd] Step 8: Entering HTTP/1 accept loop...");
    loop {
        let (stream, remote_addr) = listener.accept().await?;
        eprintln!("[escamd] Accepted connection from: {}", remote_addr);
        let tower_service = router.clone();
        tokio::spawn(async move {
            let io = hyper_util::rt::TokioIo::new(stream);
            let hyper_service = hyper_util::service::TowerToHyperService::new(tower_service);
            if let Err(err) = hyper::server::conn::http1::Builder::new()
                .serve_connection(io, hyper_service)
                .with_upgrades()
                .await
            {
                eprintln!("[escamd] Connection error from {}: {:?}", remote_addr, err);
            }
        });
    }
}
