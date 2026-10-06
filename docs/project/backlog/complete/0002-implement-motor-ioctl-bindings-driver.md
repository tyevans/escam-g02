---
id: '0002'
title: Reverse Engineer and Implement Motor ioctl Device Bindings
status: Complete
governing_adrs:
- ADR-0014
- ADR-0003
governing_prds:
- PRD-0001
governing_stories:
- US-0001
target_bc: driver
persona: Elena
mutation_scope:
- crates/escam-driver/src/motor.rs
---

# TASK-0002: Reverse Engineer and Implement Motor ioctl Device Bindings

## Problem Statement & Context
The ESCAM G02 stepper motor kernel driver (`motor.ko`) exposes character device node `/dev/motor`. In order to drive the pan and tilt motors from Rust without vendor software, we must implement type-safe bindings for the reverse-engineered `ioctl` commands: `MOTOR_STOP` (0xC0046D00), `MOTOR_RUN` (0xC0046D01 with `pandir` and `titldir`), and `MOTOR_SPEED` (0xC0046D06).

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `escam-driver::motor` module with type-safe `MotorDevice` trait and `LinuxMotorDevice`.
2. Implement `MockMotorDevice` for frontdoor unit testing and property testing without physical hardware.
3. Validate ioctl numeric values:
   - `MOTOR_STOP = 0xC0046D00`
   - `MOTOR_RUN = 0xC0046D01`
   - `MOTOR_SPEED = 0xC0046D06`
4. Verify `MotorRun` struct layout matches C ABI: `{ pandir: i32, titldir: i32 }` (8 bytes).
5. All new files strictly under 500 lines.
