//! # ESCAM PTZ Trajectory and Motion Controller Crate
//!
//! Provides jerk-limited S-curve motion profiling, virtual joystick velocity
//! mapping, soft endstop collision prevention, and smooth stepper motor control.

pub mod controller;
pub mod scurve;

pub use controller::{PtzController, PtzState};
pub use scurve::{ScurveGenerator, ScurveProfile};
