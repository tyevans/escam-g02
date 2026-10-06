---
id: '0023'
title: Aggressive Binary Size Optimization and LTO Profile Tuning
status: Complete
governing_adrs:
- ADR-0013
- ADR-0021
governing_prds:
- PRD-0007
governing_stories:
- US-0015
target_bc: core
mutation_scope:
- crates/escamd/src/main.rs
---

# TASK-0023: Aggressive Binary Size Optimization and LTO Profile Tuning

## Problem Statement & Context
On the 8MB SPI NOR flash of the ESCAM G02, minimizing executable size ensures that future firmware upgrades, astronomical calibration files, and logs have ample space. Currently `escamd` is ~1.2MB uncompressed. By tuning Cargo release profiles with size optimizations (`opt-level = "z"`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`), we can dramatically shrink the binary footprint.

## Definition of Done (Blackbox Frontdoor TDD)
1. Configure workspace `Cargo.toml` with aggressive size-optimized release profile.
2. Build for `arm-unknown-linux-musleabi` and verify the stripped binary compiles cleanly.
3. Validate that the uncompressed stripped binary size is $< 950$ KB (down from 1.2MB).
4. Run all unit tests and verify 100% test pass rate.
5. All source files strictly under 500 lines.
