# Tutorial: Getting Started with ESCAM G02 Astrophotography

Welcome to the **ESCAM G02 Astrophotography Guide**. This tutorial takes you step-by-step from powering on the camera to capturing and downloading your first calibrated, stacked deep-sky image of celestial targets.

---

## 1. Prerequisites & Setup

1. **Hardware**: ESCAM G02 camera plugged into 5V/1A USB power.
2. **Network**: Camera connected via Wi-Fi or Ethernet to your local network (e.g. `10.75.2.93`).
3. **Web Browser**: Modern browser (Chrome, Edge, Firefox, or Safari) pointed to `http://10.75.2.93:8080`.

---

## 2. Setting Optical & Night Vision Modes

1. In the Web UI, locate the **Optical Filter & Presets** card.
2. Click **Night / Astro**. You will hear the mechanical click of the bi-stable IR-cut filter pulling away from the sensor.
3. Verify that the **IR LED** is set to **OFF** (astronomical imaging requires total darkness so artificial LEDs do not wash out the sensor).

---

## 3. Selecting a Celestial Target via GoTo Slew

1. In the **Mount Quick Presets** section, locate the **Celestial GoTo Target** dropdown.
2. Select your desired celestial target:
   - `Polaris`: Aligns mount with the North Celestial Pole.
   - `Jupiter` / `Saturn`: Planetary targets with high surface brightness.
   - `Orion Nebula` / `Andromeda Galaxy`: Deep-sky extended targets.
3. The stepper motors will execute a smooth S-curve acceleration profile, slewing to the calculated Altitude and Azimuth coordinates.

---

## 4. Achieving Critical Focus

1. Observe the **Focus HUD** indicator in the video overlay (`Stars: N | FWHM: X.Xpx`).
2. Turn the manual lens barrel slightly while monitoring the median FWHM score:
   - Higher values ($> 4.5\text{px}$) indicate defocused blur circles.
   - Lowest values ($1.2 - 2.5\text{px}$) represent sharp, diffraction-limited focus.
3. When the FWHM reaches a local minimum, lock the lens ring.

---

## 5. Long Exposure & Live Stacking

1. Under **Manual Astrophotography Controls**, adjust the **Shutter Duration** to `5.0s` or `10.0s`.
2. Set **Sensor Gain** to `8x (18 dB • ISO 800)` or `16x (24 dB • ISO 1600)`.
3. Under **Live Stacking**, select **Additive** or **Average**.
4. The linear photon accumulator will integrate multiple frames in real-time. Watch faint nebulae emerge from the dark background without artificial noise amplification.

---

## 6. Downloading Scientific FITS Data

1. Click **FITS Raw** in the Mount card.
2. Your browser downloads `capture.fits`, containing uncompressed 16-bit linear Bayer data stamped with full WCS coordinates, equatorial RA/DEC, and observatory site metadata.
3. Open `capture.fits` in **Siril**, **AstroImageJ**, or **PixInsight** for dark calibration and photometric plate-solving.
