---
id: '0017'
title: Embedded Axum Web Server & Snappy Modern Single-Page App (SPA)
status: Accepted
date: 2026-10-05
deciders:
  - Samir
  - Alex
---

# ADR-0017: Embedded Axum Web Server & Snappy Modern Single-Page App (SPA)

## Status
Accepted

## Context
The stock camera web interface consists of a convoluted, multi-frame 2008-era HTML frameset with hundreds of redundant scripts, broken Chinese-English translations, and requirements for proprietary ActiveX controls. Users expect a fast, instant-loading, snappy modern Single Page Application (SPA) with sleek dark-mode styling, low-latency live video, virtual joystick PTZ steering, and comprehensive camera configuration.

## Decision
1. Implement the embedded HTTP and WebSocket server using **Axum** (Tokio ecosystem) in Rust.
2. Build a modern, zero-dependency, ultra-lightweight SPA frontend:
   - Built with modern Web Components / Preact and pure CSS, optimized for instant load times (<100KB gzipped).
   - Bundled directly into the Rust static binary using `rust-embed` or `include_bytes!` with Gzip/Brotli pre-compression.
3. Serve all API endpoints under `/api/v1/`:
   - `/api/v1/status`: Device health, memory, uptime, temperatures, and network state.
   - `/api/v1/ptz`: PTZ velocity, absolute step position, homing, and patrol commands.
   - `/api/v1/video`: Stream configuration, resolution, bitrate, and exposure settings.
   - `/api/v1/ircut`: Mechanical IR-cut filter toggle and night mode settings.
   - `/api/v1/ws`: Bidirectional WebSocket connection for live telemetry, PTZ joystick inputs, and WebRTC signaling.
4. Support keyboard shortcuts (WASD / arrow keys) and touch/pointer virtual joystick controls with haptic-like responsiveness.

## Consequences
- **Positive**: Blazing fast UI loading (<50ms from local network); beautiful, clean, modern dark aesthetic; zero external CDN dependencies; single binary distribution.
- **Negative**: Embedded assets add ~100-200KB to the compiled binary size on flash.
