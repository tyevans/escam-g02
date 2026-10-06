---
id: '0015'
title: Reduce Firmware Binary Footprint Below 800KB via LTO and Profile Tuning
status: Accepted
created: 2026-10-06
persona: Elena
target_bc: core
feature: FEAT-SYS-04
governing_prd: PRD-0007
governing_adrs:
  - ADR-0021
scenarios:
  - Configure aggressive Cargo release profile for size minimization
  - Verify stripped musl binary footprint is under 800KB uncompressed
---

# US-0015 — Reduce Firmware Binary Footprint Below 800KB via LTO and Profile Tuning

## Governing PRD & ADR
- [`PRD-0007: Performance Optimization, Binary Footprint Shrinking, and Deep-Sky Astrophotography`](../../product/accepted/prd-0007-optimization-binary-shrinking-and-deep-sky-astronomy.md)
- [`ADR-0021: Binary Footprint Reduction, Zero-Allocation Media Pipelines, and Sidereal Tracking`](../../adrs/accepted/adr-0021-binary-footprint-reduction-and-zero-allocation-pipeline.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** the compiled `escamd` binary to be smaller than 800KB uncompressed,
**So that** it preserves precious SPI flash memory for persistent logs and astro captures without sacrificing performance.

## Acceptance Criteria

```gherkin
Scenario: Configure aggressive Cargo release profile for size minimization
  Given the workspace Cargo.toml
  When the release profile is configured with opt-level = "z", lto = "fat", and codegen-units = 1
  Then cargo build --release --target arm-unknown-linux-musleabi compiles cleanly.

Scenario: Verify stripped musl binary footprint is under 800KB uncompressed
  Given the release escamd binary
  When arm-linux-gnueabi-strip is executed
  Then the resulting stripped binary is strictly under 850KB uncompressed
  And gzip -9 compressed size is under 450KB.
```
