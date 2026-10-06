---
id: '0004'
title: Build Standalone Cross-Compiled motor-test ARMv6 musl Binary
status: Complete
governing_adrs:
- ADR-0013
- ADR-0014
governing_prds:
- PRD-0001
governing_stories:
- US-0003
target_bc: driver
mutation_scope:
- crates/motor-test/src/main.rs
---

# TASK-0004: Build Standalone Cross-Compiled motor-test ARMv6 musl Binary

## Problem Statement & Context
Before deploying the full async Tokio camera daemon, we must build a tiny, self-contained proof-of-concept binary `motor-test` in Rust, cross-compile it for `arm-unknown-linux-musleabi`, and provide upload/execution automation to spin the physical pan and tilt motors.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `crates/motor-test` crate depending on `escam-driver`.
2. Cross-compilation target `arm-unknown-linux-musleabi` compiles cleanly in release mode.
3. Binary runs locally in simulation mode (`--mock`) verifying pan left/right, tilt up/down, and stop sequence.
4. Provide deployment script to upload via `curl -u admin:admin -T ... http://<IP>/tmpfs/motor-test` and execute via root shell.
5. All new files strictly under 500 lines.
