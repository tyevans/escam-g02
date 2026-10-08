---
id: '0032'
title: Open-Source Stepper Motor and GPIO Peripheral Drivers
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: driver
feature: FEAT-DRV-06
governing_prd: PRD-0009
governing_adrs:
  - ADR-0014
  - ADR-0016
  - ADR-0026
scenarios:
  - Implement open-source C kernel module for PTZ stepper motor control
  - Implement open-source GPIO driver for IR-Cut and illumination control
  - Verify complete ioctl parity with escam-driver crate
---

# US-0032 — Open-Source Stepper Motor and GPIO Peripheral Drivers

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0014: Linux Kernel Device Node Abstraction and ioctl Frontdoors`](../../adrs/accepted/adr-0014-linux-kernel-device-node-abstraction.md)
- [`ADR-0016: Smooth PTZ Stepper Motor Control via Sinusoidal S-Curve Trajectory Profiling`](../../adrs/accepted/adr-0016-smooth-ptz-stepper-motor-control.md)
- [`ADR-0026: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap`](../../adrs/accepted/adr-0026-open-source-kernel-drivers-and-gk7102-peripheral-sourcing.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** to replace the proprietary `motor.ko` and `gkio.ko` binary kernel modules with 100% open-source C drivers or standard Linux GPIO subsystem bindings,
**So that** camera motion and optical filter actuation are free from closed vendor blobs and portable to newer Linux kernels.

## Acceptance Criteria

```gherkin
Scenario: Implement open-source C kernel module for PTZ stepper motor control
  Given hardware schematics showing 8 GPIO control pins wired to the JULN2803AG Darlington array
  When an open-source stepper driver is built and loaded at `/dev/motor`
  Then it accepts the standard `MOTOR_RUN` ioctl commands (`0x4d01` - `0x4d04`)
  And drives the pan and tilt steppers with deterministic pulse intervals without missing steps.

Scenario: Implement open-source GPIO driver for IR-Cut and illumination control
  Given hardware lines controlling the UTC BA6208L H-bridge and IR illumination LEDs
  When the open-source GPIO driver exposes `/dev/gkio` or standard `/dev/gpiochip` lines
  Then issuing forward and reverse pulses triggers the mechanical IR-cut filter solenoid
  And setting the IR LED control line switches the 850nm illuminators on and off.

Scenario: Verify complete ioctl parity with escam-driver crate
  Given the existing `crates/escam-driver` codebase
  When integration tests execute against the open-source driver device nodes
  Then all motor and GPIO commands succeed without changes to userland Rust logic
  And pass 100% of blackbox frontdoor tests.
```
