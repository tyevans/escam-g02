---
id: '0016'
title: Smooth PTZ Stepper Motor Control via Sinusoidal S-Curve Trajectory Profiling
status: Accepted
date: 2026-10-05
deciders:
  - Samir
  - Marcus
  - Alex
---

# ADR-0016: Smooth PTZ Stepper Motor Control via Sinusoidal S-Curve Trajectory Profiling

## Status
Accepted

## Context
The ESCAM G02 uses 2x unipolar 4-phase stepper motors driven through a `JULN2803AG` 8-channel Darlington transistor array. Stock vendor software drives these motors with abrupt square-wave velocity steps, causing harsh acoustic clicking, resonance vibration, and mechanical overshoot. For telescope astrophotography and home surveillance, vibration and noise ruin imaging quality and cause annoying disturbance.

## Decision
1. Implement an async PTZ trajectory engine in Rust (`escam-ptz`) running a deterministic 1kHz tick loop.
2. Implement 7-phase sinusoidal (or cubic spline) S-curve acceleration and deceleration profiles:
   - Limits jerk ($j = \frac{da}{dt}$) to eliminate instantaneous motor torque spikes.
   - Smoothly ramps velocity from rest to target speed ($v_{\max}$) and back to 0.
3. Support absolute coordinate positioning, calibrated soft endstop limits, and continuous virtual joystick velocity mode.
4. Provide microsecond-level timing control via hardware timer ioctl (`MOTOR_SPEED` / timer divisor) or high-resolution Tokio interval timers.

## Consequences
- **Positive**: Whisper-quiet motor operation; zero mechanical shake or ringing on camera/telescope mounts; accurate planetary tracking and guide star alignment.
- **Negative**: Jerk-limited S-curve calculations require floating-point or fixed-point arithmetic on ARMv6 without hardware FPU. We use integer / Q16.16 fixed-point math to maintain high performance.
