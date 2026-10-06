---
id: '0016'
title: Track Celestial Bodies via Precise Sidereal Stepper Motor Micro-Pulsing
status: Accepted
created: 2026-10-06
persona: Marcus
target_bc: ptz
feature: FEAT-PTZ-04
governing_prd: PRD-0007
governing_adrs:
  - ADR-0021
scenarios:
  - Calculate discrete pulse intervals for sidereal tracking
  - Execute background sidereal tracking loop without drifting
---

# US-0016 — Track Celestial Bodies via Precise Sidereal Stepper Motor Micro-Pulsing

## Governing PRD & ADR
- [`PRD-0007: Performance Optimization, Binary Footprint Shrinking, and Deep-Sky Astrophotography`](../../product/accepted/prd-0007-optimization-binary-shrinking-and-deep-sky-astronomy.md)
- [`ADR-0021: Binary Footprint Reduction, Zero-Allocation Media Pipelines, and Sidereal Tracking`](../../adrs/accepted/adr-0021-binary-footprint-reduction-and-zero-allocation-pipeline.md)

## User Story
**As a** Dr. Marcus Vance (Astrophotographer),
**I want** `escam-ptz` to support a continuous sidereal tracking mode that steps the Pan motor at the Earth's celestial rotation rate,
**So that** stars and planets remain centered in the field of view during long-exposure astrophotography.

## Acceptance Criteria

```gherkin
Scenario: Calculate discrete pulse intervals for sidereal tracking
  Given the gear ratio and step count of the Pan motor (520 total steps per 355 degrees)
  When the SiderealTracker is initialized with celestial rate (15.041067 arcsec/sec)
  Then it calculates the exact millisecond delay between step pulses.

Scenario: Execute background sidereal tracking loop without drifting
  Given SiderealTracker enabled
  When tracking runs for 60 seconds
  Then it issues the mathematically exact step pulses to maintain equatorial alignment.
```
