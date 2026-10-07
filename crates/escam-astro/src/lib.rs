//! # ESCAM Astrophotography Subsystem
//!
//! Provides raw Bayer sensor frame extraction, standard astronomical FITS
//! container serialization, and embedded INDI protocol server for Ekos/NINA/Siril.

pub mod bayer;
pub mod fits;
pub mod indi;
pub mod stacker;

pub use bayer::{BayerFrame, BayerPattern};
pub use fits::FitsWriter;
pub use indi::{IndiMountAction, IndiServer, MountDirection};
pub use stacker::{BayerStacker, StackerError, StackingMode};
