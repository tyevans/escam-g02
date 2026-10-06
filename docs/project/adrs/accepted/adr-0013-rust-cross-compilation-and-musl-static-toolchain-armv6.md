---
id: '0013'
title: Rust Cross-Compilation and Musl Static Toolchain for ARMv6 (ARM1176JZF-S)
status: Accepted
date: 2026-10-05
deciders:
  - Elena
  - Alex
---

# ADR-0013: Rust Cross-Compilation and Musl Static Toolchain for ARMv6 (ARM1176JZF-S)

## Status
Accepted

## Context
The ESCAM G02 camera uses the Goke Microelectronics GK7102C SoC, featuring an ARM1176JZF-S (ARMv6) core running at ~600MHz with 64MB integrated SiP DDR2 memory and an embedded Linux 3.4.43 kernel. The stock userspace relies on uClibc with obsolete library versions. To run modern, memory-safe Rust binaries on this target without dynamic linking friction or glibc incompatibilities, we need a reliable static compilation pipeline.

## Decision
1. We target **`arm-unknown-linux-musleabi`** (ARMv6 soft-float EABI, static linking).
2. All compiled binaries (`motor-test`, `escamd`) are statically linked with musl libc, resulting in single self-contained binaries that execute on any Linux kernel with zero external dynamic library dependencies.
3. Cross-compilation is driven via standard `cargo build --target arm-unknown-linux-musleabi` using either Zig (`cargo-zigbuild`) or pre-configured musl cross-toolchains (`arm-linux-musleabi-gcc`).
4. Output binaries are stripped and optimized with `opt-level = "z"` / `lto = true` to maintain binary sizes under 5MB for flash deployment and <8MB runtime RAM usage.

## Consequences
- **Positive**: 100% standalone static binaries; immunity to host/target glibc version mismatches; seamless upload into camera `/tmpfs` via curl/tftp or permanent installation on SPI flash.
- **Negative**: ARMv6 musl builds require soft-float software emulation for floating point routines if hardware VFP is not configured.
