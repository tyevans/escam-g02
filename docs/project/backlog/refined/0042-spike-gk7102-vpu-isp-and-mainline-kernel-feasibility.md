---
id: '0042'
title: 'Spike: GK7102 VPU/ISP Hardware Driver Reverse Engineering and Mainline Kernel Feasibility'
status: Refined
dependencies:
  - TASK-0041
governing_adrs:
  - ADR-0020
  - ADR-0026
governing_prds:
  - PRD-0009
governing_stories:
  - US-0034
target_bc: driver
mutation_scope:
  - crates/escam-driver/src/lib.rs
---

# TASK-0042: Spike: GK7102 VPU/ISP Hardware Driver Reverse Engineering and Mainline Kernel Feasibility

## Problem Statement & Context
The proprietary `media.ko` and `hal.ko` kernel modules manage the Goke GK7102C Video Input (VI), Image Signal Processor (ISP), H.264 Video Encoder (VENC), and Media Memory Zone (MMZ). These drivers are currently the primary barrier preventing the ESCAM G02 from running modern mainline Linux (5.x/6.x). This architectural spike reverse-engineers the VPU/ISP interface, examines available SDK sources, and provides a clear technical feasibility report and roadmap for mainline migration.

## Definition of Done (Blackbox Frontdoor TDD)
1. **MMZ & Bitstream Buffer Reverse Engineering**:
   - Audit MMZ contiguous physical memory allocations (default 24 MB split at `0x0000_0000 - 0x0180_0000`).
   - Reverse-engineer `/dev/gk_video` ioctl structures: `GK_ENC_IOC_GET_STREAM`, `GK_VI_IOC_ENABLE`, and buffer ring pointers.
2. **Mainline Linux & OpenIPC Feasibility Analysis**:
   - Compare Goke BSP driver code against modern Linux kernel APIs (V4L2, DMA-BUF, videobuf2, modern interrupt handling).
   - Evaluate whether forward-porting the BSP driver to Linux 4.9 LTS / 5.4 LTS or developing a minimal V4L2 bridge is more viable.
   - Investigate OpenIPC's GK7102 branch for reusable device tree and kernel patches.
3. **Architectural Spike Report**:
   - Deliver comprehensive Diataxis explanation document: `docs/explanation/gk7102-kernel-modernization-roadmap.md`.
   - Provide concrete effort estimations, risk analysis, and hardware register references.
4. All source and documentation files strictly $< 500$ lines.
