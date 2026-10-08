---
id: '0008'
title: Autonomous Scientific Imaging, Calibration Pipeline, and Pure Open-Source Firmware
status: Accepted
author: Lead Architect
created: 2026-10-06
target_bc: core
---

# PRD-0008: Autonomous Scientific Imaging, Calibration Pipeline, and Pure Open-Source Firmware

## Executive Summary
Building on the successful deployment of the pure Rust `escamd` daemon and real-time linear photon accumulation engine on the ESCAM G02 (Goke GK7102C SoC), this product requirements document specifies the next generation of capabilities. We transition this $15 commodity pan-tilt camera into an autonomous, scientific-grade robotic observatory and precision optical sensor.

The key focus areas are:
1. **Scientific Calibration Engine**: Real-time Master Dark subtraction, Bias correction, and Flat-field normalization for raw 16-bit FITS imagery.
2. **Precision Astrometric Pipeline**: WCS celestial coordinates, IAU-standard FITS metadata headers, and sub-millisecond SNTP exposure midpoint stamping.
3. **Computer Vision & Optical Analysis**: Autonomous star detection, FWHM seeing/focus metric calculation, and meteor/satellite transient streak detection.
4. **Autonomous Motion & Guiding**: Mechanical gear backlash compensation, closed-loop optical autoguiding, and celestial catalog slew integration.
5. **On-Camera Event Recording**: Zero-copy ring-buffered event recorder with pre-roll buffering.
6. **Flash Debloating & Open Documentation**: Elimination of remaining unneeded vendor blobs, clean-boot hardening, and a Diataxis documentation suite with full OpenAPI specs.

## Target Personas
- **Dr. Marcus Vance (Astrophotographer & Researcher)**: Requires calibrated 16-bit FITS files with accurate WCS coordinates, star FWHM focus scores, and autoguiding to capture long-exposure deep-sky imagery.
- **Elena (Embedded Systems Specialist)**: Demands zero proprietary vendor dependencies, rock-solid sub-8MB RAM budgets, deterministic hardware control, and pure-Rust clean boot.
- **Alex Rivera (Night Sky Observer & Citizen Scientist)**: Wants an autonomous all-sky meteor/fireball tracker that alerts on transient streaks and automatically captures video clips.

## Success Criteria & KPIs
- **Calibration Precision**: Master Dark subtraction removes >90% of thermal hot pixels without clipping faint background signals.
- **Astrometric Accuracy**: FITS headers contain valid RA/DEC, ALT/AZ, SITELAT/SITELONG, and DATE-AVG tags parseable by standard astronomical tools (`astropy`, `Siril`, `PixInsight`).
- **Star & Seeing Metric**: Real-time centroiding and FWHM calculated within <5ms per frame on ARMv6.
- **Backlash Compensation**: Gear reversal backlash hysteresis reduced by >85% during micro-slews.
- **Resource Footprint**: `escamd` RSS memory stays strictly <10MB during active calibration and stacking.
- **Flash Utilization**: Free flash space on `/mnt/mtd/ipc/conf` maintained >500KB.
- **File Length & Code Quality**: 100% of source files strictly <500 lines (target <400 lines) with 0 `spec-ops health` warnings.
