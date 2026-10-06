---
id: '0018'
title: Astrophotography Engine, Raw 10-bit Bayer Frames, and Embedded INDI Protocol
status: Accepted
date: 2026-10-05
deciders:
  - Marcus
  - Alex
---

# ADR-0018: Astrophotography Engine, Raw 10-bit Bayer Frames, and Embedded INDI Protocol

## Status
Accepted

## Context
The ESCAM G02 features either a SmartSens SC1135 or GalaxyCore GC1034 image sensor with an M12 lens mount that can be unscrewed and replaced with an M12-to-1.25" telescope nosepiece. With 3.75µm pixels on SC1135, it matches the physical characteristics of classic planetary/guiding cameras (e.g. ZWO ASI120MC). However, standard CCTV camera software compresses video with lossy 8-bit H.264 codecs, obliterating astronomical signal. Amateur astronomers require raw Bayer frame access, manual sub-exposures (100ms - 5000ms), FITS file output, and integration with astronomy software suites (Ekos, KStars, NINA, Siril).

## Decision
1. Implement a raw frame grabber module in Rust reading uncompressed 10-bit / 12-bit Bayer frames from the Goke Video Input (`/dev/vi` / ISP capture buffer).
2. Implement standard FITS (Flexible Image Transport System) file writer in pure Rust:
   - Formats headers with standard astronomy metadata (TELESCOP, INSTRUME, EXPTIME, DATE-OBS, BAYERPAT, BITPIX=16).
3. Implement an embedded INDI (Instrument-Neutral Distributed Interface) server in Rust listening on TCP port 7624:
   - Implements INDI standard `INDI::CCD` and `INDI::Telescope` device protocols.
   - Allows astronomy platforms (KStars/Ekos, PHD2 Guiding, NINA) to detect the camera over Wi-Fi/Ethernet as a native astronomy CCD and mount.
4. Expose an API endpoint and UI toggle for the mechanical IR-cut filter solenoid to flip between daytime IR-blocking and nighttime Hydrogen-Alpha / full-spectrum imaging.

## Consequences
- **Positive**: Converts a $10 Goodwill security camera into a full-featured planetary imager, telescope autoguider, and meteor detector compatible with all major astronomy software suites.
- **Negative**: Long exposures (>1s) require sensor exposure registers to be programmed directly via I2C (`/dev/i2c-0`) or ISP APIs.
