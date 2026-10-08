---
id: '0029'
title: Real-Time Star Centroiding and FWHM Seeing Focus Analyzer
status: Complete
governing_adrs:
  - ADR-0023
governing_prds:
  - PRD-0008
governing_stories:
  - US-0021
target_bc: astro
mutation_scope:
  - crates/escam-astro/src/stars.rs
---

# TASK-0029: Real-Time Star Centroiding and FWHM Seeing Focus Analyzer

## Problem Statement & Context
Critical focus and atmospheric seeing estimation require detecting point-spread functions (PSFs) of celestial stars in the image, computing their flux-weighted centroid to sub-pixel accuracy, and determining their Full-Width at Half-Maximum (FWHM).

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `StarDetector` in `crates/escam-astro/src/stars.rs`:
   - Threshold background via dynamic standard deviation calculation ($T = \mu + k\sigma$).
   - Connected component clustering with minimum pixel count filter to reject isolated hot pixels.
   - Intensity-weighted centroid estimation: $x_c = \frac{\sum x I}{\sum I}, y_c = \frac{\sum y I}{\sum I}$.
   - Radial profile second moment calculation for FWHM in pixels and arcseconds.
2. Expose REST endpoint:
   - `GET /api/v1/astro/focus` returning detected star count, list of top stars with centroids and FWHM, and median FWHM seeing score.
3. Unit and property-based tests verifying centroid accuracy against synthesized Gaussian star fields.
4. All source files strictly <500 lines (target <400 lines).
