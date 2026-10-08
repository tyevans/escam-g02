---
id: '0023'
title: Zero-Copy Rolling Circular Pre-Roll Buffer and Event Video Recorder
status: Accepted
created: 2026-10-06
persona: Elena
target_bc: media
feature: FEAT-MEDIA-04
governing_prd: PRD-0008
governing_adrs:
  - ADR-0024
scenarios:
  - Continuously ingest H.264 NALUs into bounded in-memory circular ring buffer
  - Trigger clip export containing pre-roll and post-roll video footage
  - Serve recorded clips via HTTP REST endpoint
---

# US-0023 — Zero-Copy Rolling Circular Pre-Roll Buffer and Event Video Recorder

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0024: Transient Event Detection and Zero-Copy Ringbuffer Video Recording`](../../adrs/accepted/adr-0024-transient-event-detection-and-ringbuffer-recording.md)

## User Story
**As an** Elena (Embedded Systems Specialist),
**I want** video frames to cycle continuously through a bounded in-memory circular buffer without touching flash storage until an event triggers recording,
**So that** hardware flash wear is prevented while ensuring critical events retain 5 seconds of pre-trigger context.

## Acceptance Criteria

```gherkin
Scenario: Continuously ingest H.264 NALUs into bounded in-memory circular ring buffer
  Given an incoming 25 FPS H.264 video stream
  When frames are pushed into a 5-second circular ring buffer
  Then older frames beyond 5 seconds are evicted in O(1) time without heap re-allocations
  And memory footprint remains strictly bounded under 2.5 MB.

Scenario: Trigger clip export containing pre-roll and post-roll video footage
  Given an active ring buffer with 5 seconds of pre-roll footage
  When an event trigger occurs (motion, meteor, or REST POST /api/v1/recorder/trigger)
  Then the recorder writes the pre-roll NALUs starting from an IDR keyframe followed by post-roll NALUs
  And generates an event clip file in `/tmp/recordings/`.

Scenario: Serve recorded clips via HTTP REST endpoint
  Given stored clips in `/tmp/recordings/`
  When a client requests `GET /api/v1/recordings`
  Then the endpoint returns a JSON listing with filename, size, timestamp, and duration.
```
