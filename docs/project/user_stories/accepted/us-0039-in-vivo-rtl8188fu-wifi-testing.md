---
id: '0039'
title: In-Vivo Verification and Testing of Open-Source RTL8188FU Wi-Fi Driver
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: driver
feature: FEAT-DRV-12
governing_prd: PRD-0009
governing_adrs:
  - ADR-0026
scenarios:
  - Stage open-source 8188fu.ko to target camera
  - Verify module parameters and interface initialization
  - Test Wi-Fi association with wpa_supplicant
---

# US-0039 — In-Vivo Verification and Testing of Open-Source RTL8188FU Wi-Fi Driver

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0026: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap`](../../adrs/accepted/adr-0026-open-source-kernel-drivers-and-gk7102-peripheral-sourcing.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** to verify the open-source `8188fu.ko` on the camera hardware in vivo,
**So that** we confirm the open-source Wi-Fi driver successfully binds to the Realtek RTL8188FTV USB controller (`0bda:f179`) and connects via WPA2.

## Acceptance Criteria

```gherkin
Scenario: Stage open-source 8188fu.ko to target camera
  Given the compiled `dist/8188fu.ko`
  When transferred to camera RAM at `/mnt/mtd/ipc/tmpfs/8188fu.ko`
  Then file integrity is verified and vermagic matches the running kernel.

Scenario: Verify module parameters and interface initialization
  Given the staged driver
  When loaded via `insmod /mnt/mtd/ipc/tmpfs/8188fu.ko ifname=wlan0 if2name=wlan1`
  Then network interface `wlan0` appears in `/proc/net/dev` and `ifconfig -a`.

Scenario: Test Wi-Fi association with wpa_supplicant
  Given the initialized `wlan0` interface
  When `wpa_supplicant` is executed with the local network configuration
  Then connection is established and the camera acquires an IP address.
```
