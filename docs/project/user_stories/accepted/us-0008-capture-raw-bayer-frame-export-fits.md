---
id: '0008'
title: Capture raw 10-bit Bayer frame and export to FITS container
status: Accepted
created: 2026-10-05
persona: Marcus
target_bc: astro
feature: FEAT-ASTRO-01
governing_prd: PRD-0004
scenarios:
  - Ingest uncompressed 10-bit Bayer frame from sensor buffer
  - Serialize frame into standard FITS container with astronomy headers
  - Verify FITS file compatibility with astronomy stacking tools
---

# US-0008 — Capture raw 10-bit Bayer frame and export to FITS container

## Governing PRD
- [`PRD-0004: Astrophotography Subsystem, Raw Bayer Capture, and INDI Protocol`](../../product/accepted/prd-0004-astrophotography-subsystem-and-indi-protocol.md)

## User Story

**As a** Marcus (Astrophotographer & Optical Hacker),
**I want** to capture raw uncompressed 10-bit Bayer sensor frames and save them directly as `.fits` files,
**So that** I can stack deep-sky and planetary frames in Siril and AstroImageJ with full photometric fidelity.

## Acceptance Criteria

```gherkin
Scenario: Ingest uncompressed 10-bit Bayer frame from sensor buffer
  Given the sensor daughterboard configured with GC1034 or SC1135
  When a raw frame capture is triggered
  Then uncompressed 10-bit or 12-bit RGGB Bayer data is extracted from the Video Input ring buffer
  And zero H.264 compression, tone mapping, or lossy chroma subsampling is applied.

Scenario: Serialize frame into standard FITS container with astronomy headers
  Given a raw Bayer frame of dimensions 1280x720 with exposure time 2.5 seconds
  When the FITS exporter generates the output file
  Then the primary HDU contains standard 80-character header cards including BITPIX=16, NAXIS=2, NAXIS1=1280, NAXIS2=720, EXPTIME=2.5, and BAYERPAT='RGGB'
  And image data is padded to standard 2880-byte FITS records.

Scenario: Verify FITS file compatibility with astronomy stacking tools
  Given an exported .fits file from the camera
  When opened in astronomical software (Siril, DS9, PixInsight)
  Then the image displays with correct raw ADU values and valid Bayer matrix interpolation.
```
