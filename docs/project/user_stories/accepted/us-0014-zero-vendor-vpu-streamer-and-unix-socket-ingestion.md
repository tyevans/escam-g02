---
id: '0014'
title: Ingest Zero-Vendor Hardware H.264 VPU Stream via Unix Domain Socket
status: Accepted
created: 2026-10-05
persona: Elena
target_bc: media
feature: FEAT-MEDIA-03
governing_prd: PRD-0006
governing_adrs:
  - ADR-0020
scenarios:
  - Initialize GalaxyCore GC1034 sensor and VPU hardware encoder without ipc_server
  - Stream raw Annex-B H.264 NALUs over local Unix domain socket
  - Ingest Unix domain socket stream in escamd and tunnel to WebCodecs WebSocket
---

# US-0014 — Ingest Zero-Vendor Hardware H.264 VPU Stream via Unix Domain Socket

## Governing PRD & ADR
- [`PRD-0006: Vendor Software Ejection, Clean Boot, and Production Static Firmware`](../../product/accepted/prd-0006-vendor-software-ejection-and-clean-boot.md)
- [`ADR-0020: Zero-Vendor Hardware Video Encoding and VPU Ingestion Architecture`](../../adrs/accepted/adr-0020-zero-vendor-hardware-video-encoding-pipeline.md)

## User Story

**As an** Elena (Embedded Systems Specialist),
**I want** `escamd` to receive hardware-encoded 720p H.264 NALUs from a minimal headless VPU micro-streamer via a local Unix domain socket,
**So that** the final vendor binary (`ipc_server`) is completely eliminated, RTSP port 554 is closed, and video ingestion consumes under 1.5MB of RAM with sub-40ms latency.

## Acceptance Criteria

```gherkin
Scenario: Initialize GalaxyCore GC1034 sensor and VPU hardware encoder without ipc_server
  Given ipc_server is terminated with SIGKILL
  When the headless VPU micro-streamer (gk-vpu) is started
  Then it initializes the GC1034 sensor over /dev/i2c-0
  And loads ISP calibration from /etc/sensors/gc1034_hw.bin
  And maps the MMZ BitStream Buffer via ioctl(0x80046d00) on /dev/gk_video
  And starts H.264 720p encoding without kernel panics.

Scenario: Stream raw Annex-B H.264 NALUs over local Unix domain socket
  Given gk-vpu running with active VPU encoder
  When it receives frame completion interrupts via ioctl(0x80046537)
  Then it writes framed Annex-B NALUs (00 00 00 01) directly to /tmp/venc.sock
  And total process memory consumption remains strictly under 1.5MB RSS.

Scenario: Ingest Unix domain socket stream in escamd and tunnel to WebCodecs WebSocket
  Given escamd running on port 8080
  When escamd connects to /tmp/venc.sock
  Then it ingests H.264 NALUs asynchronously without RTSP/RTP overhead
  And broadcasts video packets to connected browser WebSocket clients (/ws/live)
  And glass-to-glass latency is observed under 50ms.
```
