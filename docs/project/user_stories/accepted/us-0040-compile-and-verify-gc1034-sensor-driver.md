---
id: '0040'
title: Cross-Compilation and Verification of Open-Source GC1034 Sensor Driver
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: driver
feature: FEAT-DRV-13
governing_prd: PRD-0009
governing_adrs:
  - ADR-0026
scenarios:
  - Cross-compile gc1034_sensor.c against Linux 3.4.43-gk tree
  - Verify V4L2 I2C subdevice driver registration
  - Probe GC1034 chip ID on physical I2C bus
---

# US-0040 — Cross-Compilation and Verification of Open-Source GC1034 Sensor Driver

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0026: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap`](../../adrs/accepted/adr-0026-open-source-kernel-drivers-and-gk7102-peripheral-sourcing.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** to cross-compile and verify `drivers/gc1034_sensor.c` against the Linux 3.4.43-gk kernel tree,
**So that** we have an open-source V4L2 subdevice driver replacing the proprietary binary blob `gc1034_ex.ko` and `sensor.ko`.

## Acceptance Criteria

```gherkin
Scenario: Cross-compile gc1034_sensor.c against Linux 3.4.43-gk tree
  Given `drivers/gc1034_sensor.c`
  When compiled using the Linux 3.4.43-gk kernel build system
  Then `dist/gc1034_sensor.ko` is created with vermagic `3.4.43-Goke`
  And license is declared as `GPL`.

Scenario: Verify V4L2 I2C subdevice driver registration
  Given the compiled `gc1034_sensor.ko` module
  When loaded into the running kernel
  Then it registers an I2C client driver for address `0x21`
  And reports module initialization in kernel `dmesg`.

Scenario: Probe GC1034 chip ID on physical I2C bus
  Given the running sensor driver
  When reading register `0xf0` over the Goke I2C controller
  Then chip ID `0x1034` is validated.
```
