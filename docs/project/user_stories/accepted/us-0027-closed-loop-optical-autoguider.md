---
id: '0027'
title: Closed-Loop Optical Autoguider and Sub-Pixel Star Drift Tracking
status: Accepted
created: 2026-10-06
persona: Marcus
target_bc: ptz
feature: FEAT-PTZ-07
governing_prd: PRD-0008
governing_adrs:
  - ADR-0023
scenarios:
  - Lock onto a selected guide star centroid
  - Compute sub-pixel guide error delta (dx, dy)
  - Issue proportional-integral pulse corrections to PTZ stepper motors
---

# US-0027 — Closed-Loop Optical Autoguider and Sub-Pixel Star Drift Tracking

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0023: Real-Time Star Detection and Closed-Loop Optical Autoguiding`](../../adrs/accepted/adr-0023-star-detection-fwhm-and-visual-autoguiding.md)

## User Story
**As a** Dr. Marcus Vance (Astrophotographer),
**I want** the camera to track a selected guide star and automatically issue micro-pulse motor corrections,
**So that** mechanical drift and periodic tracking errors are counteracted during 30-to-120 second exposures.

## Acceptance Criteria

```gherkin
Scenario: Lock onto a selected guide star centroid
  Given a detected star with lock coordinates (x_ref, y_ref)
  When autoguiding mode is activated via `POST /api/v1/astro/guide/start`
  Then the autoguider locks (x_ref, y_ref) as the guide setpoint.

Scenario: Compute sub-pixel guide error delta (dx, dy)
  Given subsequent video/stacking frames
  When the guide star centroid shifts to (x_current, y_current)
  Then the error vector (dx = x_current - x_ref, dy = y_current - y_ref) is computed in arcseconds.

Scenario: Issue proportional-integral pulse corrections to PTZ stepper motors
  Given an error vector exceeding guide tolerance (e.g. 0.2 pixels)
  When the PID autoguider evaluates the correction
  Then timed directional guide pulses (North/South/East/West) are commanded to the stepper motors
  And the star returns to the target lock point within 3 correction cycles.
```
