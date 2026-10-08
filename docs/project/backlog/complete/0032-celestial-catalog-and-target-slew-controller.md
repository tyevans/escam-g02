---
id: '0032'
title: Celestial Catalog and Automated Target Slew Controller
status: Complete
governing_adrs:
  - ADR-0016
  - ADR-0021
governing_prds:
  - PRD-0008
governing_stories:
  - US-0024
target_bc: ptz
mutation_scope:
  - crates/escam-ptz/src/catalog.rs
---

# TASK-0032: Celestial Catalog and Automated Target Slew Controller

## Problem Statement & Context
Navigating to celestial targets (planets, bright stars, deep-sky objects) requires manual positioning or external planetarium software. Having an embedded astronomical target catalog with coordinate conversion and automatic slewing makes the camera an autonomous GoTo telescope.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `CelestialCatalog` in `crates/escam-ptz/src/catalog.rs`:
   - Built-in catalog of major celestial targets (Polaris, Vega, Sirius, Jupiter, Saturn, Mars, Moon, M31 Andromeda, M42 Orion, M45 Pleiades) with J2000 RA/DEC.
   - Real-time conversion from RA/DEC to local Horizontal (Azimuth, Altitude) based on site coordinates and current UTC time.
   - Slew planner converting target Alt/Az into motor step coordinates with range validation.
2. Expose REST endpoints:
   - `GET /api/v1/astro/targets`
   - `POST /api/v1/astro/slew`
3. Frontdoor tests verifying coordinate transform accuracy and motor trajectory clamping.
4. All source files strictly <500 lines (target <400 lines).
