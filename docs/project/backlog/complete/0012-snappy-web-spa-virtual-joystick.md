---
id: '0012'
title: Responsive Snappy Web SPA with Virtual Joystick & WebRTC Player
status: Complete
governing_adrs:
- ADR-0017
- ADR-0015
governing_prds:
- PRD-0005
governing_stories:
- US-0011
target_bc: web
persona: Samir
mutation_scope:
- crates/escam-web/src/assets.rs
---

# TASK-0012: Responsive Snappy Web SPA with Virtual Joystick & WebRTC Player

## Problem Statement & Context
Replace the 2008 frameset with a snappy, polished, modern Single-Page Application (SPA) embedded directly in the Rust binary. Features a sleek dark UI, low-latency live WebRTC video player, responsive on-screen virtual joystick, keyboard hotkeys (WASD), and instant-response mode toggles.

## Definition of Done (Blackbox Frontdoor TDD)
1. Author HTML/CSS/JS SPA frontend with total uncompressed footprint <150KB.
2. Implement virtual joystick supporting touch/pointer drag events emitting normalized $(x, y)$ coordinates.
3. Implement WebRTC `<video>` player handling SDP negotiation over WebSocket.
4. Compress assets with Gzip and embed into binary using `rust-embed` or `include_bytes!`.
5. Verify initial page load responds with HTTP 200 and Content-Encoding: gzip.
6. All new files strictly under 500 lines.
