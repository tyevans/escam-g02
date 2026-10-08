---
id: '0050'
title: RootFS Flashing Safety Gates and Pre-Flight Validation
status: Refined
dependencies:
- TASK-0045
governing_adrs:
- ADR-0025
governing_prds:
- PRD-0009
governing_stories:
- US-0042
target_bc: system
mutation_scope:
- scripts/flash_rootfs.sh
---

# TASK-0050: RootFS Flashing Safety Gates and Pre-Flight Validation

## Problem Statement & Context
Flashing the SPI NOR flash partition `/dev/mtd3` is an irreversible operation that could render the camera unbootable if corrupted. We must stage `mtd3_rootfs.bin` into camera RAM, verify sha256 checksums on the target, test watchdog stability, and pause safely at the threshold of physical flash erasure so the user can review before committing.

## Definition of Done (Blackbox Frontdoor TDD)
1. **Target Pre-Flight Validation**:
   - Verify `scripts/flash_rootfs.sh --flash` transfers `dist/mtd3_rootfs.bin` to `/mnt/mtd/ipc/tmpfs/new_rootfs.bin`.
2. **On-Target Hash Verification**:
   - Verify SHA-256 calculation on the camera hardware matches the local image.
3. **Execution Gate**:
   - Confirm all non-destructive stages succeed and halt prior to physical `flash_erase` execution.
4. All source and documentation files strictly $< 500$ lines.
