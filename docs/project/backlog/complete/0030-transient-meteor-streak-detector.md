---
id: '0030'
title: Autonomous Transient Meteor, Fireball, and Satellite Streak Detector
status: Complete
governing_adrs:
  - ADR-0024
governing_prds:
  - PRD-0008
governing_stories:
  - US-0022
target_bc: astro
mutation_scope:
  - crates/escam-astro/src/transient.rs
---

# TASK-0030: Autonomous Transient Meteor, Fireball, and Satellite Streak Detector

## Problem Statement & Context
Transient astronomical phenomena (meteors, bolides, satellite glints, fireball fragmentation) produce fast, high-contrast linear streaks across sequential frames. The camera should autonomously detect these events, compute their trajectory vector, and trigger automated capture without human intervention.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `TransientDetector` in `crates/escam-astro/src/transient.rs`:
   - Difference imaging between consecutive frames ($I_t - I_{t-1}$).
   - Bounding box aspect-ratio filter ($\text{length}/\text{width} \ge 3.0$) and line vector regression ($y = mx + b$).
   - Trajectory logging (start/end coords, duration, peak brightness, velocity).
2. Hook `TransientDetector` to trigger clip recording.
3. Expose REST endpoint:
   - `GET /api/v1/astro/transients` returning list of recent transient events.
4. Comprehensive tests verifying discrimination between linear streaks and point-source stars or random noise.
5. All source files strictly <500 lines (target <400 lines).
