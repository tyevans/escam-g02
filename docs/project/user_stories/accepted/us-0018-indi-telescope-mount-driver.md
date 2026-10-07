---
id: '0018'
title: Expose PTZ Motors as an INDI Telescope Mount Device for KStars and Ekos
status: Accepted
created: 2026-10-06
persona: Marcus
target_bc: astro
feature: FEAT-ASTRO-04
governing_prd: PRD-0007
governing_adrs:
  - ADR-0018
  - ADR-0021
scenarios:
  - Expose ESCAM G02 Mount device properties on INDI port 7624
  - Execute directional telescope motion (North, South, East, West) via INDI switch commands
  - Handle telescope abort and park commands
---

# US-0018 — Expose PTZ Motors as an INDI Telescope Mount Device for KStars and Ekos

## Governing PRD & ADR
- [`PRD-0007: Performance Optimization, Binary Footprint Shrinking, and Deep-Sky Astrophotography`](../../product/accepted/prd-0007-optimization-binary-shrinking-and-deep-sky-astronomy.md)
- [`ADR-0018: Astrophotography Subsystem FITS & INDI Architecture`](../../adrs/accepted/adr-0018-astrophotography-engine-fits-and-indi-protocol.md)
- [`ADR-0021: Binary Footprint Reduction, Zero-Allocation Media Pipelines, and Sidereal Tracking`](../../adrs/accepted/adr-0021-binary-footprint-reduction-and-zero-allocation-pipeline.md)

## User Story
**As a** Dr. Marcus Vance (Astrophotographer),
**I want** the Pan/Tilt stepper motors of the ESCAM G02 to be exposed as an INDI Telescope / Mount device on port 7624,
**So that** I can connect KStars, Ekos, N.I.N.A., or Stellarium directly to the camera and command telescope slews, directional nudging, and parking without custom scripts.

## Acceptance Criteria

```gherkin
Scenario: Expose ESCAM G02 Mount device properties on INDI port 7624
  Given an INDI client connected to port 7624
  When it sends <getProperties version='1.7'/>
  Then the server returns <defSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_MOTION_NS'>
  And the server returns <defSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_MOTION_WE'>
  And the server returns <defNumberVector device='ESCAM G02 Mount' name='HORIZONTAL_COORD'>
  And the server returns <defSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_PARK'>.

Scenario: Execute directional telescope motion (North, South, East, West) via INDI switch commands
  Given an active INDI connection
  When the client sends <newSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_MOTION_NS'><oneSwitch name='MOTION_NORTH'>On</oneSwitch></newSwitchVector>
  Then the Tilt motor actuates upward
  When the client sends <oneSwitch name='MOTION_NORTH'>Off</oneSwitch>
  Then the Tilt motor stops.

Scenario: Handle telescope abort and park commands
  Given moving telescope motors
  When the client sends <newSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_ABORT_MOTION'><oneSwitch name='ABORT'>On</oneSwitch></newSwitchVector>
  Then all motor motion immediately halts.
```
