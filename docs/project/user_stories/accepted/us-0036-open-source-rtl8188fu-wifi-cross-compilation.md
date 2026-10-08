---
id: '0036'
title: Verified Cross-Compilation of RTL8188FU Wi-Fi Driver
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: driver
feature: FEAT-DRV-10
governing_prd: PRD-0009
governing_adrs:
  - ADR-0026
scenarios:
  - Configure build_rtl8188fu.sh for Linux 3.4.43-gk kernel tree
  - Compile 8188fu.ko with matching 3.4.43-Goke vermagic
  - Verify exported symbols and driver modinfo
---

# US-0036 — Verified Cross-Compilation of RTL8188FU Wi-Fi Driver

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0026: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap`](../../adrs/accepted/adr-0026-open-source-kernel-drivers-and-gk7102-peripheral-sourcing.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** to cross-compile the sourced `rtl8188fu` open-source driver against our matching Linux 3.4.43-gk kernel tree,
**So that** we have a fully verifiable, reproducible replacement for the binary vendor blob `8188fu.ko`.

## Acceptance Criteria

```gherkin
Scenario: Configure build_rtl8188fu.sh for Linux 3.4.43-gk kernel tree
  Given the OpenIPC GK710x Linux 3.4.43 kernel source tree
  When `scripts/build_rtl8188fu.sh` is invoked with kernel path and ARM cross-toolchain
  Then it points to the configured kernel headers and sets target architecture ARMv6.

Scenario: Compile 8188fu.ko with matching 3.4.43-Goke vermagic
  Given the open-source RTL8188FU repository
  When compiled under Linux 3.4.43-gk
  Then a valid `8188fu.ko` kernel module is output in `dist/`
  And its vermagic strictly matches `3.4.43-Goke`.

Scenario: Verify exported symbols and driver modinfo
  Given the compiled `dist/8188fu.ko`
  When inspected via `modinfo` and `nm`
  Then wireless extensions and `rtw_init_netdev` symbols are present
  And module dependencies match the camera's wireless subsystem.
```
