---
id: '0035'
title: Persistent In-Vivo Integration of escam_motor.ko Driver
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: driver
feature: FEAT-DRV-09
governing_prd: PRD-0009
governing_adrs:
  - ADR-0026
scenarios:
  - Deploy escam_motor.ko to persistent flash partition
  - Hook escam_motor.ko into startup script with fail-safe fallback
  - Verify escamd connects to physical hardware on boot without vendor motor modules
---

# US-0035 — Persistent In-Vivo Integration of escam_motor.ko Driver

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0026: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap`](../../adrs/accepted/adr-0026-open-source-kernel-drivers-and-gk7102-peripheral-sourcing.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** to integrate our open-source `escam_motor.ko` into the camera's persistent flash startup configuration,
**So that** the camera boots natively with our replacement motor and GPIO drivers without loading proprietary vendor `motor.ko` or `gkio.ko`.

## Acceptance Criteria

```gherkin
Scenario: Deploy escam_motor.ko to persistent flash partition
  Given the compiled `dist/escam_motor.ko` binary with vermagic 3.4.43-Goke
  When staged onto the camera at `/mnt/mtd/ipc/conf/escam_motor.ko`
  Then the file persists across cold reboots with integrity intact.

Scenario: Hook escam_motor.ko into startup script with fail-safe fallback
  Given `/mnt/mtd/ipc/conf/run` on the writable JFFS2 flash partition
  When `loadmotor()` executes at system boot
  Then it checks for `/mnt/mtd/ipc/conf/escam_motor.ko`
  And dynamically unloads `gkio` and loads `escam_motor.ko`
  And falls back to vendor `motor.ko` if the open-source driver is absent.

Scenario: Verify escamd connects to physical hardware on boot without vendor motor modules
  Given the camera boots with `escam_motor.ko` loaded
  When `escamd` initializes `/dev/motor` and `/dev/gkio`
  Then character device nodes at major 243 and 242 open successfully
  And PTZ motion and IR-cut solenoid actuation function without error.
```
