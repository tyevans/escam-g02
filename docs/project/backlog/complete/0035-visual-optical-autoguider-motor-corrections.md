---
id: '0035'
title: Optical Closed-Loop Autoguider and Micro-Pulse Motor Corrections
status: Complete
governing_adrs:
  - ADR-0023
governing_prds:
  - PRD-0008
governing_stories:
  - US-0027
target_bc: ptz
mutation_scope:
  - crates/escam-ptz/src/autoguide.rs
---

# TASK-0035: Optical Closed-Loop Autoguider and Micro-Pulse Motor Corrections

## Problem Statement & Context
During multi-minute deep sky exposures, periodic error, wind gusts, and mount alignment tolerances cause star drift, turning pinpoint stars into streaks. The optical autoguider measures guide star drift in real time and commands closed-loop micro-pulse motor corrections.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `AutoGuider` in `crates/escam-ptz/src/autoguide.rs`:
   - Lock on guide star coordinates $(x_{ref}, y_{ref})$.
   - Proportional-Integral (PI) loop calculating correction pulse durations in milliseconds:
     $$\Delta t_x = K_p \cdot e_x + K_i \int e_x dt, \quad \Delta t_y = K_p \cdot e_y + K_i \int e_y dt$$
   - Deadband filter to prevent hunting on atmospheric scintillation.
   - Dispatch pulse corrections directly to `PtzController`.
2. Expose REST endpoints:
   - `POST /api/v1/astro/guide/start`
   - `POST /api/v1/astro/guide/stop`
   - `GET /api/v1/astro/guide/status`
3. Frontdoor tests verifying PI loop convergence on synthetic drift sequences.
4. All source files strictly <500 lines (target <400 lines).
