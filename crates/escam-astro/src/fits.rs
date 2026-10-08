//! # Standard Astronomical FITS (Flexible Image Transport System) Serializer
//!
//! Formats raw Bayer frames into NASA standard FITS containers with astronomy headers
//! compatible with Siril, DS9, PixInsight, and AstroImageJ.

use crate::bayer::BayerFrame;
use crate::wcs::WcsMetadata;
use chrono::Utc;

pub const FITS_BLOCK_SIZE: usize = 2880;
pub const FITS_CARD_SIZE: usize = 80;

pub struct FitsWriter;

impl FitsWriter {
    /// Formats an 80-byte FITS header card key/value.
    pub fn format_card(key: &str, value: &str) -> [u8; FITS_CARD_SIZE] {
        let mut card = [b' '; FITS_CARD_SIZE];
        let key_bytes = key.as_bytes();
        card[..key_bytes.len().min(8)].copy_from_slice(&key_bytes[..key_bytes.len().min(8)]);

        if !value.is_empty() {
            card[8] = b'=';
            card[9] = b' ';
            let val_bytes = value.as_bytes();
            let start = 10;
            let end = (start + val_bytes.len()).min(FITS_CARD_SIZE);
            card[start..end].copy_from_slice(&val_bytes[..end - start]);
        }

        card
    }

    /// Serializes a BayerFrame into a complete FITS byte vector.
    pub fn write_fits(frame: &BayerFrame) -> Vec<u8> {
        Self::write_fits_with_wcs(frame, None)
    }

    /// Serializes a BayerFrame into a complete FITS byte vector with optional WCS metadata.
    pub fn write_fits_with_wcs(frame: &BayerFrame, wcs: Option<&WcsMetadata>) -> Vec<u8> {
        let mut output = Vec::new();

        let shift = 16u8.saturating_sub(frame.bit_depth);
        let mut min_val = u16::MAX;
        let mut max_val = 0u16;
        for &pixel in &frame.pixels {
            let scaled = ((pixel as u32) << shift).min(65535) as u16;
            if scaled < min_val {
                min_val = scaled;
            }
            if scaled > max_val {
                max_val = scaled;
            }
        }
        if min_val > max_val {
            min_val = 0;
            max_val = 65535;
        }

        // 1. Build Header Cards
        let mut cards = vec![
            Self::format_card("SIMPLE", "T / Standard FITS format"),
            Self::format_card("BITPIX", "16 / 16-bit unsigned integers"),
            Self::format_card("NAXIS", "2 / Two-dimensional image array"),
            Self::format_card("NAXIS1", &format!("{:>20} / Image width", frame.width)),
            Self::format_card("NAXIS2", &format!("{:>20} / Image height", frame.height)),
            Self::format_card("BZERO", "32768 / Offset for unsigned 16-bit integer"),
            Self::format_card("BSCALE", "1 / Default scaling factor"),
            Self::format_card("BAYERPAT", &format!("'{:<8}' / Color filter array pattern", frame.pattern.as_str())),
            Self::format_card("EXPTIME", &format!("{:>20.4} / Exposure duration in seconds", frame.exposure_secs)),
            Self::format_card("DATAMIN", &format!("{:>20} / Minimum data value", min_val)),
            Self::format_card("DATAMAX", &format!("{:>20} / Maximum data value", max_val)),
            Self::format_card("DATE-OBS", &format!("'{}' / UTC start of observation", Utc::now().to_rfc3339())),
            Self::format_card("INSTRUME", "'ESCAM G02' / Camera model"),
            Self::format_card("TELESCOP", "'ESCAM Robotic Mount' / Integrated Alt-Az mount"),
        ];

        // Append WCS cards if provided
        if let Some(wcs_meta) = wcs {
            for raw_card in wcs_meta.generate_fits_cards() {
                let mut card_bytes = [b' '; FITS_CARD_SIZE];
                let b = raw_card.as_bytes();
                card_bytes[..b.len().min(FITS_CARD_SIZE)].copy_from_slice(&b[..b.len().min(FITS_CARD_SIZE)]);
                cards.push(card_bytes);
            }
        }

        cards.push(Self::format_card("END", ""));

        // 2. Write Header Block (Padded to 2880 bytes)
        let mut header_bytes = Vec::new();
        for card in cards {
            header_bytes.extend_from_slice(&card);
        }
        let header_rem = header_bytes.len() % FITS_BLOCK_SIZE;
        if header_rem != 0 {
            header_bytes.resize(header_bytes.len() + (FITS_BLOCK_SIZE - header_rem), b' ');
        }
        output.extend_from_slice(&header_bytes);

        // 3. Write Data Block (16-bit big-endian with BZERO offset)
        let mut data_bytes = Vec::with_capacity(frame.pixels.len() * 2);
        for &pixel in &frame.pixels {
            let scaled = ((pixel as u32) << shift).min(65535) as u16;
            let val = (scaled as i32 - 32768) as i16;
            data_bytes.extend_from_slice(&val.to_be_bytes());
        }

        let data_rem = data_bytes.len() % FITS_BLOCK_SIZE;
        if data_rem != 0 {
            data_bytes.resize(data_bytes.len() + (FITS_BLOCK_SIZE - data_rem), 0);
        }
        output.extend_from_slice(&data_bytes);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bayer::{BayerFrame, BayerPattern};

    #[test]
    fn test_fits_block_multiple_size() {
        let frame = BayerFrame::new(100, 100, 10, BayerPattern::Rggb, 1.0);
        let fits_bytes = FitsWriter::write_fits(&frame);

        // Every valid FITS file must be an exact multiple of 2880 bytes
        assert_eq!(fits_bytes.len() % FITS_BLOCK_SIZE, 0);
        assert!(fits_bytes.len() >= FITS_BLOCK_SIZE * 2);

        // Header starts with SIMPLE = T
        let header_str = String::from_utf8_lossy(&fits_bytes[..80]);
        assert!(header_str.starts_with("SIMPLE  = T"));
    }

    #[test]
    fn test_fits_with_wcs() {
        let frame = BayerFrame::new(100, 100, 10, BayerPattern::Rggb, 1.0);
        let mut wcs = WcsMetadata::new(100, 100, 4.0, 3.75);
        wcs.set_horizontal_coords(180.0, 45.0, 100.0);
        let fits_bytes = FitsWriter::write_fits_with_wcs(&frame, Some(&wcs));
        assert_eq!(fits_bytes.len() % FITS_BLOCK_SIZE, 0);

        let header_str = String::from_utf8_lossy(&fits_bytes[..2880]);
        assert!(header_str.contains("CTYPE1  = 'RA---TAN'"));
        assert!(header_str.contains("CRVAL1  ="));
    }
}
