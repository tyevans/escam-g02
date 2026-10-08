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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Resolution {
    /// Standard 720p (1280x720) @ 25fps.
    #[default]
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

/// Manual astrophotography and sensor camera controls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraControls {
    /// Exposure duration in seconds (0.001s to 60.0s).
    pub exposure_secs: f32,
    /// Sensor analog/digital gain multiplier (1.0x to 64.0x / 0 to 36 dB).
    pub gain: f32,
    /// Target frame rate in FPS (e.g. 25.0, or 0.1 for 10s exposure).
    pub target_fps: f32,
    /// Capture resolution and binning preset.
    pub resolution: Resolution,
    /// Automatic exposure/gain toggle.
    pub auto_exposure: bool,
    /// Active multi-frame stacking mode ("Off", "Average", "Additive").
    pub stack_mode: String,
    /// Astronomical auto histogram stretch mode ("None", "Auto", "Mtf", "Asinh", "Aggressive").
    #[serde(default = "default_auto_stretch")]
    pub auto_stretch: String,
}

fn default_auto_stretch() -> String {
    "Auto".to_string()
}

impl Default for CameraControls {
    fn default() -> Self {
        Self {
            exposure_secs: 0.04, // ~25 FPS
            gain: 1.0,           // 1x (0 dB / ISO 100)
            target_fps: 25.0,
            resolution: Resolution::Hd720p,
            auto_exposure: true,
            stack_mode: "Off".to_string(),
            auto_stretch: "Auto".to_string(),
        }
    }
}

impl CameraControls {
    /// Normalizes camera control values within physical sensor and timing limits.
    pub fn normalize(&mut self) {
        self.exposure_secs = self.exposure_secs.clamp(0.001, 60.0);
        self.gain = self.gain.clamp(1.0, 64.0);
        let max_fps = 1.0 / self.exposure_secs;
        if self.target_fps > max_fps || !self.auto_exposure {
            self.target_fps = self.target_fps.min(max_fps);
        }
        self.target_fps = self.target_fps.clamp(0.016, 60.0);
    }
}

/// Partial update request for manual camera controls.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CameraControlsUpdate {
    pub exposure_secs: Option<f32>,
    pub gain: Option<f32>,
    pub target_fps: Option<f32>,
    pub resolution: Option<Resolution>,
    pub auto_exposure: Option<bool>,
    pub stack_mode: Option<String>,
    pub auto_stretch: Option<String>,
}

impl CameraControlsUpdate {
    pub fn apply_to(&self, controls: &mut CameraControls) {
        if let Some(exp) = self.exposure_secs {
            controls.exposure_secs = exp;
            if exp > 0.04 {
                controls.target_fps = (1.0 / exp).min(controls.target_fps);
            }
        }
        if let Some(g) = self.gain {
            controls.gain = g;
        }
        if let Some(fps) = self.target_fps {
            controls.target_fps = fps;
        }
        if let Some(res) = self.resolution {
            controls.resolution = res;
        }
        if let Some(ae) = self.auto_exposure {
            controls.auto_exposure = ae;
        }
        if let Some(ref sm) = self.stack_mode {
            controls.stack_mode = sm.clone();
        }
        if let Some(ref as_mode) = self.auto_stretch {
            controls.auto_stretch = as_mode.clone();
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
    #[serde(default)]
    pub controls: CameraControls,
    #[serde(default)]
    pub stacked_frames: u32,
    #[serde(default)]
    pub total_stacked_exposure_secs: f32,
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
            controls: CameraControls::default(),
            stacked_frames: 0,
            total_stacked_exposure_secs: 0.0,
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

    #[test]
    fn test_camera_controls_normalize_long_exposure() {
        let mut controls = CameraControls {
            exposure_secs: 10.0,
            gain: 16.0,
            target_fps: 25.0,
            resolution: Resolution::Astro1280x960,
            auto_exposure: false,
            stack_mode: "Average".to_string(),
            auto_stretch: "Auto".to_string(),
        };
        controls.normalize();
        assert_eq!(controls.exposure_secs, 10.0);
        assert_eq!(controls.gain, 16.0);
        // At 10s exposure, target_fps must be capped at 0.1 FPS
        assert!((controls.target_fps - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_camera_controls_partial_update() {
        let mut controls = CameraControls::default();
        let update = CameraControlsUpdate {
            exposure_secs: Some(10.0),
            gain: Some(8.0),
            target_fps: None,
            resolution: Some(Resolution::Astro1280x960),
            auto_exposure: Some(false),
            stack_mode: Some("Additive".to_string()),
            auto_stretch: Some("Aggressive".to_string()),
        };
        update.apply_to(&mut controls);
        controls.normalize();
        assert_eq!(controls.exposure_secs, 10.0);
        assert_eq!(controls.gain, 8.0);
        assert_eq!(controls.auto_stretch, "Aggressive");
        assert!((controls.target_fps - 0.1).abs() < 0.001);
        assert_eq!(controls.resolution, Resolution::Astro1280x960);
    }
}
