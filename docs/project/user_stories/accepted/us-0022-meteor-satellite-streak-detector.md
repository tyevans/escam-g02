---
id: '0022'
title: Transient Streak Detector for Meteors, Fireballs, and Satellites
status: Accepted
created: 2026-10-06
persona: Alex
target_bc: astro
feature: FEAT-ASTRO-08
governing_prd: PRD-0008
governing_adrs:
  - ADR-0024
scenarios:
  - Identify linear motion streak across sequential video frames
  - Reject stationary stars and diffuse scintillation noise
  - Emit transient event notification with trajectory coordinates
---

# US-0022 — Transient Streak Detector for Meteors, Fireballs, and Satellites

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0024: Transient Event Detection and Zero-Copy Ringbuffer Video Recording`](../../adrs/accepted/adr-0024-transient-event-detection-and-ringbuffer-recording.md)

## User Story
**As an** Alex Rivera (Night Sky Observer),
**I want** the camera to autonomously detect high-speed linear streaks across frames,
**So that** meteors, fireball transits, and satellite passes are detected and logged automatically without requiring hours of manual review.

## Acceptance Criteria

```gherkin
Scenario: Identify linear motion streak across sequential video frames
  Given a sequence of frames containing a moving linear streak with aspect ratio > 3.0
  When the transient detector runs temporal differencing and line fitting
  Then a transient event is flagged with start (x1, y1) and end (x2, y2) coordinates.

Scenario: Reject stationary stars and diffuse scintillation noise
  Given a background field with scintillating stationary stars
  When the streak detector analyzes consecutive frames
  Then stationary point sources are subtracted and produce zero false streak alarms.

Scenario: Emit transient event notification with trajectory coordinates
  Given an identified meteor streak event
  When the event is registered
  Then the event payload contains timestamp, duration, angular velocity, and bounding box.
```
