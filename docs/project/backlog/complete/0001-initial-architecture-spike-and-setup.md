---
id: '0001'
title: Initial Architecture Spike and Cargo Workspace Foundation
status: Complete
governing_adrs:
- ADR-0001
- ADR-0002
- ADR-0003
- ADR-0013
governing_prds:
- PRD-0001
governing_stories:
- US-0001
target_bc: core
mutation_scope:
- crates/escam-core/src/lib.rs
---

# TASK-0001: Initial Architecture Spike and Cargo Workspace Foundation

## Problem Statement & Context
Establish the root Rust multi-crate workspace for the ESCAM G02 camera system (`escam-g02`), supporting both host development/testing and cross-compilation for `arm-unknown-linux-musleabi`.

## Definition of Done (Blackbox Frontdoor TDD)
1. Initialize root `Cargo.toml` workspace with crates:
   - `escam-core` (orchestration, configuration, shared domain models)
   - `escam-driver` (Linux device node ioctls for `/dev/motor`, `/dev/gkio`, `/dev/venc`)
   - `escam-ptz` (S-curve trajectory generation, coordinate limits, virtual joystick)
   - `escam-media` (H.264 NALU packetizer, WebRTC pipeline, snapshot generator)
   - `escam-astro` (raw Bayer reader, FITS serializer, INDI protocol)
   - `escam-web` (Axum server, WebSocket signaling, embedded snappy SPA)
   - `escam-system` (init runner, watchdog heartbeat, vendor ejection)
   - `motor-test` (standalone ARMv6 musl proof-of-concept binary)
   - `escamd` (production camera daemon)
2. Verify blackbox test suite passes cleanly with `cargo test`.
3. All source files strictly under 500 lines.
4. Mutation testing scope identified for domain models in `escam-core`.
