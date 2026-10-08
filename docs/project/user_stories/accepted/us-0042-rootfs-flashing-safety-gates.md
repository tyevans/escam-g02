---
id: '0042'
title: RootFS Flashing Final Safety Gates and Non-Destructive Preflight
status: Accepted
created: 2026-10-07
persona: Alex
target_bc: system
feature: FEAT-SYS-11
governing_prd: PRD-0009
governing_adrs:
  - ADR-0025
scenarios:
  - Verify non-destructive preflight checks report 100% pass
  - Stage mtd3_rootfs.bin into RAM and verify target sha256 checksum
  - Pause execution at the point of irreversible flash erase
---

# US-0042 — RootFS Flashing Final Safety Gates and Non-Destructive Preflight

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0025: Musl Static Toolchain and RootFS Reassembly Architecture`](../../adrs/accepted/adr-0025-musl-toolchain-and-rootfs-reassembly.md)

## User Story
**As an** Alex (Systems Architect & Quality Custodian),
**I want** an enforced, non-destructive safety gate that stages the modern rootfs into RAM and verifies target checksums while halting before raw flash erasure,
**So that** autonomous processes can push verification to the absolute limit without risking an unrecoverable device brick while unattended.

## Acceptance Criteria

```gherkin
Scenario: Verify non-destructive preflight checks report 100% pass
  Given `scripts/flash_rootfs.sh --dry-run`
  When executed against the target camera hardware
  Then ICMP ping, MTD partition size, and SquashFS superblocks are verified.

Scenario: Stage mtd3_rootfs.bin into RAM and verify target sha256 checksum
  Given `scripts/flash_rootfs.sh --flash`
  When image is transferred to `/mnt/mtd/ipc/tmpfs/new_rootfs.bin`
  Then remote sha256 calculation matches the host image exactly.

Scenario: Pause execution at the point of irreversible flash erase
  Given target checksum match and watchdog stability
  When reaching the point of raw flash partition erasure (`flash_erase /dev/mtd3`)
  Then the process halts safely, presenting the exact manual verification status.
```
