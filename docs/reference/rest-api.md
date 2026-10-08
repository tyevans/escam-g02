# Reference: ESCAM G02 REST API Specification

The ESCAM G02 camera runs a high-performance, embedded HTTP server on port `8080`. All REST endpoints support JSON payloads and cross-origin resource sharing (`Access-Control-Allow-Origin: *`).

---

## Endpoint Catalog

### System & Telemetry
| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/v1/status` | System health, RSS memory, and device status |
| `GET` | `/api/v1/system/time` | Julian Date, LST, and UTC clock telemetry |
| `POST` | `/api/v1/system/time` | Configure site coordinates and UTC clock |
| `GET` | `/api/v1/system/flash` | Flash partition usage and debloat audit |
| `POST` | `/api/v1/system/debloat` | Purge legacy vendor assets and install clean boot |
| `GET` | `/api/v1/openapi.json` | OpenAPI 3.1 schema specification |

### Motion & PTZ Control
| Method | Endpoint | Description |
|---|---|---|
| `POST` | `/api/v1/ptz` | Command directional or virtual joystick step movement |
| `POST` | `/api/v1/ptz/home` | Execute soft-homing calibration to mechanical hardstop |
| `GET` | `/api/v1/ptz/backlash` | Query mechanical gear backlash settings |
| `POST` | `/api/v1/ptz/backlash` | Configure mechanical gear backlash steps |

### Astrophotography & Optics
| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/v1/camera` | Query current exposure duration, gain, and FPS |
| `POST` | `/api/v1/camera` | Update exposure duration, gain, resolution, or stretch |
| `GET` | `/api/v1/astro/capture.fits` | Download 16-bit linear uncompressed FITS with WCS |
| `GET` | `/api/v1/astro/focus` | Real-time star PSF count, centroids, and FWHM seeing |
| `GET` | `/api/v1/astro/transients` | List detected meteor, fireball, and satellite streaks |
| `GET` | `/api/v1/astro/targets` | List celestial catalog targets and current Alt/Az |
| `POST` | `/api/v1/astro/slew` | Slew mount to target coordinates |
| `POST` | `/api/v1/astro/guide/start` | Lock star and start closed-loop autoguiding |
| `POST` | `/api/v1/astro/guide/stop` | Halt closed-loop autoguiding |
| `POST` | `/api/v1/astro/calibration/dark` | Capture and save Master Dark calibration frame |
| `GET` | `/api/v1/astro/calibration/status` | Query active calibration profiles |

### Event Recording & Media
| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/v1/snapshot` | Capture single JPEG snapshot |
| `GET` | `/api/v1/stream` | Multipart MJPEG continuous video stream |
| `GET` | `/api/v1/ws` | WebSocket binary H.264 stream and PTZ command channel |
| `POST` | `/api/v1/recorder/trigger` | Flush in-memory circular buffer to video clip |
| `GET` | `/api/v1/recorder/clips` | List recorded event clips on disk |

### Sensor Hardware
| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/v1/sensor/registers` | Read CMOS sensor I2C registers (GC1034 / SC1135) |
| `POST` | `/api/v1/sensor/registers` | Write CMOS sensor registers |
| `POST` | `/api/v1/ircut` | Actuate IR-cut filter relay (`Day` vs `Night`) |
| `POST` | `/api/v1/irled` | Toggle 850nm infrared illuminator LEDs |
