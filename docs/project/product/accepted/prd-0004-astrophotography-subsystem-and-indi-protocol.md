---
id: '0004'
title: Astrophotography Subsystem, Raw Bayer Capture, and INDI Protocol
status: Accepted
created: 2026-10-05
target_persona: Marcus
component: astro
---

# PRD-0004 — Astrophotography Subsystem, Raw Bayer Capture, and INDI Protocol

## Who this is for

- **Marcus**: Backyard astrophotographer and telescope user wanting uncompressed 10-bit raw frames, manual long exposures, FITS export, and integration with Ekos/NINA/Siril.

## What the person cannot do today

- Today, the camera outputs only compressed 8-bit H.264 video at 25fps with aggressive noise reduction and gamma curves applied by the ISP, destroying deep-sky details.
- There is no support for manual sub-exposures (100ms - 5000ms) or raw Bayer frame dumping.
- Astronomy suites like Ekos, KStars, PHD2, and NINA have no driver support for the camera.

## What good looks like

1. **Raw 10-bit Bayer Frame Extraction**:
   - Ingest raw uncompressed Bayer frames (RGGB pattern) from the Goke Video Input / ISP buffer before lossy compression.
2. **Standard Astronomy FITS Writer**:
   - Pure Rust FITS container serializer writing standard 16-bit padded FITS files with astronomical metadata (TELESCOP, EXPTIME, DATE-OBS, INSTRUME, BAYERPAT).
3. **Embedded INDI Driver Server**:
   - Pure Rust INDI protocol server listening on standard TCP port 7624.
   - Responds to `getProperties`, `enableBLOB`, and `startExposure` commands for `INDI::CCD` and `INDI::Telescope`.
4. **IR-Cut Solenoid Toggle for H-Alpha**:
   - One-touch toggle to drop the IR-cut filter for 656.3nm Hydrogen-Alpha and near-infrared astronomy.

## What this does not do

- Deep-sky stacking or image post-processing on the camera itself (handled by host software like Siril, PixInsight, or AutoStakkert).

## Checkable Outcomes

1. Raw frame grab produces valid 1280x720 10-bit RGGB Bayer data without H.264 compression artifacts.
2. Exported `.fits` files open cleanly in DS9, Siril, and AstroImageJ with valid astronomy header metadata.
3. KStars/Ekos connects to camera on port 7624, detects the camera as an INDI CCD device, and triggers test exposures.

## Linked User Stories

- [`US-0008: Capture raw 10-bit Bayer frame and export to FITS container`](../../user_stories/accepted/us-0008-capture-raw-bayer-frame-export-fits.md)
- [`US-0009: Serve INDI CCD protocol over TCP port 7624 for Ekos and NINA`](../../user_stories/accepted/us-0009-serve-indi-ccd-protocol.md)

## Implementing Backlog Tasks

- `TASK-0009`: Implement raw Bayer frame reader and FITS container serializer in `escam-astro`.
- `TASK-0010`: Implement embedded INDI protocol server (TCP 7624) for Ekos/NINA integration.
