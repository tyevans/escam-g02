//! # Bresenham Time-Slice Motion Interleaver
//!
//! Generates interleaved time slices between Pan and Tilt axes for diagonal
//! motion, proportional to joystick input vectors.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisChoice {
    Pan,
    Tilt,
}

/// Fractional Bresenham accumulator for time-sliced 2D stepper motor interleaving.
#[derive(Debug, Clone, PartialEq)]
pub struct BresenhamInterleaver {
    ratio: f32,
    accumulator: f32,
}

impl BresenhamInterleaver {
    pub fn new(x: f32, y: f32) -> Self {
        let abs_x = x.abs();
        let abs_y = y.abs();
        let total = abs_x + abs_y;
        let ratio = if total > 1e-6 { abs_x / total } else { 0.5 };

        Self {
            ratio,
            accumulator: 0.0,
        }
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    pub fn update_weights(&mut self, x: f32, y: f32) {
        let abs_x = x.abs();
        let abs_y = y.abs();
        let total = abs_x + abs_y;
        self.ratio = if total > 1e-6 { abs_x / total } else { 0.5 };
    }

    /// Selects the initial axis for the first time slice and primes the accumulator.
    pub fn initial_axis(&mut self) -> AxisChoice {
        if self.ratio >= 0.5 {
            self.accumulator = self.ratio - 1.0;
            AxisChoice::Pan
        } else {
            self.accumulator = self.ratio;
            AxisChoice::Tilt
        }
    }

    /// Advances the accumulator and returns the chosen axis for subsequent time slices.
    pub fn next_axis(&mut self) -> AxisChoice {
        self.accumulator += self.ratio;
        if self.accumulator >= 0.5 {
            self.accumulator -= 1.0;
            AxisChoice::Pan
        } else {
            AxisChoice::Tilt
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interleaver_equal_ratio_alternates() {
        let mut interleaver = BresenhamInterleaver::new(0.5, 0.5);
        assert_eq!(interleaver.initial_axis(), AxisChoice::Pan);

        let mut choices = Vec::new();
        for _ in 0..6 {
            choices.push(interleaver.next_axis());
        }

        assert_eq!(
            choices,
            vec![
                AxisChoice::Tilt,
                AxisChoice::Pan,
                AxisChoice::Tilt,
                AxisChoice::Pan,
                AxisChoice::Tilt,
                AxisChoice::Pan,
            ]
        );
    }

    #[test]
    fn test_interleaver_pan_heavy_ratio() {
        let mut interleaver = BresenhamInterleaver::new(0.75, 0.25);
        let mut pan_count = 0;
        let mut tilt_count = 0;

        match interleaver.initial_axis() {
            AxisChoice::Pan => pan_count += 1,
            AxisChoice::Tilt => tilt_count += 1,
        }

        for _ in 0..11 {
            match interleaver.next_axis() {
                AxisChoice::Pan => pan_count += 1,
                AxisChoice::Tilt => tilt_count += 1,
            }
        }

        // Over 12 slices with 0.75 ratio: 9 pan, 3 tilt (3:1)
        assert_eq!(pan_count, 9);
        assert_eq!(tilt_count, 3);
    }

    #[test]
    fn test_interleaver_tilt_heavy_ratio() {
        let mut interleaver = BresenhamInterleaver::new(0.2, 0.8);
        let mut pan_count = 0;
        let mut tilt_count = 0;

        match interleaver.initial_axis() {
            AxisChoice::Pan => pan_count += 1,
            AxisChoice::Tilt => tilt_count += 1,
        }

        for _ in 0..9 {
            match interleaver.next_axis() {
                AxisChoice::Pan => pan_count += 1,
                AxisChoice::Tilt => tilt_count += 1,
            }
        }

        // Over 10 slices with 0.2 ratio: 2 pan, 8 tilt (1:4)
        assert_eq!(pan_count, 2);
        assert_eq!(tilt_count, 8);
    }

    #[test]
    fn test_interleaver_dynamic_update() {
        let mut interleaver = BresenhamInterleaver::new(0.5, 0.5);
        assert!((interleaver.ratio() - 0.5).abs() < 1e-4);

        interleaver.update_weights(0.9, 0.1);
        assert!((interleaver.ratio() - 0.9).abs() < 1e-4);
    }
}
