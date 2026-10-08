---
id: '0022'
title: Astronomical Calibration Frames, Master Dark/Flat/Bias Subtraction, and WCS FITS Metadata
status: Accepted
date: 2026-10-06
deciders: Lead Architect, Dr. Marcus Vance, Elena
governing_prds:
  - PRD-0008
---

# ADR-0022: Astronomical Calibration Frames, Master Dark/Flat/Bias Subtraction, and WCS FITS Metadata

## Context & Problem Statement
Scientific astrophotography with low-cost CMOS sensors like the GalaxyCore GC1034 suffers from thermal dark current, fixed-pattern noise (hot pixels), read noise, and lens vignetting. Without calibration frames, stacked deep-sky images exhibit severe noise and vignetting gradients. Furthermore, downstream astronomical tools (KStars, Astrometry.net, Siril) require standard IAU FITS headers and World Coordinate System (WCS) metadata to perform plate-solving and photometric analysis.

## Decision Drivers
- **Mathematical Soundness**: Linear pixel calibration must strictly preserve 16-bit dynamic range without zero-clipping background flux.
- **WCS Compliance**: FITS headers must adhere strictly to IAU standard FITS 4.0 specifications with correct coordinate transforms.
- **Bounded RAM Footprint**: Calibration masters must reside in static 16-bit integer buffers without consuming excessive heap (<4MB per master).

## Decision Outcome
We implement an in-memory `CalibrationEngine` in `escam-astro`:
1. **Master Calibration Frame Generation**:
   - `MasterDark`: Average or median of $N$ dark frames taken with IR-cut shutter closed or lens covered at matching exposure and temperature.
   - `MasterBias`: Read-noise baseline captured at minimum exposure time ($1/1000\text{s}$).
   - `MasterFlat`: Normalized vignetting correction frame.
2. **Online Calibration Pipeline**:
   - $\text{Calibrated}(x,y) = \frac{\text{Raw}(x,y) - \text{Dark}(x,y) - \text{Bias}(x,y)}{\text{Flat}(x,y) / \bar{\text{Flat}}} + \text{Pedestal}$.
3. **WCS Header Generator**:
   - Computes `CRVAL1` (RA), `CRVAL2` (DEC), `CRPIX1`, `CRPIX2`, `CDELT1`, `CDELT2`, `CTYPE1 = 'RA---TAN'`, `CTYPE2 = 'DEC--TAN'` based on current PTZ mount azimuth/elevation converted to celestial equatorial coordinates via local sidereal time (LST).
