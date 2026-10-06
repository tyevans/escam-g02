---
id: '0003'
title: Cross-compile and deploy standalone motor-test binary
status: Accepted
created: 2026-10-05
persona: Elena
target_bc: driver
feature: FEAT-DRIVER-03
governing_prd: PRD-0001
scenarios:
  - Cross-compile motor-test for arm-unknown-linux-musleabi
  - Upload motor-test into camera RAM via curl
  - Execute motor-test via root shell on port 2323
---

# US-0003 — Cross-compile and deploy standalone motor-test binary

## Governing PRD
- [`PRD-0001: Core Hardware Driver Layer & Stepper Motor Reverse Engineering`](../../product/accepted/prd-0001-core-hardware-driver-layer---stepper-motor-re.md)

## User Story

**As an** Elena (Embedded Systems Specialist),
**I want** to cross-compile a minimal static Rust binary (`motor-test`) and execute it on the physical camera over Wi-Fi,
**So that** I can empirically verify our reverse-engineered motor control ioctls on the actual hardware before building the full daemon.

## Acceptance Criteria

```gherkin
Scenario: Cross-compile motor-test for arm-unknown-linux-musleabi
  Given the Rust source code for motor-test
  When cargo builds with target arm-unknown-linux-musleabi in release mode
  Then the output binary is an ELF 32-bit ARM executable statically linked with musl
  And the binary file size is under 2 MB stripped.

Scenario: Upload motor-test into camera RAM via curl
  Given the camera is running at IP 10.75.2.93 with HTTP digest/basic auth
  When the binary is uploaded via HTTP PUT to /tmpfs/motor-test
  Then the camera web server returns HTTP 200/201 Success
  And the binary is stored in executable RAM.

Scenario: Execute motor-test via root shell on port 2323
  Given an active root Telnet connection to the camera on port 2323
  When chmod +x and /tmpfs/motor-test are invoked
  Then the binary opens /dev/motor without error
  And sweeps the pan axis 45 degrees left and right
  And outputs clean execution telemetry to stdout.
```
