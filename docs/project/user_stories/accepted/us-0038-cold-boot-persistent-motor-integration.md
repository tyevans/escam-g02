---
id: '0038'
title: Cold-Boot Persistent Hardware Motor Driver Integration
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: driver
feature: FEAT-DRV-11
governing_prd: PRD-0009
governing_adrs:
  - ADR-0026
scenarios:
  - Deploy escam_motor.ko to persistent flash on hardware
  - Patch camera run script to load escam_motor.ko on cold boot
  - Cold reboot camera and verify automatic motor driver registration
---

# US-0038 — Cold-Boot Persistent Hardware Motor Driver Integration

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0026: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap`](../../adrs/accepted/adr-0026-open-source-kernel-drivers-and-gk7102-peripheral-sourcing.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** `escam_motor.ko` to be persistently loaded from `/mnt/mtd/ipc/conf/` at system boot time,
**So that** the camera boots cold with our open-source motor and GPIO drivers active without requiring manual telnet commands.

## Acceptance Criteria

```gherkin
Scenario: Deploy escam_motor.ko to persistent flash on hardware
  Given the compiled `dist/escam_motor.ko`
  When transferred to `/mnt/mtd/ipc/conf/escam_motor.ko` on JFFS2 flash
  Then the file size and md5 checksum match between host and camera.

Scenario: Patch camera run script to load escam_motor.ko on cold boot
  Given `/mnt/mtd/ipc/conf/run` on the camera
  When updated with the clean-boot conditional
  Then `loadmotor()` checks for `escam_motor.ko`, unloads `gkio`, loads `escam_motor.ko`, and creates `/dev/motor` (243, 0) and `/dev/gkio` (242, 0).

Scenario: Cold reboot camera and verify automatic motor driver registration
  Given the updated persistent configuration
  When the camera reboots
  Then `lsmod` shows `escam_motor` is loaded
  And vendor `motor.ko` and `gkio.ko` are not loaded
  And `escamd` starts and connects to physical hardware.
```
