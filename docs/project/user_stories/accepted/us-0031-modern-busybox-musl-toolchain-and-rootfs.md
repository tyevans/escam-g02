---
id: '0031'
title: Modern Musl-Linked BusyBox Toolchain and RootFS Modernization
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: system
feature: FEAT-SYS-05
governing_prd: PRD-0009
governing_adrs:
  - ADR-0013
  - ADR-0025
scenarios:
  - Cross-compile static BusyBox 1.36+ with musl for ARMv6 architecture
  - Verify core applet functionality on Linux 3.4.43 kernel without dynamic linker
  - Package modern rootfs overlay within flash size limits
---

# US-0031 — Modern Musl-Linked BusyBox Toolchain and RootFS Modernization

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0013: Rust Cross-Compilation and Musl Static Toolchain for ARMv6`](../../adrs/accepted/adr-0013-rust-cross-compilation-and-musl-static-toolchain-armv6.md)
- [`ADR-0025: Modern Musl-Linked BusyBox Userspace and RootFS Modernization`](../../adrs/accepted/adr-0025-modern-busybox-musl-userspace-and-rootfs-modernization.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** to cross-compile and deploy modern BusyBox 1.36+ built with a static musl toolchain for ARMv6,
**So that** I have a modern, secure, bug-free POSIX userland environment that runs immediately on the existing kernel without depending on obsolete uClibc binaries or waiting for kernel driver replacement.

## Acceptance Criteria

```gherkin
Scenario: Cross-compile static BusyBox 1.36+ with musl for ARMv6 architecture
  Given a musl cross-compilation toolchain targeting `arm-unknown-linux-musleabi` (ARMv6)
  When BusyBox 1.36+ is configured with static linking and essential applets
  Then the compilation produces a standalone static binary with no external shared library dependencies
  And the binary size is strictly under 1.2 MB uncompressed.

Scenario: Verify core applet functionality on Linux 3.4.43 kernel without dynamic linker
  Given a target system running Linux 3.4.43-Goke
  When the modern BusyBox executable is executed
  Then core system applets (`ash`, `ls`, `ps`, `cat`, `grep`, `insmod`, `rmmod`, `lsmod`, `ifconfig`, `udhcpc`, `telnetd`) execute successfully
  And syscall interactions with `/proc`, `/sys`, and `/dev` succeed without runtime ABI errors.

Scenario: Package modern rootfs overlay within flash size limits
  Given a set of modernized userland configurations and BusyBox symlinks
  When a replacement rootfs or overlay image is generated for the 1.9MB `mtd3` partition
  Then the total compressed SquashFS footprint remains under 1.5 MB
  And leaves at least 400 KB of margin in flash memory.
```
