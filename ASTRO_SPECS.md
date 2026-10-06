# ESCAM G02 - Image Sensor & Astrophotography Reference

## 1. Overview

The sensor daughterboard (`YT_1135M_1034_V1.0`) features a dual-footprint layout designed for two common budget CCTV CMOS sensors:
1. **SmartSens SC1135** (Primary / higher-grade 1.3MP 1/3" sensor)
2. **GalaxyCore GC1034** (Alternate / 1.0MP 1/4" 720p sensor)

---

## 2. Sensor Technical Specifications

| Parameter | SmartSens SC1135 | GalaxyCore GC1034 | Astrophotography Significance |
| :--- | :--- | :--- | :--- |
| **Optical Format** | **1/3-inch** (~6.0 mm diagonal) | **1/4-inch** (~4.5 mm diagonal) | Sensor size / field of view (FOV) |
| **Active Array** | **1280 × 960 (1.3 MP)** | **1280 × 720 (1.0 MP)** | 4:3 (astronomy standard) vs 16:9 |
| **Pixel Pitch** | **3.0 µm – 3.75 µm** | **~2.5 µm – 2.8 µm** | Determines angular image scale per arcsec |
| **On-Chip ADC** | **12-bit (4,096 ADU levels)** | **10-bit (1,024 ADU levels)** | Tonal depth in deep-sky gradients & star cores |
| **Full Well Capacity** | **~10,000 – 12,000 $e^-$** | **~5,000 – 6,500 $e^-$** | Saturation limit before stars blow out |
| **Dynamic Range** | **~70 dB (11.6 stops)** | **~60 – 64 dB (~10 stops)** | Contrast between faint nebula wisps & bright stars |
| **Read Noise** | **~2.8 – 3.5 $e^-$ rms** | **~4.5 – 5.5 $e^-$ rms** | Determines minimum sub-exposure length |
| **Max SNR ($\text{SNR}_{\max}$)** | **~40 dB** ($\approx \sqrt{\text{FWC}}$) | **~37 dB** | Peak signal-to-noise ratio at saturation |
| **Low-Light Tech** | SmartSens "Starlight" Deep N-Well | Standard Front-Illuminated (FSI) | High QE in near-IR & Hydrogen-Alpha ($H\alpha$) |
| **Minimum Illumination** | 0.01 Lux (B/W) | ~0.1 Lux | Crucial for detecting faint guide stars |
| **Shutter** | Electronic Rolling Shutter (ERS) | Electronic Rolling Shutter (ERS) | Standard for high-frame-rate "lucky imaging" |
| **Lens Mount** | Standard **M12 × 0.5** (S-Mount) | Standard **M12 × 0.5** (S-Mount) | Unscrews to accept 1.25" telescope nosepieces |

---

## 3. Astrophotography Applications & Optics Math

### 3.1 Pixel Scale Equation
$$\text{Pixel Scale (arcsec/pixel)} = \frac{\text{Pixel Size }(\mu\text{m})}{\text{Focal Length }(\text{mm})} \times 206.265$$

### 3.2 SC1135 (3.75 µm Pixels): The "ASI120" Equivalent
A 1/3" 1280×960 sensor with 3.75 µm pixels has the **exact same geometry and pixel pitch** as the legendary **ZWO ASI120MC / Aptina AR0130**, one of the most famous entry-level planetary and autoguiding cameras in amateur astronomy.

* **On a 50mm Guide Scope ($f = 200\text{ mm}$):**
  $$\text{Scale} = \frac{3.75}{200} \times 206.265 \approx \mathbf{3.86\text{ arcsec/pixel}}$$
  *Ideal sampling for PHD2 multi-star autoguiding.*
* **On an 8" SCT ($f = 2000\text{ mm}$):**
  $$\text{Scale} = \frac{3.75}{2000} \times 206.265 \approx \mathbf{0.38\text{ arcsec/pixel}}$$
  *Near-critical sampling under typical 1–2 arcsec atmospheric seeing for Jupiter and Saturn.*

---

## 4. Hardware Astrophotography Hacks for This Board

1. **The M12 to 1.25" Telescope Adapter:**
   * The lens screwed into the daughterboard is a standard **M12 (S-mount)** lens.
   * Unscrew the stock 3.6mm CCTV lens, and screw in a standard **M12-to-1.25" barrel adapter** (~$6 on Amazon/AliExpress).
   * You can now slide the camera directly into any telescope focuser, Barlow lens, or guide scope.

2. **Full-Spectrum / Near-IR Mod (The IR-Cut Solenoid):**
   * The camera includes a motorized **IR-Cut filter slider** sitting right over the sensor.
   * By toggling the IR-cut filter open (via the camera's Night Mode or disconnecting the solenoid power lead), the sensor sees full-spectrum light, including **Hydrogen-Alpha ($H\alpha$ at 656.3 nm)** and near-infrared (NIR 850nm), where deep-sky emission nebulae shine brightest.

3. **DIY Projects:**
   * **All-Sky Meteor Camera:** Fit with a 1.8mm or 2.1mm fisheye M12 lens, seal in an acrylic dome, and run automated long-exposure meteor detection (e.g. Global Meteor Network / RMS software).
   * **Planetary Lucky Imaging:** Stream raw frames via RTSP/V4L2 into `AutoStakkert!` or `Siril` to stack the sharpest 10% of frames.
   * **Solar System Guider:** Mount on a finder scope to track guide stars.
