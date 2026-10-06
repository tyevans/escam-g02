---
id: '0007'
title: Jerk-Limited S-Curve Trajectory Profile Generator
status: Complete
governing_adrs:
- ADR-0016
- ADR-0003
governing_prds:
- PRD-0003
governing_stories:
- US-0006
target_bc: ptz
persona: Samir
mutation_scope:
- crates/escam-ptz/src/scurve.rs
---

# TASK-0007: Jerk-Limited S-Curve Trajectory Profile Generator

## Problem Statement & Context
Square-wave step pulses in the stock firmware cause harsh clicking, resonance, and mount shake. `escam-ptz` needs a deterministic, jerk-limited sinusoidal S-curve trajectory profile generator implemented using integer/fixed-point arithmetic for the 600MHz ARM11 target.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `ScurveGenerator` in `crates/escam-ptz/src/scurve.rs` calculating continuous velocity and step intervals.
2. Property-based tests via Hypothesis / proptest verify mathematical invariants:
   - Velocity is strictly non-negative and never exceeds $V_{\max}$.
   - Position delta matches exact requested step count without cumulative drift.
   - Acceleration derivative (jerk) is continuous and bounded by $J_{\max}$.
3. All new files strictly under 500 lines.
