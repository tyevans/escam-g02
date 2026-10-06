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
