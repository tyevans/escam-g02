---
id: '0021'
title: Vendor Daemon Ejection and In-Memory Persistent Boot Installer
status: Refined
governing_adrs:
- ADR-0010
- ADR-0019
governing_prds:
- PRD-0006
governing_stories:
- US-0012
- US-0013
target_bc: system
persona: Alex
mutation_scope:
- crates/escam-system/src/init.rs
---

# TASK-0021: Vendor Daemon Ejection and In-Memory Persistent Boot Installer

## Problem Statement & Context
The stock firmware runs heavy C daemons (`ipc_server`, `chksock`, `net_detect`, `watchdog`) using 45MB of the 64MB total RAM. Once `escamd` provides video streaming and PTZ control, we must eject the vendor software and configure `/mnt/mtd/ipc/conf/run` to launch `escamd` exclusively.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement automated vendor process termination (`killall -9 ipc_server chksock net_detect`).
2. Generate and test `/mnt/mtd/ipc/conf/run` boot invocation launching `escamd`.
3. Measure boot time (<2s) and memory footprint (<8MB RSS) via `ProcessMemory` telemetry auditor.
4. Verify root telnet shell remains active for emergency access.
5. All files strictly under 500 lines.
