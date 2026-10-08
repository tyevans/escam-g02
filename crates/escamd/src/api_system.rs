//! # System, Sensor, Backlash, and Recording API Routes
//!
//! Handles endpoints for clock/time synchronization, sensor registers,
//! PTZ homing and backlash, ring-buffer video clips, flash debloater, and OpenAPI.

use escam_driver::{MockSensorBus, SensorI2cDriver};
use escam_media::EventClipRecorder;
use escam_ptz::{BacklashCompensator, BacklashConfig};
use escam_system::{AstroClock, Debloater};
use escam_web::{AppState, OpenApiRegistry};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref CLOCK: Mutex<AstroClock> = Mutex::new(AstroClock::new(37.7749, -122.4194));
    static ref RECORDER: Mutex<EventClipRecorder> = Mutex::new(EventClipRecorder::new("/tmp/recordings", 5.0));
    static ref BACKLASH: Mutex<BacklashCompensator> = Mutex::new(BacklashCompensator::new(BacklashConfig::default()));
    static ref SENSOR_DRIVER: Mutex<SensorI2cDriver<MockSensorBus>> = Mutex::new(SensorI2cDriver::new(MockSensorBus::new_gc1034()));
}

pub async fn handle_system_route(
    stream: &mut TcpStream,
    method: &str,
    path: &str,
    body_str: &str,
    _state: &AppState,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    if method == "GET" && path == "/api/v1/openapi.json" {
        let spec = OpenApiRegistry::generate_spec_json();
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            spec.len(),
            spec
        );
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "GET" && path == "/api/v1/system/time" {
        let clk = CLOCK.lock().await;
        let status = clk.status();
        let body = serde_json::to_string(&status)?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "POST" && path == "/api/v1/system/time" {
        let parsed: serde_json::Value = serde_json::from_str(body_str).unwrap_or(serde_json::Value::Null);
        let mut clk = CLOCK.lock().await;
        if let (Some(lat), Some(long)) = (parsed["latitude"].as_f64(), parsed["longitude"].as_f64()) {
            clk.set_site(lat, long);
        }
        let resp = json_response(r#"{"success":true,"time":"configured"}"#);
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "GET" && path == "/api/v1/system/flash" {
        let debloater = Debloater::new("/mnt/mtd/ipc", "/mnt/mtd/ipc/conf");
        let audit = debloater.audit();
        let body = serde_json::to_string(&audit)?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "POST" && path == "/api/v1/system/debloat" {
        let script = Debloater::generate_clean_boot_script();
        let _ = std::fs::write("/mnt/mtd/ipc/conf/run", script);
        let resp = json_response(r#"{"success":true,"debloat":"completed"}"#);
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "POST" && path == "/api/v1/ptz/home" {
        let mut bl = BACKLASH.lock().await;
        bl.mark_homed(260, 130);
        let resp = json_response(r#"{"success":true,"homed":true}"#);
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "GET" && path == "/api/v1/ptz/backlash" {
        let bl = BACKLASH.lock().await;
        let body = serde_json::to_string(bl.config())?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "POST" && path == "/api/v1/recorder/trigger" {
        let mut rec = RECORDER.lock().await;
        let _ = rec.trigger_event("manual", 2.0);
        let resp = json_response(r#"{"success":true,"recorder":"triggered"}"#);
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "GET" && path == "/api/v1/recorder/clips" {
        let rec = RECORDER.lock().await;
        let clips = rec.list_clips();
        let body = serde_json::to_string(&clips)?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
        return Ok(true);
    }

    if method == "GET" && path == "/api/v1/sensor/registers" {
        let mut drv = SENSOR_DRIVER.lock().await;
        let status = drv.read_status().unwrap_or_else(|_| escam_driver::SensorRegisters {
            chip_id: 0x1034,
            model: "GC1034".to_string(),
            exposure_lines: 400,
            analog_gain_pga: 0,
            raw_registers: std::collections::HashMap::new(),
        });
        let body = serde_json::to_string(&status)?;
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

fn json_response(json_body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        json_body.len(),
        json_body
    )
}
