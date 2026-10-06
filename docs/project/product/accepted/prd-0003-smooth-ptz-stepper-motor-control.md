---
id: '0003'
title: Smooth PTZ Stepper Motor Control and S-Curve Trajectory Engine
status: Accepted
created: 2026-10-05
target_persona: Samir
component: ptz
---

# PRD-0003 — Smooth PTZ Stepper Motor Control and S-Curve Trajectory Engine

## Who this is for

- **Samir**: Privacy advocate wanting whisper-quiet indoor PTZ steering that doesn't click loudly or vibrate.
- **Marcus**: Astrophotographer requiring smooth guide star alignment and planetary tracking without mechanical ringing or backlash wobble.

## What the person cannot do today

- Today, motor movements use square-wave step pulses with abrupt acceleration, causing loud clicking noises and mount vibrations.
- There are no soft endstops, velocity-ramping controls, or continuous virtual joystick driving modes.

## What good looks like

1. **Jerk-Limited S-Curve Trajectory Profiling**:
   - Continuous sinusoidal or 7-phase S-curve profile running in an async tick loop.
   - Smoothly ramps motor velocity up and down, preventing torque shock and resonance.
2. **Virtual Joystick & Continuous Steering**:
   - Supports proportional speed control based on joystick deflection.
   - Immediate responsive stops without jerk or position overshoot.
3. **Calibrated Soft Endstops & Coordinate System**:
   - Absolute coordinate tracking (pan angle 0-355°, tilt angle -10° to 90°).
   - Saved home position and automated limit homing routines.

## What this does not do

- Motor driving without underlying kernel driver (relies on `escam-driver` and `/dev/motor`).

## Checkable Outcomes

1. Acceleration and deceleration profiles show continuous velocity transitions without discontinuous step rate jumps.
2. Physical motor noise level during high-speed slewing is noticeably quieter than stock firmware square waves.
3. Soft endstops prevent commanded motor steps from exceeding physical limits (pan: ~355°, tilt: ~90°).

## Linked User Stories

- [`US-0006: Calculate jerk-limited S-curve motor velocity profile`](../../user_stories/accepted/us-0006-calculate-jerk-limited-scurve-profile.md)
- [`US-0007: Drive PTZ motors via virtual joystick with soft limits`](../../user_stories/accepted/us-0007-drive-ptz-motors-via-virtual-joystick.md)

## Implementing Backlog Tasks

- `TASK-0007`: Implement S-curve trajectory profile generator and fixed-point math in `escam-ptz`.
- `TASK-0008`: Implement PTZ controller with soft limits, coordinate tracking, and virtual joystick mode.
