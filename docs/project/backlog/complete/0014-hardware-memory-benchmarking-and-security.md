---
id: '0014'
title: Hardware Memory Benchmarking, Latency Profiling, and Security Preflight
status: Complete
governing_adrs:
- ADR-0019
- ADR-0012
governing_prds:
- PRD-0006
governing_stories:
- US-0013
target_bc: system
persona: Elena
mutation_scope:
- crates/escam-system/src/telemetry.rs
---

# TASK-0014: Hardware Memory Benchmarking, Latency Profiling, and Security Preflight

## Problem Statement & Context
Verify that the entire `escamd` camera system meets our performance invariants: runtime RAM consumption strictly under 8MB RSS, cold-boot to live stream under 2.0 seconds, and 0 bytes of outbound WAN telemetry.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement telemetry reporter tracking Resident Set Size (RSS), heap allocations, uptime, and CPU usage.
2. Validate that runtime RSS stays below 8,192 kB during simultaneous WebRTC streaming and PTZ movement.
3. Preflight secret scanning passes with 0 secrets or hardcoded credentials.
4. All new files strictly under 500 lines.
