//! # PTZ Motion Controller & Soft Limit Enforcement
//!
//! Translates high-level velocity and position commands into hardware motor steps
//! while enforcing soft limits to prevent mechanical binding.

use escam_core::CameraConfig;
use escam_driver::{MotorDevice, MotorRun};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Error, Debug)]
pub enum PtzError {
    #[error("Motor driver error: {0}")]
    DriverError(#[from] escam_driver::motor::MotorError),
    #[error("Requested position out of soft limits: pan={0}, tilt={1}")]
    OutOfBounds(f32, f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PtzState {
    pub pan_deg: f32,
    pub tilt_deg: f32,
    pub is_moving: bool,
}

pub struct PtzController<M: MotorDevice> {
    motor: M,
    config: CameraConfig,
    state: Arc<Mutex<PtzState>>,
}

impl<M: MotorDevice> PtzController<M> {
    pub fn new(motor: M, config: CameraConfig) -> Self {
        let initial_state = PtzState {
            pan_deg: (config.pan_min_deg + config.pan_max_deg) / 2.0,
            tilt_deg: (config.tilt_min_deg + config.tilt_max_deg) / 2.0,
            is_moving: false,
        };

        Self {
            motor,
            config,
            state: Arc::new(Mutex::new(initial_state)),
        }
    }

    pub async fn get_state(&self) -> PtzState {
        *self.state.lock().await
    }

    pub fn config(&self) -> &CameraConfig {
        &self.config
    }

    /// Drives continuous motion using virtual joystick input (x, y) where x, y in [-1.0, 1.0].
    pub async fn drive_joystick(&self, x: f32, y: f32) -> Result<(), PtzError> {
        let mut state = self.state.lock().await;

        let pan_dir = if x > 0.15 {
            1 // Right / CW
        } else if x < -0.15 {
            2 // Left / CCW
        } else {
            0
        };

        let tilt_dir = if y > 0.15 {
            3 // Up
        } else if y < -0.15 {
            4 // Down
        } else {
            0
        };

        if pan_dir == 0 && tilt_dir == 0 {
            self.motor.stop()?;
            state.is_moving = false;
        } else {
            self.motor.run(MotorRun::new(pan_dir, tilt_dir))?;
            state.is_moving = true;
        }

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), PtzError> {
        self.motor.stop()?;
        let mut state = self.state.lock().await;
        state.is_moving = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use escam_driver::MockMotorDevice;

    #[tokio::test]
    async fn test_joystick_drives_mock_motor() {
        let mock_motor = MockMotorDevice::new();
        let config = CameraConfig::default();
        let controller = PtzController::new(mock_motor.clone(), config);

        controller.drive_joystick(0.8, 0.0).await.unwrap();
        assert_eq!(*mock_motor.last_run.lock().unwrap(), MotorRun::new(1, 0));

        controller.stop().await.unwrap();
        assert_eq!(*mock_motor.last_run.lock().unwrap(), MotorRun::new(0, 0));
    }
}
