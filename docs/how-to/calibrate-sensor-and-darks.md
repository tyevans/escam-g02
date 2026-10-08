# How-To: Calibrate Sensor Dark, Flat, and Bias Frames

This recipe guides you through creating and applying astronomical calibration frames (Master Darks, Flats, and Biases) on the ESCAM G02 to eliminate thermal fixed-pattern noise and optical vignetting.

---

## The Mathematics of Optical Calibration

Scientific CCD/CMOS image calibration follows the linear relation:
$$\text{Calibrated}(x,y) = \frac{\text{Raw}(x,y) - \text{Dark}(x,y) - \text{Bias}(x,y)}{\text{Flat}(x,y) / \bar{\text{Flat}}} + \text{Pedestal}$$

Where:
- $\text{Dark}(x,y)$: Thermal dark current and hot pixels accumulated over exposure time $T$.
- $\text{Bias}(x,y)$: Read noise baseline at exposure $T \to 0$.
- $\text{Flat}(x,y)$: Normalized optical vignetting and dust donut profile.
- $\text{Pedestal}$: Integer offset (default 100 ADU) preventing zero-value clipping.

---

## Step 1: Capturing Master Dark Frames

1. Cover the lens completely or click **Day (IR-Cut ON)** with lights extinguished to ensure zero incident photons reach the sensor.
2. Set exposure duration and gain to match your target light frames (e.g. 10.0s, 16x gain).
3. Send a dark capture request via curl:
   ```bash
   curl -X POST http://10.75.2.93:8080/api/v1/astro/calibration/dark
   ```
4. Or in the Web UI, click the **Dark Cal** button.
5. The `CalibrationEngine` stores the master dark profile in RAM and stamps `CALIBRAT = 'DARK'` on subsequent FITS exports.

---

## Step 2: Capturing Master Flat Frames

1. Point the camera at a uniformly illuminated, diffuse white screen (or white cloth stretched across the lens).
2. Set exposure duration so median ADU is approximately 50% of saturation (~30,000 ADU in 16-bit).
3. Post the master flat buffer via API:
   ```bash
   curl -X POST http://10.75.2.93:8080/api/v1/astro/calibration/flat \
     -H "Content-Type: application/octet-stream" --data-binary @flat.raw
   ```

---

## Step 3: Verifying Calibration Status

Query the online calibration state:
```bash
curl -s http://10.75.2.93:8080/api/v1/astro/calibration/status
```

Response:
```json
{
  "has_dark": true,
  "has_flat": false,
  "has_bias": false,
  "pedestal": 100
}
```
All subsequent `/api/v1/astro/capture.fits` and live stacking operations will automatically subtract the master dark frame.
