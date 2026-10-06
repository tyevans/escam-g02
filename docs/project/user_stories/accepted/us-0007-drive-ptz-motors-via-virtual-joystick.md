---
id: '0007'
title: Drive PTZ motors via virtual joystick with soft limits
status: Accepted
created: 2026-10-05
persona: Samir
target_bc: ptz
feature: FEAT-PTZ-02
governing_prd: PRD-0003
scenarios:
  - Translate 2D joystick vector into proportional dual-axis speeds
  - Enforce soft endstop limits to prevent mechanical collision
  - Stop immediately upon joystick release with smooth deceleration
---

# US-0007 — Drive PTZ motors via virtual joystick with soft limits

## Governing PRD
- [`PRD-0003: Smooth PTZ Stepper Motor Control and S-Curve Trajectory Engine`](../../product/accepted/prd-0003-smooth-ptz-stepper-motor-control.md)

## User Story

**As a** Samir (Privacy & Smart Home Operator),
**I want** to steer the camera using a 2D virtual joystick with proportional velocity and soft endstops,
**So that** I can intuitively explore the room without hitting mechanical hard stops.

## Acceptance Criteria

```gherkin
Scenario: Translate 2D joystick vector into proportional dual-axis speeds
  Given a virtual joystick input vector (X=0.8, Y=-0.5)
  When the PTZ controller updates target speeds
  Then the pan axis motor is commanded to 80% maximum speed clockwise
  And the tilt axis motor is commanded to 50% maximum speed upward.

Scenario: Enforce soft endstop limits to prevent mechanical collision
  Given the pan axis is at 350 degrees (soft limit 355 degrees)
  When a clockwise pan command is requested
  Then the controller limits motor steps to decelerate to a stop before 355 degrees
  And refuses further clockwise movement until rotated counter-clockwise.

Scenario: Stop immediately upon joystick release with smooth deceleration
  Given both pan and tilt axes in motion
  When the user releases the joystick (X=0, Y=0)
  Then both axes execute rapid S-curve deceleration to a halt within 150ms
  And coil power is automatically depowered after 2 seconds of inactivity.
```
