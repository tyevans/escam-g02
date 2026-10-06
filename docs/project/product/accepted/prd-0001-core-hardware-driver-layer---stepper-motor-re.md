---
id: '0001'
title: Core Hardware Driver Layer & Stepper Motor Reverse Engineering
status: Accepted
created: 2026-10-05
target_persona: Elena
component: driver
---

# PRD-0001 — Core Hardware Driver Layer & Stepper Motor Reverse Engineering

## Who this is for

- **Elena**: Embedded Systems Engineer needing direct, type-safe hardware control over the camera's stepper motors, IR-cut solenoid H-bridge, and video device nodes without running vendor daemons.

## What the person cannot do today

- Today, developers must reverse engineer obscure, undocumented closed-source binary drivers (`motor.ko`, `gkio.ko`, `ipc_server`) to understand the `ioctl` control contracts.
- There are no safe, idiomatic Rust hardware abstraction layers to drive the `JULN2803AG` stepper driver or UTC BA6208L H-bridge solenoid.
- Controlling hardware requires running the vendor `ipc_server` binary which phones home to cloud servers and uses 45+ MB RAM.

## What good looks like

1. **Type-Safe Motor Device Interface**:
   - A pure Rust driver library (`escam-driver`) wrapping `/dev/motor`.
   - Complete implementation of the mapped `motor.ko` ioctl interface:
     - `MOTOR_STOP` (`0xC0046D00`)
     - `MOTOR_RUN` (`0xC0046D01`) with structured `MotorRun { pandir: i32, titldir: i32 }`
     - `MOTOR_CYCLE` (`0xC0046D02`)
     - `MOTOR_AUTO_CHECK` (`0xC0046D05`)
     - `MOTOR_SPEED` (`0xC0046D06`)
     - `MOTOR_GET_STATUS` (`0xC0046D08`)
     - `MOTOR_GET_POS` (`0xC0046D13`)
2. **Type-Safe GPIO & IR-Cut Interface**:
   - Pure Rust `/dev/gkio` wrapper handling `0xC0046200` ioctl (`struct GpioVal { pin: u32, val: u32 }`).
   - 100ms pulse actuation for GPIO 14 (Day mode, IR-cut ON) and GPIO 17 (Night mode, IR-cut OFF).
3. **Cross-Compiled Proof-of-Concept Binary (`motor-test`)**:
   - Minimal standalone Rust binary statically compiled for `arm-unknown-linux-musleabi`.
   - Can be uploaded over Wi-Fi to `/tmpfs/motor-test` and executed via root shell to spin pan and tilt axes cleanly.

## What this does not do

- High-level trajectory interpolation and velocity smoothing (handled by PRD-0003).
- User-facing web interfaces or REST endpoints (handled by PRD-0005).

## Checkable Outcomes

1. `escam-driver` compiles cleanly with zero warnings for both host target and `arm-unknown-linux-musleabi`.
2. Mock driver tests verify 100% of ioctl encodings and struct memory layouts match the Linux 32-bit ARM ABI.
3. Standalone `motor-test` binary executes without dynamic linker errors, powers up the stepper motors, and executes a test rotation sweep.
4. IR-cut toggle function verifies GPIO 14 and 17 receive exactly 100ms active-high pulses and return to low.

## Linked User Stories

- [`US-0001: Inspect motor.ko ioctl and spin PTZ stepper`](../../user_stories/accepted/us-0001-inspect-motorko-ioctl-and-spin-ptz-stepper.md)
- [`US-0002: Actuate UTC BA6208L IR-Cut H-Bridge via /dev/gkio`](../../user_stories/accepted/us-0002-actuate-ir-cut-h-bridge-via-dev-gkio.md)
- [`US-0003: Cross-compile and deploy standalone motor-test binary`](../../user_stories/accepted/us-0003-cross-compile-and-deploy-motor-test-binary.md)

## Implementing Backlog Tasks

- `TASK-0002`: Reverse engineer and implement `escam-driver` motor ioctl bindings.
- `TASK-0003`: Implement `/dev/gkio` GPIO driver and IR-cut solenoid pulse controller.
- `TASK-0004`: Build and cross-compile standalone `motor-test` ARMv6 musl proof-of-concept binary.
