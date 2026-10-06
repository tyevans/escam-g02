---
id: '0010'
title: Embedded INDI CCD and Telescope Protocol Server
status: Complete
governing_adrs:
- ADR-0018
- ADR-0003
governing_prds:
- PRD-0004
governing_stories:
- US-0009
target_bc: astro
persona: Marcus
mutation_scope:
- crates/escam-astro/src/indi.rs
---

# TASK-0010: Embedded INDI CCD and Telescope Protocol Server

## Problem Statement & Context
To seamlessly integrate with Ekos, KStars, PHD2, and NINA without proprietary drivers, the camera must host an embedded INDI protocol server on standard TCP port 7624, supporting CCD exposure commands, BLOB FITS frame delivery, and IR-cut filter slot switching.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement async Tokio TCP listener on port 7624 handling INDI XML messages (`getProperties`, `newNumberVector`, `newSwitchVector`, `enableBLOB`).
2. Implement device state machine for `ESCAM G02 CCD` and `ESCAM G02 Telescope Mount`.
3. Support simulated and hardware-backed exposures returning uncompressed FITS BLOBs.
4. Support `FILTER_SLOT` switch controlling the UTC BA6208L IR-cut solenoid.
5. All new files strictly under 500 lines.
