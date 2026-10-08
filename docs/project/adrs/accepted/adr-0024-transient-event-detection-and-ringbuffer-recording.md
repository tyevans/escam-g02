---
id: '0024'
title: Transient Event Detection, Meteor Streak Analyzer, and Zero-Copy Ringbuffer Video Recording
status: Accepted
date: 2026-10-06
deciders: Lead Architect, Alex Rivera, Elena
governing_prds:
  - PRD-0008
---

# ADR-0024: Transient Event Detection, Meteor Streak Analyzer, and Zero-Copy Ringbuffer Video Recording

## Context & Problem Statement
Astronomical observation frequently catches fleeting phenomena: meteors, fireballs, satellite passes (ISS, Starlink), and occultations. Simultaneously, surveillance use cases require reliable event-triggered video recording without burning continuous flash writes to SPI storage.

## Decision Drivers
- **Zero-Write Standby**: Video frames must continuously cycle through a bounded RAM ring buffer without touching physical flash storage until a genuine trigger occurs.
- **Pre-Roll Capture**: The recorded clip must contain $N$ seconds of footage *prior* to the trigger point so the start of the event is preserved.
- **High-Velocity Linear Streak Discriminator**: Meteors and satellites form linear streaks across consecutive frames, distinguishable from random noise and scintillation.

## Decision Outcome
We implement `TransientStreakDetector` in `escam-astro` and `RollingRingRecorder` in `escam-media`:
1. **Streak Analysis**: Difference imaging between frame $t$ and frame $t-1$, followed by bounding box aspect-ratio filter ($\text{length}/\text{width} > 3.0$) and trajectory consistency checks across 3 frames.
2. **Circular Pre-Roll Buffer**: Fixed in-memory ring buffer (e.g. 5–10 seconds of H.264 I/P frames, ~2MB).
3. **Clip Packaging**: Upon event detection or manual HTTP trigger `/api/v1/recorder/trigger`, the ring buffer dumps pre-roll frames plus post-event frames to `/tmp/recordings/*.mp4` (or raw NALU sequence).
