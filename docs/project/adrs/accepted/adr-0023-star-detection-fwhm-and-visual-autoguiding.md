---
id: '0023'
title: Real-Time Star Detection, FWHM Seeing Analysis, and Closed-Loop Optical Autoguiding
status: Accepted
date: 2026-10-06
deciders: Lead Architect, Dr. Marcus Vance, Alex Rivera
governing_prds:
  - PRD-0008
---

# ADR-0023: Real-Time Star Detection, FWHM Seeing Analysis, and Closed-Loop Optical Autoguiding

## Context & Problem Statement
Long-exposure deep sky astrophotography requires continuous sub-pixel tracking to prevent star trailing caused by mechanical gear backlash, periodic error, and atmospheric refraction. In addition, achieving critical focus manually without visual feedback is challenging. The camera needs on-device computer vision to:
1. Detect celestial stars above local background noise.
2. Measure star Full-Width at Half-Maximum (FWHM) in arcseconds or pixels to provide an objective focus and seeing metric.
3. Track centroid drift over time and emit closed-loop PTZ pulse corrections (autoguiding).

## Decision Drivers
- **Real-Time Performance**: Centroiding and FWHM calculation must execute in $<10\text{ms}$ on ARMv6 without requiring OpenCV or floating-point bloat.
- **Robustness to Hot Pixels**: Isolated single-pixel defects must be filtered out so they are not mistaken for stars.
- **INDI Compatibility**: Autoguider pulses must map seamlessly to INDI `TELESCOPE_TIMED_GUIDE_NS` and `TELESCOPE_TIMED_GUIDE_WE` properties.

## Decision Outcome
We implement `StarDetector` and `AutoGuider` in `escam-astro` and `escam-ptz`:
1. **Star Detection Heuristic**: Local maximum search with background threshold $T = \mu + 3.5\sigma$, requiring a contiguous connected cluster of $\ge 3$ pixels to reject isolated cosmic rays and hot pixels.
2. **Sub-Pixel Centroiding**: Intensity-weighted center of mass:
   $$x_c = \frac{\sum (x \cdot I_{x,y})}{\sum I_{x,y}}, \quad y_c = \frac{\sum (y \cdot I_{x,y})}{\sum I_{x,y}}$$
3. **FWHM Estimation**: Radial profile variance fitting:
   $$\text{FWHM} = 2.355 \cdot \sigma_r$$
4. **PID Autoguider**: Closed-loop proportional-integral pulse generator commanding micro-steps to counteract drift vector $(\Delta x_c, \Delta y_c)$.
