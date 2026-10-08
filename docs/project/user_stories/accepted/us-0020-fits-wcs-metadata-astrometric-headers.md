---
id: '0020'
title: Full FITS WCS Astrometric Metadata and IAU Standard Headers
status: Accepted
created: 2026-10-06
persona: Marcus
target_bc: astro
feature: FEAT-ASTRO-06
governing_prd: PRD-0008
governing_adrs:
  - ADR-0022
scenarios:
  - Generate IAU standard primary FITS header with astrometric WCS projection
  - Populate equatorial RA/DEC from mount Alt/Az and local sidereal time
  - Validate FITS block padding and key formatting
---

# US-0020 — Full FITS WCS Astrometric Metadata and IAU Standard Headers

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0022: Astronomical Calibration Frames and WCS Metadata`](../../adrs/accepted/adr-0022-astronomical-calibration-frames-and-wcs-metadata.md)

## User Story
**As a** Dr. Marcus Vance (Astrophotographer),
**I want** exported FITS images to contain full World Coordinate System (WCS) headers, geographic observatory parameters, and ISO UTC exposure timestamps,
**So that** plate-solving tools (Astrometry.net, Siril, PixInsight) immediately recognize celestial coordinates and pixel scale without manual calibration.

## Acceptance Criteria

```gherkin
Scenario: Generate IAU standard primary FITS header with astrometric WCS projection
  Given a 1280x720 16-bit linear frame with focal length and pixel pitch configured
  When FITS serialization is executed with active WCS metadata
  Then the primary header contains CTYPE1 = 'RA---TAN' and CTYPE2 = 'DEC--TAN'
  And the primary header contains CRPIX1 = 640.0 and CRPIX2 = 360.0
  And the primary header contains CDELT1 and CDELT2 matching the sensor angular resolution.

Scenario: Populate equatorial RA/DEC from mount Alt/Az and local sidereal time
  Given a PTZ mount oriented at Azimuth 180.0 deg and Altitude 45.0 deg
  And an observatory site at Latitude 37.77 deg and Longitude -122.42 deg
  When the astrometric solver converts horizontal to equatorial coordinates
  Then CRVAL1 (RA) and CRVAL2 (DEC) are correctly computed and stamped into the header.

Scenario: Validate FITS block padding and key formatting
  Given serialized FITS bytes
  Then the total byte length is an exact positive multiple of 2880 bytes
  And every header line is exactly 80 characters long ending with valid space padding.
```
