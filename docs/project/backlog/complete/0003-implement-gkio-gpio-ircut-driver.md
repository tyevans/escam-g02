---
id: '0003'
title: Implement /dev/gkio GPIO and BA6208L IR-Cut H-Bridge Driver
status: Complete
governing_adrs:
- ADR-0014
- ADR-0003
governing_prds:
- PRD-0001
governing_stories:
- US-0002
target_bc: driver
persona: Elena
mutation_scope:
- crates/escam-driver/src/gpio.rs
---

# TASK-0003: Implement /dev/gkio GPIO and BA6208L IR-Cut H-Bridge Driver

## Problem Statement & Context
The UTC BA6208L H-bridge driver drives the mechanical IR-cut filter solenoid on the optical lens assembly. Reverse engineering revealed that GPIO 14 (Day mode / filter ON) and GPIO 17 (Night/Astro mode / filter OFF) are driven via ioctl `0xC0046200` on `/dev/gkio` with a 100ms active-high pulse before returning to low.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `escam-driver::gpio` module with type-safe `GpioDevice` trait and `LinuxGpioDevice`.
2. Implement `MockGpioDevice` for automated testing.
3. Validate ioctl numeric value: `GKIO_SET_VALUE = 0xC0046200` with 8-byte payload `{ pin: u32, val: u32 }`.
4. Implement `IrCutController` that manages Day / Night mode transitions, generating the precise 100ms pulse and verifying that pins return to 0 to prevent coil burnout.
5. All new files strictly under 500 lines.
