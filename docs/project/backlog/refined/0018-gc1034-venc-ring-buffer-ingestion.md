---
id: '0018'
title: Goke GC1034 /dev/gk_video Ring Buffer Ingestion Engine
status: Refined
governing_adrs:
- ADR-0014
- ADR-0015
governing_prds:
- PRD-0003
governing_stories:
- US-0004
target_bc: media
persona: Marcus
mutation_scope:
- crates/escam-media/src/nalu.rs
---

# TASK-0018: Goke GC1034 /dev/gk_video Ring Buffer Ingestion Engine

## Problem Statement & Context
The GC1034 sensor streams 720p H.264 video through `media.ko` and `/dev/gk_video` (channel 11). `escam-media` must read NALUs directly from the character device or local RTSP loopback and parse SPS, PPS, and IDR/P-slice NALUs.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement ring buffer stream reader extracting Annex-B formatted NALUs (`00 00 00 01`).
2. Verify SPS/PPS extraction and parameter validation in `escam-media`.
3. Add unit and property tests verifying arbitrary packet stream extraction.
4. Verify extraction against live hardware video channel.
5. All files strictly under 500 lines.
