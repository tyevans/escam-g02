---
id: '0005'
title: Embedded Axum Web Server and Snappy Modern Single-Page App (SPA)
status: Accepted
created: 2026-10-05
target_persona: Samir
component: web
---

# PRD-0005 — Embedded Axum Web Server and Snappy Modern Single-Page App (SPA)

## Who this is for

- **Samir**: Smart home operator demanding a fast, responsive, modern web interface to control the camera and view low-latency live video from any desktop or mobile browser.

## What the person cannot do today

- The vendor web UI uses broken 2008 HTML framesets that require Internet Explorer and obsolete ActiveX controls.
- The vendor UI is sluggish, unstyled, and confusing to navigate.

## What good looks like

1. **Embedded Axum Web Server**:
   - Modern, async HTTP server using Axum in Rust, with zero external web server dependencies.
   - Serves static assets directly from memory using compression (Brotli/Gzip).
2. **Snappy, Modern SPA Frontend**:
   - Clean dark-mode UI inspired by modern camera management platforms.
   - Sub-50ms load time over local Wi-Fi/Ethernet.
   - Embedded WebRTC `<video>` player with instantaneous stream connection.
   - On-screen touch and pointer virtual joystick for smooth PTZ steering.
   - Keyboard hotkey support (WASD, Arrow keys, spacebar to stop).
   - Fast controls for IR-cut filter toggle, exposure, resolution, and motor speed.
3. **REST and WebSocket APIs**:
   - Clean, documented `/api/v1` endpoints for integration into third-party dashboards.

## What this does not do

- Heavy JavaScript frameworks (React 18 bundles > 150KB); uses lightweight vanilla / Preact components.

## Checkable Outcomes

1. Frontend loads in browser in under 100ms with total asset transfer under 150KB.
2. PTZ virtual joystick sends velocity updates at 20Hz over WebSocket with <15ms round-trip latency.
3. Live WebRTC video streams smoothly inside the UI video container with zero browser warnings.

## Linked User Stories

- [`US-0010: Embed Axum HTTP server and serve compressed static assets`](../../user_stories/accepted/us-0010-embed-axum-http-server-static-assets.md)
- [`US-0011: Provide snappy web SPA with virtual joystick and live WebRTC`](../../user_stories/accepted/us-0011-snappy-web-spa-virtual-joystick-webrtc.md)

## Implementing Backlog Tasks

- `TASK-0011`: Implement Axum HTTP REST API and WebSocket router in `escam-web`.
- `TASK-0012`: Build lightweight, responsive Single Page Application with virtual joystick and live WebRTC player.
