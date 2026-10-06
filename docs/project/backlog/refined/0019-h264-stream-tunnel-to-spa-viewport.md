---
id: '0019'
title: Low-Latency H.264 Stream Tunneling to SPA Video Viewport
status: Refined
governing_adrs:
- ADR-0015
- ADR-0017
governing_prds:
- PRD-0003
- PRD-0005
governing_stories:
- US-0005
- US-0011
target_bc: media
persona: Samir
mutation_scope:
- crates/escam-web/src/assets.rs
---

# TASK-0019: Low-Latency H.264 Stream Tunneling to SPA Video Viewport

## Problem Statement & Context
The embedded web SPA currently displays a placeholder HUD overlay. We must connect the live H.264 video feed into the browser `<video>` player via WebRTC data channel or MSE (Media Source Extensions) stream tunnel with sub-100ms latency.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement video stream delivery handler in `escam-web` serving continuous H.264 video packets.
2. Update SPA frontend to ingest stream into HTML5 `<video>` or canvas with automatic reconnection.
3. Validate stream playback with Playwright browser test asserting non-zero video frame dimensions.
4. Verify sub-100ms glass-to-glass latency.
5. All files strictly under 500 lines.
