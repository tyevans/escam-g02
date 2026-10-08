---
id: '0034'
title: GK7102 VPU/ISP Hardware Driver and Mainline Kernel Feasibility
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: driver
feature: FEAT-DRV-08
governing_prd: PRD-0009
governing_adrs:
  - ADR-0020
  - ADR-0026
scenarios:
  - Dissect and document GK7102 VPU and MMZ memory management structures
  - Analyze BSP driver forward-porting delta to modern Linux kernels
  - Publish architectural feasibility spike report with mainline migration path
---

# US-0034 — GK7102 VPU/ISP Hardware Driver and Mainline Kernel Feasibility

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0020: Zero-Vendor Hardware Video Encoding and VPU Ingestion Architecture`](../../adrs/accepted/adr-0020-zero-vendor-hardware-video-encoding-pipeline.md)
- [`ADR-0026: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap`](../../adrs/accepted/adr-0026-open-source-kernel-drivers-and-gk7102-peripheral-sourcing.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** to reverse-engineer and document the Goke GK7102C VPU/ISP driver internals and evaluate kernel forward-porting options,
**So that** the project has an empirical, risk-mitigated technical roadmap to completely migrate off the legacy 3.4 kernel onto a modern mainline Linux distribution.

## Acceptance Criteria

```gherkin
Scenario: Dissect and document GK7102 VPU and MMZ memory management structures
  Given the binary interface of `media.ko` and `/dev/gk_video` ioctl definitions
  When kernel memory maps and hardware registers for the VPU and MMZ (Media Memory Zone) are audited
  Then the physical contiguous DMA buffer allocation mechanism and ring buffer offsets are documented
  And the minimal initialization ioctl handshake is captured.

Scenario: Analyze BSP driver forward-porting delta to modern Linux kernels
  Given available Goke ADI SDK C sources and OpenIPC GK7102 kernel repositories
  When compared against Linux 4.9 LTS and Linux 5.4+ kernel driver subsystem changes
  Then breaking kernel API shifts (V4L2 videobuf2, DMA-BUF, timer APIs, locking primitives) are identified
  And the necessary code adaptation patches are enumerated.

Scenario: Publish architectural feasibility spike report with mainline migration path
  Given the results of memory analysis and kernel porting audits
  When the research synthesis is finalized
  Then an empirical spike report is committed under `docs/explanation/` or `docs/project/spikes/`
  With clear effort estimations, trade-off matrices, and a recommended phase-by-phase execution plan.
```
