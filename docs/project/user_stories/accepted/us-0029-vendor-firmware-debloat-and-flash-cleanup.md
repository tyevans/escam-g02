---
id: '0029'
title: Vendor Firmware Debloater and Clean-Boot Security Lockdown
status: Accepted
created: 2026-10-06
persona: Elena
target_bc: system
feature: FEAT-SYS-04
governing_prd: PRD-0008
governing_adrs:
  - ADR-0010
  - ADR-0019
scenarios:
  - Remove dead vendor scripts, unused web assets, and audio files from flash
  - Enforce clean startup boot script running only escamd
  - Verify zero listening ports except authorized web (8080) and INDI (7624)
---

# US-0029 — Vendor Firmware Debloater and Clean-Boot Security Lockdown

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0010: Zero-Trust Autonomous Worker Process Sandboxing`](../../adrs/accepted/adr-0010-zero-trust-worker-process-sandboxing.md)
- [`ADR-0019: Vendor Software Ejection, Clean Boot, and Init Orchestration`](../../adrs/accepted/adr-0019-vendor-software-ejection-and-clean-boot.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** to purge all unused legacy vendor binaries, cloud telemetry, Chinese voice audio prompts, and insecure daemons from flash storage,
**So that** the device runs as a clean, hardened open-source appliance with maximum free flash and zero attack surface.

## Acceptance Criteria

```gherkin
Scenario: Remove dead vendor scripts, unused web assets, and audio files from flash
  Given unused vendor assets in `/mnt/mtd/ipc` (onvif, ddns, web, audio .g711 prompts)
  When the debloater script runs
  Then unused files are removed, recovering over 1MB of storage space.

Scenario: Enforce clean startup boot script running only escamd
  Given system init sequence `/mnt/mtd/ipc/conf/run`
  When the camera boots from cold power cycle
  Then `escamd` starts automatically with watchdog supervision
  And no vendor daemons (`p2p_srv`, `net_detect`, `cloud`) are launched.

Scenario: Verify zero listening ports except authorized web (8080) and INDI (7624)
  Given a running system after boot
  When a port scan audits `0.0.0.0`
  Then only port 8080 (Web SPA / API) and port 7624 (INDI) are open externally
  And telnet (2323) is restricted or protected.
```
