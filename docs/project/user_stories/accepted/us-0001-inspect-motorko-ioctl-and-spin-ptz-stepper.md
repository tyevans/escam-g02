---
id: '0001'
title: Inspect motor.ko ioctl and spin PTZ stepper
status: Accepted
created: 2026-10-05
persona: Elena
target_bc: driver
feature: FEAT-DRIVER-01
governing_prd: PRD-0001
scenarios:
  - Verify motor ioctl command encoding and struct layout
  - Command pan and tilt motor rotation via safe driver wrapper
  - Halt motor rotation and verify coil power off
---

# US-0001 — Inspect motor.ko ioctl and spin PTZ stepper

## Governing PRD
- [`PRD-0001: Core Hardware Driver Layer & Stepper Motor Reverse Engineering`](../../product/accepted/prd-0001-core-hardware-driver-layer---stepper-motor-re.md)

## User Story

**As an** Elena (Embedded Systems Specialist),
**I want** a type-safe Rust driver wrapping the `/dev/motor` kernel device node,
**So that** I can command pan and tilt stepper motors directly with verified `ioctl` commands without proprietary vendor daemons.

## Acceptance Criteria

```gherkin
Scenario: Verify motor ioctl command encoding and struct layout
  Given the reverse-engineered motor.ko kernel module definitions
  When the driver encodes MOTOR_RUN, MOTOR_STOP, and MOTOR_SPEED commands
  Then MOTOR_STOP evaluates to 0xC0046D00
  And MOTOR_RUN evaluates to 0xC0046D01 with an 8-byte payload matching struct MotorRun
  And MOTOR_SPEED evaluates to 0xC0046D06 with a 4-byte APB timer divisor.

Scenario: Command pan and tilt motor rotation via safe driver wrapper
  Given a mock or physical motor device node at /dev/motor
  When the driver sends a run command with pan direction Right and tilt direction Up
  Then the device driver receives MOTOR_RUN with pandir=1 and titldir=3
  And the motor begins stepping without errors.

Scenario: Halt motor rotation and verify coil power off
  Given an actively stepping motor
  When the driver issues a stop command
  Then the device driver receives MOTOR_STOP (0xC0046D00)
  And all Darlington driver phases are de-energized to prevent coil overheating.
```
