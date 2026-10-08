---
id: '0034'
title: Precision Astronomical Clock, SNTP Synchronization, and Exposure Midpoint Stamping
status: Complete
governing_adrs:
  - ADR-0022
governing_prds:
  - PRD-0008
governing_stories:
  - US-0026
target_bc: system
mutation_scope:
  - crates/escam-system/src/clock.rs
---

# TASK-0034: Precision Astronomical Clock, SNTP Synchronization, and Exposure Midpoint Stamping

## Problem Statement & Context
Astronomical observations demand microsecond-accurate UTC timestamps for occultations, asteroid tracking, and astrometric catalog reduction. The camera clock must synchronize via SNTP and stamp accurate exposure midpoint (`DATE-AVG`) into all FITS headers.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `AstroClock` in `crates/escam-system/src/clock.rs`:
   - Embedded SNTP client with round-trip delay compensation and server drift estimation.
   - Calculation of Julian Date (JD), Modified Julian Date (MJD), and Greenwich/Local Sidereal Time (LST).
   - Accurate calculation of exposure start (`DATE-OBS`) and exposure midpoint (`DATE-AVG`).
2. Expose REST endpoints:
   - `GET /api/v1/system/time`
   - `POST /api/v1/system/time` (manual timestamp & GPS coordinates)
   - `POST /api/v1/system/sntp/sync`
3. Frontdoor tests verifying Julian Date calculation against USNO reference values and SNTP packet formatting.
4. All source files strictly <500 lines (target <400 lines).
