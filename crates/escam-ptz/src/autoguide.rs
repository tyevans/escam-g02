//! Closed-Loop Optical Autoguider and Micro-Pulse Motor Corrections.
//!
//! Tracks a guide star centroid and evaluates closed-loop PI micro-pulse corrections
//! to compensate for periodic mount error and atmospheric drift.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideConfig {
    pub kp: f64,              // Proportional gain (ms of pulse per pixel error)
    pub ki: f64,              // Integral gain
    pub deadband_pixels: f64, // Deadband to reject atmospheric scintillation
    pub max_pulse_ms: u32,    // Maximum pulse duration clamp
    pub min_pulse_ms: u32,    // Minimum pulse threshold
}

impl Default for GuideConfig {
    fn default() -> Self {
        Self {
            kp: 25.0,
            ki: 2.0,
            deadband_pixels: 0.15,
            max_pulse_ms: 500,
            min_pulse_ms: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideCorrection {
    pub dx_pixels: f64,
    pub dy_pixels: f64,
    pub pan_pulse_ms: i32,  // >0 East / Right, <0 West / Left
    pub tilt_pulse_ms: i32, // >0 North / Up, <0 South / Down
    pub correction_active: bool,
}

pub struct AutoGuider {
    config: GuideConfig,
    lock_x: Option<f64>,
    lock_y: Option<f64>,
    integral_x: f64,
    integral_y: f64,
    is_active: bool,
}

impl AutoGuider {
    pub fn new(config: GuideConfig) -> Self {
        Self {
            config,
            lock_x: None,
            lock_y: None,
            integral_x: 0.0,
            integral_y: 0.0,
            is_active: false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn lock_target(&mut self, x: f64, y: f64) {
        self.lock_x = Some(x);
        self.lock_y = Some(y);
        self.integral_x = 0.0;
        self.integral_y = 0.0;
        self.is_active = true;
    }

    pub fn stop(&mut self) {
        self.is_active = false;
        self.lock_x = None;
        self.lock_y = None;
    }

    /// Evaluates guide error from current star centroid and computes pulse corrections.
    pub fn update(&mut self, current_x: f64, current_y: f64, dt_sec: f64) -> GuideCorrection {
        if !self.is_active {
            return GuideCorrection {
                dx_pixels: 0.0,
                dy_pixels: 0.0,
                pan_pulse_ms: 0,
                tilt_pulse_ms: 0,
                correction_active: false,
            };
        }

        let ref_x = self.lock_x.unwrap_or(current_x);
        let ref_y = self.lock_y.unwrap_or(current_y);

        let dx = current_x - ref_x;
        let dy = current_y - ref_y;

        let dt = dt_sec.clamp(0.01, 10.0);

        // Accumulate integrals with anti-windup clamp
        self.integral_x = (self.integral_x + dx * dt).clamp(-10.0, 10.0);
        self.integral_y = (self.integral_y + dy * dt).clamp(-10.0, 10.0);

        let pan_pulse_ms = if dx.abs() > self.config.deadband_pixels {
            let p = self.config.kp * dx + self.config.ki * self.integral_x;
            let clamped = p.clamp(-(self.config.max_pulse_ms as f64), self.config.max_pulse_ms as f64);
            if clamped.abs() >= self.config.min_pulse_ms as f64 { clamped.round() as i32 } else { 0 }
        } else {
            0
        };

        let tilt_pulse_ms = if dy.abs() > self.config.deadband_pixels {
            let p = self.config.kp * dy + self.config.ki * self.integral_y;
            let clamped = p.clamp(-(self.config.max_pulse_ms as f64), self.config.max_pulse_ms as f64);
            if clamped.abs() >= self.config.min_pulse_ms as f64 { clamped.round() as i32 } else { 0 }
        } else {
            0
        };

        GuideCorrection {
            dx_pixels: dx,
            dy_pixels: dy,
            pan_pulse_ms,
            tilt_pulse_ms,
            correction_active: pan_pulse_ms != 0 || tilt_pulse_ms != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autoguider_correction_and_deadband() {
        let mut guider = AutoGuider::new(GuideConfig::default());
        guider.lock_target(100.0, 100.0);

        // Within deadband (0.1 pixels) -> 0 pulse
        let c1 = guider.update(100.05, 100.05, 1.0);
        assert_eq!(c1.pan_pulse_ms, 0);
        assert_eq!(c1.tilt_pulse_ms, 0);

        // Drift +1.0 pixel -> Positive pulse
        let c2 = guider.update(101.0, 100.0, 1.0);
        assert!(c2.pan_pulse_ms > 0);
        assert_eq!(c2.tilt_pulse_ms, 0);
    }
}
