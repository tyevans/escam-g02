---
id: '0043'
title: Persistent In-Vivo Integration of escam_motor Driver
status: Complete
dependencies:
- TASK-0040
governing_adrs:
- ADR-0026
governing_prds:
- PRD-0009
governing_stories:
- US-0035
target_bc: driver
mutation_scope:
- drivers/escam_motor.c
- crates/escam-system/src/debloat.rs
---

# TASK-0043: Persistent In-Vivo Integration of escam_motor Driver

## Problem Statement & Context
Now that `escam_motor.ko` has been successfully cross-compiled, verified in-vivo against the Linux 3.4.43-gk kernel, and actuated through `motor-test` and `escamd`, it must be persistently integrated into the camera's writable JFFS2 flash (`/mnt/mtd/ipc/conf/run`) so that it loads automatically at boot instead of vendor `motor.ko` and `gkio.ko`.

## Definition of Done (Blackbox Frontdoor TDD)
1. **Persistent Flash Staging**:
   - Install `dist/escam_motor.ko` to `/mnt/mtd/ipc/conf/escam_motor.ko` on the camera.
2. **Clean Boot Integration**:
   - Update `/mnt/mtd/ipc/conf/run` and `crates/escam-system/src/debloat.rs` with clean boot logic: dynamically unload `gkio.ko` and load `escam_motor.ko`, keeping fail-safe fallback to vendor `motor.ko` if the open-source driver is missing.
3. **Blackbox Frontdoor Parity**:
   - Verify that upon reboot, `escamd` acquires `/dev/motor` and `/dev/gkio` directly and processes PTZ REST requests without vendor motor modules loaded.
4. All source and documentation files strictly $< 500$ lines.
