//! # PTZ Controller Integration Tests
//!
//! Verifies state deduplication, Bresenham interleaved diagonal motion,
//! deadband filtering, and soft limit enforcement through public frontdoors.

use escam_core::CameraConfig;
use escam_driver::{MockMotorDevice, MotorDevice, MotorRun};
use escam_ptz::{PtzController, PtzError, DEFAULT_DEADBAND};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Default, Clone)]
struct CountingMockMotor {
    inner: MockMotorDevice,
    run_calls: Arc<std::sync::Mutex<usize>>,
    stop_calls: Arc<std::sync::Mutex<usize>>,
}

impl MotorDevice for CountingMockMotor {
    fn stop(&self) -> Result<(), escam_driver::motor::MotorError> {
        *self.stop_calls.lock().unwrap() += 1;
        self.inner.stop()
    }

    fn run(&self, run_cmd: MotorRun) -> Result<(), escam_driver::motor::MotorError> {
        *self.run_calls.lock().unwrap() += 1;
        self.inner.run(run_cmd)
    }

    fn set_speed(&self, speed: u32) -> Result<(), escam_driver::motor::MotorError> {
        self.inner.set_speed(speed)
    }

    fn auto_check_home(&self) -> Result<(), escam_driver::motor::MotorError> {
        self.inner.auto_check_home()
    }

    fn get_position(&self) -> Result<(i32, i32), escam_driver::motor::MotorError> {
        self.inner.get_position()
    }
}

#[tokio::test]
async fn test_state_deduplication_single_axis() {
    let motor = CountingMockMotor::default();
    let controller = PtzController::new(motor.clone(), CameraConfig::default());

    // Repeated calls in the same direction should invoke run() only once
    for _ in 0..5 {
        controller.drive_joystick(0.8, 0.0).await.unwrap();
    }
    assert_eq!(*motor.run_calls.lock().unwrap(), 1);

    // Direction change to Left should invoke run() a second time
    controller.drive_joystick(-0.8, 0.0).await.unwrap();
    assert_eq!(*motor.run_calls.lock().unwrap(), 2);
    assert_eq!(*motor.inner.last_run.lock().unwrap(), MotorRun::new(4, 0));

    // Multiple stop calls should deduplicate to a single stop()
    for _ in 0..5 {
        controller.stop().await.unwrap();
    }
    assert_eq!(*motor.stop_calls.lock().unwrap(), 1);
    assert_eq!(*motor.inner.last_run.lock().unwrap(), MotorRun::new(0, 0));
}

#[tokio::test]
async fn test_deadband_filtering() {
    let motor = CountingMockMotor::default();
    let controller = PtzController::new(motor.clone(), CameraConfig::default());

    // Small deviations below 0.12 deadband threshold should stay stopped
    controller.drive_joystick(0.08, -0.05).await.unwrap();
    assert_eq!(*motor.run_calls.lock().unwrap(), 0);
    assert_eq!(controller.get_state().await.is_moving, false);

    // Pushing Pan beyond deadband activates motor
    controller.drive_joystick(0.25, 0.05).await.unwrap();
    assert_eq!(*motor.run_calls.lock().unwrap(), 1);
    assert_eq!(*motor.inner.last_run.lock().unwrap(), MotorRun::new(3, 0));

    // Returning to center gracefully stops
    controller.drive_joystick(0.0, 0.0).await.unwrap();
    assert_eq!(*motor.stop_calls.lock().unwrap(), 1);
    assert_eq!(controller.get_state().await.is_moving, false);
}

#[tokio::test]
async fn test_soft_limits_clamp_motion() {
    let motor = CountingMockMotor::default();
    let config = CameraConfig::default();
    let controller = PtzController::new(motor.clone(), config.clone());

    // Set position to upper soft limits
    controller
        .set_position(config.pan_max_deg, config.tilt_max_deg)
        .await
        .unwrap();

    // Driving right/up against soft limits is clamped to zero
    controller.drive_joystick(0.8, 0.8).await.unwrap();
    assert_eq!(*motor.run_calls.lock().unwrap(), 0);
    assert_eq!(controller.get_state().await.is_moving, false);

    // Driving left/down away from limits is permitted
    controller.drive_joystick(-0.8, -0.8).await.unwrap();
    assert_eq!(*motor.run_calls.lock().unwrap(), 1);
    assert_eq!(controller.get_state().await.is_moving, true);

    // Setting out of bounds produces error
    let err = controller.set_position(400.0, 50.0).await.unwrap_err();
    assert!(matches!(err, PtzError::OutOfBounds(..)));
}

#[tokio::test]
async fn test_diagonal_interleaving_alternates_axes() {
    let motor = CountingMockMotor::default();
    let config = CameraConfig::default();
    // Use 20ms slice duration for fast, deterministic unit testing
    let controller = PtzController::with_options(
        motor.clone(),
        config,
        DEFAULT_DEADBAND,
        Duration::from_millis(20),
    );

    controller.drive_joystick(0.8, 0.8).await.unwrap();
    // Slice 0: Pan initial burst
    assert_eq!(*motor.inner.last_run.lock().unwrap(), MotorRun::new(3, 0));

    // Wait ~30ms for slice 1 to alternate to Tilt
    tokio::time::sleep(Duration::from_millis(30)).await;
    assert_eq!(*motor.inner.last_run.lock().unwrap(), MotorRun::new(0, 1));

    // Wait ~25ms for slice 2 to alternate back to Pan
    tokio::time::sleep(Duration::from_millis(25)).await;
    assert_eq!(*motor.inner.last_run.lock().unwrap(), MotorRun::new(3, 0));

    // Graceful stop terminates the interleaving task
    controller.stop().await.unwrap();
    assert_eq!(*motor.inner.last_run.lock().unwrap(), MotorRun::new(0, 0));
    assert_eq!(controller.get_state().await.is_moving, false);
}
