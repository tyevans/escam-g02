---
id: 0008
title: PTZ Coordinate Engine, Virtual Joystick, and Soft Limits
status: Complete
governing_adrs:
- ADR-0016
- ADR-0003
governing_prds:
- PRD-0003
governing_stories:
- US-0007
target_bc: ptz
mutation_scope:
- crates/escam-ptz/src/controller.rs
---

# TASK-0008: PTZ Coordinate Engine, Virtual Joystick, and Soft Limits

## Problem Statement & Context
A responsive camera requires continuous proportional velocity control driven by an on-screen joystick or keyboard, absolute angular coordinates (pan 0-355°, tilt -10° to 90°), and soft limits to protect against cable winding and mechanical binding.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `PtzController` with support for:
   - Proportional 2D velocity inputs $(v_x, v_y) \in [-1.0, 1.0]$.
   - Absolute degree coordinate system with calibrated steps-per-degree ratios.
   - Enforced software endstops stopping motors before physical limits.
2. Frontdoor tests verify that joystick commands approaching soft limits smoothly clamp velocity to zero without hard crashes.
3. Automated coil de-energization timer turns off motor driver phases after 2s of idle inactivity.
4. All new files strictly under 500 lines.
