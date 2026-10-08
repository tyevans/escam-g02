//! Userspace Direct-GPIO Stepper Motor Driver.
//!
//! Governed by ADR-0026 and TASK-0040.
//! Provides direct-phase stepping for the Pan and Tilt stepper motors
//! through standard Linux GPIO lines (sysfs / gpiochip), completely eliminating
//! the need for proprietary vendor kernel modules (motor.ko).

pub const PAN_PINS: [u32; 4] = [0, 1, 2, 3];
pub const TILT_PINS: [u32; 4] = [4, 5, 6, 7];

/// 8-step half-stepping sequence table for 28BYJ-48 style unipolar steppers
/// driven by JULN2803AG Darlington array.
pub const HALF_STEP_TABLE: [u8; 8] = [
    0x01, // 0001
    0x03, // 0011
    0x02, // 0010
    0x06, // 0110
    0x04, // 0100
    0x0C, // 1100
    0x08, // 1000
    0x09, // 1001
];

#[derive(Debug, Clone)]
pub struct UserspaceMotorController {
    pan_index: usize,
    tilt_index: usize,
    pan_active: bool,
    tilt_active: bool,
}

impl Default for UserspaceMotorController {
    fn default() -> Self {
        Self::new()
    }
}

impl UserspaceMotorController {
    pub fn new() -> Self {
        Self {
            pan_index: 0,
            tilt_index: 0,
            pan_active: false,
            tilt_active: false,
        }
    }

    /// Advances the Pan stepper by one half-step.
    /// Returns the 4-bit coil bitmask for pins [0..3].
    pub fn step_pan(&mut self, clockwise: bool) -> u8 {
        self.pan_active = true;
        if clockwise {
            self.pan_index = (self.pan_index + 1) & 7;
        } else {
            self.pan_index = (self.pan_index + 7) & 7;
        }
        HALF_STEP_TABLE[self.pan_index]
    }

    /// Advances the Tilt stepper by one half-step.
    /// Returns the 4-bit coil bitmask for pins [4..7].
    pub fn step_tilt(&mut self, up: bool) -> u8 {
        self.tilt_active = true;
        if up {
            self.tilt_index = (self.tilt_index + 1) & 7;
        } else {
            self.tilt_index = (self.tilt_index + 7) & 7;
        }
        HALF_STEP_TABLE[self.tilt_index]
    }

    /// De-energizes all motor coils to prevent coil heating and conserve power.
    pub fn release_coils(&mut self) -> (u8, u8) {
        self.pan_active = false;
        self.tilt_active = false;
        (0x00, 0x00)
    }

    /// Current coil bitmasks (pan_coils, tilt_coils).
    pub fn current_coils(&self) -> (u8, u8) {
        let pan = if self.pan_active {
            HALF_STEP_TABLE[self.pan_index]
        } else {
            0
        };
        let tilt = if self.tilt_active {
            HALF_STEP_TABLE[self.tilt_index]
        } else {
            0
        };
        (pan, tilt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_step_sequence_progression() {
        let mut ctrl = UserspaceMotorController::new();
        let mask1 = ctrl.step_pan(true);
        assert_eq!(mask1, HALF_STEP_TABLE[1]);

        let mask2 = ctrl.step_pan(true);
        assert_eq!(mask2, HALF_STEP_TABLE[2]);

        // Reversal
        let mask_rev = ctrl.step_pan(false);
        assert_eq!(mask_rev, HALF_STEP_TABLE[1]);
    }

    #[test]
    fn test_coil_release_de_energizes() {
        let mut ctrl = UserspaceMotorController::new();
        ctrl.step_pan(true);
        ctrl.step_tilt(true);
        assert_ne!(ctrl.current_coils(), (0, 0));

        let (pan, tilt) = ctrl.release_coils();
        assert_eq!(pan, 0);
        assert_eq!(tilt, 0);
        assert_eq!(ctrl.current_coils(), (0, 0));
    }
}
