---
id: '0037'
title: Vendor Firmware Debloater and Clean-Boot Security Lockdown
status: Complete
governing_adrs:
  - ADR-0010
  - ADR-0019
governing_prds:
  - PRD-0008
governing_stories:
  - US-0029
target_bc: system
mutation_scope:
  - crates/escam-system/src/debloat.rs
---

# TASK-0037: Vendor Firmware Debloater and Clean-Boot Security Lockdown

## Problem Statement & Context
The camera still contains obsolete vendor directories, cloud daemons, and audio prompts taking up flash storage and posing security risks. We need a robust debloater and clean boot sequence to ensure pure open-source operation and free flash headroom.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `Debloater` in `crates/escam-system/src/debloat.rs`:
   - Safe inspection and removal of non-essential vendor binaries and assets (`/mnt/mtd/ipc/web`, audio prompt files, dead scripts).
   - Validation that essential hardware kernel drivers (`hal.ko`, `media.ko`, `sensor.ko`, `gc1034_ex.ko`, `motor.ko`, `gkio.ko`) remain intact.
   - Generation of a clean, hardened `/mnt/mtd/ipc/conf/run` init script that starts `escamd` with watchdog supervision and zero vendor background daemons.
2. Expose REST endpoints:
   - `GET /api/v1/system/flash`
   - `POST /api/v1/system/debloat`
3. Frontdoor tests verifying protected system paths are never deleted and flash calculation is accurate.
4. All source files strictly <500 lines (target <400 lines).
