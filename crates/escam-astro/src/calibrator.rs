//! Astronomical Calibration Engine for Master Darks, Flats, and Bias Subtraction.
//!
//! Provides linear scientific calibration for 16-bit CMOS raw Bayer frames.
//! Formula: Calibrated(x,y) = ((Raw(x,y) - Dark(x,y) - Bias(x,y)) / (Flat(x,y) / MeanFlat)) + Pedestal

#[derive(Debug, Clone)]
pub struct CalibrationEngine {
    master_dark: Option<Vec<u16>>,
    master_flat: Option<Vec<f32>>,
    master_bias: Option<Vec<u16>>,
    width: usize,
    height: usize,
    pedestal: u16,
}

impl CalibrationEngine {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            master_dark: None,
            master_flat: None,
            master_bias: None,
            width,
            height,
            pedestal: 100, // Standard 100 ADU astronomical pedestal to prevent zero-clipping
        }
    }

    pub fn set_pedestal(&mut self, pedestal: u16) {
        self.pedestal = pedestal;
    }

    pub fn pedestal(&self) -> u16 {
        self.pedestal
    }

    pub fn set_master_dark(&mut self, dark: Vec<u16>) {
        if dark.len() == self.width * self.height {
            self.master_dark = Some(dark);
        }
    }

    pub fn set_master_flat(&mut self, flat: Vec<u16>) {
        if flat.len() != self.width * self.height {
            return;
        }
        let sum: f64 = flat.iter().map(|&v| v as f64).sum();
        let mean = if !flat.is_empty() { sum / flat.len() as f64 } else { 1.0 };
        let mean_f32 = if mean > 0.0 { mean as f32 } else { 1.0 };

        // Normalize flat so mean = 1.0
        let norm: Vec<f32> = flat
            .iter()
            .map(|&v| {
                let f = v as f32 / mean_f32;
                if f < 0.05 { 0.05 } else { f } // Avoid division by near-zero in vignetted corners
            })
            .collect();
        self.master_flat = Some(norm);
    }

    pub fn set_master_bias(&mut self, bias: Vec<u16>) {
        if bias.len() == self.width * self.height {
            self.master_bias = Some(bias);
        }
    }

    pub fn has_dark(&self) -> bool {
        self.master_dark.is_some()
    }

    pub fn has_flat(&self) -> bool {
        self.master_flat.is_some()
    }

    pub fn has_bias(&self) -> bool {
        self.master_bias.is_some()
    }

    pub fn clear(&mut self) {
        self.master_dark = None;
        self.master_flat = None;
        self.master_bias = None;
    }

    /// Calibrates an incoming 16-bit raw light frame.
    pub fn calibrate_frame(&self, raw: &[u16]) -> Vec<u16> {
        let count = self.width * self.height;
        if raw.len() != count {
            return raw.to_vec();
        }

        let dark = self.master_dark.as_deref();
        let flat = self.master_flat.as_deref();
        let bias = self.master_bias.as_deref();

        let mut out = Vec::with_capacity(count);

        for i in 0..count {
            let mut val = raw[i] as f32;

            if let Some(d) = dark {
                val -= d[i] as f32;
            }
            if let Some(b) = bias {
                val -= b[i] as f32;
            }

            if let Some(f) = flat {
                val /= f[i];
            }

            val += self.pedestal as f32;

            let clamped = if val < 0.0 {
                0u16
            } else if val > 65535.0 {
                65535u16
            } else {
                val.round() as u16
            };
            out.push(clamped);
        }

        out
    }

    /// Accumulates N frames into a master calibration frame (average).
    pub fn compute_master_average(frames: &[&[u16]], width: usize, height: usize) -> Vec<u16> {
        let count = width * height;
        if frames.is_empty() {
            return vec![0; count];
        }

        let mut acc = vec![0u32; count];
        let n = frames.len() as u32;

        for frame in frames {
            if frame.len() == count {
                for i in 0..count {
                    acc[i] += frame[i] as u32;
                }
            }
        }

        acc.into_iter().map(|sum| (sum / n) as u16).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_subtraction_and_pedestal() {
        let mut cal = CalibrationEngine::new(2, 2);
        let dark = vec![50, 60, 70, 80];
        cal.set_master_dark(dark);

        let raw = vec![150, 160, 170, 180];
        let result = cal.calibrate_frame(&raw);

        // Result: (raw - dark) + 100 pedestal
        assert_eq!(result, vec![200, 200, 200, 200]);
    }

    #[test]
    fn test_flat_field_normalization() {
        let mut cal = CalibrationEngine::new(2, 2);
        // Flat with center bright and corners dimmed
        let flat = vec![1000, 2000, 2000, 1000];
        cal.set_master_flat(flat);

        let raw = vec![1500, 1500, 1500, 1500];
        let result = cal.calibrate_frame(&raw);
        assert_eq!(result.len(), 4);
    }
}
