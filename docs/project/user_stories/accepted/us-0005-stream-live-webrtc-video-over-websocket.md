---
id: '0005'
title: Stream live low-latency WebRTC video over WebSocket signaling
status: Accepted
created: 2026-10-05
persona: Samir
target_bc: media
feature: FEAT-MEDIA-02
governing_prd: PRD-0002
scenarios:
  - Negotiate SDP offer and answer over WebSocket signaling
  - Stream RTP video over DTLS/SRTP peer connection
  - Maintain sub-100ms latency without video buffer bloat
---

# US-0005 — Stream live low-latency WebRTC video over WebSocket signaling

## Governing PRD
- [`PRD-0002: Low-Latency WebRTC & Video Streaming Pipeline`](../../product/accepted/prd-0002-low-latency-webrtc-and-video-streaming.md)

## User Story

**As a** Samir (Privacy & Smart Home Operator),
**I want** to connect to the camera over WebRTC with WebSocket signaling,
**So that** I can view live video directly inside my browser with <100ms latency and no plugins or cloud dependencies.

## Acceptance Criteria

```gherkin
Scenario: Negotiate SDP offer and answer over WebSocket signaling
  Given a browser client opening a WebSocket connection to /api/v1/ws
  When the client sends an SDP Offer with H.264 video codec capability
  Then the camera daemon responds with an SDP Answer within 250ms
  And local ICE candidates are exchanged successfully.

Scenario: Stream RTP video over DTLS/SRTP peer connection
  Given a completed SDP negotiation between browser and camera
  When media packets begin flowing over the peer connection
  Then DTLS handshake establishes encryption keys
  And SRTP packets deliver H.264 video frames directly to the browser video element.

Scenario: Maintain sub-100ms latency without video buffer bloat
  Given an active WebRTC streaming session
  When the client measures glass-to-glass latency
  Then the round-trip latency remains consistently under 100ms
  And packet loss triggers immediate NACK / PLI keyframe refresh without stream freeze.
```
