---
id: '0015'
title: Wire Physical PTZ & IR-Cut Drivers to Web Frontdoor and HTTP/1 Daemon
status: Complete
governing_adrs:
- ADR-0014
- ADR-0016
- ADR-0017
governing_prds:
- PRD-0004
- PRD-0005
governing_stories:
- US-0007
- US-0011
target_bc: web
mutation_scope:
- crates/escamd/src/main.rs
- crates/escam-web/src/api.rs
---

# TASK-0015: Wire Physical PTZ & IR-Cut Drivers to Web Frontdoor and HTTP/1 Daemon

## Problem Statement & Context
Now that the standalone `motor-test` proves direct `/dev/motor` and `/dev/gkio` hardware control, and the INDI TCP 7624 server runs reliably on the camera, we need to bridge the physical hardware into the core daemon (`escamd`) and expose a rock-solid HTTP/1 frontdoor. The web SPA's virtual joystick and IR-cut buttons must directly drive the camera's physical stepper motors and UTC BA6208L H-bridge solenoid in real time.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement a type-erased or dynamic hardware device abstraction in `AppState` so `PtzController` and `IrCutController` can bind to `LinuxMotorDevice` and `LinuxGpioDevice` with graceful fallback to mock implementations when device nodes are not present.
2. Build an efficient, bulletproof HTTP/1 server loop in `escamd` that serves the embedded glassmorphic SPA (`/`), telemetry status (`/api/v1/status`), PTZ steering (`/api/v1/ptz`), and IR-cut toggle (`/api/v1/ircut`) without relying on ARMv6-incompatible HTTP/2 atomics or bloated middleware layers.
3. Validate frontdoor contract endpoints with automated integration tests in `tests/test_frontdoors.py` and `cargo test`.
4. Cross-compile for `arm-unknown-linux-musleabi`, deploy to the camera, and verify physical motor actuation and IR-cut clicks when interacting via HTTP.
5. All modified files strictly under 500 lines.
