# How-To: Connect KStars, Ekos, and N.I.N.A. via Embedded INDI Protocol

The ESCAM G02 firmware includes an embedded INDI (Instrument Neutral Distributed Interface) protocol server on port 7624, exposing both a CCD Camera device (`ESCAM G02 CCD`) and an Alt-Az Telescope Mount device (`ESCAM G02 Mount`).

---

## 1. Network Verification

Ensure port 7624 is reachable from your host workstation:
```bash
nc -zv 10.75.2.93 7624
```
Expected output:
```
Connection to 10.75.2.93 7624 port [tcp/*] succeeded!
```

---

## 2. Configuring KStars / Ekos

1. Open **KStars** on your Linux, macOS, or Windows computer.
2. Launch the **Ekos** control panel (Tools -> Ekos or Ctrl+K).
3. Under **Profile Wizard**, create a new profile:
   - **Profile Name**: `ESCAM G02`
   - **Mode**: `Remote`
   - **Remote Host**: `10.75.2.93`
   - **Remote Port**: `7624`
   - **CCD**: Select `ESCAM G02 CCD` (or `INDI WebCam / Custom`)
   - **Telescope**: Select `ESCAM G02 Mount`
4. Click **Save Profile** and then **Start INDI**.

---

## 3. Controlling Telescope Motion from Ekos

1. In the Ekos **Mount** tab, enable the directional motion buttons (North, South, East, West).
2. Pressing **North** or **South** commands the Tilt stepper motor upward or downward.
3. Pressing **East** or **West** commands the Pan stepper motor clockwise or counter-clockwise.
4. Set motion speed between `1` (fine guiding rate) and `4` (maximum slew rate).

---

## 4. Capturing Exposures in Ekos

1. Navigate to the Ekos **Capture** tab.
2. Configure exposure duration (e.g. `2.0` seconds) and frame count.
3. Click **Start Capture**. Ekos will send `<newNumberVector>` to port 7624.
4. The captured uncompressed 16-bit FITS frame will download directly into the FITS Viewer with complete WCS headers and Bayer pattern RGGB.
