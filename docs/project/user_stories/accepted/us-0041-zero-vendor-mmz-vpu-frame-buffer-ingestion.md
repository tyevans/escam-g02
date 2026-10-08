---
id: '0041'
title: Zero-Vendor MMZ Physical VPU Frame Buffer Ingestion
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: media
feature: FEAT-MED-08
governing_prd: PRD-0009
governing_adrs:
  - ADR-0020
  - ADR-0026
scenarios:
  - Audit physical MMZ buffer layout via /dev/mem
  - Parse hardware ring buffer headers in Rust
  - Ingest H.264 NALUs directly from memory zone
---

# US-0041 — Zero-Vendor MMZ Physical VPU Frame Buffer Ingestion

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0020: Zero-Vendor Hardware Video Encoding and VPU Ingestion Architecture`](../../adrs/accepted/adr-0020-zero-vendor-hardware-video-encoding-pipeline.md)
- [`ADR-0026: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap`](../../adrs/accepted/adr-0026-open-source-kernel-drivers-and-gk7102-peripheral-sourcing.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** `escam-media` to read H.264 bitstream buffers directly from the GK7102C Media Memory Zone (MMZ, `0x0000_0000 - 0x0180_0000`),
**So that** video streaming operates completely without vendor `ipc_server` loopback or proprietary userland libraries.

## Acceptance Criteria

```gherkin
Scenario: Audit physical MMZ buffer layout via /dev/mem
  Given `/dev/mem` access on the Goke GK7102C architecture
  When mapping the lower 24MB MMZ region
  Then physical frame ring buffer boundaries and write pointers are accessible.

Scenario: Parse hardware ring buffer headers in Rust
  Given memory slices from the MMZ bitstream buffer
  When parsing Goke packet headers (timestamp, length, frame type)
  Then Annex-B H.264 NALUs (SPS, PPS, IDR, P) are successfully extracted.

Scenario: Ingest H.264 NALUs directly from memory zone
  Given extracted NALU packets
  When pushed into `escam-media`'s broadcast ring
  Then WebRTC and RTSP streaming clients receive live video frames.
```
