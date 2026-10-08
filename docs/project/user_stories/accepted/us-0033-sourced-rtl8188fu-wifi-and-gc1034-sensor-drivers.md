---
id: '0033'
title: Sourced Realtek RTL8188FTV Wi-Fi and GC1034 Sensor Drivers
status: Accepted
created: 2026-10-07
persona: Elena
target_bc: driver
feature: FEAT-DRV-07
governing_prd: PRD-0009
governing_adrs:
  - ADR-0026
scenarios:
  - Source and compile open-source rtl8188fu USB Wi-Fi driver
  - Verify WPA2-PSK client association using open-source Wi-Fi driver
  - Adapt open-source GC1034 image sensor driver under V4L2 subdevice architecture
---

# US-0033 — Sourced Realtek RTL8188FTV Wi-Fi and GC1034 Sensor Drivers

## Governing PRD & ADR
- [`PRD-0009: Open-Source Kernel Drivers, Mainline Linux Transition, and Modern Userspace`](../../product/accepted/prd-0009-open-source-kernel-drivers-and-modern-userspace.md)
- [`ADR-0026: Open-Source Kernel Drivers and GK7102 Peripheral Sourcing Roadmap`](../../adrs/accepted/adr-0026-open-source-kernel-drivers-and-gk7102-peripheral-sourcing.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** to compile the community-maintained open-source `rtl8188fu` Wi-Fi driver and adapt an open-source GalaxyCore GC1034 sensor driver,
**So that** camera network connectivity and sensor register sequencing rely entirely on public, inspectable, and rebuildable source code.

## Acceptance Criteria

```gherkin
Scenario: Source and compile open-source rtl8188fu USB Wi-Fi driver
  Given the community open-source `rtl8188fu` Git repository
  When cross-compiled against target Linux kernel headers
  Then the module compiles with zero proprietary blob links
  And loads cleanly when the internal USB Wi-Fi dongle (`0bda:f179`) is enumerated.

Scenario: Verify WPA2-PSK client association using open-source Wi-Fi driver
  Given a running system with the open-source `rtl8188fu` driver loaded
  When `wpa_supplicant` initiates WPA2-PSK authentication to a local access point
  Then the `wlan0` interface associates and receives a valid DHCP lease
  And sustains stable continuous TCP/UDP streaming without kernel panic.

Scenario: Adapt open-source GC1034 image sensor driver under V4L2 subdevice architecture
  Given open-source GC1034 I2C driver sources from Linux kernel and OpenIPC trees
  When configured for 1280x720 10-bit raw Bayer output over DVP
  Then sensor registers for exposure duration, analog gain, and clocking are initialized successfully
  And produce valid pixel data to the video input hardware block.
```
