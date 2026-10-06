---
id: '0021'
title: Binary Footprint Reduction, Zero-Allocation Media Pipelines, and Sidereal Tracking
status: Accepted
date: 2026-10-06
deciders: Lead Architect, Elena, Dr. Marcus Vance
governing_prds:
  - PRD-0007
---

# ADR-0021: Binary Footprint Reduction, Zero-Allocation Media Pipelines, and Sidereal Tracking

## Context & Problem Statement
On the 64MB Goke GK7102C SoC, every megabyte of flash and RAM is critical. While `escamd` currently consumes only ~12MB RSS and 1.2MB of uncompressed flash, further shrinking and performance tuning will unlock headroom for advanced computational photography:
1. Rust binary size can be significantly reduced using aggressive release flags (`opt-level = "z"`, `codegen-units = 1`, `lto = "fat"`, `panic = "abort"`).
2. High-frequency H.264 NALU parsing currently incurs frequent dynamic heap allocations, causing potential memory fragmentation under 24/7 runtime.
3. For deep-sky astrophotography, the stepper motors must support continuous micro-pulsing at the Earth's sidereal rotation rate ($15.041067\text{ arcsec/sec}$).

## Decision Drivers
- **Extreme Flash Compactness**: Binary must fit into minimal flash partition alongside kernels.
- **Zero Heap Fragmentation**: Media ingestion must reuse bounded buffers.
- **Celestial Precision**: Stepper timing must accurately track the night sky without motor overheating.

## Considered Options
1. **Option A: Status Quo**: Maintain current 1.2MB binary and per-frame heap allocations.
2. **Option B: Aggressive Optimization & Feature Expansion**:
   - Tune Cargo release profiles with size optimizations.
   - Refactor `escam-media` to use pre-allocated static frame buffers.
   - Implement `SiderealTracker` in `escam-ptz` with discrete sub-step timing.
   - Implement in-memory frame stacking accumulator in `escam-astro`.

## Decision Outcome
Chosen **Option B**.

### Architectural Invariants:
1. **Compilation Invariant**: The release profile for `arm-unknown-linux-musleabi` must enforce `opt-level = "z"`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, and symbol stripping.
2. **Allocation Invariant**: Streaming loops must maintain a pre-allocated fixed buffer pool for Annex-B NALU extraction.
3. **Tracking Invariant**: Sidereal rate pulses must respect minimum motor pulse width to avoid step skipping.
