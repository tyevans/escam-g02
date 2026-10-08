//! # ESCAM System Lifecycle & Vendor Ejection Crate
//!
//! Provides hardware watchdog feeding (/dev/watchdog), startup init coordination,
//! telemetry reporting, precision astronomical clock, and vendor software debloater.

pub mod clock;
pub mod debloat;
pub mod init;
pub mod telemetry;
pub mod watchdog;

pub use clock::{AstroClock, ClockStatus};
pub use debloat::{Debloater, FlashAudit};
pub use init::EjectionManager;
pub use telemetry::{ProcessMemory, TelemetryAuditor, MAX_ALLOWED_RSS_KB};
pub use watchdog::{LinuxWatchdog, MockWatchdog, WatchdogDevice};
