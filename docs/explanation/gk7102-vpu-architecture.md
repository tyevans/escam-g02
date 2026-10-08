# Explanation: Goke GK7102C Video Processing Unit (VPU) Architecture

The **Goke GK7102C** is a cost-optimized System-on-Chip (SoC) combining an ARM1176JZF-S (ARMv6) core running at 600 MHz with a dedicated hardware Video Processing Unit (VPU), an Image Signal Processor (ISP), and an H.264 video encoder (VENC).

---

## Hardware Pipeline Flow

```
[GC1034 CMOS Sensor] 
       │ (10-bit Raw Bayer via DVP/MIPI)
       ▼
 [VI Subsystem] (/dev/gk_video ioctl 0x80047670)
       │
       ▼
 [ISP Subsystem] (Demosaicing, Bad Pixel Correction, 3D Denoise)
       │
       ▼
[VENC Subsystem] (H.264 Baseline/Main Profile Encoder)
       │
       ├──► Frame Buffer Ring (Annex-B NALUs: SPS, PPS, IDR, P-frames)
       │           │
       │           └──► [escamd: escam-media] (Zero-copy pre-roll ring buffer)
       │
       └──► [VPU Unix Domain Stream / RTSP Loopback]
```

---

## Memory Architecture & Hardware Constraints

1. **Integrated DRAM**: The GK7102C features 64 MB of integrated DDR2 memory (co-packaged inside the SoC).
2. **Memory Map Split**:
   - `0x0000_0000 - 0x0180_0000` (24 MB): Reserved for hardware VPU MMZ (Media Memory Zone) frame buffers and encoder references.
   - `0x0180_0000 - 0x0400_0000` (40 MB): Linux kernel and user space.
3. **Firmware Daemon Budget**:
   - `escamd` is strictly engineered to run within $< 10.0$ MB of Resident Set Size (RSS).
   - High-throughput video streaming utilizes zero-allocation ring buffers to avoid heap fragmentation and unpredictable garbage collection stalls on single-threaded ARM11 cores.

---

## Kernel Drivers & Character Devices

- `/dev/gk_video`: Goke video device node managing VI, ISP, and VENC configuration via ioctl calls with magic `'v'` (`0x76`).
- `/dev/motor`: 2-axis stepper motor driver driven by `motor.ko`, actuating Pan and Tilt step phases.
- `/dev/gkio`: General-purpose I/O device driven by `gkio.ko`, actuating the bi-stable magnetic IR-cut filter and 850nm illuminators.
