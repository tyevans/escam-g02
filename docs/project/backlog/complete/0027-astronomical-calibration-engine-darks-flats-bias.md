---
id: '0027'
title: Astronomical Calibration Engine for Master Darks, Flats, and Bias Subtraction
status: Complete
governing_adrs:
  - ADR-0022
governing_prds:
  - PRD-0008
governing_stories:
  - US-0019
target_bc: astro
mutation_scope:
  - crates/escam-astro/src/calibrator.rs
---

# TASK-0027: Astronomical Calibration Engine for Master Darks, Flats, and Bias Subtraction

## Problem Statement & Context
CMOS sensors at ambient temperature generate thermal dark current and fixed-pattern noise (hot pixels), and optical lenses introduce vignetting. To achieve scientific quality, raw 16-bit frames must be calibrated using master dark, flat, and bias frames before stacking and FITS export.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `CalibrationEngine` in `crates/escam-astro/src/calibrator.rs`:
   - Store optional `master_dark`, `master_flat`, and `master_bias` buffers (16-bit unsigned / 32-bit float).
   - Provide `calibrate_frame(&self, raw: &[u16], width: usize, height: usize, pedestal: u16) -> Vec<u16>`.
   - Provide accumulator to average multiple dark/flat/bias frames into masters.
2. Integrate `CalibrationEngine` into `FITS` export and `LiveStacker`.
3. Expose REST endpoints:
   - `POST /api/v1/astro/calibration/dark` (capture and set master dark)
   - `POST /api/v1/astro/calibration/flat` (capture and set master flat)
   - `GET /api/v1/astro/calibration/status`
4. Add comprehensive frontdoor tests and property invariants verifying linear math and boundary clamping.
5. All source files strictly <500 lines (target <400 lines).
