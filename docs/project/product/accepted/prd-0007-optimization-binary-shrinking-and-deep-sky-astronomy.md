---
id: '0007'
title: Performance Optimization, Binary Footprint Shrinking, and Deep-Sky Astrophotography
status: Accepted
author: Lead Architect
created: 2026-10-06
target_bc: core
---

# PRD-0007: Performance Optimization, Binary Footprint Shrinking, and Deep-Sky Astrophotography

## Executive Summary
Having successfully neutralized the vendor stack, jailed legacy services strictly to localhost, and deployed the pure Rust `escamd` daemon on the ESCAM G02 (Goke GK7102C, 64MB RAM), this milestone drives a comprehensive optimization pass. We focus on three core pillars:
1. **Extreme Footprint Shrinking**: Reducing binary size and RAM footprint through compiler optimization and zero-allocation ring buffers.
2. **Deep-Sky Astrophotography Capabilities**: Adding celestial sidereal tracking, in-memory live frame stacking, and dark-frame subtraction via INDI.
3. **Low-Latency Adaptive Video Streaming**: Enhancing the WebCodecs pipeline with dynamic jitter estimation and zero-copy NALU ingestion.

## Target Personas
- **Dr. Marcus Vance (Astrophotographer)**: Needs equatorial sidereal tracking, long-exposure frame integration, and clean FITS export to image nebulae and planets.
- **Elena (Embedded Systems Specialist)**: Demands deterministic memory bounds (<8MB RSS), low CPU usage (<15% idle), and ultra-compact flash usage.

## Success Criteria & Key Performance Indicators (KPIs)
- **Binary Footprint**: Uncompressed `escamd` binary $< 800$ KB (compressed $< 400$ KB).
- **RAM Overhead**: Pure Rust runtime memory strictly $< 8.0$ MB RSS, maintaining $> 52$ MB free RAM.
- **PTZ Tracking**: Sidereal drift rate sustained with $< 0.1$ pixel jitter over 60-second exposures.
- **Live Stacking**: Up to 32 Bayer frames integrated in-memory without exceeding 12MB RAM.
- **Glass-to-Glass Latency**: Sub-60ms average over local 2.4GHz 802.11n Wi-Fi.

## Scope & Technical Boundaries
- Target architecture: ARMv6 (`arm-unknown-linux-musleabi`, ARM1176JZF-S @ 600MHz).
- Strictly enforce file length invariant (<500 lines, target <400 lines) across all crates.
- Maintain 100% test pass rate through public frontdoors without mock backdoors.
