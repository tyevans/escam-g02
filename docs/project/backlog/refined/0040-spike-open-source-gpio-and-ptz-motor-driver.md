---
id: '0040'
title: 'Spike: Open-Source GPIO and PTZ Stepper Motor Driver Replacement'
status: Refined
dependencies:
  - TASK-0039
governing_adrs:
  - ADR-0014
  - ADR-0016
  - ADR-0026
governing_prds:
  - PRD-0009
governing_stories:
  - US-0032
target_bc: driver
mutation_scope:
  - crates/escam-driver/src/motor.rs
  - crates/escam-driver/src/gpio.rs
---

# TASK-0040: Spike: Open-Source GPIO and PTZ Stepper Motor Driver Replacement

## Problem Statement & Context
The ESCAM G02 relies on proprietary vendor kernel modules `motor.ko` and `gkio.ko` for stepper motor actuation and GPIO line control (IR-cut filter and IR LEDs). Because these modules only control digital GPIO pins connected to a JULN2803AG Darlington array and a BA6208L H-bridge, they can be replaced by a clean, open-source C kernel module or userspace GPIO timer driver.

## Definition of Done (Blackbox Frontdoor TDD)
1. **Hardware Pin Mapping & Reverse Engineering**:
   - Document exact SoC GPIO pin assignments for pan (4 phases), tilt (4 phases), IR-cut forward/reverse pulse lines, and IR LED toggle.
2. **Open-Source Driver Implementation**:
   - Provide a clean, standalone open-source C driver (`drivers/gk_motor.c` or equivalent) or userspace pulse engine that matches the ioctl interface of `/dev/motor` (`0x4d01` - `0x4d04`) and `/dev/gkio`.
   - Implement microsecond-level step timing using standard Linux `hrtimer` or high-resolution timers.
3. **Blackbox Frontdoor Parity**:
   - Verify that `crates/escam-driver` executes pan, tilt, and IR-cut operations against the open-source driver with 100% test pass rate.
   - Verify smooth S-curve trajectory execution without step loss or timing jitter.
4. All source and documentation files strictly $< 500$ lines.
