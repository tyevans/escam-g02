---
id: '0016'
title: Persistent Hardware Watchdog Keepalive Daemon and Magic Close Handler
status: Complete
governing_adrs:
- ADR-0014
- ADR-0019
governing_prds:
- PRD-0006
governing_stories:
- US-0012
target_bc: system
mutation_scope:
- crates/escam-system/src/watchdog.rs
---

# TASK-0016: Persistent Hardware Watchdog Keepalive Daemon and Magic Close Handler

## Problem Statement & Context
The Goke GK7102C SoC kernel initializes `gk_wdt_v1_00` with a 60-second hardware watchdog timer. If `/dev/watchdog` is not fed continuously or closed without writing the magic character `'V'`, the hardware reboots the SoC. `escamd` must manage this device safely in a background async task.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement persistent background feeding loop in `escamd` sending `\0` every 5 seconds.
2. Implement safe shutdown hook writing `'V'` (`WDT_MAGIC_CLOSE`) on process termination.
3. Validate with unit tests in `crates/escam-system` and frontdoor tests in `tests/test_frontdoors.py`.
4. Verify on camera hardware that uptime exceeds 120 seconds without watchdog reboot.
5. All files strictly under 500 lines.
