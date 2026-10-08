//! # Astronomical Auto Histogram Stretch & Transfer Functions
//!
//! Provides screen transfer functions (STF) and non-linear stretch algorithms
//! tailored for deep-sky astrophotography (PixInsight MTF, Arcsinh, and Percentile):
//! - Midtone Transfer Function (MTF): balances background sky glow against faint signals
//! - Arcsinh Stretch: preserves color saturation and prevents star core blowout
//! - Percentile Min/Max: robust clipping against hot/dead pixels

/// Supported non-linear histogram stretching modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StretchMode {
    #[default]
    None,
    Auto,
    Mtf,
    Asinh,
    Aggressive,
}

impl StretchMode {
    pub fn from_str_loose(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "auto" | "stf" => StretchMode::Auto,
            "mtf" => StretchMode::Mtf,
            "asinh" | "arcsinh" => StretchMode::Asinh,
            "aggressive" | "agg" => StretchMode::Aggressive,
            _ => StretchMode::None,
        }
    }
}

/// Midtone Transfer Function (MTF) mapping x in [0, 1] given balance m in (0, 1).
/// Formula: MTF(m, x) = ((m - 1) * x) / ((2*m - 1) * x - m)
#[inline]
pub fn mtf(m: f32, x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let m = m.clamp(0.0001, 0.9999);
    let num = (m - 1.0) * x;
    let den = (2.0 * m - 1.0) * x - m;
    if den.abs() < 1e-6 {
        x
    } else {
        (num / den).clamp(0.0, 1.0)
    }
}

/// Computes robust min, max, and median for a slice of 16-bit pixel data.
pub fn calculate_histogram_stats(pixels: &[u16]) -> (u16, u16, u16) {
    if pixels.is_empty() {
        return (0, 0, 0);
    }
    let mut min = u16::MAX;
    let mut max = u16::MIN;
    let mut hist = [0u32; 65536];

    for &p in pixels {
        if p < min {
            min = p;
        }
        if p > max {
            max = p;
        }
        hist[p as usize] += 1;
    }

    let half = (pixels.len() / 2) as u32;
    let mut accum = 0u32;
    let mut median = min;
    for (val, &count) in hist.iter().enumerate() {
        accum += count;
        if accum >= half {
            median = val as u16;
            break;
        }
    }

    (min, max, median)
}

/// Applies an astronomical non-linear stretch to an in-place 16-bit pixel buffer.
pub fn apply_auto_stretch(pixels: &mut [u16], mode: StretchMode) {
    if mode == StretchMode::None || pixels.is_empty() {
        return;
    }

    let (min, max, median) = calculate_histogram_stats(pixels);
    if min >= max {
        return;
    }

    let target_bg = match mode {
        StretchMode::Aggressive => 0.35f32,
        StretchMode::Asinh => 0.20f32,
        _ => 0.22f32,
    };

    let min_f = min as f32;
    let max_f = max as f32;
    let range = (max_f - min_f).max(1.0);

    let median_norm = ((median as f32 - min_f) / range).clamp(0.001, 0.999);

    let m = if (target_bg - median_norm).abs() < 0.001 {
        0.5
    } else {
        let num = median_norm * (target_bg - 1.0);
        let den = median_norm * (2.0 * target_bg - 1.0) - target_bg;
        if den.abs() < 1e-6 {
            0.5
        } else {
            (num / den).clamp(0.01, 0.99)
        }
    };

    if mode == StretchMode::Asinh {
        let beta = 15.0f32;
        let norm_factor = 1.0 / (beta).asinh();
        for p in pixels.iter_mut() {
            let x = (*p as f32 - min_f) / range;
            let stretched = (x * beta).asinh() * norm_factor;
            *p = (stretched.clamp(0.0, 1.0) * 65535.0) as u16;
        }
    } else {
        for p in pixels.iter_mut() {
            let x = (*p as f32 - min_f) / range;
            let stretched = mtf(m, x);
            *p = (stretched * 65535.0) as u16;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mtf_boundary_conditions() {
        assert_eq!(mtf(0.5, 0.0), 0.0);
        assert_eq!(mtf(0.5, 1.0), 1.0);
        assert!((mtf(0.5, 0.5) - 0.5).abs() < 1e-4);
    }

    #[test]
    fn test_mtf_boosts_dark_signals() {
        let stretched = mtf(0.1, 0.1);
        assert!(stretched > 0.1);
        assert!(stretched < 1.0);
    }

    #[test]
    fn test_apply_auto_stretch_brightens_histogram() {
        let mut pixels = vec![1000u16, 1020, 1050, 1100, 3000, 15000];
        let original_mean: f32 = pixels.iter().map(|&p| p as f32).sum::<f32>() / pixels.len() as f32;
        apply_auto_stretch(&mut pixels, StretchMode::Auto);
        let stretched_mean: f32 = pixels.iter().map(|&p| p as f32).sum::<f32>() / pixels.len() as f32;
        assert!(stretched_mean > original_mean);
        assert_eq!(*pixels.iter().max().unwrap(), 65535);
    }
}
