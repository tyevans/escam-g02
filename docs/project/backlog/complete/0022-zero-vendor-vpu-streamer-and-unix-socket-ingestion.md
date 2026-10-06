---
id: '0022'
title: Zero-Vendor Hardware VPU Streamer and Unix Socket Ingestion
status: Complete
governing_adrs:
- ADR-0010
- ADR-0019
- ADR-0020
governing_prds:
- PRD-0006
governing_stories:
- US-0014
target_bc: media
mutation_scope:
- crates/escam-media/src/vpu_stream.rs
---

# TASK-0022: Zero-Vendor Hardware VPU Streamer and Unix Socket Ingestion

## Problem Statement & Context
Currently, `ipc_server` is the sole remaining vendor binary running on the ESCAM G02 camera. It consumes >30MB virtual memory, spawns 40+ pthreads, and runs an internal RTSP server on port 554. Per ADR-0020, we must replace `ipc_server` with a zero-copy Unix domain socket pipeline (`/tmp/venc.sock`) and a lightweight headless VPU micro-streamer (`gk-vpu`), allowing `ipc_server` to be eradicated permanently.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `VpuUnixStreamReader` in `crates/escam-media/src/vpu_stream.rs` to ingest framed H.264 Annex-B NALUs from a Unix domain socket asynchronously.
2. Wire `escamd` to connect to `/tmp/venc.sock` with fallback to loopback RTSP if socket is absent.
3. Build minimal headless VPU micro-streamer (`gk-vpu`) initializing the GalaxyCore GC1034 sensor and H.264 720p hardware encoder, streaming to `/tmp/venc.sock`.
4. Deploy to ESCAM G02, kill `ipc_server`, and verify live WebSockets/WebCodecs video streaming with `ipc_server` completely stopped.
5. All files strictly under 500 lines (proactive check: <400 lines).
