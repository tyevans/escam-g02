---
id: '0005'
title: Hardware H.264 NALU Ingestion and RFC 6184 RTP Packetizer
status: Complete
governing_adrs:
- ADR-0015
- ADR-0003
governing_prds:
- PRD-0002
governing_stories:
- US-0004
target_bc: media
mutation_scope:
- crates/escam-media/src/nalu.rs
- crates/escam-media/src/rtp.rs
---

# TASK-0005: Hardware H.264 NALU Ingestion and RFC 6184 RTP Packetizer

## Problem Statement & Context
The hardware video encoder produces continuous H.264 Annex-B streams through `/dev/venc`. To deliver sub-100ms video without transcoding, `escam-media` must parse Annex-B start codes, extract discrete NAL units, and packetize them into RFC 6184 RTP packets (Single NALU, STAP-A for SPS/PPS, and FU-A fragmentation for large keyframes).

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement Annex-B bitstream parser supporting 3-byte and 4-byte start codes.
2. Implement RFC 6184 RTP packetizer with MTU-bounded (1400 bytes) FU-A fragmentation.
3. Frontdoor unit tests and property-based tests verify randomized bitstreams are lossless when reconstructed from RTP packets.
4. All new files strictly under 500 lines.
