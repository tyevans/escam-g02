//! # In-Memory Bayer Multi-Frame Live Stacker
//!
//! Accumulates consecutive raw sensor frames in 32-bit buffers to increase
//! astronomical signal-to-noise ratio (SNR) for deep-sky imaging.

use crate::bayer::{BayerFrame, BayerPattern};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum StackerError {
    #[error("Frame dimensions {0}x{1} do not match stacker dimensions {2}x{3}")]
    DimensionMismatch(u32, u32, u32, u32),
    #[error("Frame Bayer pattern {0:?} does not match stacker pattern {1:?}")]
    PatternMismatch(BayerPattern, BayerPattern),
    #[error("Cannot export stacked frame: stack is empty")]
    EmptyStack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackingMode {
    /// Arithmetic mean: divides accumulated sum by number of frames.
    Average,
    /// Additive sum: accumulates signal and clamps to 16-bit range (65535).
    Additive,
}

/// Accumulates raw Bayer frames in a 32-bit buffer.
#[derive(Debug, Clone)]
pub struct BayerStacker {
    width: u32,
    height: u32,
    pattern: BayerPattern,
    mode: StackingMode,
    accumulator: Vec<u32>,
    frame_count: u32,
    total_exposure_secs: f32,
}

impl BayerStacker {
    /// Create a new stacker with target dimensions and Bayer pattern.
    pub fn new(width: u32, height: u32, pattern: BayerPattern, mode: StackingMode) -> Self {
        let total_pixels = (width * height) as usize;
        Self {
            width,
            height,
            pattern,
            mode,
            accumulator: vec![0u32; total_pixels],
            frame_count: 0,
            total_exposure_secs: 0.0,
        }
    }

    /// Number of frames currently accumulated in the stack.
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    /// Total integrated exposure time in seconds.
    pub fn total_exposure_secs(&self) -> f32 {
        self.total_exposure_secs
    }

    /// Current stacking mode.
    pub fn mode(&self) -> StackingMode {
        self.mode
    }

    /// Push a new raw Bayer frame into the accumulation buffer.
    pub fn push_frame(&mut self, frame: &BayerFrame) -> Result<(), StackerError> {
        if frame.width != self.width || frame.height != self.height {
            return Err(StackerError::DimensionMismatch(
                frame.width,
                frame.height,
                self.width,
                self.height,
            ));
        }

        if frame.pattern != self.pattern {
            return Err(StackerError::PatternMismatch(frame.pattern, self.pattern));
        }

        for (acc, &pixel) in self.accumulator.iter_mut().zip(frame.pixels.iter()) {
            *acc += pixel as u32;
        }

        self.frame_count += 1;
        self.total_exposure_secs += frame.exposure_secs;
        Ok(())
    }

    /// Reset accumulator back to empty state.
    pub fn reset(&mut self) {
        for acc in self.accumulator.iter_mut() {
            *acc = 0;
        }
        self.frame_count = 0;
        self.total_exposure_secs = 0.0;
    }

    /// Produce the final stacked 16-bit Bayer frame.
    pub fn finish(&self) -> Result<BayerFrame, StackerError> {
        if self.frame_count == 0 {
            return Err(StackerError::EmptyStack);
        }

        let mut output = BayerFrame::new(
            self.width,
            self.height,
            16,
            self.pattern,
            self.total_exposure_secs,
        );

        match self.mode {
            StackingMode::Average => {
                let count = self.frame_count as u32;
                for (out_px, &acc) in output.pixels.iter_mut().zip(self.accumulator.iter()) {
                    *out_px = (acc / count).min(65535) as u16;
                }
            }
            StackingMode::Additive => {
                for (out_px, &acc) in output.pixels.iter_mut().zip(self.accumulator.iter()) {
                    *out_px = acc.min(65535) as u16;
                }
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_stacking_calculates_mean() {
        let mut stacker = BayerStacker::new(4, 4, BayerPattern::Rggb, StackingMode::Average);

        let mut frame1 = BayerFrame::new(4, 4, 10, BayerPattern::Rggb, 1.0);
        frame1.set_pixel(0, 0, 100);
        let mut frame2 = BayerFrame::new(4, 4, 10, BayerPattern::Rggb, 1.0);
        frame2.set_pixel(0, 0, 300);

        stacker.push_frame(&frame1).unwrap();
        stacker.push_frame(&frame2).unwrap();

        assert_eq!(stacker.frame_count(), 2);
        assert_eq!(stacker.total_exposure_secs(), 2.0);

        let stacked = stacker.finish().unwrap();
        assert_eq!(stacked.get_pixel(0, 0), 200); // (100 + 300) / 2
        assert_eq!(stacked.bit_depth, 16);
    }

    #[test]
    fn test_additive_stacking_accumulates_and_clamps() {
        let mut stacker = BayerStacker::new(2, 2, BayerPattern::Rggb, StackingMode::Additive);

        let mut frame1 = BayerFrame::new(2, 2, 16, BayerPattern::Rggb, 2.0);
        frame1.set_pixel(0, 0, 40000);
        let mut frame2 = BayerFrame::new(2, 2, 16, BayerPattern::Rggb, 2.0);
        frame2.set_pixel(0, 0, 30000);

        stacker.push_frame(&frame1).unwrap();
        stacker.push_frame(&frame2).unwrap();

        let stacked = stacker.finish().unwrap();
        // 40000 + 30000 = 70000 -> clamps to 65535
        assert_eq!(stacked.get_pixel(0, 0), 65535);
        assert_eq!(stacked.exposure_secs, 4.0);
    }

    #[test]
    fn test_dimension_mismatch_error() {
        let mut stacker = BayerStacker::new(4, 4, BayerPattern::Rggb, StackingMode::Average);
        let invalid_frame = BayerFrame::new(8, 8, 10, BayerPattern::Rggb, 1.0);
        let res = stacker.push_frame(&invalid_frame);
        assert!(matches!(res, Err(StackerError::DimensionMismatch(8, 8, 4, 4))));
    }

    #[test]
    fn test_empty_stack_error() {
        let stacker = BayerStacker::new(4, 4, BayerPattern::Rggb, StackingMode::Average);
        let res = stacker.finish();
        assert_eq!(res, Err(StackerError::EmptyStack));
    }
}
