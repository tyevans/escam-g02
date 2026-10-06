//! # Stepper Motor Driver Interface (/dev/motor)
//!
//! Maps out the reverse-engineered `motor.ko` character device node commands
//! controlling the 8 Darlington transistor phases for Pan and Tilt.

use escam_core::Direction;
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Reverse-engineered ioctl command codes for motor.ko
pub const MOTOR_IOCTL_STOP: u32 = 0xC004_6D00;
pub const MOTOR_IOCTL_RUN: u32 = 0xC004_6D01;
pub const MOTOR_IOCTL_CYCLE: u32 = 0xC004_6D02;
pub const MOTOR_IOCTL_AUTO_CHECK: u32 = 0xC004_6D05;
pub const MOTOR_IOCTL_SPEED: u32 = 0xC004_6D06;
pub const MOTOR_IOCTL_GET_STATUS: u32 = 0xC004_6D08;
pub const MOTOR_IOCTL_GET_POS: u32 = 0xC004_6D13;

#[derive(Error, Debug)]
pub enum MotorError {
    #[error("Failed to open motor device: {0}")]
    OpenFailed(#[from] std::io::Error),
    #[error("Motor ioctl error (cmd: 0x{0:08X}): {1}")]
    IoctlError(u32, std::io::Error),
    #[error("Invalid motor speed divisor: {0}")]
    InvalidSpeed(u32),
}

/// C ABI struct matching `struct stMotorRun` passed to MOTOR_IOCTL_RUN (8 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MotorRun {
    /// 1 = Right / CW, 2 = Left / CCW, 0 = Stop
    pub pandir: i32,
    /// 3 = Up, 4 = Down, 0 = Stop
    pub titldir: i32,
}

impl MotorRun {
    pub fn new(pandir: i32, titldir: i32) -> Self {
        Self { pandir, titldir }
    }

    pub fn from_direction(dir: Direction) -> Self {
        match dir {
            Direction::Stop => Self::new(0, 0),
            Direction::PanRight => Self::new(1, 0),
            Direction::PanLeft => Self::new(2, 0),
            Direction::TiltUp => Self::new(0, 3),
            Direction::TiltDown => Self::new(0, 4),
        }
    }
}

/// Speed control parameter (APB timer divisor)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MotorSpeed {
    pub speed: u32,
}

/// Abstract motor device interface to enable seamless mock testing on host.
pub trait MotorDevice: Send + Sync {
    fn stop(&self) -> Result<(), MotorError>;
    fn run(&self, run_cmd: MotorRun) -> Result<(), MotorError>;
    fn set_speed(&self, speed: u32) -> Result<(), MotorError>;
    fn auto_check_home(&self) -> Result<(), MotorError>;
    fn get_position(&self) -> Result<(i32, i32), MotorError>;
}

impl<T: MotorDevice + ?Sized> MotorDevice for Arc<T> {
    fn stop(&self) -> Result<(), MotorError> {
        (**self).stop()
    }
    fn run(&self, run_cmd: MotorRun) -> Result<(), MotorError> {
        (**self).run(run_cmd)
    }
    fn set_speed(&self, speed: u32) -> Result<(), MotorError> {
        (**self).set_speed(speed)
    }
    fn auto_check_home(&self) -> Result<(), MotorError> {
        (**self).auto_check_home()
    }
    fn get_position(&self) -> Result<(i32, i32), MotorError> {
        (**self).get_position()
    }
}

/// Physical Linux /dev/motor implementation using libc::ioctl
pub struct LinuxMotorDevice {
    file: File,
}

impl LinuxMotorDevice {
    pub fn open() -> Result<Self, MotorError> {
        Self::open_path("/dev/motor")
    }

    pub fn open_path(path: &str) -> Result<Self, MotorError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(path)?;
        Ok(Self { file })
    }
}

impl MotorDevice for LinuxMotorDevice {
    fn stop(&self) -> Result<(), MotorError> {
        let fd = self.file.as_raw_fd();
        let ret = unsafe { libc::ioctl(fd, MOTOR_IOCTL_STOP as _, 0) };
        if ret < 0 {
            Err(MotorError::IoctlError(MOTOR_IOCTL_STOP, std::io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }

    fn run(&self, mut run_cmd: MotorRun) -> Result<(), MotorError> {
        let fd = self.file.as_raw_fd();
        let ret = unsafe { libc::ioctl(fd, MOTOR_IOCTL_RUN as _, &mut run_cmd) };
        if ret < 0 {
            Err(MotorError::IoctlError(MOTOR_IOCTL_RUN, std::io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }

    fn set_speed(&self, speed: u32) -> Result<(), MotorError> {
        let fd = self.file.as_raw_fd();
        let mut spd = MotorSpeed { speed };
        let ret = unsafe { libc::ioctl(fd, MOTOR_IOCTL_SPEED as _, &mut spd) };
        if ret < 0 {
            Err(MotorError::IoctlError(MOTOR_IOCTL_SPEED, std::io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }

    fn auto_check_home(&self) -> Result<(), MotorError> {
        let fd = self.file.as_raw_fd();
        let ret = unsafe { libc::ioctl(fd, MOTOR_IOCTL_AUTO_CHECK as _, 0) };
        if ret < 0 {
            Err(MotorError::IoctlError(MOTOR_IOCTL_AUTO_CHECK, std::io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }

    fn get_position(&self) -> Result<(i32, i32), MotorError> {
        let fd = self.file.as_raw_fd();
        let mut pos = [0i32; 2];
        let ret = unsafe { libc::ioctl(fd, MOTOR_IOCTL_GET_POS as _, pos.as_mut_ptr()) };
        if ret < 0 {
            Err(MotorError::IoctlError(MOTOR_IOCTL_GET_POS, std::io::Error::last_os_error()))
        } else {
            Ok((pos[0], pos[1]))
        }
    }
}

/// Simulated motor device for testing without physical hardware.
#[derive(Debug, Default, Clone)]
pub struct MockMotorDevice {
    pub last_run: Arc<Mutex<MotorRun>>,
    pub is_stopped: Arc<Mutex<bool>>,
    pub current_speed: Arc<Mutex<u32>>,
    pub position: Arc<Mutex<(i32, i32)>>,
}

impl MockMotorDevice {
    pub fn new() -> Self {
        Self {
            last_run: Arc::new(Mutex::new(MotorRun::default())),
            is_stopped: Arc::new(Mutex::new(true)),
            current_speed: Arc::new(Mutex::new(100)),
            position: Arc::new(Mutex::new((0, 0))),
        }
    }
}

impl MotorDevice for MockMotorDevice {
    fn stop(&self) -> Result<(), MotorError> {
        *self.is_stopped.lock().unwrap() = true;
        *self.last_run.lock().unwrap() = MotorRun::default();
        Ok(())
    }

    fn run(&self, run_cmd: MotorRun) -> Result<(), MotorError> {
        *self.is_stopped.lock().unwrap() = false;
        *self.last_run.lock().unwrap() = run_cmd;
        Ok(())
    }

    fn set_speed(&self, speed: u32) -> Result<(), MotorError> {
        *self.current_speed.lock().unwrap() = speed;
        Ok(())
    }

    fn auto_check_home(&self) -> Result<(), MotorError> {
        *self.position.lock().unwrap() = (0, 0);
        *self.is_stopped.lock().unwrap() = true;
        Ok(())
    }

    fn get_position(&self) -> Result<(i32, i32), MotorError> {
        Ok(*self.position.lock().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioctl_constants_match_kernel_module() {
        assert_eq!(MOTOR_IOCTL_STOP, 0xC004_6D00);
        assert_eq!(MOTOR_IOCTL_RUN, 0xC004_6D01);
        assert_eq!(MOTOR_IOCTL_CYCLE, 0xC004_6D02);
        assert_eq!(MOTOR_IOCTL_AUTO_CHECK, 0xC004_6D05);
        assert_eq!(MOTOR_IOCTL_SPEED, 0xC004_6D06);
        assert_eq!(MOTOR_IOCTL_GET_STATUS, 0xC004_6D08);
        assert_eq!(MOTOR_IOCTL_GET_POS, 0xC004_6D13);
    }

    #[test]
    fn test_motor_run_c_layout_size() {
        assert_eq!(std::mem::size_of::<MotorRun>(), 8);
        assert_eq!(std::mem::align_of::<MotorRun>(), 4);
    }

    #[test]
    fn test_mock_motor_device_operations() {
        let mock = MockMotorDevice::new();
        assert!(*mock.is_stopped.lock().unwrap());

        mock.run(MotorRun::new(1, 3)).unwrap();
        assert!(!*mock.is_stopped.lock().unwrap());
        assert_eq!(*mock.last_run.lock().unwrap(), MotorRun::new(1, 3));

        mock.stop().unwrap();
        assert!(*mock.is_stopped.lock().unwrap());
        assert_eq!(*mock.last_run.lock().unwrap(), MotorRun::new(0, 0));
    }
}
