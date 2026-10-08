---
id: '0044'
title: Compile and Verify Open-Source RTL8188FU Wi-Fi Driver
status: Complete
dependencies:
- TASK-0041
governing_adrs:
- ADR-0026
governing_prds:
- PRD-0009
governing_stories:
- US-0036
target_bc: driver
mutation_scope:
- scripts/build_rtl8188fu.sh
---

# TASK-0044: Compile and Verify Open-Source RTL8188FU Wi-Fi Driver

## Problem Statement & Context
The project previously sourced the RTL8188FU repository and authored `scripts/build_rtl8188fu.sh`. Now that the OpenIPC GK710x Linux 3.4.43-gk kernel tree has been cloned and prepared, we must verify that `scripts/build_rtl8188fu.sh` compiles `8188fu.ko` against this exact kernel ABI, matching vermagic `3.4.43-Goke` and producing a drop-in replacement for the proprietary vendor blob.

## Definition of Done (Blackbox Frontdoor TDD)
1. **Compilation Automation**:
   - Update `scripts/build_rtl8188fu.sh` to accept `KERNEL_SRC` pointing to the Linux 3.4.43-gk tree and build `8188fu.ko`.
2. **Binary Verification**:
   - Verify `modinfo dist/8188fu.ko` reports vermagic `3.4.43-Goke` and license `GPL`.
3. **Module Inspection**:
   - Verify exported symbols include wireless extensions and `rtw_init_netdev`.
4. All source and documentation files strictly $< 500$ lines.
