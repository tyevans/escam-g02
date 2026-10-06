---
id: '0020'
title: Zero-Vendor Hardware Video Encoding and VPU Ingestion Architecture
status: Accepted
date: 2026-10-05
deciders:
  - Elena
  - Samir
  - Alex
---

# ADR-0020: Zero-Vendor Hardware Video Encoding and VPU Ingestion Architecture

## Status
Accepted

## Context
With the completion of TASK-0021 (Vendor Software Ejection), all non-essential vendor daemons (`onvif`, `net_detect`, `platform.sh`, `sd.sh`, `chksock`, `proxy`, `loadAround`, `watchdog`) have been permanently disabled from boot. Only a single vendor binary remains: `ipc_server`.

Currently, `ipc_server` runs solely to provide a local loopback RTSP feed (`rtsp://127.0.0.1:554/live/ch0`) which `escamd` ingests and forwards to clients over WebSockets and WebCodecs. However, `ipc_server` is heavy: it spawns over 40 threads, consumes >30 MB of virtual memory, opens multiple unused sockets, and contains legacy security vulnerabilities.

### Hardware Constraints
1. **Processor**: The Goke GK7102C is an ARMv6 (ARM1176JZF-S) clocked at 600 MHz without NEON SIMD extensions.
2. **Computational Feasibility**: Software video encoding (e.g. `x264`, `libvpx`, or software JPEG) is computationally impossible at 720p (>100% CPU yields <1 FPS).
3. **Hardware VPU & Memory Architecture**:
   - The SoC incorporates a dedicated H.264 hardware Video Processing Unit (VPU).
   - Raw video frames from the GalaxyCore GC1034 sensor travel through the VI (Video Input) hardware block and ISP (Image Signal Processor) into physically contiguous MMZ (Media Memory Zone) memory.
   - The hardware encoder writes compressed H.264 Annex-B NALUs into a kernel BitStream Buffer (BSB) managed by `media.ko` via `/dev/gk_video` (major 248, minor 0).
   - Empirical reverse engineering revealed that reading frames directly via `ioctl(0x80046537)` (`GK_ENC_IOC_GET_STREAM`) blocks on a kernel wait queue (`wait_event_interruptible`) unless the VPU channel and sensor ISP pipeline are actively initialized and started on that specific file descriptor session.

## Considered Options
1. **Option 1: Complete From-Scratch Pure-Rust ISP & VPU Register Driver**
   - Reverse-engineer and implement every register write across I2C (`/dev/i2c-0`), GC1034 sensor registers, ISP 3A algorithms (Auto Exposure, Auto White Balance), and VENC registers in pure Rust.
   - *Verdict*: Rejected. The ISP alone contains thousands of undocumented tuning registers and matrix tables. Attempting to author a 3A control loop without vendor documentation risks uncalibrated, overexposed, or unstable imaging.
2. **Option 2: OpenIPC / Majestic Third-Party Streamer**
   - Flash or execute OpenIPC's `majestic` binary to handle video capture.
   - *Verdict*: Rejected. Replaces one third-party closed/hybrid daemon with another, introduces foreign YAML configuration layers, and prevents achieving a unified, zero-dependency, pure-firmware architecture.
3. **Option 3: Two-Phase Native Hardware VPU Ingestion (Selected)**
   - **Phase 1 (Immediate Ejection)**: Extract the minimal hardware initialization and stream pumping routines from the Goke ADI SDK into a tiny standalone micro-daemon (`gk-vpu`) (<40 KB binary, <1 MB RAM, single event loop). It maps the MMZ BSB buffer via `ioctl(0x80046d00)` and pushes raw H.264 NALUs directly across a local Unix domain socket (`/tmp/venc.sock`). `ipc_server` is terminated and purged permanently.
   - **Phase 2 (In-Process Integration)**: Integrate the low-level VPU ioctl bindings and MMZ buffer reader directly into `crates/escam-driver/src/vpu.rs` via native Rust FFI bindings, embedding the hardware pump directly inside `escamd`.

## Decision
We adopt **Option 3: Two-Phase Native Hardware VPU Ingestion**.

### Architectural Design
1. **Eradication of `ipc_server`**:
   - `ipc_server` will be permanently killed (`killall -9 ipc_server`) and removed from the boot script.
   - Port 554 (RTSP) and port 80 (legacy HTTP snapshot) will be closed permanently.
2. **Minimal VPU Headless Ingestion Interface**:
   - The VPU streamer will configure the GC1034 sensor (1280x720 @ 25 FPS, GOP 50, CBR 1500 kbps), load `/etc/sensors/gc1034_hw.bin`, initialize the ISP, and stream raw Annex-B NALUs (`00 00 00 01`) directly to a Unix domain socket (`/tmp/venc.sock`).
3. **Zero-Copy Ingestion in `escamd`**:
   - `crates/escam-media` will replace its RTSP client pipeline with an asynchronous Unix domain socket reader (`tokio::net::UnixStream::connect("/tmp/venc.sock")`).
   - Frame parsing overhead drops to zero: NALUs are read directly from local memory without TCP/IP stack traversal, RTP depacketization, or RTSP session renegotiation.
4. **Snapshot Capture**:
   - Snapshots will be extracted by capturing the immediate next IDR/I-frame NALU or issuing an in-band JPEG snapshot request to the VPU channel.

## Consequences

### Positive
- **100% Vendor Daemon Ejection**: Eliminates the very last vendor binary (`ipc_server`), achieving complete ownership of userland processes.
- **Resource Reclaim**: System RAM consumption for video ingestion drops from ~35 MB to <1.5 MB, and thread count drops from 42 to 1.
- **Ultra-Low Latency**: Bypassing loopback RTSP/RTP networking reduces glass-to-glass latency by an additional 15-25 ms (achieving true sub-40 ms streaming).
- **Security Hardening**: Closes all remaining legacy network ports (RTSP 554, vendor HTTP 80); the only listening ports on the device become `escamd` (8080) and debug telnet (2323).

### Negative
- Requires maintaining the minimal GC1034 sensor initialization sequence and ISP calibration loader.
