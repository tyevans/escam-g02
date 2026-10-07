# Compatible Hardware & Board Ecosystem Reference

This reference documents the board provenance, OEM lineage, and candidate sibling devices compatible with the **ESCAM G02** pure Rust embedded firmware stack ([`escamd`](file:///home/ty/workspace/research/escam-g02/crates/escamd/src/main.rs)).

---

## 1. Board Provenance & System Architecture

While retailed under the consumer brand **ESCAM** (Shenzhen Escam Technology Co., Ltd.), the camera is a white-label integration of a turnkey reference design produced by the Shenzhen video surveillance ODM ecosystem.

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                      Turnkey Reference Platform                         │
│                                                                         │
│   SoC Manufacturer:     Goke Microelectronics (Hunan / Shenzhen)        │
│   SoC Model:            GK7102 / GK7102C (ARM1176JZF-S @ 600 MHz)       │
│   Board ODM:            Shenzhen YuanTe Technology Co., Ltd.            │
│   Main PCB Marking:     YT_710XC_M_2.1                                  │
│   Sensor PCB Marking:   YT_1135M_1034_V1.0                              │
│   Firmware Subsystem:   Hipcam / HiP2P (Legacy HiSilicon CGI API)       │
└─────────────────────────────────────────────────────────────────────────┘
```

### Core Hardware Specifications

| Subsystem | Component | Specifications & Role |
| :--- | :--- | :--- |
| **Processor (SoC)** | **Goke GK7102C** | ARM1176JZF-S (ARMv6l) @ 600 MHz; hardware H.264 VPU engine |
| **System Memory** | SiP DDR2 DRAM | 64 MB integrated System-in-Package |
| **Storage (NOR Flash)**| XM25QH64AHIG | 8 MB (64 Mbit) SPI NOR Flash (Winbond W25Q64 compatible) |
| **Wi-Fi Subsystem** | Realtek RTL8188FTV | 802.11b/g/n 2.4 GHz; connected via internal **USB 2.0 Host** (`8188fu.ko`) |
| **Ethernet PHY** | Integrated Goke MAC | 10/100 Mbps discrete magnetics (HB1601SNL / 1.5 kV isolation) |
| **Pan/Tilt Motor Driver**| JULN2803AG | 8-channel Darlington transistor array sinking unipolar stepper coils |
| **Physical Steppers** | 2x 28BYJ-48 style | 5V unipolar 4-phase steppers (4 phases Pan + 4 phases Tilt) |
| **IR-Cut Solenoid** | UTC BA6208L | Bi-directional H-bridge driving mechanical IR-cut filter glass |
| **Optics Mount** | Standard S-Mount | **M12 × 0.5** threaded mount (accepts M12-to-1.25" telescope adapters) |
| **Serial Console** | Header `J3` | Unpopulated 4-pin 3.3V TTL UART (115200 8N1: TX, GND, RX, 3.3V) |

---

## 2. Tier 1 Candidates: Direct Clones & Drop-In Siblings ($12 – $20)

Between 2017 and 2022, YuanTe manufactured millions of identical and near-identical boards for various consumer security brands. These devices share the **Goke GK7102 / GK7102C** processor, the `motor.ko` Darlington driver, the RTL8188FTV Wi-Fi interface, and standard M12 lens threading.

The static ARMv6 musl binary ([`escamd`](file:///home/ty/workspace/research/escam-g02/crates/escamd/src/main.rs)) and the loopback jail interceptor ([`libgk_vpu.c`](file:///home/ty/workspace/research/escam-g02/crates/escam-driver/src/libgk_vpu.c)) run on these targets with zero or minimal adjustments:

| Device Model | Brand / Vendor | Sensor & Resolution | Compatibility Notes |
| :--- | :--- | :--- | :--- |
| **ESCAM QF002 / G01** | ESCAM | GC1034 / SC1135 (720p) | **100% Drop-in**. Identical plastics, PCB layout (`YT_710XC`), and MTD partitions. |
| **Digoo DG-M1Q** | Digoo | GC1034 (720p) | **Direct Match**. Extremely popular mini-robot PTZ; identical `/dev/motor` ioctl codes. |
| **Digoo DG-MYQ / DG-W01F** | Digoo | GC1034 / SC2135 | Fixed & PTZ variants. Same Goke Linux 3.4.43 kernel and Wi-Fi stack. |
| **GUUDGO GD-SC01 / GD-SC03** | GUUDGO | GC1034 (720p) | Widely documented in open firmware communities; uses identical Darlington pinouts. |
| **ZS-GX1 / Snowman SRC-001** | White-label / OEM | GC1034 / SC1135 (720p) | The classic reference hardware for early Goke reverse-engineering (`zsgx1hacks`). |
| **Wanscam K21 / HW0021** | Wanscam | 720p GC1034 | Shares the CamHi / HiP2P firmware ecosystem and `/cgi-bin/hi3510/` CGI schema. |
| **INQMEGA IL-HIP291** | INQMEGA | GC1034 (720p) | Dual-antenna robot PTZ dome; identical internal chassis and stepper gearing. |
| **KERUI CIPC-GC13H / GC15HE** | KERUI | SC1135 / GC1034 | Standard PTZ dome; same U-Boot bootloader (`loadaddr 0xC1000000`). |

---

## 3. Tier 2 Candidates: Upgraded Sensor Siblings (1080p & Astrophotography)

The sensor daughterboard (`YT_1135M_1034_V1.0`) features a dual-footprint PCB designed to support higher-end sensors. Sourcing cameras with upgraded sensors provides significant optical advantages:

### 1. SmartSens SC1135 (1.3 MP 1/3" CMOS)
* **Astrophotography Advantage**: Features **3.75 µm pixels**—the exact same geometry and pixel pitch as the **ZWO ASI120MC / Aptina AR0130**, one of the most widely used planetary and autoguiding sensors.
* **Sensitivity**: Deep N-Well "Starlight" architecture with high quantum efficiency in near-IR and Hydrogen-Alpha ($H\alpha$ 656.3 nm).
* **Driver**: Requires loading `sc1135.ko` instead of `gc1034_ex.ko`.

### 2. Goke GK7102S + SmartSens SC2235 / SC2135 (2.0 MP 1080p)
* **Hardware Profile**: The **GK7102S** is Goke's pin-compatible 1080p refresh of the GK7102. It retains the ARM1176 core and 64 MB SiP DDR2, but increases maximum encode resolution to 1920×1080 @ 25 FPS.
* **Target Models**: Suffix models like **GUUDGO GD-SC11**, **Digoo DG-M1Z**, and **INQMEGA 1080P Robot**.
* **Driver**: Uses `sc2235.ko` or `sc2135.ko`.

---

## 4. Sourcing Guide: How to Identify Candidates Online

When sourcing surplus, refurbished, or clearance units on AliExpress, eBay, Banggood, or Amazon, look for these signatures:

```text
       Front View                         Rear View
     ┌─────────────┐                    ┌─────────────┐
     │   ┌─────┐   │                    │     ▲ ▲     │  (Dual 2.4GHz
     │  (   ●   )  │  <-- Black Eye     │     │ │     │   Swivel Antennas)
     │   └─────┘   │      (M12 Mount)   │  ┌───────┐  │
     │  [MicroSD]  │  <-- Chin Slot     │  │ RJ-45 │  │  <-- 10/100 Ethernet
   ┌─┴─────────────┴─┐                ┌─┴──┴───────┴──┴─┐
   │    Motorized    │                │  (O) 5V DC In   │  <-- Barrel / Micro-USB
   │    Cradle       │                │  (.) Reset Pin  │
   └─────────────────┘                └─────────────────┘
```

### Key Signatures to Verify Before Purchasing

1. **Physical Clues**:
   - Classic **"Snowman" / "Robot" dual-antenna PTZ** enclosure (white body with black eyeball sphere).
   - MicroSD slot hidden underneath the lens eyeball (accessible by tilting camera upward).
   - Rear panel containing both an **RJ45 Ethernet jack** and a 5V DC port (barrel or micro-USB).
   - Front bezel unscrews to expose a standard **M12 × 0.5** threaded lens mount.
2. **Software & Protocol Clues**:
   - Camera manual or box mentions **"CamHi"**, **"CamHiPro"**, **"HiP2P"**, or **"YCC365 Plus"**.
   - Specifications state: **H.264 video compression**, **720p (1280×720) or 1080p (1920×1080)**, and **355° Pan / 90° Tilt**.
3. **What to Avoid**:
   - Units advertising **"H.265 3MP / 5MP / 4K"** (these typically feature Ingenic T31, SigmaStar, or Xiongmai XM530 processors).
   - Battery-powered or solar wire-free cameras (different low-power RTOS architectures).
   - Cameras that only support Bluetooth / BLE provisioning with no Ethernet jack.

---

## 5. Verification & Onboarding Checklist for New Units

Follow this protocol when onboarding newly acquired candidate devices into the `escamd` fleet:

1. **Serial Console Break-in**:
   - Disassemble base by removing rubber footpads and 4 Phillips screws.
   - Locate 4-pin unpopulated header **`J3`** near the Realtek Wi-Fi module.
   - Connect 3.3V USB-UART adapter: `Pin 1 = TX`, `Pin 2 = GND`, `Pin 3 = RX` at 115200 8N1.
   - Power on device and halt autoboot to verify the `GK7102 #` U-Boot prompt.
2. **Backup Stock Flash**:
   - In U-Boot or via network telnet, dump the 8 MB SPI NOR image (`cat /dev/mtd0 > /tmp/flash.bin` or via TFTP).
   - Confirm MTD partition table aligns with standard 8 MB layout:
     ```text
     mtd0: 00030000 (U-Boot)
     mtd1: 00010000 (Boot Env)
     mtd2: 00150000 (Kernel)
     mtd3: 001d0000 (RootFS SquashFS)
     mtd4: 00300000 (Vendor App SquashFS)
     mtd5: 001a0000 (Conf JFFS2)
     ```
3. **Sensor Identification**:
   - Check `ls /app/modules/` or kernel logs (`dmesg | grep -i sensor`) to verify sensor driver (`gc1034_ex.ko`, `sc1135.ko`, or `sc2235.ko`).
4. **Deploy `escamd`**:
   - Deploy `libgk_vpu.so` into `/home/` to intercept vendor socket binds and jail RTSP to `127.0.0.1`.
   - Launch `escamd` static binary (`arm-unknown-linux-musleabi`) to gain WebSockets low-latency streaming, Bresenham PTZ kinematics, and the INDI CCD astrophotography interface.
