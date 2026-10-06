---
id: '0013'
title: Measure and verify boot time <2s and RAM consumption <8MB
status: Accepted
created: 2026-10-05
persona: Elena
target_bc: system
feature: FEAT-SYS-02
governing_prd: PRD-0006
scenarios:
  - Measure elapsed time from power-on to live stream readiness
  - Verify total daemon memory footprint remains below 8MB RSS
  - Verify zero unauthorized outbound WAN connections
---

# US-0013 — Measure and verify boot time <2s and RAM consumption <8MB

## Governing PRD
- [`PRD-0006: Vendor Software Ejection, Clean Boot, and Production Static Firmware`](../../product/accepted/prd-0006-vendor-software-ejection-and-clean-boot.md)

## User Story

**As an** Elena (Embedded Systems Specialist),
**I want** to verify that `escamd` runs in <8MB of RAM, boots in <2.0 seconds, and transmits zero WAN telemetry,
**So that** the system performs reliably on resource-constrained 64MB hardware without security leaks.

## Acceptance Criteria

```gherkin
Scenario: Measure elapsed time from power-on to live stream readiness
  Given the camera powered on from cold state
  When the Linux kernel boots and launches escamd
  Then the time from userland launch to WebRTC service ready is less than 2.0 seconds.

Scenario: Verify total daemon memory footprint remains below 8MB RSS
  Given escamd actively streaming WebRTC video and accepting PTZ commands
  When process memory is queried via /proc/<pid>/status
  Then the Resident Set Size (VmRSS) is strictly below 8,192 kB (8 MB).

Scenario: Verify zero unauthorized outbound WAN connections
  Given the camera connected to a monitored local network
  When a packet capture inspects all egress traffic
  Then zero outbound DNS queries, NTP connections, or cloud P2P packets are observed
  And all communication is restricted strictly to local subnet clients.
```
