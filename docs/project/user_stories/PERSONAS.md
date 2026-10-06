# User Personas
# Project: escam-g02 (Open-Source Rust Firmware & PTZ Camera Platform)

Archetypes representing the core users, operators, and architects interacting with the ESCAM G02 platform.

---

## 1. Elena — The Embedded Systems & Reverse Engineering Specialist
- **Archetype**: Embedded Systems Engineer & Hardware Hacker.
- **Background**: Experienced in Linux device drivers, kernel module reverse engineering, and low-level systems programming in Rust.
- **Pain Points**:
  - Closed-source vendor binaries (`ipc_server`, `chksock`, `net_detect`, `media.ko`) have unknown vulnerabilities, opaque memory leaks, and excessive cloud phone-home traffic.
  - Proprietary stepper motor drivers click loudly, cause acoustic noise, and lack smooth trajectory planning.
  - Cross-compilation workflows for obsolete ARMv6 (ARM1176JZF-S) cores with static musl libc are cumbersome without disciplined automation.
- **Goals with this platform**:
  - Replace closed-source vendor userland with a single, memory-safe static Rust daemon (`escamd`) under 8 MB RAM footprint.
  - Map kernel device nodes (`/dev/motor`, `/dev/gkio`, `/dev/venc`, `/dev/vi`) to type-safe, idiomatic Rust hardware abstractions.
  - Clean boot in under 2 seconds directly from SPI flash init with zero cloud telemetry.

---

## 2. Marcus — The Astrophotographer & Optical Hacker
- **Archetype**: Backyard Astrophotographer & Planetary Imager.
- **Background**: Uses amateur telescopes, autoguiders (PHD2), and imaging suites (Ekos, KStars, NINA, Siril) for lunar, planetary, and deep-sky imaging.
- **Pain Points**:
  - Commercial CCTV cameras compress video into lossy 8-bit H.264 streams, destroying faint deep-sky nebulae and stellar dynamic range.
  - Stock firmware enforces automatic exposure loops (25-30 fps) with no way to take 1s, 2s, or 5s manual sub-exposures.
  - Mechanical IR-cut filters cannot be permanently or selectively disengaged via open APIs for Hydrogen-Alpha (Hα, 656.3nm) imaging.
- **Goals with this platform**:
  - Adapt the M12 camera sensor mount to standard 1.25" telescope focusers.
  - Extract raw uncompressed 10-bit Bayer frames directly into standard astronomical FITS containers.
  - Provide an embedded INDI (Instrument-Neutral Distributed Interface) driver over TCP port 7624 for native Ekos/NINA/Siril integration.
  - Remotely toggle the UTC BA6208L H-bridge solenoid to flip between daytime visible and nighttime full-spectrum Hα modes.

---

## 3. Samir — The Self-Hosted Privacy Advocate & Smart Home Operator
- **Archetype**: Privacy-Conscious Homeowner & Self-Hoster.
- **Background**: Runs local Home Assistant / Scrypted / Frigate instances, with strict network segmentation and all camera WAN access blocked.
- **Pain Points**:
  - Stock web UI requires obsolete ActiveX or Flash controls and Internet Explorer.
  - Mobile apps require proprietary P2P cloud relay servers based in foreign datacenters.
  - Jerky stepper motor movement sounds loud, alerts intruders, and vibrates camera mounts.
- **Goals with this platform**:
  - Sub-100ms ultra-low latency live video streaming directly in modern mobile/desktop web browsers via WebRTC (zero plugins).
  - Modern, responsive, snappy Single Page App (SPA) loaded directly from the camera's local web server.
  - Whisper-quiet, smooth PTZ steering with virtual joystick and sinusoidal S-curve velocity profiling.

---

## 4. Alex — The Systems Architect & Quality Custodian
- **Role**: SpecOps SDLC Architect and Software Craftsperson.
- **Goals with this platform**:
  - Maintain unbroken bidirectional traceability across Personas -> PRDs -> Stories -> Tasks -> ADRs -> Commits.
  - Enforce strict single-responsibility modularity (<500 lines per file) and frontdoor blackbox verification with zero backdoor mocking.
