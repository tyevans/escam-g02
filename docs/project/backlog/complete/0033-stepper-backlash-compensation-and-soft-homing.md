---
id: '0033'
title: Stepper Motor Mechanical Backlash Compensation and Autonomous Soft-Homing
status: Complete
governing_adrs:
  - ADR-0016
governing_prds:
  - PRD-0008
governing_stories:
  - US-0025
target_bc: ptz
mutation_scope:
  - crates/escam-ptz/src/backlash.rs
---

# TASK-0033: Stepper Motor Mechanical Backlash Compensation and Autonomous Soft-Homing

## Problem Statement & Context
Spur gear trains in small pan-tilt mechanisms have mechanical backlash (gear teeth slack). When reversing direction, several steps are lost before the gear meshes. Furthermore, without a soft-homing routine, absolute position reference can drift across power cycles.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `BacklashCompensator` in `crates/escam-ptz/src/backlash.rs`:
   - Configurable backlash steps per axis (e.g. 6-12 steps for Pan, 4-8 steps for Tilt).
   - Track last movement direction; on direction reversal, automatically inject compensation steps before executing trajectory.
2. Implement soft-homing calibration routine:
   - Slew to physical hardstops at low speed, detect limit, center to midpoint step (e.g. step 260), and reset logical coordinates.
3. Expose REST endpoints:
   - `POST /api/v1/ptz/home`
   - `POST /api/v1/ptz/backlash`
   - `GET /api/v1/ptz/backlash`
4. Frontdoor tests verifying step injection on direction reversal and zero backlash on unidirectional movement.
5. All source files strictly <500 lines (target <400 lines).
