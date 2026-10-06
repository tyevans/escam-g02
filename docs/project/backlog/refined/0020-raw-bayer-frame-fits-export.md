---
id: '0020'
title: Raw 10-bit Bayer Frame Capture from Sensor Pipeline for FITS Export
status: Refined
governing_adrs:
- ADR-0014
- ADR-0018
governing_prds:
- PRD-0002
governing_stories:
- US-0008
- US-0009
target_bc: astro
persona: Elena
mutation_scope:
- crates/escam-astro/src/fits.rs
---

# TASK-0020: Raw 10-bit Bayer Frame Capture from Sensor Pipeline for FITS Export

## Problem Statement & Context
For deep-sky astrophotography, Elena needs uncompressed 10-bit Bayer RGGB raw sensor frames dumped into standard NASA FITS files rather than lossy compressed H.264 video.

## Definition of Done (Blackbox Frontdoor TDD)
1. Hook the VI (Video Input) sensor dump into `escam-astro::bayer`.
2. Format raw Bayer grid into 2880-byte padded FITS primary header and data units.
3. Expose FITS snapshot endpoint `/api/v1/astro/capture.fits`.
4. Validate FITS file integrity with `astropy` or `fitsinfo` reader in `tests/test_frontdoors.py`.
5. All files strictly under 500 lines.
