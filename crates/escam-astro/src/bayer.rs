//! # Raw Bayer Sensor Frame Container
//!
//! Stores raw uncompressed 10-bit or 12-bit Bayer CFA (Color Filter Array) pixels
//! straight from the GalaxyCore GC1034 or SmartSens SC1135 CMOS sensors.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BayerPattern {
    Rggb,
    Bgra,
    Grbg,
    Gbrg,
}

impl BayerPattern {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rggb => "RGGB",
            Self::Bgra => "BGGR",
            Self::Grbg => "GRBG",
            Self::Gbrg => "GBRG",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BayerFrame {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub pattern: BayerPattern,
    pub exposure_secs: f32,
    /// 16-bit expanded pixel array (values 0..1023 for 10-bit, or 0..4095 for 12-bit)
    pub pixels: Vec<u16>,
}

impl BayerFrame {
    pub fn new(width: u32, height: u32, bit_depth: u8, pattern: BayerPattern, exposure_secs: f32) -> Self {
        let total_pixels = (width * height) as usize;
        Self {
            width,
            height,
            bit_depth,
            pattern,
            exposure_secs,
            pixels: vec![0u16; total_pixels],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: u16) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.pixels[idx] = value;
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u16 {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.pixels[idx]
        } else {
            0
        }
    }

    /// Decodes a JPEG image buffer into a 16-bit BayerFrame matching the sensor frame.
    pub fn from_jpeg(jpeg_bytes: &[u8], exposure_secs: f32) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        use std::io::Cursor;
        let mut decoder = jpeg_decoder::Decoder::new(Cursor::new(jpeg_bytes));
        let raw_pixels = decoder.decode()?;
        let info = decoder.info().ok_or("Missing JPEG header metadata")?;
        let width = info.width as u32;
        let height = info.height as u32;

        let mut frame = Self::new(width, height, 16, BayerPattern::Rggb, exposure_secs);

        match info.pixel_format {
            jpeg_decoder::PixelFormat::L8 => {
                for (i, &lum) in raw_pixels.iter().enumerate().take(frame.pixels.len()) {
                    frame.pixels[i] = (lum as u16) * 257;
                }
            }
            jpeg_decoder::PixelFormat::RGB24 => {
                for y in 0..height {
                    for x in 0..width {
                        let idx = (y * width + x) as usize;
                        let rgb_idx = idx * 3;
                        if rgb_idx + 2 < raw_pixels.len() {
                            let r = raw_pixels[rgb_idx] as u16;
                            let g = raw_pixels[rgb_idx + 1] as u16;
                            let b = raw_pixels[rgb_idx + 2] as u16;
                            let val = match (y % 2 == 0, x % 2 == 0) {
                                (true, true) => r,
                                (true, false) => g,
                                (false, true) => g,
                                (false, false) => b,
                            };
                            frame.pixels[idx] = val * 257;
                        }
                    }
                }
            }
            _ => {
                for (i, p) in frame.pixels.iter_mut().enumerate() {
                    let src_idx = i.min(raw_pixels.len().saturating_sub(1));
                    *p = (raw_pixels[src_idx] as u16) * 257;
                }
            }
        }

        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bayer_frame_creation() {
        let mut frame = BayerFrame::new(1280, 720, 10, BayerPattern::Rggb, 1.5);
        assert_eq!(frame.pixels.len(), 1280 * 720);
        frame.set_pixel(10, 20, 1023);
        assert_eq!(frame.get_pixel(10, 20), 1023);
        assert_eq!(frame.pattern.as_str(), "RGGB");
    }
}
