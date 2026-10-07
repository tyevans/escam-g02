---
id: '0026'
title: Embedded INDI Telescope Mount Driver for PTZ Actuation
status: Complete
governing_adrs:
- ADR-0018
- ADR-0021
governing_prds:
- PRD-0007
governing_stories:
- US-0018
target_bc: astro
mutation_scope:
- crates/escam-astro/src/indi.rs
---

# TASK-0026: Embedded INDI Telescope Mount Driver for PTZ Actuation

## Problem Statement & Context
Currently, the INDI server in `crates/escam-astro/src/indi.rs` only defines the `ESCAM G02 CCD` camera properties (exposure, filter slot). To enable planetary framing, telescope autoguiding, and mount slewing from Ekos, KStars, and Stellarium, the Pan and Tilt stepper motors must be exposed as a standard INDI Mount/Telescope device (`ESCAM G02 Mount`).

## Definition of Done (Blackbox Frontdoor TDD)
1. Enhance `crates/escam-astro/src/indi.rs` to expose `ESCAM G02 Mount` alongside `ESCAM G02 CCD`:
   - `TELESCOPE_MOTION_NS` (North/South = Tilt Up/Down).
   - `TELESCOPE_MOTION_WE` (West/East = Pan Left/Right).
   - `TELESCOPE_ABORT_MOTION` (Instant halt).
   - `TELESCOPE_PARK` (Park to home position).
   - `HORIZONTAL_COORD` (Current Azimuth / Altitude degrees).
2. Wire `IndiServer` with an optional `PtzController` hook so incoming directional switch events actuate the physical motors asynchronously.
3. Handle incoming `<newSwitchVector>` and `<newNumberVector>` XML commands for both the CCD and Mount devices.
4. Add unit and integration tests verifying XML parsing and motor command dispatch.
5. Deploy to ESCAM G02 and verify connection from INDI clients over port 7624.
6. All source files strictly under 500 lines (proactive check: <400 lines).
