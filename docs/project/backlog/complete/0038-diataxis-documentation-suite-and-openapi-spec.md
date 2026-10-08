---
id: '0038'
title: Comprehensive OpenAPI 3.1 Specification and Diataxis Documentation Suite
status: Complete
governing_adrs:
  - ADR-0001
governing_prds:
  - PRD-0008
governing_stories:
  - US-0030
target_bc: core
mutation_scope:
  - crates/escam-web/src/openapi.rs
---

# TASK-0038: Comprehensive OpenAPI 3.1 Specification and Diataxis Documentation Suite

## Problem Statement & Context
An open-source scientific instrument requires world-class developer documentation. Users and astronomers need complete OpenAPI 3.1 specifications for all REST endpoints, along with Diataxis-compliant guides across Tutorials, How-To guides, Reference specs, and Explanations.

## Definition of Done (Blackbox Frontdoor TDD)
1. Implement `OpenApiRegistry` in `crates/escam-web/src/openapi.rs`:
   - Serve dynamic OpenAPI 3.1 JSON at `/api/v1/openapi.json`.
   - Document all REST endpoints (PTZ, Camera, Sensor, Astro, Calibration, Recording, Clock, Telemetry).
2. Author complete Diataxis documentation:
   - `docs/tutorials/getting-started-astrophotography.md`
   - `docs/how-to/calibrate-sensor-and-darks.md`
   - `docs/how-to/setup-kstars-ekos-indi.md`
   - `docs/reference/rest-api.md`
   - `docs/reference/fits-metadata.md`
   - `docs/explanation/gk7102-vpu-architecture.md`
3. Frontdoor tests verifying valid OpenAPI JSON schema and documentation link integrity.
4. All source files strictly <500 lines (target <400 lines).
