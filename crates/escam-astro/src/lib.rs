//! # ESCAM Astrophotography Subsystem
//!
//! Provides raw Bayer sensor frame extraction, standard astronomical FITS
//! container serialization, embedded INDI protocol server for Ekos/NINA/Siril,
//! live stacking, calibration engines, WCS metadata, star detection, and transient analysis.

pub mod bayer;
pub mod calibrator;
pub mod fits;
pub mod indi;
pub mod stacker;
pub mod stars;
pub mod stretch;
pub mod transient;
pub mod wcs;

pub use bayer::{BayerFrame, BayerPattern};
pub use calibrator::CalibrationEngine;
pub use fits::FitsWriter;
pub use indi::{IndiMountAction, IndiServer, MountDirection};
pub use stacker::{BayerStacker, StackerError, StackingMode};
pub use stars::{DetectedStar, FocusMetric, StarDetector};
pub use stretch::{apply_auto_stretch, calculate_histogram_stats, mtf, StretchMode};
pub use transient::{TransientDetector, TransientEvent};
pub use wcs::WcsMetadata;
