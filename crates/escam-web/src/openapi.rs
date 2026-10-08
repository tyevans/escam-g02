//! Comprehensive OpenAPI 3.1 Specification Generator.
//!
//! Generates the formal OpenAPI 3.1 JSON specification for all ESCAM REST endpoints.

pub struct OpenApiRegistry;

impl OpenApiRegistry {
    pub fn generate_spec_json() -> String {
        r#"{
  "openapi": "3.1.0",
  "info": {
    "title": "ESCAM G02 Pure Rust Firmware & Astrophotography API",
    "version": "1.0.0",
    "description": "Scientific astrophotography, PTZ motion, computer vision, and camera control API."
  },
  "servers": [
    { "url": "http://10.75.2.93:8080", "description": "Physical ESCAM G02 Device" }
  ],
  "paths": {
    "/api/v1/status": {
      "get": {
        "summary": "Retrieve system telemetry, memory usage, and camera state",
        "responses": { "200": { "description": "Telemetry status object" } }
      }
    },
    "/api/v1/ptz": {
      "post": {
        "summary": "Execute PTZ directional motion or velocity joystick command",
        "responses": { "200": { "description": "PTZ execution acknowledgment" } }
      }
    },
    "/api/v1/ptz/home": {
      "post": {
        "summary": "Calibrate soft-homing position against mechanical hardstops",
        "responses": { "200": { "description": "Homing complete" } }
      }
    },
    "/api/v1/ptz/backlash": {
      "get": { "summary": "Get gear backlash compensation settings" },
      "post": { "summary": "Configure gear backlash compensation steps" }
    },
    "/api/v1/camera": {
      "get": { "summary": "Get optical and sensor control parameters" },
      "post": { "summary": "Set exposure duration, gain, and framerate" }
    },
    "/api/v1/sensor/registers": {
      "get": { "summary": "Read raw CMOS sensor I2C registers (GC1034 / SC1135)" },
      "post": { "summary": "Write raw CMOS sensor register lines and gain" }
    },
    "/api/v1/snapshot": {
      "get": {
        "summary": "Acquire current instantaneous processed JPEG snapshot",
        "responses": { "200": { "description": "image/jpeg" } }
      }
    },
    "/api/v1/astro/capture.fits": {
      "get": {
        "summary": "Download uncompressed 16-bit linear astronomical FITS container with WCS",
        "responses": { "200": { "description": "image/fits" } }
      }
    },
    "/api/v1/astro/focus": {
      "get": {
        "summary": "Compute real-time star PSF count, centroids, and FWHM seeing score",
        "responses": { "200": { "description": "FocusMetric JSON" } }
      }
    },
    "/api/v1/astro/transients": {
      "get": {
        "summary": "List detected meteor, fireball, and satellite linear streaks",
        "responses": { "200": { "description": "List of TransientEvent objects" } }
      }
    },
    "/api/v1/astro/targets": {
      "get": {
        "summary": "List celestial catalog targets with calculated Alt/Az coordinates"
      }
    },
    "/api/v1/astro/slew": {
      "post": {
        "summary": "Slew PTZ mount to selected celestial target"
      }
    },
    "/api/v1/astro/guide/start": {
      "post": { "summary": "Lock onto guide star and start optical closed-loop autoguiding" }
    },
    "/api/v1/astro/calibration/dark": {
      "post": { "summary": "Capture and set master dark frame" }
    },
    "/api/v1/recorder/trigger": {
      "post": { "summary": "Trigger pre-roll circular buffer video clip export" }
    },
    "/api/v1/recorder/clips": {
      "get": { "summary": "List saved event video clips" }
    },
    "/api/v1/system/time": {
      "get": { "summary": "Get precision Julian Date, LST, and UTC clock status" },
      "post": { "summary": "Set manual or GPS NMEA UTC clock and site coordinates" }
    },
    "/api/v1/system/flash": {
      "get": { "summary": "Audit persistent flash storage and reclaimable space" }
    },
    "/api/v1/system/debloat": {
      "post": { "summary": "Execute vendor software purge and clean-boot lockdown" }
    }
  }
}"#.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_openapi_json() {
        let json_str = OpenApiRegistry::generate_spec_json();
        let val: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(val["openapi"], "3.1.0");
        assert!(val["paths"]["/api/v1/astro/capture.fits"].is_object());
        assert!(val["paths"]["/api/v1/ptz"].is_object());
    }
}
