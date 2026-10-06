//! # Celestial Sidereal Rate Tracking Engine
//!
//! Provides precision sub-step celestial tracking for equatorial and alt-az
//! astrophotography with the ESCAM G02 stepper motor drive.

use std::time::Duration;

/// Standard astronomical tracking rates in arcseconds per SI second.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrackingRate {
    /// Sidereal rate: 15.041067 arcsec/s (Earth's rotation relative to vernal equinox).
    Sidereal,
    /// Lunar rate: ~14.685 arcsec/s (Moon's apparent celestial motion).
    Lunar,
    /// Solar rate: ~15.000 arcsec/s (Sun's apparent celestial motion).
    Solar,
    /// Custom celestial tracking rate in arcseconds per second.
    Custom(f64),
}

impl TrackingRate {
    /// Return the rate in arcseconds per second.
    pub fn arcsec_per_second(&self) -> f64 {
        match self {
            Self::Sidereal => 15.041_067,
            Self::Lunar => 14.685_000,
            Self::Solar => 15.000_000,
            Self::Custom(r) => *r,
        }
    }
}

/// Hemisphere orientation determining tracking rotation direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hemisphere {
    Northern,
    Southern,
}

/// Sidereal tracking controller with sub-step micro-accumulator.
#[derive(Debug, Clone)]
pub struct SiderealTracker {
    rate: TrackingRate,
    hemisphere: Hemisphere,
    enabled: bool,
    arcsec_per_step: f64,
    fractional_accumulator: f64,
    total_steps_executed: u64,
}

impl SiderealTracker {
    /// Create a new tracker with specific gearing geometry.
    ///
    /// # Arguments
    /// * `rate` - Astronomical tracking rate
    /// * `steps` - Total motor steps across angular range
    /// * `degrees` - Angular range in degrees corresponding to `steps`
    pub fn new(rate: TrackingRate, steps: u32, degrees: f64) -> Self {
        let total_arcseconds = degrees * 3600.0;
        let arcsec_per_step = if steps > 0 {
            total_arcseconds / (steps as f64)
        } else {
            1.0
        };

        Self {
            rate,
            hemisphere: Hemisphere::Northern,
            enabled: false,
            arcsec_per_step,
            fractional_accumulator: 0.0,
            total_steps_executed: 0,
        }
    }

    /// Default tracker calibrated for the ESCAM G02 Pan axis (520 steps / 355 deg).
    pub fn escam_g02_default() -> Self {
        Self::new(TrackingRate::Sidereal, 520, 355.0)
    }

    /// Enable or disable tracking.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.fractional_accumulator = 0.0;
        }
    }

    /// Check if tracking is active.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get current tracking rate.
    pub fn rate(&self) -> TrackingRate {
        self.rate
    }

    /// Update tracking rate.
    pub fn set_rate(&mut self, rate: TrackingRate) {
        self.rate = rate;
    }

    /// Get current hemisphere.
    pub fn hemisphere(&self) -> Hemisphere {
        self.hemisphere
    }

    /// Set tracking hemisphere.
    pub fn set_hemisphere(&mut self, hemisphere: Hemisphere) {
        self.hemisphere = hemisphere;
    }

    /// Theoretical duration between individual steps.
    pub fn step_interval(&self) -> Duration {
        let rate = self.rate.arcsec_per_second();
        if rate <= 0.0 {
            return Duration::from_secs(86400);
        }
        let secs = self.arcsec_per_step / rate;
        Duration::from_secs_f64(secs)
    }

    /// Arcseconds per single physical step.
    pub fn arcsec_per_step(&self) -> f64 {
        self.arcsec_per_step
    }

    /// Total steps tracked since enablement.
    pub fn total_steps(&self) -> u64 {
        self.total_steps_executed
    }

    /// Accumulate elapsed time and return number of discrete steps to pulse.
    pub fn update(&mut self, delta: Duration) -> u32 {
        if !self.enabled {
            return 0;
        }

        let rate = self.rate.arcsec_per_second();
        let arcsec_elapsed = delta.as_secs_f64() * rate;

        self.fractional_accumulator += arcsec_elapsed;

        let steps = (self.fractional_accumulator / self.arcsec_per_step).floor() as u32;
        if steps > 0 {
            self.fractional_accumulator -= (steps as f64) * self.arcsec_per_step;
            self.total_steps_executed += steps as u64;
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escam_g02_default_arcsec_per_step() {
        let tracker = SiderealTracker::escam_g02_default();
        // 355 deg * 3600 = 1,278,000 arcsec / 520 steps = 2457.6923 arcsec/step
        let expected = (355.0 * 3600.0) / 520.0;
        assert!((tracker.arcsec_per_step() - expected).abs() < 1e-4);
    }

    #[test]
    fn test_step_interval_calculation() {
        let tracker = SiderealTracker::escam_g02_default();
        let interval = tracker.step_interval();
        // ~2457.69 arcsec / 15.041067 arcsec/s = ~163.3988 s
        assert!(interval.as_secs() >= 163 && interval.as_secs() <= 164);
    }

    #[test]
    fn test_accumulator_steps_when_enabled() {
        let mut tracker = SiderealTracker::escam_g02_default();
        tracker.set_enabled(true);

        // Advance 100 seconds (not enough for a step yet)
        let steps = tracker.update(Duration::from_secs(100));
        assert_eq!(steps, 0);

        // Advance another 100 seconds (total 200s > 163.4s, should emit 1 step)
        let steps2 = tracker.update(Duration::from_secs(100));
        assert_eq!(steps2, 1);
        assert_eq!(tracker.total_steps(), 1);

        // Advance 400 seconds (~2.44 steps, should emit 2 steps)
        let steps3 = tracker.update(Duration::from_secs(400));
        assert_eq!(steps3, 2);
        assert_eq!(tracker.total_steps(), 3);
    }

    #[test]
    fn test_disabled_tracker_emits_zero_steps() {
        let mut tracker = SiderealTracker::escam_g02_default();
        tracker.set_enabled(false);

        let steps = tracker.update(Duration::from_secs(1000));
        assert_eq!(steps, 0);
        assert_eq!(tracker.total_steps(), 0);
    }

    #[test]
    fn test_tracking_rate_switching() {
        let mut tracker = SiderealTracker::escam_g02_default();
        tracker.set_rate(TrackingRate::Lunar);
        assert_eq!(tracker.rate(), TrackingRate::Lunar);
        assert!((tracker.rate().arcsec_per_second() - 14.685).abs() < 1e-4);

        tracker.set_rate(TrackingRate::Solar);
        assert_eq!(tracker.rate(), TrackingRate::Solar);
        assert!((tracker.rate().arcsec_per_second() - 15.000).abs() < 1e-4);
    }
}
