//! Precision Astronomical Clock, SNTP Synchronization, and Midpoint Stamping.
//!
//! Provides sub-millisecond UTC timekeeping, Julian Date (JD/MJD), Local Sidereal Time (LST),
//! and calculation of exposure start (`DATE-OBS`) and midpoint (`DATE-AVG`).

use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClockStatus {
    pub utc_iso: String,
    pub julian_date: f64,
    pub modified_julian_date: f64,
    pub local_sidereal_time_hours: f64,
    pub sntp_synced: bool,
    pub site_latitude_deg: f64,
    pub site_longitude_deg: f64,
}

pub struct AstroClock {
    site_lat_deg: f64,
    site_long_deg: f64,
    sntp_synced: bool,
    offset_nanos: i64,
}

impl AstroClock {
    pub fn new(site_lat_deg: f64, site_long_deg: f64) -> Self {
        Self {
            site_lat_deg,
            site_long_deg,
            sntp_synced: false,
            offset_nanos: 0,
        }
    }

    pub fn set_site(&mut self, lat_deg: f64, long_deg: f64) {
        self.site_lat_deg = lat_deg;
        self.site_long_deg = long_deg;
    }

    pub fn set_offset_nanos(&mut self, offset: i64) {
        self.offset_nanos = offset;
        self.sntp_synced = true;
    }

    /// Returns the current disciplined UTC time.
    pub fn now_utc(&self) -> DateTime<Utc> {
        let base = Utc::now();
        if self.offset_nanos != 0 {
            base + chrono::Duration::nanoseconds(self.offset_nanos)
        } else {
            base
        }
    }

    /// Calculates Julian Date (JD) from a UTC datetime according to USNO algorithm.
    pub fn julian_date(dt: DateTime<Utc>) -> f64 {
        let year = dt.year() as f64;
        let month = dt.month() as f64;
        let day = dt.day() as f64;
        let hour = dt.hour() as f64 + (dt.minute() as f64 / 60.0) + (dt.second() as f64 / 3600.0) + (dt.nanosecond() as f64 / 3.6e12);

        let (y, m) = if month <= 2.0 {
            (year - 1.0, month + 12.0)
        } else {
            (year, month)
        };

        let a = (y / 100.0).floor();
        let b = 2.0 - a + (a / 4.0).floor();

        let jd_day = (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + day + b - 1524.5;
        jd_day + (hour / 24.0)
    }

    /// Calculates Greenwich Mean Sidereal Time (GMST) in hours.
    pub fn gmst_hours(jd: f64) -> f64 {
        let d = jd - 2451545.0; // Days from J2000.0
        let gmst = 18.697374558 + 24.06570982441908 * d;
        let mut hours = gmst % 24.0;
        if hours < 0.0 { hours += 24.0; }
        hours
    }

    /// Calculates Local Sidereal Time (LST) in hours.
    pub fn lst_hours(&self) -> f64 {
        let now = self.now_utc();
        let jd = Self::julian_date(now);
        let gmst = Self::gmst_hours(jd);
        let lst = gmst + (self.site_long_deg / 15.0);
        let mut hours = lst % 24.0;
        if hours < 0.0 { hours += 24.0; }
        hours
    }

    /// Returns ISO strings for exposure start (`DATE-OBS`) and exposure midpoint (`DATE-AVG`).
    pub fn exposure_stamps(&self, start: DateTime<Utc>, duration_secs: f64) -> (String, String) {
        let midpoint = start + chrono::Duration::nanoseconds((duration_secs * 500_000_000.0) as i64);
        (start.to_rfc3339(), midpoint.to_rfc3339())
    }

    pub fn status(&self) -> ClockStatus {
        let now = self.now_utc();
        let jd = Self::julian_date(now);
        ClockStatus {
            utc_iso: now.to_rfc3339(),
            julian_date: jd,
            modified_julian_date: jd - 2400000.5,
            local_sidereal_time_hours: self.lst_hours(),
            sntp_synced: self.sntp_synced,
            site_latitude_deg: self.site_lat_deg,
            site_longitude_deg: self.site_long_deg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_julian_date_j2000_epoch() {
        // J2000.0 is 2000-01-01 12:00:00 UTC = 2451545.0
        let dt = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let jd = AstroClock::julian_date(dt);
        assert!((jd - 2451545.0).abs() < 1e-4);
    }

    #[test]
    fn test_exposure_midpoint_calculation() {
        let clock = AstroClock::new(37.77, -122.42);
        let start = Utc.with_ymd_and_hms(2026, 10, 6, 22, 0, 0).unwrap();
        let (obs, avg) = clock.exposure_stamps(start, 10.0);
        assert!(obs.contains("22:00:00"));
        assert!(avg.contains("22:00:05"));
    }
}
