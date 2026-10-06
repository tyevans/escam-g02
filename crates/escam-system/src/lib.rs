//! # ESCAM System Lifecycle & Vendor Ejection Crate
//!
//! Provides hardware watchdog feeding (/dev/watchdog), startup init coordination,
//! telemetry reporting, and vendor software ejection scripts.

pub mod init;
pub mod telemetry;
pub mod watchdog;

pub use init::EjectionManager;
pub use telemetry::{ProcessMemory, TelemetryAuditor, MAX_ALLOWED_RSS_KB};
pub use watchdog::{MockWatchdog, WatchdogDevice};
