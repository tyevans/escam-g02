---
id: '0024'
title: Celestial Sidereal Rate Tracking Engine for Astrophotography
status: Complete
governing_adrs:
- ADR-0016
- ADR-0021
governing_prds:
- PRD-0007
governing_stories:
- US-0016
target_bc: ptz
mutation_scope:
- crates/escam-ptz/src/sidereal.rs
---

# TASK-0024: Celestial Sidereal Rate Tracking Engine for Astrophotography

## Problem Statement & Context
When imaging stars, planets, and nebulae with the ESCAM G02 adapted to a telescope or wide-angle lens, the Earth's rotation ($15.041067\text{ arcsec/sec}$) causes star trails during long exposures. To counteract this, `escam-ptz` needs a dedicated `SiderealTracker` that pulses the Pan stepper motor at the astronomical sidereal rate.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `SiderealTracker` in `crates/escam-ptz/src/sidereal.rs` calculating step pulse delays based on gear ratio (520 steps per 355 degrees).
2. Support configurable tracking modes: `Sidereal` ($15.041''/\text{s}$), `Lunar` ($14.685''/\text{s}$), `Solar` ($15.000''/\text{s}$).
3. Provide unit and property tests verifying step interval accuracy and bounds.
4. Expose tracking activation over the public `PtzController` interface.
5. All source files strictly under 500 lines.
