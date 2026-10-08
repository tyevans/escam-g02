//! Stepper Motor Mechanical Backlash Compensation and Autonomous Soft-Homing.
//!
//! Compensates for gear train hysteresis by injecting pre-tensioning micro-steps
//! when reversing motor direction, and provides soft-homing calibration.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorDirection {
    Forward,
    Reverse,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacklashConfig {
    pub pan_backlash_steps: u16,
    pub tilt_backlash_steps: u16,
    pub pan_max_steps: i32,
    pub tilt_max_steps: i32,
}

impl Default for BacklashConfig {
    fn default() -> Self {
        Self {
            pan_backlash_steps: 8,
            tilt_backlash_steps: 6,
            pan_max_steps: 520,
            tilt_max_steps: 260,
        }
    }
}

pub struct BacklashCompensator {
    config: BacklashConfig,
    last_pan_dir: MotorDirection,
    last_tilt_dir: MotorDirection,
    current_pan_step: i32,
    current_tilt_step: i32,
    is_homed: bool,
}

impl BacklashCompensator {
    pub fn new(config: BacklashConfig) -> Self {
        Self {
            config,
            last_pan_dir: MotorDirection::Stopped,
            last_tilt_dir: MotorDirection::Stopped,
            current_pan_step: 260, // Centered default
            current_tilt_step: 130, // Centered default
            is_homed: false,
        }
    }

    pub fn config(&self) -> &BacklashConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: BacklashConfig) {
        self.config = config;
    }

    pub fn is_homed(&self) -> bool {
        self.is_homed
    }

    pub fn mark_homed(&mut self, pan_center: i32, tilt_center: i32) {
        self.current_pan_step = pan_center;
        self.current_tilt_step = tilt_center;
        self.is_homed = true;
    }

    /// Evaluates Pan step motion with backlash compensation.
    /// Returns (effective_steps_to_pulse, new_logical_pos).
    pub fn plan_pan_motion(&mut self, target_step: i32) -> (i32, i32) {
        let clamped_target = target_step.clamp(0, self.config.pan_max_steps);
        let delta = clamped_target - self.current_pan_step;

        if delta == 0 {
            return (0, self.current_pan_step);
        }

        let new_dir = if delta > 0 { MotorDirection::Forward } else { MotorDirection::Reverse };
        let mut pulse_steps = delta;

        // If reversing direction from previous motion, inject backlash compensation
        if self.last_pan_dir != MotorDirection::Stopped && self.last_pan_dir != new_dir {
            let comp = self.config.pan_backlash_steps as i32;
            pulse_steps += if delta > 0 { comp } else { -comp };
        }

        self.last_pan_dir = new_dir;
        self.current_pan_step = clamped_target;
        (pulse_steps, clamped_target)
    }

    /// Evaluates Tilt step motion with backlash compensation.
    pub fn plan_tilt_motion(&mut self, target_step: i32) -> (i32, i32) {
        let clamped_target = target_step.clamp(0, self.config.tilt_max_steps);
        let delta = clamped_target - self.current_tilt_step;

        if delta == 0 {
            return (0, self.current_tilt_step);
        }

        let new_dir = if delta > 0 { MotorDirection::Forward } else { MotorDirection::Reverse };
        let mut pulse_steps = delta;

        if self.last_tilt_dir != MotorDirection::Stopped && self.last_tilt_dir != new_dir {
            let comp = self.config.tilt_backlash_steps as i32;
            pulse_steps += if delta > 0 { comp } else { -comp };
        }

        self.last_tilt_dir = new_dir;
        self.current_tilt_step = clamped_target;
        (pulse_steps, clamped_target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backlash_step_injection_on_reversal() {
        let mut comp = BacklashCompensator::new(BacklashConfig::default());

        // Step forward from 260 to 270 (delta = +10, no reversal)
        let (pulse1, _) = comp.plan_pan_motion(270);
        assert_eq!(pulse1, 10);

        // Continue forward from 270 to 280 (delta = +10, same dir)
        let (pulse2, _) = comp.plan_pan_motion(280);
        assert_eq!(pulse2, 10);

        // Reverse from 280 to 270 (delta = -10, reversal!)
        // Injects 8 backlash steps in reverse direction -> -18
        let (pulse3, pos3) = comp.plan_pan_motion(270);
        assert_eq!(pulse3, -18);
        assert_eq!(pos3, 270);
    }
}
