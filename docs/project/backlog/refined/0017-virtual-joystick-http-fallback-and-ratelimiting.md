---
id: '0017'
title: Virtual Joystick HTTP/1 Fallback & Web UI Rate-Limiting
status: Refined
governing_adrs:
- ADR-0016
- ADR-0017
governing_prds:
- PRD-0004
- PRD-0005
governing_stories:
- US-0007
- US-0011
target_bc: web
persona: Samir
mutation_scope:
- crates/escam-web/src/assets.rs
---

# TASK-0017: Virtual Joystick HTTP/1 Fallback & Web UI Rate-Limiting

## Problem Statement & Context
The on-screen virtual joystick in the embedded SPA must steer the camera reliably whether connected via WebSocket or pure HTTP/1 REST. In pure HTTP mode, continuous touch/pointer movements need client-side rate limiting (~12 Hz) to prevent network congestion over Wi-Fi.

## Definition of Done (Blackbox Frontdoor TDD)
1. Update virtual joystick controller in `INDEX_HTML` to throttle HTTP POST requests to `/api/v1/ptz` at ~75ms intervals.
2. Emit automatic `{"action":"Stop"}` payload on `pointerup` and `pointercancel`.
3. Validate with Playwright browser test clicking and dragging the virtual joystick.
4. Verify motor moves in response to joystick drag on physical camera.
5. All files strictly under 500 lines.
