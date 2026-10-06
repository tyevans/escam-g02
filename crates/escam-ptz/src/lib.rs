//! # ESCAM PTZ Trajectory and Motion Controller Crate
//!
//! Provides jerk-limited S-curve motion profiling, virtual joystick velocity
//! mapping, soft endstop collision prevention, and smooth stepper motor control.

pub mod controller;
pub mod interleaver;
pub mod scurve;
pub mod sidereal;

pub use controller::{PtzController, PtzError, PtzState, DEFAULT_DEADBAND, DEFAULT_SLICE_DURATION};
pub use interleaver::{AxisChoice, BresenhamInterleaver};
pub use scurve::{ScurveGenerator, ScurveProfile};
pub use sidereal::{Hemisphere, SiderealTracker, TrackingRate};
