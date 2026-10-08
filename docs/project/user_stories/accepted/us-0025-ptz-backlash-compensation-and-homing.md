---
id: '0025'
title: Mechanical Gear Backlash Compensation and Autonomous Soft-Homing
status: Accepted
created: 2026-10-06
persona: Elena
target_bc: ptz
feature: FEAT-PTZ-06
governing_prd: PRD-0008
governing_adrs:
  - ADR-0016
scenarios:
  - Inject anti-backlash step compensation on motor direction reversal
  - Calibrate soft-home position against physical mechanical hardstops
  - Enforce absolute position bounds to prevent cable wrapping
---

# US-0025 — Mechanical Gear Backlash Compensation and Autonomous Soft-Homing

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0016: Smooth PTZ Stepper Motor Control`](../../adrs/accepted/adr-0016-smooth-ptz-stepper-motor-s-curve-profiling.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** the PTZ motor driver to automatically compensate for mechanical gear train backlash and establish an absolute reference zero via soft-homing,
**So that** pointing repeatability is preserved when changing pan/tilt direction and motor positions remain calibrated across reboots.

## Acceptance Criteria

```gherkin
Scenario: Inject anti-backlash step compensation on motor direction reversal
  Given a backlash compensation profile of 8 steps on the Pan axis
  When the Pan motor direction flips from clockwise to counter-clockwise
  Then the controller immediately pulses 8 compensation steps at high acceleration before starting the target trajectory
  And pointing error due to gear slack is eliminated.

Scenario: Calibrate soft-home position against physical mechanical hardstops
  Given a home calibration command `POST /api/v1/ptz/home`
  When executed
  Then the motors step toward physical hardstops, detect stall/limit, center the coordinate system, and set logical (0, 0) coordinates.

Scenario: Enforce absolute position bounds to prevent cable wrapping
  Given calibrated physical limits
  When motion commands attempt to exceed allowable steps (-260 to +260)
  Then commands are clamped at the boundary and motors are prevented from grinding.
```
