---
id: '0028'
title: World Coordinate System (WCS) and Astrometric FITS Metadata Generation
status: Complete
governing_adrs:
  - ADR-0022
governing_prds:
  - PRD-0008
governing_stories:
  - US-0020
target_bc: astro
mutation_scope:
  - crates/escam-astro/src/wcs.rs
---

# TASK-0028: World Coordinate System (WCS) and Astrometric FITS Metadata Generation

## Problem Statement & Context
Astronomical software suites require standard IAU WCS headers (`CRVAL1`, `CRVAL2`, `CRPIX1`, `CRPIX2`, `CDELT1`, `CDELT2`, `CTYPE1`, `CTYPE2`) and equatorial RA/DEC coordinates to plate-solve images and overlay star catalogs.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `WcsGenerator` in `crates/escam-astro/src/wcs.rs`:
   - Convert Horizontal coordinates (Azimuth, Altitude) to Equatorial coordinates (Right Ascension, Declination) given site Latitude, Longitude, and Local Sidereal Time.
   - Compute TAN (gnomonic) tangent plane projection matrix (`CDELT1`, `CDELT2`, `CROTA2`).
   - Format 80-column FITS header cards conforming to FITS 4.0 standards.
2. Integrate WCS metadata into `crates/escam-astro/src/fits.rs`.
3. Provide unit and property tests verifying mathematical round-tripping and FITS 2880-byte block padding.
4. All source files strictly <500 lines (target <400 lines).
