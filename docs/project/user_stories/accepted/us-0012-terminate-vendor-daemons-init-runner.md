---
id: '0012'
title: Terminate vendor daemons and establish clean init runner
status: Accepted
created: 2026-10-05
persona: Elena
target_bc: system
feature: FEAT-SYS-01
governing_prd: PRD-0006
scenarios:
  - Safely terminate stock vendor daemons without kernel panic
  - Install custom startup run script on /mnt/mtd/ipc/conf/run
  - Verify clean startup of escamd under hardware watchdog supervision
---

# US-0012 — Terminate vendor daemons and establish clean init runner

## Governing PRD
- [`PRD-0006: Vendor Software Ejection, Clean Boot, and Production Static Firmware`](../../product/accepted/prd-0006-vendor-software-ejection-and-clean-boot.md)

## User Story

**As an** Elena (Embedded Systems Specialist),
**I want** an ejection procedure that stops the vendor daemons and installs `escamd` into the boot sequence,
**So that** the camera boots exclusively into open-source Rust software with zero vendor telemetry.

## Acceptance Criteria

```gherkin
Scenario: Safely terminate stock vendor daemons without kernel panic
  Given the stock camera firmware running ipc_server, chksock, and net_detect
  When the ejection sequence is triggered
  Then all vendor processes receive SIGTERM followed by SIGKILL
  And kernel drivers remain loaded and responsive without kernel panic.

Scenario: Install custom startup run script on /mnt/mtd/ipc/conf/run
  Given root shell access on the camera
  When the custom run script is written to /mnt/mtd/ipc/conf/run
  Then the script is marked executable
  And persistent across reboots on the writable JFFS2 flash partition.

Scenario: Verify clean startup of escamd under hardware watchdog supervision
  Given a cold boot of the camera hardware
  When Linux completes kernel init
  Then the run script launches escamd directly
  And escamd writes periodic heartbeats to /dev/watchdog to maintain system stability.
```
