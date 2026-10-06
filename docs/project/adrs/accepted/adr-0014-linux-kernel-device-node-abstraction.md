---
id: '0014'
title: Linux Kernel Device Node Abstraction and ioctl Frontdoors
status: Accepted
date: 2026-10-05
deciders:
  - Elena
  - Alex
---

# ADR-0014: Linux Kernel Device Node Abstraction and ioctl Frontdoors

## Status
Accepted

## Context
The stock vendor userland (`ipc_server`) communicates with low-level hardware peripherals through kernel character device nodes:
1. `/dev/motor`: Managed by `motor.ko` for 2-axis stepper motors driven via a `JULN2803AG` Darlington transistor array.
2. `/dev/gkio`: Managed by `gkio.ko` for general-purpose I/O, specifically driving GPIO 14 and GPIO 17 to control the UTC BA6208L H-bridge for the mechanical IR-cut filter.
3. `/dev/venc` / `/dev/vi`: Managed by `media.ko` for Goke video encoding and ISP frame acquisition.

To eradicate vendor daemon dependencies, our Rust platform must communicate directly with these kernel interfaces using safe, typed Rust wrappers.

## Decision
1. Implement a pure Rust hardware abstraction crate (`escam-driver`) wrapping Linux `ioctl` and `mmap` syscalls using `nix` / `libc`.
2. Map the reverse-engineered `motor.ko` ioctl commands into a type-safe enum:
   - `MOTOR_STOP` (`0xC0046D00`): Immediate coil power-off.
   - `MOTOR_RUN` (`0xC0046D01`): Struct `{ pandir: i32, titldir: i32 }` (1=CW, 2=CCW, 3=Up, 4=Down).
   - `MOTOR_CYCLE` (`0xC0046D02`): Continuous patrol scan.
   - `MOTOR_AUTO_CHECK` (`0xC0046D05`): Limit calibration and homing.
   - `MOTOR_SPEED` (`0xC0046D06`): Timer divisor speed configuration.
   - `MOTOR_GET_STATUS` (`0xC0046D08`): Busy/moving check.
   - `MOTOR_GET_POS` (`0xC0046D13`): Position step readout.
3. Map `/dev/gkio` ioctl `0xC0046200` (`_IOWR('b', 0, struct { u32 pin, u32 val })`) with a dedicated 100ms pulse driver for GPIO 14 (Day / Filter ON) and GPIO 17 (Night / Filter OFF).
4. Provide simulated mock backends (`MockMotorDevice`, `MockGpioDevice`) for host-side unit and property-based verification without requiring physical hardware.

## Consequences
- **Positive**: Complete independence from proprietary vendor daemons; zero risk of memory corruption from closed-source C code; full local testing on development host.
- **Negative**: Requires careful synchronization of ioctl timing and pulse durations to avoid coil overheating.
