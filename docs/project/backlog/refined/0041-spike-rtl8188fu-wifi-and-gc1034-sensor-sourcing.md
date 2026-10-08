---
id: '0041'
title: 'Spike: Sourcing Open-Source RTL8188FTV Wi-Fi and GC1034 Sensor Drivers'
status: Refined
dependencies:
  - TASK-0040
governing_adrs:
  - ADR-0026
governing_prds:
  - PRD-0009
governing_stories:
  - US-0033
target_bc: driver
mutation_scope:
  - crates/escam-driver/src/sensor.rs
---

# TASK-0041: Spike: Sourcing Open-Source RTL8188FTV Wi-Fi and GC1034 Sensor Drivers

## Problem Statement & Context
The stock firmware uses vendor proprietary kernel modules `8188fu.ko` (Realtek RTL8188FTV USB Wi-Fi) and `gc1034_ex.ko` / `sensor.ko` (GalaxyCore GC1034 CMOS image sensor). Both components have open-source driver equivalents available in the Linux community and OpenIPC projects. This spike sources, compiles, and verifies these open-source drivers to eliminate these proprietary binary blobs.

## Definition of Done (Blackbox Frontdoor TDD)
1. **RTL8188FTV Open-Source Driver**:
   - Clone and configure community open-source `rtl8188fu` driver for ARMv6 cross-compilation.
   - Verify kernel module builds cleanly against target Linux headers and recognizes USB device ID `0bda:f179`.
   - Test WPA2 network connection and ping stability.
2. **GC1034 Sensor Driver Adaptation**:
   - Source the open-source GC1034 Linux I2C subdevice driver from OpenIPC / mainline Linux staging trees.
   - Verify I2C device address `0x21` initialization sequence, PLL clock setup, and Bayer output format over DVP.
   - Cross-verify register programming with `SensorI2cDriver` in `crates/escam-driver/src/sensor.rs`.
3. All source and documentation files strictly $< 500$ lines.
