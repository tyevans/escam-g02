---
id: '0002'
title: Low-Latency WebRTC & Video Streaming Pipeline
status: Accepted
created: 2026-10-05
target_persona: Samir
component: media
---

# PRD-0002 — Low-Latency WebRTC & Video Streaming Pipeline

## Who this is for

- **Samir**: Self-hosted privacy advocate wanting real-time, glass-to-glass video in browser tabs with <100ms latency, zero third-party cloud telemetry, and zero browser plugins.

## What the person cannot do today

- Today, streaming requires legacy RTSP/RTMP protocols with 1-3 seconds latency, or insecure HTTP-FLV streams.
- Modern web browsers cannot natively display RTSP without separate transcoding proxies.
- Stock firmware routes video through proprietary P2P cloud relay servers.

## What good looks like

1. **Hardware H.264 Ingestion**:
   - Direct zero-copy ingestion of H.264 NAL units from `/dev/venc` ring buffers.
2. **WebRTC Streaming Server**:
   - Pure Rust async WebRTC pipeline generating SDP offers/answers over WebSocket signaling.
   - Sub-100ms latency directly inside standard browser `<video>` tags via `RTCPeerConnection`.
3. **High-Performance Snapshot Fallback**:
   - Single-frame JPEG snapshot endpoint (`/api/v1/snapshot`) for home automation integrations (Home Assistant, Scrypted).

## What this does not do

- Cloud video recording or external cloud relay services.
- Lossy re-encoding in software (hardware H.264 is used exclusively).

## Checkable Outcomes

1. WebRTC video stream renders in browser with measured glass-to-glass latency below 100ms.
2. WebSocket signaling negotiates SDP answer within 250ms of connection initiation.
3. System CPU utilization on 600MHz ARM11 core stays under 25% during active WebRTC streaming.

## Linked User Stories

- [`US-0004: Ingest hardware H.264 NALUs and packetize to RTP`](../../user_stories/accepted/us-0004-ingest-hardware-h264-nalus-and-packetize-rtp.md)
- [`US-0005: Stream live low-latency WebRTC video over WebSocket signaling`](../../user_stories/accepted/us-0005-stream-live-webrtc-video-over-websocket.md)

## Implementing Backlog Tasks

- `TASK-0005`: Ingest hardware H.264 NALUs and implement RTP packetizer.
- `TASK-0006`: Implement async WebRTC peer connection and WebSocket signaling server.
