---
id: '0010'
title: Embed Axum HTTP server and serve compressed static assets
status: Accepted
created: 2026-10-05
persona: Samir
target_bc: web
feature: FEAT-WEB-01
governing_prd: PRD-0005
scenarios:
  - Serve embedded SPA HTML/CSS/JS with pre-compression
  - Route REST API requests for camera telemetry and control
  - Handle static asset 304 Not Modified caching headers
---

# US-0010 — Embed Axum HTTP server and serve compressed static assets

## Governing PRD
- [`PRD-0005: Embedded Axum Web Server and Snappy Modern Single-Page App (SPA)`](../../product/accepted/prd-0005-embedded-axum-web-server-and-snappy-spa.md)

## User Story

**As a** Samir (Privacy & Smart Home Operator),
**I want** an embedded Axum HTTP server serving compressed web assets directly from flash memory,
**So that** the camera UI loads in under 50ms without external CDN dependencies.

## Acceptance Criteria

```gherkin
Scenario: Serve embedded SPA HTML/CSS/JS with pre-compression
  Given a browser request to the root URL "/"
  When the Axum web server responds
  Then the response serves index.html with Content-Encoding gzip
  And total transferred bytes for the entire web bundle is under 150KB.

Scenario: Route REST API requests for camera telemetry and control
  Given an authenticated HTTP client
  When a GET request is made to /api/v1/status
  Then the server returns JSON status containing CPU load, free RAM, uptime, and PTZ angles
  And the response time is under 10ms.

Scenario: Handle static asset 304 Not Modified caching headers
  Given a browser sending an If-None-Match ETag header for a cached asset
  When the server evaluates the request
  Then it responds with HTTP 304 Not Modified with zero body bytes transferred.
```
