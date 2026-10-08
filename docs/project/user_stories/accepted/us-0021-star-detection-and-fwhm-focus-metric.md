---
id: '0021'
title: Real-Time Star Detection, Centroiding, and FWHM Seeing Focus Metric
status: Accepted
created: 2026-10-06
persona: Marcus
target_bc: astro
feature: FEAT-ASTRO-07
governing_prd: PRD-0008
governing_adrs:
  - ADR-0023
scenarios:
  - Detect celestial star candidates above background noise
  - Compute sub-pixel centroid coordinates via intensity weighting
  - Measure radial Full-Width at Half-Maximum (FWHM) focus metric
---

# US-0021 — Real-Time Star Detection, Centroiding, and FWHM Seeing Focus Metric

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0023: Real-Time Star Detection and FWHM Seeing Analysis`](../../adrs/accepted/adr-0023-star-detection-fwhm-and-visual-autoguiding.md)

## User Story
**As a** Dr. Marcus Vance (Astrophotographer),
**I want** the camera to detect stars in the field of view and compute their sub-pixel centroids and FWHM seeing score in real-time,
**So that** I can achieve razor-sharp manual focus and quantify optical seeing conditions objectively.

## Acceptance Criteria

```gherkin
Scenario: Detect celestial star candidates above background noise
  Given a 16-bit linear frame with simulated stars and Gaussian background noise
  When the star detector analyzes the frame with threshold 3.5 sigma
  Then candidate peaks are identified while single-pixel hot noise spikes are rejected.

Scenario: Compute sub-pixel centroid coordinates via intensity weighting
  Given a candidate star profile spanning a 5x5 window
  When the centroid algorithm calculates the flux-weighted first moment
  Then the returned centroid (x, y) resolves to within 0.1 pixels of the true simulated center.

Scenario: Measure radial Full-Width at Half-Maximum (FWHM) focus metric
  Given a detected star with a Gaussian radial distribution
  When the FWHM estimator calculates the second moment profile
  Then the reported FWHM in pixels matches the ground truth distribution within 10% tolerance
  And the seeing metric is exposed via HTTP endpoint `/api/v1/astro/focus`.
```
