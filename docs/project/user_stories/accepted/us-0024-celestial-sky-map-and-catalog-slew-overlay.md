---
id: '0024'
title: Celestial Sky Map and Target Catalog Slew Controller
status: Accepted
created: 2026-10-06
persona: Marcus
target_bc: ptz
feature: FEAT-PTZ-05
governing_prd: PRD-0008
governing_adrs:
  - ADR-0016
  - ADR-0021
scenarios:
  - Query astronomical target catalog for RA/DEC and current Alt/Az
  - Command automated S-curve slew to selected celestial target
  - Maintain active celestial tracking at target position
---

# US-0024 — Celestial Sky Map and Target Catalog Slew Controller

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0016: Smooth PTZ Stepper Motor Control`](../../adrs/accepted/adr-0016-smooth-ptz-stepper-motor-s-curve-profiling.md)
- [`ADR-0021: Binary Footprint Reduction and Sidereal Tracking`](../../adrs/accepted/adr-0021-binary-footprint-reduction-and-zero-allocation-pipeline.md)

## User Story
**As a** Dr. Marcus Vance (Astrophotographer),
**I want** to select astronomical targets (Messier objects, planets, bright stars) from a catalog and command the camera to slew smoothly to them,
**So that** I do not have to manually align the camera to celestial targets.

## Acceptance Criteria

```gherkin
Scenario: Query astronomical target catalog for RA/DEC and current Alt/Az
  Given an embedded celestial catalog containing major targets (Moon, Jupiter, M31, M42, Polaris)
  When a client queries `GET /api/v1/astro/targets`
  Then the system returns target names, J2000 RA/DEC, and computed local Alt/Az coordinates.

Scenario: Command automated S-curve slew to selected celestial target
  Given a requested target from the catalog
  When the client sends `POST /api/v1/astro/slew` with target name
  Then the PTZ trajectory generator executes a smooth jerk-limited slew to the target's Alt/Az.

Scenario: Maintain active celestial tracking at target position
  Given arrival at target coordinates
  When the slew completes
  Then the sidereal tracking engine automatically activates to maintain the target centered in the field of view.
```
