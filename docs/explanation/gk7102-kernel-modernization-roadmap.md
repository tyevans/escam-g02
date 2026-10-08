# Explanation: Goke GK7102C Kernel Modernization and Mainline Migration Roadmap

This document provides a comprehensive architectural analysis of the proprietary multimedia subsystem on the ESCAM G02 (Goke GK7102C SoC), details the reverse-engineered hardware blocks, and establishes a verified technical roadmap for migrating from Linux 3.4.43-Goke to modern mainline Linux (5.x/6.x) and OpenIPC.

---

## 1. System-on-Chip & Memory Architecture

The Goke GK7102C integrates an **ARM1176JZF-S (ARMv6)** core clocked at 600 MHz with **64 MB of System-in-Package (SiP) DDR2 memory**.

### Physical DRAM Memory Map

```
0x0000_0000 ┌─────────────────────────────────────────┐
            │                                         │
            │   MMZ (Media Memory Zone)               │
            │   Reserved for Hardware DMA & VPU       │
            │   (24 MB contiguous physical RAM)       │
            │                                         │
0x0180_0000 ├─────────────────────────────────────────┤
            │                                         │
            │   Linux Kernel & User Space             │
            │   (40 MB physical RAM)                  │
            │   - escamd (<10 MB RSS)                 │
            │   - Static Musl BusyBox                 │
            │                                         │
0x0400_0000 └─────────────────────────────────────────┘
```

The proprietary memory allocator (`media.ko`) hard-reserves the lower 24 MB (`0x0000_0000 - 0x0180_0000`) at boot using the `mem=40M` kernel command-line parameter. Hardware VPU frame buffers and ISP temporary accumulation buffers reside exclusively in this contiguous zone.

---

## 2. Multimedia Subsystem Decomposition

The vendor video pipeline is divided across three layers:

```
[GC1034 Sensor]
       │ (10-bit raw Bayer via DVP/MIPI)
       ▼
 [VI Adapter] ◄─── Managed by hal.ko (MIPI reset & clock setup)
       │
       ▼
  [DSP Coprocessor] ◄── Firmware loaded from /app/sensors/gk_fw.bin
       │ (Lens Shading, AE, AWB, 3D Noise Reduction)
       ▼
 [VENC Hardware] ◄── Managed by media.ko via /dev/gk_video
       │
       ▼
 [MMZ BSB Buffer Ring] (Annex-B H.264 NALUs)
```

1. **`hal.ko` (Hardware Abstraction Layer)**:
   - Communicates with an on-die DSP coprocessor.
   - At system initialization, uploads the proprietary DSP microcode blob (`gk_fw.bin`, ~180 KB) into DSP RAM.
   - Dispatches operational commands via mailbox rings (`HAL_DSP_StartCmdblk`, `HAL_DSP_WaitCmdqEmpty`).
2. **`media.ko` (Media Driver)**:
   - Exposes `/dev/gk_video` (major dynamic, character device).
   - Manages the BitStream Buffer (BSB) ring via `GK_ENC_IOC_GET_STREAM` (`0x80046537`).
   - Handles Video Input configuration (`GK_VI_IOC_ENABLE`, `0x80047670`).

---

## 3. Kernel API Breaking Changes (3.4 vs 5.x/6.x)

Porting the vendor BSP drivers or replacing them with native open-source drivers requires bridging significant Linux kernel subsystem evolutions:

| Subsystem | Linux 3.4.43 (Stock) | Modern Mainline (5.4 / 6.x) | Migration Strategy |
|:---|:---|:---|:---|
| **Board Setup** | Hardcoded C board files (`mach-goke`) | Device Tree (`.dts` / `.dtsi`) | Author `gk7102.dtsi` defining pinmux, clocks, and memory nodes |
| **Video DMA** | Proprietary MMZ physical mapping | Standard V4L2 `videobuf2-dma-contig` & DMA-BUF | Replace MMZ ioctls with standard V4L2 buffer queues |
| **Timers** | `init_timer()` / `setup_timer()` | `timer_setup()` / modern `hrtimer` | Update driver timer initializations |
| **GPIO** | Legacy integer GPIO numbers (`gpio_set_value`) | Modern GPIOD descriptor API (`gpiod_set_value`) | Port `escam_motor.c` to use `devm_gpiod_get()` |
| **I2C Bus** | Legacy board-info registration | Device Tree I2C nodes & standard V4L2 async subdevs | Use standard `v4l2_async_register_subdev()` |

---

## 4. Phased Execution Roadmap

```
Phase 1: Userspace Modernization [COMPLETE]
  └─ Build static musl BusyBox 1.36+ and deploy to rootfs.
Phase 2: Open-Source Peripheral Sourcing [COMPLETE]
  ├─ Sourced open-source Wi-Fi (rtl8188fu.ko).
  ├─ Sourced open-source GC1034 V4L2 sensor subdevice (gc1034_sensor.c).
  └─ Authored open-source stepper motor & GPIO driver (escam_motor.c).
Phase 3: Kernel Forward-Porting & OpenIPC Evaluation [ACTIVE]
  ├─ Adapt OpenIPC GK7102 kernel branch (Linux 4.9 LTS / 5.4 LTS).
  └─ Construct Device Tree bindings for Goke GK7102C SiP.
Phase 4: Mainline V4L2 Raw Bayer Capture
  └─ Pure V4L2 / CMA driver bypassing vendor DSP microcode for scientific astrophotography.
```
