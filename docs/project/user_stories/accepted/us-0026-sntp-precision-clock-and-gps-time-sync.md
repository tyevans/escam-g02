---
id: '0026'
title: Sub-Millisecond SNTP Precision Astronomical Clock and Midpoint Stamping
status: Accepted
created: 2026-10-06
persona: Marcus
target_bc: system
feature: FEAT-SYS-03
governing_prd: PRD-0008
governing_adrs:
  - ADR-0022
scenarios:
  - Synchronize system clock with external SNTP server
  - Compute high-precision exposure midpoint timestamp DATE-AVG
  - Provide HTTP endpoint for setting manual or GPS NMEA coordinates and time
---

# US-0026 — Sub-Millisecond SNTP Precision Astronomical Clock and Midpoint Stamping

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0022: Astronomical Calibration Frames and WCS Metadata`](../../adrs/accepted/adr-0022-astronomical-calibration-frames-and-wcs-metadata.md)

## User Story
**As a** Dr. Marcus Vance (Astrophotographer),
**I want** the camera system clock to maintain sub-millisecond precision via SNTP and record the exact midpoint timestamp (`DATE-AVG`) of each exposure,
**So that** time-critical observations such as asteroid occultations and exoplanet transits have microsecond-accurate temporal calibration.

## Acceptance Criteria

```gherkin
Scenario: Synchronize system clock with external SNTP server
  Given network connectivity to a configured NTP/SNTP time server (e.g. pool.ntp.org)
  When the precision clock engine synchronizes
  Then the local system clock is disciplined with sub-millisecond offset estimation.

Scenario: Compute high-precision exposure midpoint timestamp DATE-AVG
  Given an exposure initiated at T_start with duration T_duration
  When FITS metadata is generated
  Then DATE-OBS records T_start in UTC ISO 8601 format
  And DATE-AVG records T_start + (T_duration / 2.0).

Scenario: Provide HTTP endpoint for setting manual or GPS NMEA coordinates and time
  Given a mobile field observation without Internet access
  When a client sends `POST /api/v1/system/time` with timestamp, latitude, and longitude
  Then system time and site coordinates are atomically updated.
```
