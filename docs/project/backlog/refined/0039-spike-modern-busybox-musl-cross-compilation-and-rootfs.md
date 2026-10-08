---
id: '0039'
title: 'Spike: Modern Musl-Linked BusyBox Cross-Compilation and RootFS Modernization'
status: Refined
dependencies:
  - TASK-0037
governing_adrs:
  - ADR-0013
  - ADR-0025
governing_prds:
  - PRD-0009
governing_stories:
  - US-0031
target_bc: system
mutation_scope:
  - crates/escam-system/src/debloat.rs
---

# TASK-0039: Spike: Modern Musl-Linked BusyBox Cross-Compilation and RootFS Modernization

## Problem Statement & Context
The ESCAM G02 stock root filesystem (`mtd3_rootfs.bin`) contains an outdated BusyBox v1.18.1 dynamically linked against `uClibc-0.9.33.2`. We must verify that modern BusyBox (1.36+) cross-compiled statically with `arm-unknown-linux-musleabi` can run directly on the existing Linux 3.4.43-Goke kernel without waiting for kernel module replacements. This spike establishes the reproducible build recipe, verifies applet compatibility, and packages a lightweight rootfs overlay.

## Definition of Done (Blackbox Frontdoor TDD)
1. **Toolchain & Build Recipe**:
   - Provide automated build script/configuration under `scripts/build_busybox.sh` or Buildroot recipe for BusyBox 1.36+ static musl ARMv6.
   - Verify uncompressed binary footprint is $< 1.2$ MB.
2. **Compatibility Verification**:
   - Audit system call compatibility on Linux 3.4 kernel (`init_module`, `syslog`, `ioctl`, `mdev`).
   - Validate that BusyBox applets (`ash`, `ls`, `ps`, `cat`, `grep`, `insmod`, `rmmod`, `lsmod`, `ifconfig`, `udhcpc`, `telnetd`) execute without ABI faults.
3. **RootFS Overlay Packaging**:
   - Create a minimal SquashFS rootfs structure (`mtd3_rootfs.bin`) containing modern BusyBox symlinks, essential `/etc` skeletons, and device nodes.
   - Verify the resulting SquashFS image fits comfortably within the 1.9 MB flash partition ($< 1.5$ MB target).
4. **Integration with `escam-system`**:
   - Update clean-boot and debloat logic to recognize and verify the modernized BusyBox binary path.
5. All source and documentation files strictly $< 500$ lines.
