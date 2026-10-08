---
id: '0047'
title: In-Vivo Verification and Testing of Open-Source RTL8188FU Wi-Fi Driver
status: Complete
dependencies:
- TASK-0044
governing_adrs:
- ADR-0026
governing_prds:
- PRD-0009
governing_stories:
- US-0039
target_bc: driver
mutation_scope:
- scripts/build_rtl8188fu.sh
---

# TASK-0047: In-Vivo Verification and Testing of Open-Source RTL8188FU Wi-Fi Driver

## Problem Statement & Context
The open-source `dist/8188fu.ko` has been successfully cross-compiled with vermagic `3.4.43-Goke`. We must stage it on the camera hardware in RAM, verify module loading parameters, check interface creation (`wlan0`), and test network connectivity.

## Definition of Done (Blackbox Frontdoor TDD)
1. **Target Staging**:
   - Transfer `dist/8188fu.ko` to `/mnt/mtd/ipc/tmpfs/8188fu.ko` via TFTP.
2. **Dynamic In-Vivo Verification**:
   - Verify `insmod /mnt/mtd/ipc/tmpfs/8188fu.ko` binds to USB ID `0bda:f179` and registers wireless network device.
3. **Blackbox Frontdoor Parity**:
   - Verify test suite confirms driver compatibility and module parameters.
4. All source and documentation files strictly $< 500$ lines.
