---
id: '0031'
title: Zero-Copy Circular Pre-Roll Buffer and Event Clip Recorder
status: Complete
governing_adrs:
  - ADR-0024
governing_prds:
  - PRD-0008
governing_stories:
  - US-0023
target_bc: media
mutation_scope:
  - crates/escam-media/src/recorder.rs
---

# TASK-0031: Zero-Copy Circular Pre-Roll Buffer and Event Clip Recorder

## Problem Statement & Context
Flash storage on embedded cameras degrades rapidly under continuous write cycles. To capture security motion and astronomical transients safely, video frames must cycle through an in-memory ring buffer (pre-roll) and only flush to disk upon an event trigger.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `EventClipRecorder` in `crates/escam-media/src/recorder.rs`:
   - Bounded ring buffer holding $N$ seconds (configurable, default 5s) of H.264 NALUs.
   - Keyframe alignment (ensures recorded clips always start with an SPS/PPS/IDR frame).
   - Trigger mechanism (`trigger_event(label, duration_secs)`) that writes pre-roll + post-roll footage to `/tmp/recordings/clip_<timestamp>.h264`.
2. Expose REST endpoints:
   - `POST /api/v1/recorder/trigger`
   - `GET /api/v1/recorder/clips`
   - `GET /api/v1/recorder/clips/:filename`
   - `DELETE /api/v1/recorder/clips/:filename`
3. Frontdoor tests verifying ring-buffer eviction without memory leaks and proper IDR keyframe alignment.
4. All source files strictly <500 lines (target <400 lines).
