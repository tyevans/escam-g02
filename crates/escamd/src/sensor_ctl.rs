//! # Sensor and ISP Hardware Control (Vendor IPC Bridge)
//!
//! Provides bidirectional control over the physical CMOS sensor (GC1034)
//! and Goke GK7102C ISP pipeline via the loopback IPC interface:
//! - Hardware Gain Control (-gc)
//! - Exposure / Shutter Control (-shutter, -ae)
//! - Brightness & Contrast fine-tuning
//! - Process supervisor ensuring vendor camera daemon remains active

use escam_core::CameraControls;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{info, warn};

/// Applies manual astrophotography and sensor controls to the physical camera ISP.
pub async fn apply_hardware_sensor_controls(
    controls: &CameraControls,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Calculate hardware Gain Control (gc): GC1034 supports 0 to 64
    // 1x = 10, 4x = 24 (vendor default), 16x = 42, 64x = 64
    let gc = if controls.gain <= 1.0 {
        10
    } else {
        let log_ratio = controls.gain.log2() / 6.0; // 0.0 at 1x, 1.0 at 64x
        ((log_ratio * 54.0 + 10.0).clamp(0.0, 64.0)).round() as u32
    };

    // 2. Exposure & Shutter
    // ae: 4 = auto exposure, 0 = manual
    let (ae, shutter) = if controls.auto_exposure || controls.exposure_secs <= 0.04 {
        (4, 65535)
    } else {
        (0, 65535)
    };

    // 3. Brightness: base 64, slightly boosted on high-gain long-exposures
    let brightness = if controls.gain > 8.0 { 72 } else { 64 };
    let contrast = if controls.gain > 8.0 { 20 } else { 16 };

    let query = format!(
        "cmd=setimageattr&-act=set&-gc={}&-brightness={}&-contrast={}&-shutter={}&-ae={}",
        gc, brightness, contrast, shutter, ae
    );

    let resp = send_vendor_cgi(&query).await?;
    info!(
        "[sensor_ctl] Applied hardware sensor settings: gc={}, ae={}, shutter={}, resp={}",
        gc, ae, shutter, resp.trim()
    );
    Ok(())
}

/// Sends a CGI parameter command to the vendor loopback web server (127.0.0.1:80).
pub async fn send_vendor_cgi(query: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut sock = match tokio::time::timeout(
        Duration::from_millis(400),
        TcpStream::connect("127.0.0.1:8081"),
    )
    .await
    {
        Ok(Ok(s)) => s,
        _ => {
            tokio::time::timeout(
                Duration::from_millis(400),
                TcpStream::connect("127.0.0.1:80"),
            )
            .await??
        }
    };

    let req = format!(
        "GET /cgi-bin/hi3510/param.cgi?{} HTTP/1.0\r\nAuthorization: Basic YWRtaW46YWRtaW4=\r\nConnection: close\r\n\r\n",
        query
    );
    sock.write_all(req.as_bytes()).await?;

    let mut buf = Vec::with_capacity(2048);
    let mut chunk = [0u8; 512];
    loop {
        match tokio::time::timeout(Duration::from_millis(300), sock.read(&mut chunk)).await {
            Ok(Ok(n)) if n > 0 => buf.extend_from_slice(&chunk[..n]),
            _ => break,
        }
    }
    Ok(String::from_utf8_lossy(&buf).to_string())
}

/// Fetches the latest live JPEG snapshot directly from the hardware video encoder.
pub async fn fetch_camera_jpeg() -> Option<Vec<u8>> {
    for path in &["/mnt/mtd/ipc/tmpfs/live.jpg", "/mnt/mtd/ipc/tmpfs/snap.jpg"] {
        if let Ok(data) = tokio::fs::read(path).await {
            if data.len() > 100 && data.starts_with(&[0xFF, 0xD8]) {
                return Some(data);
            }
        }
    }
    None
}

/// Ensures the vendor camera service is running; resurrects it if stopped.
pub async fn ensure_vendor_ipc() {
    if tokio::time::timeout(Duration::from_millis(400), TcpStream::connect("127.0.0.1:554"))
        .await
        .map(|r| r.is_ok())
        .unwrap_or(false)
    {
        return;
    }

    warn!("[sensor_ctl] Vendor IPC daemon inactive on 127.0.0.1:554. Resurrecting...");
    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg("trap '' HUP; /mnt/mtd/ipc/tmpfs/ipc_server >/dev/null 2>&1 &")
        .spawn();
}
