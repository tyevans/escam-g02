//! # GPIO & IR-Cut Solenoid Driver (/dev/gkio)
//!
//! Controls general purpose I/O lines and specifically drives the UTC BA6208L
//! H-bridge solenoid to toggle the physical IR-cut filter.

use escam_core::IrCutMode;
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use thiserror::Error;

/// Reverse-engineered ioctl commands for gkio.ko
pub const GKIO_IOCTL_SET_VALUE: u32 = 0xC004_6200;
pub const GKIO_IOCTL_GET_VALUE: u32 = 0xC004_6201;

/// Hardware GPIO Pin Numbers for the UTC BA6208L H-Bridge and IR Illumination
pub const GPIO_IRCUT_DAY_FORWARD: u32 = 14;
pub const GPIO_IRCUT_NIGHT_REVERSE: u32 = 17;
pub const GPIO_IRLED: u32 = 10;

/// Solenoid activation pulse duration (100ms as determined by vendor disassembly)
pub const IRCUT_PULSE_DURATION: Duration = Duration::from_millis(100);

#[derive(Error, Debug)]
pub enum GpioError {
    #[error("Failed to open /dev/gkio: {0}")]
    OpenFailed(#[from] std::io::Error),
    #[error("GPIO ioctl failed on pin {0} (cmd 0x{1:08X}): {2}")]
    IoctlFailed(u32, u32, std::io::Error),
}

/// C ABI representation matching `struct { u32 pin; u32 val; }` (8 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GpioVal {
    pub pin: u32,
    pub val: u32,
}

pub trait GpioDevice: Send + Sync {
    fn set_value(&self, pin: u32, val: u32) -> Result<(), GpioError>;
    fn get_value(&self, pin: u32) -> Result<u32, GpioError>;
}

impl<T: GpioDevice + ?Sized> GpioDevice for Arc<T> {
    fn set_value(&self, pin: u32, val: u32) -> Result<(), GpioError> {
        (**self).set_value(pin, val)
    }
    fn get_value(&self, pin: u32) -> Result<u32, GpioError> {
        (**self).get_value(pin)
    }
}

pub struct LinuxGpioDevice {
    file: File,
}

impl LinuxGpioDevice {
    pub fn open() -> Result<Self, GpioError> {
        Self::open_path("/dev/gkio")
    }

    pub fn open_path(path: &str) -> Result<Self, GpioError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(path)?;
        Ok(Self { file })
    }
}

impl GpioDevice for LinuxGpioDevice {
    fn set_value(&self, pin: u32, val: u32) -> Result<(), GpioError> {
        let fd = self.file.as_raw_fd();
        let mut req = GpioVal { pin, val };
        let ret = unsafe { libc::ioctl(fd, GKIO_IOCTL_SET_VALUE as _, &mut req) };
        if ret < 0 {
            Err(GpioError::IoctlFailed(pin, GKIO_IOCTL_SET_VALUE, std::io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }

    fn get_value(&self, pin: u32) -> Result<u32, GpioError> {
        let fd = self.file.as_raw_fd();
        let mut req = GpioVal { pin, val: 0 };
        let ret = unsafe { libc::ioctl(fd, GKIO_IOCTL_GET_VALUE as _, &mut req) };
        if ret < 0 {
            Err(GpioError::IoctlFailed(pin, GKIO_IOCTL_GET_VALUE, std::io::Error::last_os_error()))
        } else {
            Ok(req.val)
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockGpioDevice {
    pub pin_states: Arc<Mutex<std::collections::HashMap<u32, u32>>>,
    pub pulse_history: Arc<Mutex<Vec<(u32, u32)>>>,
}

impl MockGpioDevice {
    pub fn new() -> Self {
        Self {
            pin_states: Arc::new(Mutex::new(std::collections::HashMap::new())),
            pulse_history: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl GpioDevice for MockGpioDevice {
    fn set_value(&self, pin: u32, val: u32) -> Result<(), GpioError> {
        self.pin_states.lock().unwrap().insert(pin, val);
        self.pulse_history.lock().unwrap().push((pin, val));
        Ok(())
    }

    fn get_value(&self, pin: u32) -> Result<u32, GpioError> {
        Ok(*self.pin_states.lock().unwrap().get(&pin).unwrap_or(&0))
    }
}

/// High-level controller for the mechanical IR-Cut filter solenoid and IR LEDs
pub struct IrCutController<D: GpioDevice> {
    gpio: D,
    current_mode: Mutex<IrCutMode>,
    irled_state: Mutex<bool>,
}

impl<D: GpioDevice> IrCutController<D> {
    pub fn new(gpio: D) -> Self {
        Self {
            gpio,
            current_mode: Mutex::new(IrCutMode::Day),
            irled_state: Mutex::new(false),
        }
    }

    pub fn mode(&self) -> IrCutMode {
        *self.current_mode.lock().unwrap()
    }

    pub fn ir_led(&self) -> bool {
        *self.irled_state.lock().unwrap()
    }

    pub fn set_ir_led(&self, enable: bool) -> Result<(), GpioError> {
        let val = if enable { 1 } else { 0 };
        // 1. Write via kernel driver /dev/gkio
        let _ = self.gpio.set_value(GPIO_IRLED, val);
        // 2. Also ensure direct sysfs node is updated if available
        let _ = std::fs::write("/sys/class/gpio/gpio10/value", if enable { "1" } else { "0" });
        *self.irled_state.lock().unwrap() = enable;
        Ok(())
    }

    pub fn set_mode(&self, mode: IrCutMode) -> Result<(), GpioError> {
        let active_pin = match mode {
            IrCutMode::Day => GPIO_IRCUT_DAY_FORWARD,
            IrCutMode::Night => GPIO_IRCUT_NIGHT_REVERSE,
        };

        // 1. Assert active high pulse to trigger H-bridge solenoid
        self.gpio.set_value(active_pin, 1)?;
        // 2. Hold pulse for 100ms
        sleep(IRCUT_PULSE_DURATION);
        // 3. De-assert back to 0 to prevent coil burnout
        self.gpio.set_value(active_pin, 0)?;

        *self.current_mode.lock().unwrap() = mode;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpio_ioctl_constants() {
        assert_eq!(GKIO_IOCTL_SET_VALUE, 0xC004_6200);
        assert_eq!(GKIO_IOCTL_GET_VALUE, 0xC004_6201);
        assert_eq!(GPIO_IRCUT_DAY_FORWARD, 14);
        assert_eq!(GPIO_IRCUT_NIGHT_REVERSE, 17);
    }

    #[test]
    fn test_ircut_controller_pulses() {
        let mock = MockGpioDevice::new();
        let controller = IrCutController::new(mock.clone());

        assert_eq!(controller.mode(), IrCutMode::Day);

        controller.set_mode(IrCutMode::Night).unwrap();
        assert_eq!(controller.mode(), IrCutMode::Night);

        let history = mock.pulse_history.lock().unwrap().clone();
        assert_eq!(history, vec![(17, 1), (17, 0)]);

        let pin_17_val = mock.get_value(17).unwrap();
        assert_eq!(pin_17_val, 0);
    }

    #[test]
    fn test_irled_toggle() {
        let mock = MockGpioDevice::new();
        let controller = IrCutController::new(mock.clone());

        assert!(!controller.ir_led());
        controller.set_ir_led(true).unwrap();
        assert!(controller.ir_led());
        assert_eq!(mock.get_value(GPIO_IRLED).unwrap(), 1);

        controller.set_ir_led(false).unwrap();
        assert!(!controller.ir_led());
        assert_eq!(mock.get_value(GPIO_IRLED).unwrap(), 0);
    }
}
