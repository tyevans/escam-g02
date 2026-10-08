---
id: '0028'
title: Direct Sensor I2C Register Control and Shutter Integration Timing
status: Accepted
created: 2026-10-06
persona: Elena
target_bc: driver
feature: FEAT-DRV-05
governing_prd: PRD-0008
governing_adrs:
  - ADR-0014
scenarios:
  - Read and write raw CMOS sensor I2C registers (GC1034 / SC1135)
  - Configure hardware analog gain and integration row registers directly
  - Expose sensor telemetry (sensor ID, chip revision, temperature)
---

# US-0028 — Direct Sensor I2C Register Control and Shutter Integration Timing

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0014: Linux Kernel Device Node Abstraction and ioctl Frontdoors`](../../adrs/accepted/adr-0014-linux-kernel-device-node-abstraction.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** to read and write sensor I2C registers directly without relying on proprietary vendor CGI scripts,
**So that** we have hardware-level control over CMOS sensor exposure lines, analog PGA gain, black level calibration, and chip temperature.

## Acceptance Criteria

```gherkin
Scenario: Read and write raw CMOS sensor I2C registers (GC1034 / SC1135)
  Given the `/dev/sensor` or `/dev/i2c-0` kernel interface
  When reading register 0x00 (Chip ID)
  Then the returned value matches the GC1034 (0x1034) or SC1135 sensor signature.

Scenario: Configure hardware analog gain and integration row registers directly
  Given a requested analog gain of 4x and integration time of 500 rows
  When written to the sensor register bank
  Then the hardware sensor registers reflect the target gain and exposure parameters immediately.

Scenario: Expose sensor telemetry (sensor ID, chip revision, temperature)
  Given active sensor communication
  When querying `GET /api/v1/sensor/registers`
  Then a JSON response lists the current register dump and interpreted optical parameters.
```
