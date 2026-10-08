---
id: '0045'
title: RootFS Flashing Script and Preflight Safety Automation
status: Complete
dependencies:
- TASK-0039
governing_adrs:
- ADR-0025
governing_prds:
- PRD-0009
governing_stories:
- US-0037
target_bc: system
mutation_scope:
- scripts/flash_rootfs.sh
---

# TASK-0045: RootFS Flashing Script and Preflight Safety Automation

## Problem Statement & Context
A rebuilt SquashFS 4.0 rootfs (`dist/mtd3_rootfs.bin`) containing modern Musl BusyBox 1.36.1 has been created and verified under `scripts/rebuild_rootfs.sh`. To flash this image onto `/dev/mtd3` on the camera without risking a brick, we need an automated flashing script with rigorous preflight validation (partition size check, sha256 checksum matching, watchdog suppression, dry-run mode, and post-write verification).

## Definition of Done (Blackbox Frontdoor TDD)
1. **Flashing Script Implementation**:
   - Provide `scripts/flash_rootfs.sh` with `--check`, `--dry-run`, and `--flash` modes.
2. **Preflight Guardrails**:
   - Verify image size $\le 1,992,294$ bytes (0x1D0000).
   - Verify target partition `/dev/mtd3` matches name "rootfs".
   - Calculate sha256 hash locally and verify target file integrity prior to write.
3. **Safety Verification**:
   - Include automated fallback documentation and recovery procedures.
4. All source and documentation files strictly $< 500$ lines.
