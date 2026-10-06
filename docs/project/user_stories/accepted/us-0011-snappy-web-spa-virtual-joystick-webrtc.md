---
id: '0011'
title: Provide snappy web SPA with virtual joystick and live WebRTC
status: Accepted
created: 2026-10-05
persona: Samir
target_bc: web
feature: FEAT-WEB-02
governing_prd: PRD-0005
scenarios:
  - Render dark-mode responsive camera control dashboard
  - Stream WebRTC video in low-latency canvas/video player
  - Stream continuous PTZ joystick commands over WebSocket
---

# US-0011 — Provide snappy web SPA with virtual joystick and live WebRTC

## Governing PRD
- [`PRD-0005: Embedded Axum Web Server and Snappy Modern Single-Page App (SPA)`](../../product/accepted/prd-0005-embedded-axum-web-server-and-snappy-spa.md)

## User Story

**As a** Samir (Privacy & Smart Home Operator),
**I want** a polished, snappy single-page web dashboard with an on-screen virtual joystick and live WebRTC player,
**So that** I can monitor and maneuver the camera seamlessly from my smartphone or desktop browser.

## Acceptance Criteria

```gherkin
Scenario: Render dark-mode responsive camera control dashboard
  Given a modern web browser opening the camera URL
  When the page loads
  Then a responsive dark-themed dashboard renders with video viewport, PTZ controls, and status telemetry
  And the initial page render completes in under 100ms.

Scenario: Stream WebRTC video in low-latency canvas/video player
  Given the web dashboard loaded in a browser supporting WebRTC
  When the page initializes the WebSocket signaling connection
  Then the video element automatically transitions from connecting to playing
  And video playback latency is under 100ms with zero stutter.

Scenario: Stream continuous PTZ joystick commands over WebSocket
  Given the on-screen virtual joystick touched or dragged by pointer
  When the user deflects the joystick handle
  Then WebSocket messages containing {"type":"ptz_velocity","x":...,"y":...} are transmitted at 20Hz
  And the camera motors respond with immediate smooth movement.
```
