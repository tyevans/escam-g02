//! # Astronomical & Scientific Image API Routes
//!
//! Handles endpoints for FITS download, star focus metrics, transient events,
//! celestial targets catalog, slews, autoguiding, and calibration.

use escam_astro::{
    CalibrationEngine, StarDetector, TransientDetector,
    TransientEvent,
};
use escam_ptz::CelestialCatalog;
use escam_web::AppState;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref CALIBRATOR: Mutex<CalibrationEngine> = Mutex::new(CalibrationEngine::new(1280, 720));
    static ref CATALOG: Mutex<CelestialCatalog> = Mutex::new(CelestialCatalog::new(37.7749, -122.4194));
    static ref TRANSIENT_DETECTOR: Mutex<TransientDetector> = Mutex::new(TransientDetector::new(1280, 720));
    static ref GUIDING_ACTIVE: AtomicBool = AtomicBool::new(false);
}

pub async fn handle_astro_route(
    stream: &mut TcpStream,
    method: &str,
    path: &str,
    body_str: &str,
    _state: &AppState,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    if method == "GET" && path == "/api/v1/astro/focus" {
        // Evaluate focus metric on synthetic / current frame
        let detector = StarDetector::new(1280, 720);
        let frame = vec![120u16; 1280 * 720]; // Baseline flux
        let metric = detector.detect_stars(&frame);
        let json = serde_json::to_string(&metric)?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            json.len(),
            json
        );
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "GET" && path == "/api/v1/astro/transients" {
        let events: Vec<TransientEvent> = Vec::new();
        let json = serde_json::to_string(&events)?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            json.len(),
            json
        );
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "GET" && path == "/api/v1/astro/targets" {
        let cat = CATALOG.lock().await;
        let targets = cat.list_targets();
        let json = serde_json::to_string(targets)?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            json.len(),
            json
        );
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "POST" && path == "/api/v1/astro/slew" {
        let parsed: serde_json::Value = serde_json::from_str(body_str).unwrap_or(serde_json::Value::Null);
        let target_name = parsed["target"].as_str().unwrap_or("Polaris");
        let cat = CATALOG.lock().await;
        let pos = cat.compute_target_position(target_name, 12.0);

        let json = if let Some(p) = pos {
            serde_json::json!({ "success": true, "target": target_name, "position": p })
        } else {
            serde_json::json!({ "success": false, "error": "Target not found" })
        };
        let body = serde_json::to_string(&json)?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "POST" && path == "/api/v1/astro/guide/start" {
        GUIDING_ACTIVE.store(true, Ordering::SeqCst);
        let resp = json_response(r#"{"success":true,"guiding":"active"}"#);
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "POST" && path == "/api/v1/astro/guide/stop" {
        GUIDING_ACTIVE.store(false, Ordering::SeqCst);
        let resp = json_response(r#"{"success":true,"guiding":"stopped"}"#);
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "POST" && path == "/api/v1/astro/calibration/dark" {
        let mut cal = CALIBRATOR.lock().await;
        let dark = vec![50u16; 1280 * 720];
        cal.set_master_dark(dark);
        let resp = json_response(r#"{"success":true,"calibrator":"dark_set"}"#);
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "GET" && path == "/api/v1/astro/calibration/status" {
        let cal = CALIBRATOR.lock().await;
        let json = serde_json::json!({
            "has_dark": cal.has_dark(),
            "has_flat": cal.has_flat(),
            "has_bias": cal.has_bias(),
            "pedestal": cal.pedestal()
        });
        let body = serde_json::to_string(&json)?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    Ok(false)
}

pub fn generate_synthetic_bayer(width: u32, height: u32, exposure: f32, gain: f32) -> escam_astro::BayerFrame {
    use escam_astro::{BayerFrame, BayerPattern};
    let mut frame = BayerFrame::new(width, height, 10, BayerPattern::Rggb, exposure);
    let bg_flux = ((100.0f32 + 250.0f32 * (1.0f32 - (-exposure / 4.0f32).exp())) * gain.sqrt())
        .clamp(80.0f32, 950.0f32) as u32;

    for y in 0..height {
        for x in 0..width {
            let noise = ((x.wrapping_mul(31) ^ y.wrapping_mul(17)) % 19) as u32;
            let mut val = (bg_flux + noise) as u16;
            let star_grid_x = x % 160;
            let star_grid_y = y % 120;
            let dx = (star_grid_x as i32) - 80;
            let dy = (star_grid_y as i32) - 60;
            let dist_sq = (dx * dx + dy * dy) as u32;

            if dist_sq < 36 {
                let star_brightness = ((1000.0 * (exposure / (exposure + 0.5)) * gain.sqrt())
                    / (1.0 + (dist_sq as f32) * 0.4)) as u32;
                val = (val as u32 + star_brightness).min(1023) as u16;
            }
            frame.set_pixel(x, y, val);
        }
    }
    frame
}

fn json_response(json_body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        json_body.len(),
        json_body
    )
}
