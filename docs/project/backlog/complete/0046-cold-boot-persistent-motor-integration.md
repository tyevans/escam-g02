---
id: '0046'
title: Cold-Boot Persistent Hardware Motor Driver Integration
status: Complete
dependencies:
- TASK-0043
governing_adrs:
- ADR-0026
governing_prds:
- PRD-0009
governing_stories:
- US-0038
target_bc: driver
mutation_scope:
- crates/escam-system/src/debloat.rs
---

# TASK-0046: Cold-Boot Persistent Hardware Motor Driver Integration

## Problem Statement & Context
To ensure `escam_motor.ko` is persistently loaded every time the camera powers on, the startup script in JFFS2 flash (`/mnt/mtd/ipc/conf/run`) must be patched to unload vendor `gkio` and load `/mnt/mtd/ipc/conf/escam_motor.ko` at boot, with fail-safe fallback to vendor `motor.ko` if the custom module is missing.

## Definition of Done (Blackbox Frontdoor TDD)
1. **Persistent Flash Staging**:
   - Verify `/mnt/mtd/ipc/conf/escam_motor.ko` is staged on camera JFFS2 flash.
2. **Init Script Integration**:
   - Patch `/mnt/mtd/ipc/conf/run` on the camera to execute the clean-boot motor logic.
3. **Cold Boot Verification**:
   - Trigger a cold reboot, verify the camera returns online on Wi-Fi, `lsmod` confirms `escam_motor` is loaded (and `motor.ko` is NOT loaded), and `escamd` accepts PTZ REST commands.
4. All source and documentation files strictly $< 500$ lines.
