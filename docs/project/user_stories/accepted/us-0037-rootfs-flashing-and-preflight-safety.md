---
id: '0037'
title: RootFS Flashing and Preflight Safety Automation
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: system
feature: FEAT-SYS-10
governing_prd: PRD-0009
governing_adrs:
  - ADR-0025
scenarios:
  - Verify SquashFS image integrity and partition boundaries
  - Dry-run validation of MTD flashing script
  - Safe staging of image and checksum on hardware
---

# US-0037 — RootFS Flashing and Preflight Safety Automation

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0025: Musl Static Toolchain and RootFS Reassembly Architecture`](../../adrs/accepted/adr-0025-musl-toolchain-and-rootfs-reassembly.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** an automated, multi-gate preflight flashing script for `dist/mtd3_rootfs.bin`,
**So that** upgrading the camera's base rootfs from factory BusyBox 1.18.1 to modern Musl BusyBox 1.36.1 is completely safeguarded against bricking or partial writes.

## Acceptance Criteria

```gherkin
Scenario: Verify SquashFS image integrity and partition boundaries
  Given a rebuilt rootfs image at `dist/mtd3_rootfs.bin`
  When validated by `scripts/flash_rootfs.sh --check`
  Then the image size is verified to be strictly <= 1,992,294 bytes (0x1D0000)
  And contains a valid SquashFS 4.0 superblock and checksum.

Scenario: Dry-run validation of MTD flashing script
  Given the flash script running in `--dry-run` mode
  When checking target MTD partition `/dev/mtd3`
  Then it verifies partition name "rootfs", calculates erase blocks, and aborts before write.

Scenario: Safe staging of image and checksum on hardware
  Given the staging command with host TFTP
  When `mtd3_rootfs.bin` is transferred to camera RAM
  Then sha256 checksum is calculated and compared on the target before any flash operation.
```
