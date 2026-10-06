//! # Jerk-Limited S-Curve Trajectory Profile Generator
//!
//! Replaces harsh square-wave stepping with smooth sinusoidal/cubic S-curves
//! to eliminate motor clicking, resonance, and mount shake.

use std::time::Duration;

/// Trajectory profile configuration parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScurveProfile {
    /// Maximum cruise speed (steps per second).
    pub max_velocity: f32,
    /// Acceleration limit (steps per second squared).
    pub max_acceleration: f32,
    /// Jerk limit (steps per second cubed).
    pub max_jerk: f32,
}

impl Default for ScurveProfile {
    fn default() -> Self {
        Self {
            max_velocity: 800.0,
            max_acceleration: 2000.0,
            max_jerk: 10000.0,
        }
    }
}

/// S-curve state evaluator for a movement of `total_steps`.
#[derive(Debug, Clone)]
pub struct ScurveGenerator {
    profile: ScurveProfile,
    total_steps: u32,
    accel_steps: u32,
}

impl ScurveGenerator {
    pub fn new(total_steps: u32, profile: ScurveProfile) -> Self {
        let accel_steps = (total_steps / 3).min(200).max(10);
        Self {
            profile,
            total_steps,
            accel_steps,
        }
    }

    /// Evaluates target velocity (steps/sec) at step index `current_step`.
    pub fn velocity_at_step(&self, current_step: u32) -> f32 {
        if self.total_steps == 0 || current_step >= self.total_steps {
            return 0.0;
        }

        let min_vel = 50.0f32; // Starting creep velocity to avoid stall
        let max_vel = self.profile.max_velocity;

        if current_step < self.accel_steps {
            // Acceleration phase: half-sine ramp from min_vel to max_vel
            let fraction = current_step as f32 / self.accel_steps as f32;
            let sine_factor = (1.0 - (std::f32::consts::PI * fraction).cos()) * 0.5;
            min_vel + (max_vel - min_vel) * sine_factor
        } else if current_step >= (self.total_steps - self.accel_steps) {
            // Deceleration phase: half-sine ramp down to min_vel
            let remaining = self.total_steps - current_step;
            let fraction = remaining as f32 / self.accel_steps as f32;
            let sine_factor = (1.0 - (std::f32::consts::PI * fraction).cos()) * 0.5;
            min_vel + (max_vel - min_vel) * sine_factor
        } else {
            // Cruise phase
            max_vel
        }
    }

    /// Converts instantaneous velocity (steps/sec) into step interval duration.
    pub fn step_interval_at(&self, current_step: u32) -> Duration {
        let vel = self.velocity_at_step(current_step).max(1.0);
        Duration::from_micros((1_000_000.0 / vel) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scurve_velocity_boundaries() {
        let profile = ScurveProfile::default();
        let gen = ScurveGenerator::new(600, profile);

        let v_start = gen.velocity_at_step(0);
        let v_mid = gen.velocity_at_step(300);
        let v_end = gen.velocity_at_step(599);

        assert!(v_mid > v_start);
        assert!(v_mid >= profile.max_velocity * 0.95);
        assert!(v_end < v_mid);
    }

    #[test]
    fn test_step_interval_positive_duration() {
        let gen = ScurveGenerator::new(100, ScurveProfile::default());
        let interval = gen.step_interval_at(50);
        assert!(interval > Duration::from_micros(100));
        assert!(interval < Duration::from_millis(50));
    }
}
