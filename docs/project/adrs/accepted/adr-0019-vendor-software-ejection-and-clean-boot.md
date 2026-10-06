---
id: '0019'
title: Vendor Software Ejection, Clean Boot, and Init Orchestration
status: Accepted
date: 2026-10-05
deciders:
  - Elena
  - Samir
  - Alex
---

# ADR-0019: Vendor Software Ejection, Clean Boot, and Init Orchestration

## Status
Accepted

## Context
The stock firmware starts a swarm of closed-source background daemons on boot from `/mnt/mtd/ipc/conf/run` and `/app/run`:
- `ipc_server`: Monolithic 1.2MB C binary handling web, RTSP, PTZ, and cloud P2P.
- `chksock`: Socket watchdog pinging cloud servers.
- `net_detect`: Continuously reconfiguring network interfaces and checking WAN connectivity.
- `watchdog`: Hardware/software watchdog daemon resetting the camera if vendor processes crash.

These binaries consume ~45 MB of the camera's 64 MB RAM, trigger frequent CPU spikes, and establish unencrypted outbound cloud connections.

## Decision
1. Provide a phased migration strategy:
   - **Phase A (Development / RAM testing)**: Upload static Rust test binaries (`motor-test`, `escamd`) to RAM via HTTP PUT (`/tmpfs/`) or curl, and execute via the existing root shell on port 2323.
   - **Phase B (Ejection & Hijack)**: Create a replacement startup script `/mnt/mtd/ipc/conf/run` that terminates vendor daemons (`killall -9 ipc_server chksock net_detect watchdog`) and launches `escamd` exclusively.
   - **Phase C (Pure Init)**: Package a clean SquashFS/JFFS2 filesystem where `escamd` acts as the primary userland process and handles Linux watchdog heartbeat (`/dev/watchdog`) directly.
2. Ensure `escamd` maintains a total runtime RAM footprint under 8MB, leaving >54MB of RAM free for video buffers, network sockets, and astronomical image caching.
3. Cold boot time to live WebRTC streaming target is <2.0 seconds.

## Consequences
- **Positive**: Complete privacy; 0% chance of vendor backdoor communication; dramatically improved system reliability and responsiveness.
- **Negative**: Modifying `/mnt/mtd/ipc/conf/run` requires maintaining a fail-safe recovery mechanism (e.g. holding physical reset button or UART console recovery) to prevent bricking.
