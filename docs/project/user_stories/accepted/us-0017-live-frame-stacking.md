---
id: '0017'
title: In-Memory Multi-Frame Integration and Real-time Bayer Stacking
status: Accepted
created: 2026-10-06
persona: Marcus
target_bc: astro
feature: FEAT-ASTRO-03
governing_prd: PRD-0007
governing_adrs:
  - ADR-0021
scenarios:
  - Accumulate multiple Bayer frames in high-depth integer buffers
  - Normalize stacked frames and serialize to 16-bit FITS files
---

# US-0017 — In-Memory Multi-Frame Integration and Real-time Bayer Stacking

## Governing PRD & ADR
- [`PRD-0007: Performance Optimization, Binary Footprint Shrinking, and Deep-Sky Astrophotography`](../../product/accepted/prd-0007-optimization-binary-shrinking-and-deep-sky-astronomy.md)
- [`ADR-0021: Binary Footprint Reduction, Zero-Allocation Media Pipelines, and Sidereal Tracking`](../../adrs/accepted/adr-0021-binary-footprint-reduction-and-zero-allocation-pipeline.md)

## User Story
**As a** Dr. Marcus Vance (Astrophotographer),
**I want** `escam-astro` to accumulate multiple Bayer frames in a 32-bit in-memory buffer,
**So that** faint deep-sky astronomical signals can be integrated above the sensor's thermal noise floor and saved as 16-bit FITS.

## Acceptance Criteria

```gherkin
Scenario: Accumulate multiple Bayer frames in high-depth integer buffers
  Given an in-memory FrameStacker with capacity for 16 frames
  When consecutive 8-bit or 10-bit raw Bayer frames are pushed
  Then pixel values are accumulated in a 32-bit buffer without overflow.

Scenario: Normalize stacked frames and serialize to 16-bit FITS files
  Given an accumulated stack of 16 frames
  When export_fits is invoked with average normalization
  Then a valid FITS file is generated with BITPIX=16
  And total RAM usage for the stack remains strictly under 10MB.
```
