---
id: '0013'
title: Startup Init Daemon, Watchdog Heartbeat, and Vendor Ejection Runner
status: Complete
governing_adrs:
- ADR-0019
- ADR-0003
governing_prds:
- PRD-0006
governing_stories:
- US-0012
target_bc: system
persona: Elena
mutation_scope:
- crates/escam-system/src/init.rs
- crates/escam-system/src/watchdog.rs
---

# TASK-0013: Startup Init Daemon, Watchdog Heartbeat, and Vendor Ejection Runner

## Problem Statement & Context
To permanently boot into `escamd` without proprietary background software, `escam-system` provides the vendor ejection script, replaces `/mnt/mtd/ipc/conf/run`, initializes kernel modules (`motor.ko`, `gkio.ko`, `media.ko`), and services the hardware watchdog (`/dev/watchdog`) with periodic heartbeats.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement watchdog manager feeding `/dev/watchdog` at configurable intervals (e.g., 5 seconds) to prevent hardware reboots.
2. Implement vendor ejection script (`eject-vendor.sh`) terminating `ipc_server`, `chksock`, `net_detect`, and `watchdog`.
3. Generate replacement `/mnt/mtd/ipc/conf/run` launch script.
4. Provide safe fallback/recovery mode if launch fails.
5. All new files strictly under 500 lines.
