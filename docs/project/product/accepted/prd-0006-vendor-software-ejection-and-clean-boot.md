---
id: '0006'
title: Vendor Software Ejection, Clean Boot, and Production Static Firmware
status: Accepted
created: 2026-10-05
target_persona: Elena
component: system
---

# PRD-0006 — Vendor Software Ejection, Clean Boot, and Production Static Firmware

## Who this is for

- **Elena**: Embedded systems specialist wanting complete eradication of proprietary vendor binaries, instant 2-second boot, and zero cloud telemetry.

## What the person cannot do today

- Today, 4 separate vendor daemons (`ipc_server`, `chksock`, `net_detect`, `watchdog`) run constantly, consuming 45 MB of 64 MB RAM, spawning unencrypted UDP/TCP connections to foreign IP addresses, and rebooting unexpectedly if watchdog queries fail.

## What good looks like

1. **Vendor Daemon Ejection Script**:
   - Clean shutdown sequence that safely halts `ipc_server`, `chksock`, `net_detect`, and proprietary watchdogs without kernel panics.
2. **Permanent Startup Override (`/mnt/mtd/ipc/conf/run`)**:
   - Replaces the vendor boot script on the writable JFFS2 partition to launch `escamd` exclusively.
3. **Instant Clean Boot & Watchdog Management**:
   - System boots from power-on to active live stream in under 2.0 seconds.
   - Total runtime RAM consumption under 8.0 MB (leaving >54 MB free).
   - `escamd` feeds the hardware watchdog timer (`/dev/watchdog`) directly with a clean heartbeat.

## What this does not do

- Overwriting the U-Boot bootloader partition (`mtd0_boot.bin`), ensuring the hardware remains unbrickable via serial console.

## Checkable Outcomes

1. `netstat -tlpn` and `ps` on target device verify zero vendor daemons running and zero WAN outbound connections.
2. Free memory reported by `cat /proc/meminfo` confirms >50 MB available RAM.
3. Hardware watchdog does not trip or reboot during continuous operation.

## Linked User Stories

- [`US-0012: Terminate vendor daemons and establish clean init runner`](../../user_stories/accepted/us-0012-terminate-vendor-daemons-init-runner.md)
- [`US-0013: Measure and verify boot time <2s and RAM consumption <8MB`](../../user_stories/accepted/us-0013-verify-boot-time-and-memory-footprint.md)

## Implementing Backlog Tasks

- `TASK-0013`: Implement startup orchestration, watchdog heartbeat, and vendor ejection script in `escam-system`.
- `TASK-0014`: Benchmark memory usage, boot timing, and network traffic isolation on target device.
