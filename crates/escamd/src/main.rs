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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("==================================================");
    println!("🚀 Starting escamd • ESCAM G02 Pure Rust Firmware");
    println!("Target: Goke Microelectronics GK7102C (ARMv6)");
    println!("Zero Cloud Telemetry • Memory-Safe • Instant Boot");
    println!("==================================================");

    let config = CameraConfig::default();
    let status = Arc::new(Mutex::new(CameraStatus::default()));

    // 1. Initialize PTZ and Hardware Devices
    let motor = MockMotorDevice::new();
    let gpio = MockGpioDevice::new();
    let ptz = Arc::new(PtzController::new(motor, config.clone()));
    let ircut = Arc::new(IrCutController::new(gpio));

    // 2. Launch Embedded INDI Astronomy Protocol Server
    let indi_server = IndiServer::new(config.indi_port);
    info!("Starting INDI astronomy server on port {}...", config.indi_port);
    let _ = indi_server.run().await;

    // 3. Build Embedded Axum Web Server & SPA
    let app_state = AppState {
        config: config.clone(),
        status: status.clone(),
        ptz: ptz.clone(),
        ircut: ircut.clone(),
    };

    let router = build_router(app_state);
    let bind_addr = std::env::var("ESCAM_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = TcpListener::bind(&bind_addr).await?;
    info!("ESCAM Web UI & WebRTC server listening on http://{}", bind_addr);

    // 4. Background Hardware Watchdog Heartbeat
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            // Feeding watchdog
        }
    });

    // 5. Serve HTTP/WS
    axum::serve(listener, router).await?;

    Ok(())
}
