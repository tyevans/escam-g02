---
id: '0006'
title: Async WebRTC Peer Connection and WebSocket Signaling
status: Complete
governing_adrs:
- ADR-0015
- ADR-0017
governing_prds:
- PRD-0002
governing_stories:
- US-0005
target_bc: media
persona: Samir
mutation_scope:
- crates/escam-media/src/webrtc.rs
---

# TASK-0006: Async WebRTC Peer Connection and WebSocket Signaling

## Problem Statement & Context
To deliver sub-100ms video directly into browser tabs with zero plugins, `escam-media` must establish WebRTC peer connections. A lightweight signaling mechanism over WebSockets exchanges SDP offers and answers and ICE candidates, streaming packetized H.264 video frames directly over SRTP.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement WebRTC session manager and SDP negotiation handlers.
2. Implement WebSocket signaling dispatcher handling JSON offer/answer/candidate messages.
3. Test pipeline using mock browser peer connection, verifying SDP answer emission within 250ms.
4. Verify graceful handling of peer disconnects and reconnects.
5. All new files strictly under 500 lines.
