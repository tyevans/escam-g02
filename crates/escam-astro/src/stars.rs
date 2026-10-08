//! Real-Time Star Detection, Centroiding, and FWHM Seeing Focus Metric.
//!
//! Evaluates optical focus and atmospheric seeing by identifying point-spread functions (PSFs).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedStar {
    pub x: f64,
    pub y: f64,
    pub peak_flux: u16,
    pub total_flux: f64,
    pub fwhm_pixels: f64,
    pub snr: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusMetric {
    pub star_count: usize,
    pub median_fwhm: f64,
    pub best_fwhm: f64,
    pub stars: Vec<DetectedStar>,
}

pub struct StarDetector {
    pub width: usize,
    pub height: usize,
    pub sigma_threshold: f64,
    pub min_area: usize,
}

impl StarDetector {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            sigma_threshold: 3.5,
            min_area: 2, // Reject isolated 1-pixel hot noise
        }
    }

    /// Detects stars in a 16-bit linear frame and calculates their centroids and FWHM.
    pub fn detect_stars(&self, frame: &[u16]) -> FocusMetric {
        if frame.len() != self.width * self.height || frame.is_empty() {
            return FocusMetric {
                star_count: 0,
                median_fwhm: 0.0,
                best_fwhm: 0.0,
                stars: Vec::new(),
            };
        }

        // 1. Calculate rough background mean and standard deviation via fast sampling
        let step = (frame.len() / 1000).max(1);
        let mut sample_sum = 0f64;
        let mut sample_count = 0f64;
        for i in (0..frame.len()).step_by(step) {
            sample_sum += frame[i] as f64;
            sample_count += 1.0;
        }
        let mean = sample_sum / sample_count;

        let mut sample_var = 0f64;
        for i in (0..frame.len()).step_by(step) {
            let diff = frame[i] as f64 - mean;
            sample_var += diff * diff;
        }
        let std_dev = (sample_var / sample_count).sqrt().max(1.0);
        let threshold = mean + self.sigma_threshold * std_dev;

        // 2. Local peak search
        let mut detected = Vec::new();
        let w = self.width;
        let h = self.height;

        for y in 4..(h - 4) {
            for x in 4..(w - 4) {
                let idx = y * w + x;
                let val = frame[idx] as f64;
                if val <= threshold {
                    continue;
                }

                // Check if strict local maximum in 3x3 window
                let mut is_peak = true;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        let neighbor = frame[((y as isize + dy) * w as isize + (x as isize + dx)) as usize] as f64;
                        if neighbor >= val {
                            is_peak = false;
                            break;
                        }
                    }
                    if !is_peak { break; }
                }

                if !is_peak { continue; }

                // 3. Sub-pixel centroiding in 5x5 window
                let mut sum_flux = 0.0;
                let mut sum_x = 0.0;
                let mut sum_y = 0.0;
                let mut neighbor_count = 0;

                for dy in -2..=2 {
                    for dx in -2..=2 {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        let nval = frame[(ny * w as isize + nx) as usize] as f64 - mean;
                        if nval > std_dev {
                            sum_flux += nval;
                            sum_x += nx as f64 * nval;
                            sum_y += ny as f64 * nval;
                            neighbor_count += 1;
                        }
                    }
                }

                if neighbor_count < self.min_area || sum_flux <= 0.0 {
                    continue; // Hot pixel rejection
                }

                let cx = sum_x / sum_flux;
                let cy = sum_y / sum_flux;

                // 4. Second-moment radial variance for FWHM
                let mut var_r = 0.0;
                for dy in -2..=2 {
                    for dx in -2..=2 {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        let nval = (frame[(ny * w as isize + nx) as usize] as f64 - mean).max(0.0);
                        let r2 = (nx as f64 - cx).powi(2) + (ny as f64 - cy).powi(2);
                        var_r += r2 * nval;
                    }
                }
                let sigma_r = (var_r / sum_flux).sqrt().max(0.2);
                let fwhm = (2.355 * sigma_r).clamp(0.5, 20.0);
                let snr = (val - mean) / std_dev;

                detected.push(DetectedStar {
                    x: cx,
                    y: cy,
                    peak_flux: val as u16,
                    total_flux: sum_flux,
                    fwhm_pixels: fwhm,
                    snr,
                });
            }
        }

        // Sort by SNR descending
        detected.sort_by(|a, b| b.snr.partial_cmp(&a.snr).unwrap_or(std::cmp::Ordering::Equal));

        let star_count = detected.len();
        let (median_fwhm, best_fwhm) = if !detected.is_empty() {
            let mut fwhms: Vec<f64> = detected.iter().map(|s| s.fwhm_pixels).collect();
            fwhms.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            (fwhms[fwhms.len() / 2], fwhms[0])
        } else {
            (0.0, 0.0)
        };

        FocusMetric {
            star_count,
            median_fwhm,
            best_fwhm,
            stars: detected.into_iter().take(20).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_detector_on_synthetic_psf() {
        let w = 32;
        let h = 32;
        let mut frame = vec![100u16; w * h];

        // Synthesize Gaussian star at (16.0, 16.0)
        for y in 12..=20 {
            for x in 12..=20 {
                let r2 = ((x as f64 - 16.0).powi(2) + (y as f64 - 16.0).powi(2)) as f64;
                let flux = 2000.0 * (-r2 / (2.0 * 1.5 * 1.5)).exp();
                frame[y * w + x] += flux as u16;
            }
        }

        let detector = StarDetector::new(w, h);
        let metric = detector.detect_stars(&frame);

        assert_eq!(metric.star_count, 1);
        let star = &metric.stars[0];
        assert!((star.x - 16.0).abs() < 0.2);
        assert!((star.y - 16.0).abs() < 0.2);
        assert!(star.fwhm_pixels > 1.0 && star.fwhm_pixels < 5.0);
    }
}
