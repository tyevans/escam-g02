---
id: '0002'
title: Actuate UTC BA6208L IR-Cut H-Bridge via /dev/gkio
status: Accepted
created: 2026-10-05
persona: Elena
target_bc: driver
feature: FEAT-DRIVER-02
governing_prd: PRD-0001
scenarios:
  - Pulse GPIO 14 to engage Day mode IR-cut filter
  - Pulse GPIO 17 to engage Night mode full-spectrum filter
  - Verify GPIO return to zero after 100ms pulse
---

# US-0002 — Actuate UTC BA6208L IR-Cut H-Bridge via /dev/gkio

## Governing PRD
- [`PRD-0001: Core Hardware Driver Layer & Stepper Motor Reverse Engineering`](../../product/accepted/prd-0001-core-hardware-driver-layer---stepper-motor-re.md)

## User Story

**As an** Elena (Embedded Systems Specialist),
**I want** to toggle the UTC BA6208L H-bridge driver using `/dev/gkio` ioctl calls,
**So that** I can mechanically switch the optical IR-cut filter between Day mode (IR blocked) and Night/Astro mode (Hα and full spectrum passed).

## Acceptance Criteria

```gherkin
Scenario: Pulse GPIO 14 to engage Day mode IR-cut filter
  Given the camera optical shutter is in Night mode
  When the driver requests Day mode activation
  Then an ioctl command 0xC0046200 is issued to /dev/gkio setting GPIO pin 14 to high (1)
  And after a 100ms delay, GPIO pin 14 is set back to low (0)
  And the mechanical IR-cut filter slides over the image sensor.

Scenario: Pulse GPIO 17 to engage Night mode full-spectrum filter
  Given the camera optical shutter is in Day mode
  When the driver requests Night or Astro mode activation
  Then an ioctl command 0xC0046200 is issued to /dev/gkio setting GPIO pin 17 to high (1)
  And after a 100ms delay, GPIO pin 17 is set back to low (0)
  And the mechanical filter retracts to allow near-IR and Hydrogen-Alpha light.

Scenario: Verify GPIO return to zero after 100ms pulse
  Given an IR-cut transition is executed on either GPIO 14 or GPIO 17
  When the actuation cycle completes
  Then both GPIO 14 and GPIO 17 are verified in low state (0)
  And zero continuous current flows through the H-bridge solenoid coils.
```
