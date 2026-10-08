---
id: '0030'
title: Comprehensive OpenAPI 3.1 Specification and Diataxis Documentation Suite
status: Accepted
created: 2026-10-06
persona: Marcus
target_bc: core
feature: FEAT-DOC-02
governing_prd: PRD-0008
governing_adrs:
  - ADR-0001
scenarios:
  - Provide interactive OpenAPI 3.1 documentation for all REST and WebSocket endpoints
  - Author Diataxis tutorial for astrophotography workflow
  - Validate documentation build with zero dead links
---

# US-0030 — Comprehensive OpenAPI 3.1 Specification and Diataxis Documentation Suite

## Governing PRD & ADR
- [`PRD-0008: Autonomous Scientific Imaging and Calibration Pipeline`](../../product/accepted/prd-0008-autonomous-scientific-imaging-and-vendor-elimination.md)
- [`ADR-0001: Specification as Code and Opinionated SDLC Guardrails`](../../adrs/accepted/adr-0001-specification-as-code-architecture.md)

## User Story
**As a** Developer / Astrophotographer,
**I want** complete OpenAPI specifications and Diataxis documentation for all camera interfaces,
**So that** third-party developers, Python scripts, and astronomy suites can integrate with the ESCAM G02 easily and reliably.

## Acceptance Criteria

```gherkin
Scenario: Provide interactive OpenAPI 3.1 documentation for all REST and WebSocket endpoints
  Given the running web server on port 8080
  When a client requests `GET /api/v1/openapi.json`
  Then a valid OpenAPI 3.1 JSON schema is returned describing all endpoints, request bodies, and responses.

Scenario: Author Diataxis tutorial for astrophotography workflow
  Given documentation structure under `docs/`
  When reviewing `docs/tutorials/getting-started-astrophotography.md`
  Then a complete step-by-step learning guide walks the user from unboxing to first calibrated stacked image.

Scenario: Validate documentation build with zero dead links
  Given all Diataxis files across `tutorials/`, `how-to/`, `reference/`, and `explanation/`
  When executing documentation verification
  Then all cross-references, links, and schemas resolve cleanly without warnings.
```
