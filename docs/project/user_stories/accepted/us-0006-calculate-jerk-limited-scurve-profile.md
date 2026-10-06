---
id: '0006'
title: Calculate jerk-limited S-curve motor velocity profile
status: Accepted
created: 2026-10-05
persona: Samir
target_bc: ptz
feature: FEAT-PTZ-01
governing_prd: PRD-0003
scenarios:
  - Generate S-curve velocity ramp from rest to max speed
  - Decelerate smoothly to halt at target step position
  - Avoid discontinuous acceleration jumps to prevent clicking
---

# US-0006 — Calculate jerk-limited S-curve motor velocity profile

## Governing PRD
- [`PRD-0003: Smooth PTZ Stepper Motor Control and S-Curve Trajectory Engine`](../../product/accepted/prd-0003-smooth-ptz-stepper-motor-control.md)

## User Story

**As a** Samir (Privacy & Smart Home Operator),
**I want** the PTZ trajectory engine to calculate jerk-limited sinusoidal velocity curves,
**So that** stepper motor acceleration is whisper-quiet and vibration-free.

## Acceptance Criteria

```gherkin
Scenario: Generate S-curve velocity ramp from rest to max speed
  Given a stepper motor at rest with velocity 0 steps/sec
  When a move command specifies target speed Vmax with S-curve acceleration
  Then the computed step interval follows a smooth sinusoidal curve
  And the instantaneous acceleration increases and decreases smoothly without square-wave edges.

Scenario: Decelerate smoothly to halt at target step position
  Given a motor moving at cruise velocity approaching a target coordinate
  When the deceleration phase triggers
  Then the profile decelerates with negative jerk until velocity reaches 0 exactly at the target step
  And zero step overshoot occurs.

Scenario: Avoid discontinuous acceleration jumps to prevent clicking
  Given an S-curve profile computed for an 800-step move
  When the derivative of acceleration (jerk) is evaluated across all time steps
  Then jerk remains bounded within max_jerk limits
  And zero audible clicking or acoustic resonance occurs.
```
