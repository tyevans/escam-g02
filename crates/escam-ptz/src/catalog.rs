//! Celestial Target Catalog and Automated GoTo Slew Controller.
//!
//! Provides built-in ephemeris/coordinates for major astronomical objects
//! and converts Equatorial coordinates (RA, DEC) to horizontal mount coordinates (Az, Alt).

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelestialTarget {
    pub name: String,
    pub designation: String,
    pub ra_hours: f64,
    pub dec_deg: f64,
    pub magnitude: f32,
    pub object_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetHorizontalPosition {
    pub name: String,
    pub azimuth_deg: f64,
    pub altitude_deg: f64,
    pub is_visible: bool,
    pub pan_step: i32,
    pub tilt_step: i32,
}

pub struct CelestialCatalog {
    targets: Vec<CelestialTarget>,
    site_lat_deg: f64,
    site_long_deg: f64,
}

impl CelestialCatalog {
    pub fn new(site_lat_deg: f64, site_long_deg: f64) -> Self {
        let targets = vec![
            CelestialTarget {
                name: "Polaris".to_string(),
                designation: "Alpha UMi".to_string(),
                ra_hours: 2.530,
                dec_deg: 89.264,
                magnitude: 1.98,
                object_type: "Star".to_string(),
            },
            CelestialTarget {
                name: "Vega".to_string(),
                designation: "Alpha Lyr".to_string(),
                ra_hours: 18.6156,
                dec_deg: 38.783,
                magnitude: 0.03,
                object_type: "Star".to_string(),
            },
            CelestialTarget {
                name: "Sirius".to_string(),
                designation: "Alpha CMa".to_string(),
                ra_hours: 6.7525,
                dec_deg: -16.716,
                magnitude: -1.46,
                object_type: "Star".to_string(),
            },
            CelestialTarget {
                name: "Jupiter".to_string(),
                designation: "Planet".to_string(),
                ra_hours: 4.150,
                dec_deg: 20.450,
                magnitude: -2.4,
                object_type: "Planet".to_string(),
            },
            CelestialTarget {
                name: "Saturn".to_string(),
                designation: "Planet".to_string(),
                ra_hours: 22.850,
                dec_deg: -9.820,
                magnitude: 0.6,
                object_type: "Planet".to_string(),
            },
            CelestialTarget {
                name: "Andromeda Galaxy".to_string(),
                designation: "M31 / NGC 224".to_string(),
                ra_hours: 0.7123,
                dec_deg: 41.269,
                magnitude: 3.44,
                object_type: "Galaxy".to_string(),
            },
            CelestialTarget {
                name: "Orion Nebula".to_string(),
                designation: "M42 / NGC 1976".to_string(),
                ra_hours: 5.588,
                dec_deg: -5.391,
                magnitude: 4.0,
                object_type: "Nebula".to_string(),
            },
            CelestialTarget {
                name: "Pleiades".to_string(),
                designation: "M45".to_string(),
                ra_hours: 3.791,
                dec_deg: 24.105,
                magnitude: 1.6,
                object_type: "Open Cluster".to_string(),
            },
        ];

        Self {
            targets,
            site_lat_deg,
            site_long_deg,
        }
    }

    pub fn set_site(&mut self, lat_deg: f64, long_deg: f64) {
        self.site_lat_deg = lat_deg;
        self.site_long_deg = long_deg;
    }

    pub fn list_targets(&self) -> &[CelestialTarget] {
        &self.targets
    }

    /// Computes horizontal mount coordinates (Azimuth, Altitude) for a target given local sidereal time.
    pub fn compute_target_position(&self, target_name: &str, lst_hours: f64) -> Option<TargetHorizontalPosition> {
        let target = self.targets.iter().find(|t| t.name.eq_ignore_ascii_case(target_name))?;

        let ra_deg = target.ra_hours * 15.0;
        let dec_rad = target.dec_deg.to_radians();
        let lat_rad = self.site_lat_deg.to_radians();
        let lst_deg = lst_hours * 15.0;

        let mut ha_deg = lst_deg - ra_deg;
        while ha_deg < -180.0 { ha_deg += 360.0; }
        while ha_deg > 180.0 { ha_deg -= 360.0; }
        let ha_rad = ha_deg.to_radians();

        // sin(alt) = sin(dec)*sin(lat) + cos(dec)*cos(lat)*cos(ha)
        let sin_alt = dec_rad.sin() * lat_rad.sin() + dec_rad.cos() * lat_rad.cos() * ha_rad.cos();
        let alt_rad = sin_alt.clamp(-1.0, 1.0).asin();
        let alt_deg = alt_rad.to_degrees();

        // cos(az) = (sin(dec) - sin(lat)*sin(alt)) / (cos(lat)*cos(alt))
        let cos_lat = lat_rad.cos();
        let cos_alt = alt_rad.cos();
        let az_rad = if cos_lat.abs() > 1e-6 && cos_alt.abs() > 1e-6 {
            let cos_az = ((dec_rad.sin() - lat_rad.sin() * sin_alt) / (cos_lat * cos_alt)).clamp(-1.0, 1.0);
            let az = cos_az.acos();
            if ha_rad.sin() > 0.0 { 2.0 * PI - az } else { az }
        } else {
            0.0
        };
        let az_deg = az_rad.to_degrees();

        // Map Alt/Az to stepper motor steps
        // Pan: 520 steps = 355 deg (1.46 deg/step)
        // Tilt: 260 steps = 120 deg (2.16 deg/step)
        let pan_step = ((az_deg / 355.0) * 520.0).round() as i32;
        let tilt_step = ((alt_deg.max(0.0) / 120.0) * 260.0).round() as i32;

        Some(TargetHorizontalPosition {
            name: target.name.clone(),
            azimuth_deg: az_deg,
            altitude_deg: alt_deg,
            is_visible: alt_deg > 0.0,
            pan_step: pan_step.clamp(0, 520),
            tilt_step: tilt_step.clamp(0, 260),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polaris_altitude_matches_latitude() {
        let catalog = CelestialCatalog::new(37.77, -122.42);
        let pos = catalog.compute_target_position("Polaris", 12.0).unwrap();
        // For Polaris (Dec ~ 89.26), Altitude is very close to site latitude
        assert!((pos.altitude_deg - 37.77).abs() < 2.0);
    }
}
