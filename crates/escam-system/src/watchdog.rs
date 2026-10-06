//! # Linux Hardware Watchdog Feeder (/dev/watchdog)
//!
//! Feeds the hardware watchdog timer periodically to prevent spontaneous reboots.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};
use thiserror::Error;

pub const WDT_MAGIC_CLOSE: &[u8] = b"V";

#[derive(Error, Debug)]
pub enum WatchdogError {
    #[error("Failed to access /dev/watchdog: {0}")]
    Io(#[from] std::io::Error),
}

pub trait WatchdogDevice: Send + Sync {
    fn feed(&mut self) -> Result<(), WatchdogError>;
    fn disarm(&mut self) -> Result<(), WatchdogError>;
}

pub struct LinuxWatchdog {
    file: File,
    disarmed: bool,
}

impl LinuxWatchdog {
    pub fn open() -> Result<Self, WatchdogError> {
        Self::open_path("/dev/watchdog")
    }

    pub fn open_path(path: &str) -> Result<Self, WatchdogError> {
        let file = OpenOptions::new().write(true).open(path)?;
        Ok(Self {
            file,
            disarmed: false,
        })
    }
}

impl WatchdogDevice for LinuxWatchdog {
    fn feed(&mut self) -> Result<(), WatchdogError> {
        // Writing any byte feeds the Linux watchdog
        self.file.write_all(b"\0")?;
        self.file.flush()?;
        Ok(())
    }

    fn disarm(&mut self) -> Result<(), WatchdogError> {
        if !self.disarmed {
            self.file.write_all(WDT_MAGIC_CLOSE)?;
            self.file.flush()?;
            self.disarmed = true;
        }
        Ok(())
    }
}

impl Drop for LinuxWatchdog {
    fn drop(&mut self) {
        let _ = self.disarm();
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockWatchdog {
    pub feed_count: Arc<Mutex<usize>>,
    pub disarmed: Arc<Mutex<bool>>,
}

impl MockWatchdog {
    pub fn new() -> Self {
        Self {
            feed_count: Arc::new(Mutex::new(0)),
            disarmed: Arc::new(Mutex::new(false)),
        }
    }
}

impl WatchdogDevice for MockWatchdog {
    fn feed(&mut self) -> Result<(), WatchdogError> {
        *self.feed_count.lock().unwrap() += 1;
        Ok(())
    }

    fn disarm(&mut self) -> Result<(), WatchdogError> {
        *self.disarmed.lock().unwrap() = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_watchdog_feeds() {
        let mut wd = MockWatchdog::new();
        assert_eq!(*wd.feed_count.lock().unwrap(), 0);
        assert!(!*wd.disarmed.lock().unwrap());

        wd.feed().unwrap();
        wd.feed().unwrap();
        assert_eq!(*wd.feed_count.lock().unwrap(), 2);

        wd.disarm().unwrap();
        assert!(*wd.disarmed.lock().unwrap());
    }
}
