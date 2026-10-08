//! World Coordinate System (WCS) and Astrometric FITS Metadata Generation.
//!
//! Provides IAU FITS 4.0 TAN gnomonic projection parameters and horizontal-to-equatorial
//! coordinate conversions for astronomical plate-solving.

use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct WcsMetadata {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub crpix1: f64,
    pub crpix2: f64,
    pub cdelt1: f64, // deg per pixel (X)
    pub cdelt2: f64, // deg per pixel (Y)
    pub crota2: f64, // rotation deg
    pub sitelat_deg: f64,
    pub sitelong_deg: f64,
    pub date_obs_utc: String,
    pub date_avg_utc: String,
}

impl WcsMetadata {
    pub fn new(width: usize, height: usize, focal_length_mm: f64, pixel_size_um: f64) -> Self {
        // Pixel scale in degrees per pixel
        let scale_deg = (pixel_size_um * 1e-3 / focal_length_mm).atan().to_degrees();
        Self {
            ra_deg: 0.0,
            dec_deg: 90.0, // Default North Celestial Pole
            crpix1: (width as f64) / 2.0,
            crpix2: (height as f64) / 2.0,
            cdelt1: -scale_deg, // Standard RA increases to the left
            cdelt2: scale_deg,
            crota2: 0.0,
            sitelat_deg: 37.7749,
            sitelong_deg: -122.4194,
            date_obs_utc: "2026-10-06T00:00:00.000".to_string(),
            date_avg_utc: "2026-10-06T00:00:00.000".to_string(),
        }
    }

    /// Converts Horizontal Coordinates (Azimuth, Altitude) to Equatorial (RA, DEC)
    /// given local sidereal time (LST) and site latitude.
    pub fn set_horizontal_coords(&mut self, az_deg: f64, alt_deg: f64, lst_deg: f64) {
        let lat_rad = self.sitelat_deg.to_radians();
        let alt_rad = alt_deg.to_radians();
        let az_rad = az_deg.to_radians();

        // sin(dec) = sin(alt)*sin(lat) + cos(alt)*cos(lat)*cos(az)
        let sin_dec = alt_rad.sin() * lat_rad.sin() + alt_rad.cos() * lat_rad.cos() * az_rad.cos();
        let dec_rad = sin_dec.clamp(-1.0, 1.0).asin();
        self.dec_deg = dec_rad.to_degrees();

        // cos(h) = (sin(alt) - sin(lat)*sin(dec)) / (cos(lat)*cos(dec))
        let cos_lat = lat_rad.cos();
        let cos_dec = dec_rad.cos();
        let ha_rad = if cos_lat.abs() > 1e-6 && cos_dec.abs() > 1e-6 {
            let cos_h = ((alt_rad.sin() - lat_rad.sin() * sin_dec) / (cos_lat * cos_dec)).clamp(-1.0, 1.0);
            let h = cos_h.acos();
            if az_rad.sin() > 0.0 { 2.0 * PI - h } else { h }
        } else {
            0.0
        };

        let ha_deg = ha_rad.to_degrees();
        let mut ra_deg = lst_deg - ha_deg;
        while ra_deg < 0.0 { ra_deg += 360.0; }
        while ra_deg >= 360.0 { ra_deg -= 360.0; }
        self.ra_deg = ra_deg;
    }

    /// Formats 80-column FITS header cards for WCS.
    pub fn generate_fits_cards(&self) -> Vec<String> {
        let mut cards = Vec::new();

        cards.push(format_card("RADESYS ", "'FK5     '", "Equatorial coordinate system"));
        cards.push(format_card("EQUINOX ", "2000.0", "Epoch of equatorial coordinates"));
        cards.push(format_card("WCSAXES ", "2", "Number of WCS axes"));
        cards.push(format_card("CTYPE1  ", "'RA---TAN'", "TAN (gnomonic) projection for RA"));
        cards.push(format_card("CTYPE2  ", "'DEC--TAN'", "TAN (gnomonic) projection for DEC"));
        cards.push(format_card("CRPIX1  ", &format!("{:.2}", self.crpix1), "Reference pixel X"));
        cards.push(format_card("CRPIX2  ", &format!("{:.2}", self.crpix2), "Reference pixel Y"));
        cards.push(format_card("CRVAL1  ", &format!("{:.6}", self.ra_deg), "Reference RA in degrees"));
        cards.push(format_card("CRVAL2  ", &format!("{:.6}", self.dec_deg), "Reference DEC in degrees"));
        cards.push(format_card("CDELT1  ", &format!("{:.8}", self.cdelt1), "Pixel scale X (deg/pix)"));
        cards.push(format_card("CDELT2  ", &format!("{:.8}", self.cdelt2), "Pixel scale Y (deg/pix)"));
        cards.push(format_card("CROTA2  ", &format!("{:.4}", self.crota2), "Coordinate rotation angle"));
        cards.push(format_card("SITELAT ", &format!("{:.4}", self.sitelat_deg), "Observatory latitude"));
        cards.push(format_card("SITELONG", &format!("{:.4}", self.sitelong_deg), "Observatory longitude"));
        cards.push(format_card("DATE-OBS", &format!("'{}'", self.date_obs_utc), "UTC exposure start"));
        cards.push(format_card("DATE-AVG", &format!("'{}'", self.date_avg_utc), "UTC exposure midpoint"));

        cards
    }
}

fn format_card(key: &str, val: &str, comment: &str) -> String {
    let key_trimmed = key.trim();
    let val_comment = format!("{} / {}", val, comment);
    let mut card = format!("{:<8}= {:<69}", key_trimmed, val_comment);
    if card.len() > 80 {
        card.truncate(80);
    } else {
        while card.len() < 80 {
            card.push(' ');
        }
    }
    card
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wcs_generation_and_card_length() {
        let mut wcs = WcsMetadata::new(1280, 720, 4.0, 3.75);
        wcs.set_horizontal_coords(180.0, 45.0, 100.0);
        let cards = wcs.generate_fits_cards();
        assert!(!cards.is_empty());
        for card in &cards {
            assert_eq!(card.len(), 80, "FITS card must be exactly 80 chars");
        }
    }
}
