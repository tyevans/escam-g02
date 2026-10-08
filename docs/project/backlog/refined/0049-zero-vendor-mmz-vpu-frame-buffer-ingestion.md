---
id: '0049'
title: Zero-Vendor MMZ Physical VPU Frame Buffer Ingestion
status: Refined
dependencies:
- TASK-0042
governing_adrs:
- ADR-0020
- ADR-0026
governing_prds:
- PRD-0009
governing_stories:
- US-0041
target_bc: media
mutation_scope:
- crates/escam-driver/src/vpu.rs
- crates/escam-media/src/vpu_stream.rs
---

# TASK-0049: Zero-Vendor MMZ Physical VPU Frame Buffer Ingestion

## Problem Statement & Context
The proprietary `media.ko` and `hal.ko` drivers manage the H.264 video encoder by allocating frame buffers in the Media Memory Zone (MMZ, `0x0000_0000 - 0x0180_0000`). To achieve zero-vendor video streaming, we must implement a direct MMZ physical frame reader via `/dev/mem` that extracts Annex-B H.264 NALUs and forwards them directly to `escam-media`'s broadcast ring.

## Definition of Done (Blackbox Frontdoor TDD)
1. **MMZ Buffer Reader Implementation**:
   - Provide a memory-mapped ring reader in `crates/escam-driver/src/vpu.rs` or `crates/escam-media/src/vpu_stream.rs` targeting the MMZ bitstream zone.
2. **Packet Parsing**:
   - Parse packet descriptors, extract SPS/PPS/IDR/P NALUs, and push into `escam-media`'s broadcast channel.
3. **Blackbox Frontdoor Parity**:
   - Verify 100% test pass rate with simulated MMZ ring buffers on host and live hardware compatibility.
4. All source and documentation files strictly $< 500$ lines.
