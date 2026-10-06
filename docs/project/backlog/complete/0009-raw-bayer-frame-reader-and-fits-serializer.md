---
id: 0009
title: Raw Bayer Sensor Frame Reader and FITS Serializer
status: Complete
governing_adrs:
- ADR-0018
- ADR-0003
governing_prds:
- PRD-0004
governing_stories:
- US-0008
target_bc: astro
mutation_scope:
- crates/escam-astro/src/fits.rs
- crates/escam-astro/src/bayer.rs
---

# TASK-0009: Raw Bayer Sensor Frame Reader and FITS Serializer

## Problem Statement & Context
Astronomical lucky imaging and deep-sky astrophotography require uncompressed, linear Bayer frames rather than 8-bit lossy H.264 video. `escam-astro` must read raw 10-bit RGGB frames and serialize them into standard 16-bit FITS containers with astronomy metadata headers.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `BayerFrame` representation for 10-bit / 12-bit sensor data.
2. Implement pure Rust `FitsWriter` generating valid FITS blocks (2880-byte header records, 16-bit big-endian image array, proper padding).
3. Validate output FITS headers: BITPIX=16, NAXIS=2, NAXIS1, NAXIS2, BZERO=32768, BSCALE=1, BAYERPAT='RGGB', EXPTIME, DATE-OBS.
4. Blackbox tests verify generated FITS files can be parsed back and match raw sensor pixel values exactly.
5. All new files strictly under 500 lines.
