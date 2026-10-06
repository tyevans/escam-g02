---
id: '0009'
title: Serve INDI CCD protocol over TCP port 7624 for Ekos and NINA
status: Accepted
created: 2026-10-05
persona: Marcus
target_bc: astro
feature: FEAT-ASTRO-02
governing_prd: PRD-0004
scenarios:
  - Respond to INDI getProperties query with CCD device descriptors
  - Execute exposure timer and stream BLOB image data
  - Remotely toggle IR-cut solenoid via INDI switch property
---

# US-0009 — Serve INDI CCD protocol over TCP port 7624 for Ekos and NINA

## Governing PRD
- [`PRD-0004: Astrophotography Subsystem, Raw Bayer Capture, and INDI Protocol`](../../product/accepted/prd-0004-astrophotography-subsystem-and-indi-protocol.md)

## User Story

**As a** Marcus (Astrophotographer & Optical Hacker),
**I want** the camera to run an embedded INDI server on TCP port 7624,
**So that** astronomy control suites (KStars/Ekos, PHD2 Guiding, NINA) can control exposures and autoguiding directly over Wi-Fi.

## Acceptance Criteria

```gherkin
Scenario: Respond to INDI getProperties query with CCD device descriptors
  Given an INDI client connecting to TCP port 7624
  When the client sends an XML getProperties message
  Then the server returns defTextVector, defNumberVector, and defSwitchVector properties for "ESCAM G02 CCD"
  And declares support for CCD_EXPOSURE, CCD_FRAME, and FILTER_SLOT properties.

Scenario: Execute exposure timer and stream BLOB image data
  Given a connected INDI client with BLOB transfers enabled
  When the client sends a newNumberVector for CCD_EXPOSURE with value 1.0 second
  Then the camera captures an exposure for exactly 1.0 second
  And transmits a setBLOBVector containing the uncompressed FITS frame back to the client.

Scenario: Remotely toggle IR-cut solenoid via INDI switch property
  Given an INDI client connected to the camera
  When the client updates the IR_CUT_FILTER switch property to "OFF (Night/Astro)"
  Then the camera actuates GPIO 17 to retract the IR-cut filter
  And confirms the updated switch state to the client.
```
