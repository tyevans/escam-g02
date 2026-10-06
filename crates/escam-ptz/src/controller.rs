//! # PTZ Motion Controller & Soft Limit Enforcement
//!
//! Translates high-level velocity and position commands into hardware motor steps
//! while enforcing soft limits to prevent mechanical binding.
//! Provides state deduplication and Bresenham time-sliced interleaving for buttery
//! smooth diagonal pan/tilt motion without kernel motor driver starvation.

use crate::interleaver::{AxisChoice, BresenhamInterleaver};
use escam_core::CameraConfig;
use escam_driver::{MotorDevice, MotorRun};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::Mutex;

/// Default deadband threshold around (0, 0) normalized joystick inputs.
pub const DEFAULT_DEADBAND: f32 = 0.12;

/// Default time slice duration for interleaved diagonal motion bursts (~50ms).
pub const DEFAULT_SLICE_DURATION: Duration = Duration::from_millis(50);

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

#[derive(Debug, Clone, Copy, PartialEq)]
struct DiagonalCommand {
    pan_dir: i32,
    tilt_dir: i32,
    x: f32,
    y: f32,
}

struct PtzInner {
    state: PtzState,
    active_interleave: Option<tokio::task::JoinHandle<()>>,
    interleave_tx: Option<tokio::sync::watch::Sender<DiagonalCommand>>,
    last_hw_cmd: MotorRun,
}

pub struct PtzController<M: MotorDevice + 'static> {
    motor: Arc<M>,
    config: CameraConfig,
    inner: Arc<Mutex<PtzInner>>,
    deadband: f32,
    slice_duration: Duration,
}

impl<M: MotorDevice + 'static> PtzController<M> {
    pub fn new(motor: M, config: CameraConfig) -> Self {
        Self::with_options(motor, config, DEFAULT_DEADBAND, DEFAULT_SLICE_DURATION)
    }

    pub fn with_options(
        motor: M,
        config: CameraConfig,
        deadband: f32,
        slice_duration: Duration,
    ) -> Self {
        let initial_state = PtzState {
            pan_deg: (config.pan_min_deg + config.pan_max_deg) / 2.0,
            tilt_deg: (config.tilt_min_deg + config.tilt_max_deg) / 2.0,
            is_moving: false,
        };

        let inner = PtzInner {
            state: initial_state,
            active_interleave: None,
            interleave_tx: None,
            last_hw_cmd: MotorRun::default(),
        };

        Self {
            motor: Arc::new(motor),
            config,
            inner: Arc::new(Mutex::new(inner)),
            deadband,
            slice_duration,
        }
    }

    pub async fn get_state(&self) -> PtzState {
        self.inner.lock().await.state
    }

    pub fn config(&self) -> &CameraConfig {
        &self.config
    }

    pub fn deadband(&self) -> f32 {
        self.deadband
    }

    pub fn slice_duration(&self) -> Duration {
        self.slice_duration
    }

    pub async fn set_position(&self, pan_deg: f32, tilt_deg: f32) -> Result<(), PtzError> {
        if pan_deg < self.config.pan_min_deg
            || pan_deg > self.config.pan_max_deg
            || tilt_deg < self.config.tilt_min_deg
            || tilt_deg > self.config.tilt_max_deg
        {
            return Err(PtzError::OutOfBounds(pan_deg, tilt_deg));
        }
        let mut inner = self.inner.lock().await;
        inner.state.pan_deg = pan_deg;
        inner.state.tilt_deg = tilt_deg;
        Ok(())
    }

    /// Drives continuous motion using virtual joystick input (x, y) where x, y in [-1.0, 1.0].
    /// Applies deadband filtering, soft limits, state deduplication, and interleaved diagonal motion.
    pub async fn drive_joystick(&self, x: f32, y: f32) -> Result<(), PtzError> {
        let effective_x = if x.abs() >= self.deadband { x } else { 0.0 };
        let effective_y = if y.abs() >= self.deadband { y } else { 0.0 };

        let mut pan_dir = if effective_x > 0.0 {
            3 // Right / CW
        } else if effective_x < 0.0 {
            4 // Left / CCW
        } else {
            0
        };

        let mut tilt_dir = if effective_y > 0.0 {
            1 // Up
        } else if effective_y < 0.0 {
            2 // Down
        } else {
            0
        };

        let mut inner = self.inner.lock().await;

        // Enforce soft limits against current coordinates
        if pan_dir == 3 && inner.state.pan_deg >= self.config.pan_max_deg {
            pan_dir = 0;
        } else if pan_dir == 4 && inner.state.pan_deg <= self.config.pan_min_deg {
            pan_dir = 0;
        }

        if tilt_dir == 1 && inner.state.tilt_deg >= self.config.tilt_max_deg {
            tilt_dir = 0;
        } else if tilt_dir == 2 && inner.state.tilt_deg <= self.config.tilt_min_deg {
            tilt_dir = 0;
        }

        if pan_dir == 0 && tilt_dir == 0 {
            // Graceful stop / Centered joystick
            if let Some(handle) = inner.active_interleave.take() {
                handle.abort();
            }
            inner.interleave_tx = None;

            if inner.last_hw_cmd != MotorRun::new(0, 0) {
                self.motor.stop()?;
                inner.last_hw_cmd = MotorRun::new(0, 0);
            }
            inner.state.is_moving = false;
        } else if pan_dir != 0 && tilt_dir != 0 {
            // Diagonal motion: Interleave axes with Bresenham time slices
            let diag_cmd = DiagonalCommand {
                pan_dir,
                tilt_dir,
                x: effective_x,
                y: effective_y,
            };

            if let Some(ref tx) = inner.interleave_tx {
                let _ = tx.send(diag_cmd);
            } else {
                let mut interleaver = BresenhamInterleaver::new(effective_x, effective_y);
                let first_axis = interleaver.initial_axis();
                let first_cmd = match first_axis {
                    AxisChoice::Pan => MotorRun::new(pan_dir, 0),
                    AxisChoice::Tilt => MotorRun::new(0, tilt_dir),
                };

                if inner.last_hw_cmd != first_cmd {
                    self.motor.run(first_cmd)?;
                    inner.last_hw_cmd = first_cmd;
                }

                let (tx, rx) = tokio::sync::watch::channel(diag_cmd);
                inner.interleave_tx = Some(tx);

                let motor_clone = self.motor.clone();
                let slice_dur = self.slice_duration;

                let handle = tokio::spawn(async move {
                    run_interleaved_loop(motor_clone, rx, interleaver, slice_dur, first_cmd).await;
                });
                inner.active_interleave = Some(handle);
            }
            inner.state.is_moving = true;
        } else {
            // Single-axis motion (pan only or tilt only)
            if let Some(handle) = inner.active_interleave.take() {
                handle.abort();
            }
            inner.interleave_tx = None;

            let target_cmd = MotorRun::new(pan_dir, tilt_dir);
            if inner.last_hw_cmd != target_cmd {
                self.motor.run(target_cmd)?;
                inner.last_hw_cmd = target_cmd;
            }
            inner.state.is_moving = true;
        }

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), PtzError> {
        let mut inner = self.inner.lock().await;
        if let Some(handle) = inner.active_interleave.take() {
            handle.abort();
        }
        inner.interleave_tx = None;

        if inner.last_hw_cmd != MotorRun::new(0, 0) {
            self.motor.stop()?;
            inner.last_hw_cmd = MotorRun::new(0, 0);
        }

        inner.state.is_moving = false;
        Ok(())
    }
}

async fn run_interleaved_loop<M: MotorDevice + 'static>(
    motor: Arc<M>,
    mut rx: tokio::sync::watch::Receiver<DiagonalCommand>,
    mut interleaver: BresenhamInterleaver,
    slice_duration: Duration,
    initial_cmd: MotorRun,
) {
    let mut last_cmd = initial_cmd;
    loop {
        tokio::select! {
            _ = tokio::time::sleep(slice_duration) => {
                let cmd = *rx.borrow();
                interleaver.update_weights(cmd.x, cmd.y);
                let axis = interleaver.next_axis();
                let next_cmd = match axis {
                    AxisChoice::Pan => MotorRun::new(cmd.pan_dir, 0),
                    AxisChoice::Tilt => MotorRun::new(0, cmd.tilt_dir),
                };

                if last_cmd != next_cmd {
                    let _ = motor.run(next_cmd);
                    last_cmd = next_cmd;
                }
            }
            res = rx.changed() => {
                if res.is_err() {
                    break;
                }
            }
        }
    }
}

impl<M: MotorDevice + 'static> Drop for PtzController<M> {
    fn drop(&mut self) {
        if let Ok(mut inner) = self.inner.try_lock() {
            if let Some(handle) = inner.active_interleave.take() {
                handle.abort();
            }
            inner.interleave_tx = None;
        }
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
        assert_eq!(*mock_motor.last_run.lock().unwrap(), MotorRun::new(3, 0));

        controller.stop().await.unwrap();
        assert_eq!(*mock_motor.last_run.lock().unwrap(), MotorRun::new(0, 0));
    }
}

