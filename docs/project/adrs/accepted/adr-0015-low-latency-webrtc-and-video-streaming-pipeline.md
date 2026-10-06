---
id: '0015'
title: Low-Latency WebRTC & Video Streaming Pipeline
status: Accepted
date: 2026-10-05
deciders:
  - Samir
  - Elena
  - Alex
---

# ADR-0015: Low-Latency WebRTC & Video Streaming Pipeline

## Status
Accepted

## Context
Traditional IP camera web interfaces rely on legacy RTSP over TCP/UDP, RTMP, or obsolete Flash/ActiveX plugins. RTSP streams suffer from 1-3 seconds of buffering latency and cannot be played natively in web browsers without transcoding gateways (like MediaMTX or WebRTC bridges). The Goke GK7102C SoC includes a dedicated hardware H.264 video encoder outputting NAL units via `media.ko` (`/dev/venc`).

## Decision
1. Ingest hardware H.264 Annex-B NAL units directly from `/dev/venc` (or video ring buffers).
2. Package H.264 frames directly into RTP packets with standard H.264 payload formatting (RFC 6184).
3. Implement a zero-copy WebRTC peer-to-peer streaming pipeline using pure Rust async libraries (`webrtc-rs` or lightweight STUN/DTLS/SRTP micro-pipeline).
4. Provide a WebSocket signaling channel hosted on the embedded Axum web server to negotiate SDP offers/answers and ICE candidates.
5. In addition to WebRTC, provide an ultra-lightweight raw HTTP MJPEG / JPEG snapshot endpoint (`/api/snapshot`) and raw RTSP stream for backwards compatibility.

## Consequences
- **Positive**: Sub-100ms glass-to-glass latency in any modern mobile or desktop browser (Safari, Chrome, Firefox) without plugins, mobile apps, or cloud intermediaries.
- **Negative**: WebRTC DTLS/SRTP cryptography incurs minor CPU overhead on the 600MHz ARM11 core, requiring optimized crypto primitives.
