---
id: '0025'
title: In-Memory Multi-Frame Integration and Real-time Bayer Stacking
status: Complete
governing_adrs:
- ADR-0018
- ADR-0021
governing_prds:
- PRD-0007
governing_stories:
- US-0017
target_bc: astro
mutation_scope:
- crates/escam-astro/src/stacker.rs
---

# TASK-0025: In-Memory Multi-Frame Integration and Real-time Bayer Stacking

## Problem Statement & Context
The GalaxyCore GC1034 sensor has modest low-light sensitivity. For planetary and deep-sky astrophotography, stacking multiple short exposures significantly boosts the signal-to-noise ratio (SNR) by averaging out random thermal and read noise.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `BayerStacker` in `crates/escam-astro/src/stacker.rs` with 32-bit accumulation buffers.
2. Support `Additive` and `Average` stacking modes with optional 16-bit normalization.
3. Keep RAM usage bounded ($< 6$ MB for $1280\times 720\times 4$ bytes).
4. Integrate with `FITS` serialization pipeline to produce stacked 16-bit FITS files.
5. All source files strictly under 500 lines.
