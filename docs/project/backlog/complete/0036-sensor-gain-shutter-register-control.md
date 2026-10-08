---
id: '0036'
title: Direct Sensor I2C Register Control and Fine Hardware Integration Timing
status: Complete
governing_adrs:
  - ADR-0014
governing_prds:
  - PRD-0008
governing_stories:
  - US-0028
target_bc: driver
mutation_scope:
  - crates/escam-driver/src/sensor.rs
---

# TASK-0036: Direct Sensor I2C Register Control and Fine Hardware Integration Timing

## Problem Statement & Context
Hardware camera sensors (GalaxyCore GC1034 / SC1135) feature internal registers for analog programmable gain amplifiers (PGA), integration line counters (shutter length), black level clamp, and test patterns. Controlling these registers directly via I2C enables true hardware long exposure and zero-vendor reliance.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `SensorI2cDriver` in `crates/escam-driver/src/sensor.rs`:
   - Direct register read/write over `/dev/i2c-0` or ioctl interface with mock fallback for host testing.
   - High-level methods: `set_analog_gain(gain: f32)`, `set_shutter_lines(lines: u32)`, `get_sensor_id() -> u16`, `set_black_level(offset: u8)`.
   - Comprehensive register map for GC1034 (Chip ID 0x1034) and SC1135.
2. Expose REST endpoints:
   - `GET /api/v1/sensor/registers`
   - `POST /api/v1/sensor/registers`
3. Frontdoor tests verifying register bitfield packing and gain scaling calculation.
4. All source files strictly <500 lines (target <400 lines).
