---
id: '0004'
title: Ingest hardware H.264 NALUs and packetize to RTP
status: Accepted
created: 2026-10-05
persona: Samir
target_bc: media
feature: FEAT-MEDIA-01
governing_prd: PRD-0002
scenarios:
  - Parse H.264 Annex-B start codes from device stream
  - Fragment oversized NAL units into RFC 6184 FU-A RTP packets
  - Maintain monotonic timestamp synchronization
---

# US-0004 — Ingest hardware H.264 NALUs and packetize to RTP

## Governing PRD
- [`PRD-0002: Low-Latency WebRTC & Video Streaming Pipeline`](../../product/accepted/prd-0002-low-latency-webrtc-and-video-streaming.md)

## User Story

**As a** Samir (Privacy & Smart Home Operator),
**I want** the media pipeline to read raw H.264 NALUs from the hardware encoder and encapsulate them into RFC 6184 RTP packets,
**So that** live video frames can be streamed into WebRTC sessions with zero software transcoding latency.

## Acceptance Criteria

```gherkin
Scenario: Parse H.264 Annex-B start codes from device stream
  Given a raw H.264 bitstream chunk containing SPS, PPS, IDR, or Non-IDR slices
  When the bitstream reader scans the bytes
  Then 3-byte and 4-byte Annex-B prefixes (0x000001 and 0x00000001) are parsed into discrete NAL units
  And NAL unit header types (SPS=7, PPS=8, IDR=5, non-IDR=1) are correctly identified.

Scenario: Fragment oversized NAL units into RFC 6184 FU-A RTP packets
  Given an IDR keyframe NAL unit larger than standard MTU (1400 bytes)
  When the RTP packetizer processes the NAL unit
  Then the NAL unit is segmented into FU-A packets with Start (S) and End (E) bits set appropriately
  And each packet payload remains under the 1400-byte MTU threshold.

Scenario: Maintain monotonic timestamp synchronization
  Given a 25 fps video stream from the hardware encoder
  When consecutive RTP video frames are emitted
  Then the 90kHz RTP timestamp increases monotonically by 3600 clock ticks per frame
  And the sequence number increments by 1 per emitted RTP packet.
```
