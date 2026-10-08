---
id: '0026'
title: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap
status: Accepted
date: 2026-10-07
deciders:
  - Elena
  - Alex
---

# ADR-0026: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap

## Status
Accepted

## Context
The ESCAM G02 camera currently requires several proprietary vendor kernel modules (`.ko` files) residing in `/mnt/mtd/ipc/modules/` built specifically with vermagic `3.4.43-Goke`. Because Linux has no stable internal kernel ABI, these proprietary binary modules are the sole hard blocker preventing upgrading the operating system to a modern mainline Linux kernel (5.x or 6.x) or adopting mainline OpenIPC distributions.

To achieve complete firmware independence, we require a systematic roadmap to replace or open-source each peripheral driver.

## Decision
We classify and execute the replacement of vendor kernel modules across four difficulty tiers:

### Tier 1: Realtek RTL8188FTV Wi-Fi (`8188fu.ko`, `rtkm.ko`)
- **Status**: Trivial / Already Available.
- **Approach**: The Realtek RTL8188FTV USB Wi-Fi chipset has well-maintained community open-source drivers on GitHub (`rtl8188fu` / `r8188eu`). We source and cross-compile this driver cleanly from public source trees, eliminating proprietary blobs for 802.11n networking.

### Tier 2: GPIO & PTZ Stepper Motor (`gkio.ko`, `motor.ko`)
- **Status**: Low / Highly Feasible.
- **Approach**:
  - `gkio.ko` controls simple memory-mapped GPIO lines (IR-Cut filter solenoid, IR illuminator LEDs, day/night light sensor). We replace it with standard Linux `gpio-sysfs` / `/dev/gpiochip` or a lightweight open-source C kernel driver maintaining exact ioctl compatibility.
  - `motor.ko` sequences step pulses to the JULN2803AG Darlington array across 8 GPIO output lines. We replace it with an open-source driver or a high-resolution timer (`hrtimer`) kernel module that exposes identical `/dev/motor` ioctls (`0x4d01` - `0x4d04`).

### Tier 3: GalaxyCore GC1034 Sensor (`sensor.ko`, `gc1034_ex.ko`)
- **Status**: Moderate.
- **Approach**: The GC1034 is a standard I2C-controlled CMOS sensor communicating via DVP/MIPI. Open-source Linux drivers for GC1034 exist in Rockchip, Allwinner, and OpenIPC repositories. We adapt the standard V4L2 subdevice driver to program exposure, gain, and Bayer pixel timing.

### Tier 4: GK7102 VPU & ISP Subsystems (`media.ko`, `hal.ko`)
- **Status**: High / Architectural Spike Required.
- **Approach**:
  - `media.ko` orchestrates the hardware H.264 encoder, VI input engine, ISP demosaicing/AE/AWB pipeline, and MMZ contiguous memory zone.
  - Phase 1: Conduct a focused architectural spike reverse-engineering the VPU register space and DMA ring buffer structures.
  - Phase 2: Evaluate forward-porting the vendor GK7102 SDK C driver sources to Linux 4.9 LTS / 5.4 LTS vs adopting OpenIPC GK7102 kernel branches.

## Consequences
### Positive
- Clear, phased pathway to completely eliminating proprietary kernel binaries.
- Tier 1 and Tier 2 drivers can be replaced immediately with zero architectural risk.
- Opens the door to booting modern mainline Linux (5.x/6.x) once Tier 3 and Tier 4 are resolved.

### Negative
- Reverse-engineering or forward-porting the VPU/ISP driver requires deep embedded Linux kernel development and access to hardware register documentation.
