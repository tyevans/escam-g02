---
id: '0011'
title: Axum HTTP Server and REST Telemetry API
status: Complete
governing_adrs:
- ADR-0017
- ADR-0003
governing_prds:
- PRD-0005
governing_stories:
- US-0010
target_bc: web
mutation_scope:
- crates/escam-web/src/api.rs
- crates/escam-web/src/server.rs
---

# TASK-0011: Axum HTTP Server and REST Telemetry API

## Problem Statement & Context
The camera needs a robust, memory-efficient HTTP/WebSocket server. Using Axum in the Tokio ecosystem, `escam-web` provides `/api/v1` REST endpoints for telemetry, video snapshots, PTZ control, and IR-cut mode, alongside WebSocket signaling.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement Axum application router with endpoints:
   - `GET /api/v1/status`: Device health, memory, uptime, temperature, coordinates.
   - `POST /api/v1/ptz`: Command pan/tilt steps, speed, or home.
   - `POST /api/v1/ircut`: Toggle Day/Night mode.
   - `GET /api/v1/snapshot`: Returns JPEG frame.
   - `GET /api/v1/ws`: WebSocket upgrade for real-time telemetry and PTZ joystick.
2. Blackbox tests via `tower::ServiceExt` / `axum::test` exercise all endpoints via HTTP without running real network sockets.
3. All new files strictly under 500 lines.
