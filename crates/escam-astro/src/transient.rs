//! Autonomous Transient Meteor, Fireball, and Satellite Streak Detector.
//!
//! Analyzes inter-frame differences to detect fast-moving linear streaks in astronomical video.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientEvent {
    pub id: u64,
    pub timestamp_ms: u64,
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub length_pixels: f64,
    pub aspect_ratio: f64,
    pub peak_flux: u16,
    pub velocity_pix_per_sec: f64,
}

pub struct TransientDetector {
    pub width: usize,
    pub height: usize,
    pub min_aspect_ratio: f64,
    pub min_length_pixels: f64,
    pub diff_threshold: u16,
    previous_frame: Option<Vec<u16>>,
    next_id: u64,
}

impl TransientDetector {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            min_aspect_ratio: 2.5,
            min_length_pixels: 10.0,
            diff_threshold: 400,
            previous_frame: None,
            next_id: 1,
        }
    }

    pub fn reset(&mut self) {
        self.previous_frame = None;
    }

    /// Processes a new frame, comparing it with the previous frame to detect linear streaks.
    pub fn process_frame(&mut self, current: &[u16], timestamp_ms: u64, delta_t_sec: f64) -> Vec<TransientEvent> {
        let count = self.width * self.height;
        if current.len() != count {
            return Vec::new();
        }

        let prev = match &self.previous_frame {
            Some(p) => p.clone(),
            None => {
                self.previous_frame = Some(current.to_vec());
                return Vec::new();
            }
        };

        self.previous_frame = Some(current.to_vec());

        // 1. Calculate positive difference image (new light in current frame)
        let mut diff_points = Vec::new();
        let w = self.width;
        let mut max_flux = 0u16;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * w + x;
                let c = current[idx];
                let p = prev[idx];
                if c > p {
                    let diff = c - p;
                    if diff > self.diff_threshold {
                        diff_points.push((x as f64, y as f64));
                        if c > max_flux {
                            max_flux = c;
                        }
                    }
                }
            }
        }

        if diff_points.len() < (self.min_length_pixels as usize) {
            return Vec::new();
        }

        // 2. Compute bounding box and principal axis
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;

        for &(x, y) in &diff_points {
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }

        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let length = (dx * dx + dy * dy).sqrt();

        if length < self.min_length_pixels {
            return Vec::new();
        }

        // Linear streak regression to verify collinearity
        let n = diff_points.len() as f64;
        let mean_x = diff_points.iter().map(|p| p.0).sum::<f64>() / n;
        let mean_y = diff_points.iter().map(|p| p.1).sum::<f64>() / n;

        let mut cov_xx = 0.0;
        let mut cov_yy = 0.0;
        let mut cov_xy = 0.0;

        for &(x, y) in &diff_points {
            let rx = x - mean_x;
            let ry = y - mean_y;
            cov_xx += rx * rx;
            cov_yy += ry * ry;
            cov_xy += rx * ry;
        }

        // Eigenvalues of covariance matrix give major and minor axis lengths
        let trace = cov_xx + cov_yy;
        let det = cov_xx * cov_yy - cov_xy * cov_xy;
        let disc = (trace * trace - 4.0 * det).max(0.0).sqrt();
        let lambda1 = (trace + disc) / 2.0;
        let lambda2 = ((trace - disc) / 2.0).max(1e-4);

        let aspect_ratio = (lambda1 / lambda2).sqrt();

        if aspect_ratio >= self.min_aspect_ratio {
            let velocity = if delta_t_sec > 0.0 { length / delta_t_sec } else { 0.0 };
            let event = TransientEvent {
                id: self.next_id,
                timestamp_ms,
                start_x: min_x,
                start_y: min_y,
                end_x: max_x,
                end_y: max_y,
                length_pixels: length,
                aspect_ratio,
                peak_flux: max_flux,
                velocity_pix_per_sec: velocity,
            };
            self.next_id += 1;
            return vec![event];
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streak_detector_finds_meteor_line() {
        let w = 100;
        let h = 100;
        let mut detector = TransientDetector::new(w, h);

        let f0 = vec![100u16; w * h];
        let mut f1 = vec![100u16; w * h];

        // Draw linear streak from (10, 10) to (50, 50)
        for i in 10..=50 {
            f1[i * w + i] = 1200;
        }

        detector.process_frame(&f0, 1000, 0.1);
        let events = detector.process_frame(&f1, 1100, 0.1);

        assert_eq!(events.len(), 1);
        let ev = &events[0];
        assert!(ev.length_pixels > 40.0);
        assert!(ev.aspect_ratio >= 2.5);
    }
}
