//! # ESCAM Core Domain Models & Invariants
//!
//! Provides the foundational domain models, configuration types, and shared state
//! for the ESCAM G02 camera system across all bounded contexts.

use serde::{Deserialize, Serialize};

/// Operational mode for the mechanical IR-cut filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrCutMode {
    /// Day mode: mechanical IR filter active (blocks IR, passes visible light).
    Day,
    /// Night/Astro mode: mechanical IR filter retracted (passes full spectrum + near-IR + H-alpha).
    Night,
}

impl Default for IrCutMode {
    fn default() -> Self {
        Self::Day
    }
}

/// Direction command for stepper motor movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Stop = 0,
    PanRight = 1,
    PanLeft = 2,
    TiltUp = 3,
    TiltDown = 4,
}

/// Video resolution preset for streaming and still capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Resolution {
    /// Standard 720p (1280x720) @ 25fps.
    Hd720p,
    /// Sub-stream 360p (640x360) @ 25fps.
    Sd360p,
    /// Astrophotography 1.3MP SC1135 full array (1280x960).
    Astro1280x960,
}

impl Resolution {
    pub const fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::Hd720p => (1280, 720),
            Self::Sd360p => (640, 360),
            Self::Astro1280x960 => (1280, 960),
        }
    }
}

/// Core runtime configuration for the ESCAM camera daemon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraConfig {
    /// Friendly device name.
    pub device_name: String,
    /// HTTP and WebSocket listen address.
    pub web_bind_addr: String,
    /// Port for INDI protocol astronomy daemon.
    pub indi_port: u16,
    /// Maximum pan speed (steps/second).
    pub max_pan_speed: u32,
    /// Maximum tilt speed (steps/second).
    pub max_tilt_speed: u32,
    /// Soft limit for minimum pan angle in degrees.
    pub pan_min_deg: f32,
    /// Soft limit for maximum pan angle in degrees.
    pub pan_max_deg: f32,
    /// Soft limit for minimum tilt angle in degrees.
    pub tilt_min_deg: f32,
    /// Soft limit for maximum tilt angle in degrees.
    pub tilt_max_deg: f32,
    /// Watchdog refresh interval in seconds.
    pub watchdog_interval_secs: u64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            device_name: "ESCAM-G02-Rust".to_string(),
            web_bind_addr: "0.0.0.0:80".to_string(),
            indi_port: 7624,
            max_pan_speed: 1000,
            max_tilt_speed: 800,
            pan_min_deg: 0.0,
            pan_max_deg: 355.0,
            tilt_min_deg: -10.0,
            tilt_max_deg: 90.0,
            watchdog_interval_secs: 5,
        }
    }
}

/// Instantaneous status telemetry of the camera system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraStatus {
    pub uptime_secs: u64,
    pub free_ram_kb: u64,
    pub current_pan_deg: f32,
    pub current_tilt_deg: f32,
    pub ircut_mode: IrCutMode,
    #[serde(default)]
    pub irled_enabled: bool,
    pub streaming_active: bool,
    pub peer_connections: usize,
}

impl Default for CameraStatus {
    fn default() -> Self {
        Self {
            uptime_secs: 0,
            free_ram_kb: 52400,
            current_pan_deg: 177.5,
            current_tilt_deg: 40.0,
            ircut_mode: IrCutMode::Day,
            irled_enabled: false,
            streaming_active: false,
            peer_connections: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = CameraConfig::default();
        assert_eq!(config.indi_port, 7624);
        assert!(config.pan_max_deg > config.pan_min_deg);
        assert!(config.tilt_max_deg > config.tilt_min_deg);
    }

    #[test]
    fn test_resolution_dimensions() {
        assert_eq!(Resolution::Hd720p.dimensions(), (1280, 720));
        assert_eq!(Resolution::Sd360p.dimensions(), (640, 360));
        assert_eq!(Resolution::Astro1280x960.dimensions(), (1280, 960));
    }

    #[test]
    fn test_ircut_mode_serialization() {
        let serialized = serde_json::to_string(&IrCutMode::Night).unwrap();
        assert_eq!(serialized, "\"Night\"");
        let deserialized: IrCutMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, IrCutMode::Night);
    }
}
