//! # ESCAM PTZ Trajectory and Motion Controller Crate
//!
//! Provides jerk-limited S-curve motion profiling, virtual joystick velocity
//! mapping, soft endstop collision prevention, smooth stepper motor control,
//! celestial target catalogs, gear backlash compensation, and autoguiding.

pub mod autoguide;
pub mod backlash;
pub mod catalog;
pub mod controller;
pub mod interleaver;
pub mod scurve;
pub mod sidereal;

pub use autoguide::{AutoGuider, GuideConfig, GuideCorrection};
pub use backlash::{BacklashCompensator, BacklashConfig, MotorDirection};
pub use catalog::{CelestialCatalog, CelestialTarget, TargetHorizontalPosition};
pub use controller::{PtzController, PtzError, PtzState, DEFAULT_DEADBAND, DEFAULT_SLICE_DURATION};
pub use interleaver::{AxisChoice, BresenhamInterleaver};
pub use scurve::{ScurveGenerator, ScurveProfile};
pub use sidereal::{Hemisphere, SiderealTracker, TrackingRate};
