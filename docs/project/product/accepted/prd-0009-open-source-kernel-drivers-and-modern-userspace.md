---
id: '0009'
title: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace
status: Accepted
author: Lead Architect
created: 2026-10-07
target_persona: Elena
target_bc: driver
---

# PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace

## Executive Summary
Having successfully replaced the vendor userland with the pure-Rust `escamd` daemon and debloated unneeded cloud daemons, the camera still relies on legacy vendor kernel modules (`media.ko`, `hal.ko`, `gc1034_ex.ko`, `sensor.ko`, `motor.ko`, `gkio.ko`, `8188fu.ko`) locked to an obsolete Linux 3.4.43-Goke kernel with a 2010-era BusyBox v1.18.1 userland.

This product milestone defines the strategy and execution path to:
1. **Modernize RootFS & BusyBox Immediately**: Cross-compile modern BusyBox (1.36+) statically using musl for ARMv6, replacing obsolete uClibc 0.9.33 utilities without waiting for kernel module changes.
2. **Replace Peripheral Kernel Modules with Open-Source Drivers**:
   - Replace `motor.ko` with a clean, open-source stepper driver or standard Linux GPIO/timer pulse generator.
   - Replace `gkio.ko` with standard Linux GPIO interfaces (`gpio-sysfs` / `gpiod`).
   - Source community-maintained open-source drivers for Realtek RTL8188FTV Wi-Fi (`rtl8188fu`).
   - Source/adapt GalaxyCore GC1034 image sensor driver under the Linux V4L2 subdevice architecture.
3. **Conduct Architectural Spikes for GK7102 VPU/ISP & Mainline Linux**:
   - Reverse-engineer and document the Goke GK7102C VPU/ISP hardware register map and MMZ DMA buffer allocation.
   - Assess feasibility and establish a roadmap for porting GK7102C BSP drivers to modern mainline Linux (5.x/6.x) or an OpenIPC-based firmware image.

## Target Personas
- **Elena (The Embedded Systems Specialist)**: Seeks 100% open-source driver visibility, reproducible rootfs builds, modern POSIX userland tools, and elimination of proprietary `.ko` black boxes.
- **Alex (The Systems Architect & Quality Custodian)**: Requires deterministic hardware abstraction, clean device node contracts, and zero reliance on unmaintained vendor toolchains.

## Success Criteria & Key Performance Indicators (KPIs)
- **Userspace Modernization**: Static BusyBox 1.36+ runs on ARMv6 with zero dynamic linking errors, fitting inside $< 1.2$ MB uncompressed.
- **Peripheral Driver Openness**: Pan/tilt stepper motors and IR-Cut GPIO operate with 100% open-source C kernel modules or userspace drivers, achieving parity with legacy `motor.ko` and `gkio.ko`.
- **Wi-Fi Reproducibility**: Open-source `rtl8188fu` driver compiles against kernel headers and establishes WPA2 Wi-Fi association without vendor binary blobs.
- **Mainline Feasibility Clear Path**: An empirical architectural spike report detailing GK7102 VPU/ISP register blocks, memory requirements, and a concrete Linux 5.x porting strategy.
- **File Length & Health**: 100% of specification and source files $< 500$ lines with 0 `spec-ops health` warnings.
