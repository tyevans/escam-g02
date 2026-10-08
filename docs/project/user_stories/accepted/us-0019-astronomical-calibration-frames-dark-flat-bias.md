---
id: '0019'
title: Master Dark, Flat, and Bias Astronomical Calibration Frame Engine
status: Accepted
created: 2026-10-06
persona: Marcus
target_bc: astro
feature: FEAT-ASTRO-05
governing_prd: PRD-0008
governing_adrs:
  - ADR-0022
scenarios:
  - Capture and generate master dark frame with closed optical shutter
  - Apply online calibration subtraction to raw 16-bit linear frame
  - Normalize flat field vignetting correction
---

# US-0019 — Master Dark, Flat, and Bias Astronomical Calibration Frame Engine

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0022: Astronomical Calibration Frames and WCS Metadata`](../../adrs/accepted/adr-0022-astronomical-calibration-frames-and-wcs-metadata.md)

## User Story
**As a** Dr. Marcus Vance (Astrophotographer),
**I want** to capture and apply Master Dark, Flat, and Bias calibration frames directly inside the camera engine,
**So that** fixed-pattern thermal noise, hot pixels, and optical vignetting are eliminated from raw 16-bit FITS captures before post-processing.

## Acceptance Criteria

```gherkin
Scenario: Capture and generate master dark frame with closed optical shutter
  Given the camera has an active raw sensor pipeline
  When a master dark capture is requested for 10.0 seconds with 5 averaged frames
  Then the system accumulates 5 dark frames into a 16-bit master dark buffer
  And hot pixels with values exceeding 3-sigma background are preserved in the master dark profile.

Scenario: Apply online calibration subtraction to raw 16-bit linear frame
  Given an active master dark frame loaded in the calibrator
  When a raw light frame is acquired
  Then the calibrator computes calibrated = max(0, light - dark + pedestal) for every pixel
  And the exported FITS image metadata records CALIBRAT = 'DARK' in its header.

Scenario: Normalize flat field vignetting correction
  Given an active master flat field buffer
  When a calibrated light frame is processed through flat normalization
  Then corner vignetting attenuation is compensated by the inverse flat field profile.
```
