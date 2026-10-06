# ESCAM G02 IP Camera - Reverse Engineering & Setup Reference

## 1. Device Overview

* **Brand / Model:** ESCAM G02
* **Type:** Indoor Pan/Tilt 720p Wi-Fi & Ethernet IP Camera
* **Main SoC:** **Goke Microelectronics GK7102C**
  * **Package Markings:** `GOKE GK7102C A 1824 1023 D5N3`
  * **Date Code:** `1824` = 2018, Week 24 (~June 2018)
  * **Architecture:** ARM11 (ARMv6) Core @ ~600MHz
  * **Memory:** Integrated SiP (System-in-Package) 64MB DDR2 DRAM
  * **Video Engine:** Hardware H.264 Encoder (720p @ 25-30fps)
  * **OS:** Embedded Linux (3.x kernel) with BusyBox userland
* **Flash Memory (SPI NOR):** **XMC XM25QH64AHIG**
  * **Package Markings:** `XMC QH64AHIG`
  * **Manufacturer:** Wuhan Xinxin Semiconductor (XMC)
  * **Capacity:** 64 Megabits (8 Megabytes / 8,388,608 bytes)
  * **Interface:** Standard / Dual / Quad SPI @ 3.3V, SOP8 package
  * **Pinout / Protocol:** 100% pin-compatible with Winbond W25Q64 / GD25Q64
* **Wi-Fi Controller:** **Realtek RTL8188FTV**
  * **Package Markings:** `REALTEK 8188FTV J812931`
  * **Standards:** 802.11b/g/n (1T1R, up to 150 Mbps, 2.4 GHz only)
  * **Host Interface:** **USB 2.0** (Connected internally to the GK7102C USB Host controller via D+/D- lines)
  * **Driver in Linux:** `rtl8188fu` / `8188fu.ko`
* **Ethernet Magnetics:** **HB1601SNL**
  * **Package Markings:** `GTR 1920G HB1601SNL`
  * **Function:** 10/100Base-T LAN discrete isolation transformer & common-mode EMI filter (16-pin SOP package)
  * **Location:** Directly inline between the RJ45 socket and the GK7102C Ethernet PHY pins (provides 1500 Vrms galvanic isolation)
* **Motor Driver:** **JULN2803AG**
  * **Type:** 8-Channel Darlington Transistor Array (SOP-18 package)
  * **Function:** Sinks high-current drive to the two unipolar stepper motors (4 phases for Pan + 4 phases for Tilt = 8 channels total).
  * **Control:** Driven by GPIO/PWM pins directly from the Goke GK7102C SoC.
* **Power Management:** **Silergy YS19BKJ**
  * **Type:** DC-DC Step-Down Buck Converter (SOT23-5 / SOT23-6)
  * **Function:** Steps down incoming 5V DC barrel/USB power to 3.3V (I/O, Flash, Wi-Fi) and 1.8V/1.2V (ARM core & SiP DDR2).
* **Default Credentials:**
  * **Username:** `admin`
  * **Password:** `admin` (or empty string upon factory reset)

### Hardware Bill of Materials (BOM) Summary

| Subsystem | Component | Package | Role / Notes |
| :--- | :--- | :--- | :--- |
| **Main SoC** | Goke GK7102C A (`1824 1023 D5N3`) | QFN | ARM11 @ 600MHz, 64MB SiP DDR2 RAM, H.264 engine |
| **Flash Memory** | XMC XM25QH64AHIG (`QH64AHIG`) | SOP-8 | 8 MB SPI NOR Flash (U-Boot, Kernel, RootFS, Apps) |
| **Wi-Fi** | Realtek RTL8188FTV (`J812931`) | QFN | 802.11b/g/n 2.4GHz, connected via internal **USB 2.0 Host** |
| **Motor Driver** | JULN2803AG | SOP-18 | 8x Darlington array driving 2x 4-phase unipolar stepper motors |
| **IR-Cut Driver** | UTC BA6208L (`QKT8 81`) | SOP-8 | Bi-directional H-Bridge driver for the mechanical IR-cut filter solenoid |
| **LAN Magnetics**| HB1601SNL (`GTR 1920G`) | SOP-16 | 10/100Base-T discrete 1.5kV isolation transformer |
| **Power PMIC** | Silergy (`YS19BKJ`) | SOT23-5/6 | DC-DC buck converter stepping down 5V input |
| **Main PCB** | `YT_710XC_M_2.1` | PCB | YuanTe GK710X Mainboard, Hardware Revision 2.1 |
| **Sensor PCB** | `YT_1135M_1034_V1.0` | PCB | Populated with **GalaxyCore GC1034** (driver `gc1034_ex.ko`) |

### Extracted Kernel Modules (`app/modules/`)

* **`gc1034_ex.ko`**: Sensor driver for GalaxyCore GC1034 720p CMOS sensor
* **`sensor.ko`**: Core sensor abstraction layer
* **`media.ko`**: Hardware H.264 video encoder & ISP pipeline (`/dev/venc`, `/dev/vi`)
* **`motor.ko`**: PTZ stepper motor driver (controls `JULN2803AG` Darlington channels)
* **`8188fu.ko`**: Realtek RTL8188FTV Wi-Fi driver
* **`gkio.ko`**: Goke GPIO driver
* **`audio.ko`** / **`i2s.ko`**: Audio input/output DAC/ADC driver

### Firmware Dump & MTD Layout (`firmware_dump/`)

* **Full 8 MB ROM Image:** [`escam_g02_full_8mb_rom.bin`](file:///home/ty/workspace/research/escam-g02/firmware_dump/escam_g02_full_8mb_rom.bin) (`MD5: 8239d21c768e071003c3d70048f04fcf`)

| Partition | Size | Address Range | Format | MD5 Checksum | Notes |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `mtd0_boot.bin` | 192 KB | `0x000000 - 0x030000` | Raw binary | `9b3230db0d433292e101f65eb7193c04` | U-Boot bootloader |
| `mtd1_bootenv.bin` | 64 KB | `0x030000 - 0x040000` | Raw binary | `92f05c9f9e8be0d825f8f89403b2ce58` | U-Boot environment variables |
| `mtd2_kernel.bin` | 1.4 MB | `0x040000 - 0x190000` | ARM zImage | `d5ed65ad6e724d09a46c14038d8aaa80` | Linux 3.4.43-gk kernel |
| `mtd3_rootfs.bin` | 1.9 MB | `0x190000 - 0x360000` | SquashFS 4.0 XZ | `8515157732a967406ff6da0749fd628a` | Base Linux OS & BusyBox |
| `mtd4_app.bin` | 3.0 MB | `0x360000 - 0x660000` | SquashFS 4.0 XZ | `b65b8dc5c27f7fcca8e0fc2d22102377` | Vendor apps, web UI, kernel modules |
| `mtd5_conf.bin` | 1.7 MB | `0x660000 - 0x800000` | JFFS2 (RW) | `ead95b9882fcdb1eeaa67f9ebba707a4` | Persistent writable settings & run script |

---

## 2. Root Shell Access

A permanent, unauthenticated root shell is configured and running on port 2323:

```bash
# Connect directly to the camera root shell over Wi-Fi
telnet 10.75.2.93 2323
```
*(Spawns directly as `uid=0(root)` with no username or password prompt).*

---

## 3. Hardware & Network Interfaces

| Interface | MAC Address | Notes |
| :--- | :--- | :--- |
| **Ethernet (RJ45)** | `00:E0:F8:5E:D1:85` | 10/100 Mbps auto-negotiation |
| **Wi-Fi (802.11 b/g/n)** | `FC:6B:F0:5E:0D:60` | 2.4 GHz only; WPA2-PSK AES supported |

> **Firewall / Quarantine Note:**
> The Wi-Fi MAC is distinct from the Ethernet MAC. To prevent the camera from phoning home or transmitting telemetry externally, ensure both MAC addresses (or the DHCP reservation IP) have all outbound WAN traffic blocked at your router/firewall.

---

## 3. Open Network Services & Ports

* **TCP 80:** HTTP Web Management & CGI API (`Server: Hipcam`)
* **TCP 554:** RTSP Video Streaming (H.264 / G.711)
* **TCP 1935:** RTMP Video Streaming
* **TCP 8080:** ONVIF SOAP Service

---

## 4. Reverse Engineered CGI API

The camera uses an embedded CGI interface rooted at `/cgi-bin/hi3510/`. Requests require HTTP Basic Authentication (`Authorization: Basic YWRtaW46YWRtaW4=`).

### 4.1 System & Network Discovery

* **Fetch System & Video Attributes:**
  ```bash
  curl -u admin:admin "http://<IP>/cgi-bin/hi3510/param.cgi?cmd=getlanguage&cmd=getvencattr&-chn=11&cmd=getvencattr&-chn=12&cmd=getsetupflag&cmd=getaudioflag&cmd=getrtmpattr"
  ```
* **Network Status (IP, Gateway, MAC, Connection Mode):**
  ```bash
  curl -u admin:admin "http://<IP>/cgi-bin/hi3510/param.cgi?cmd=getnetattr"
  ```
* **P2P Cloud Service Status:**
  ```bash
  curl -u admin:admin "http://<IP>/cgi-bin/hi3510/param.cgi?cmd=gethip2pattr"
  ```
  *(Returns `hip2p_enable="0"`; verified disabled by default).*

### 4.2 Wi-Fi Scanning & Provisioning

* **Trigger Live Wi-Fi Scan:**
  ```bash
  curl -u admin:admin "http://<IP>/cgi-bin/hi3510/param.cgi?cmd=searchwireless"
  ```
  *Returns JavaScript array with `waccess_points`, `wessid`, `wrssi`, `wauth`, `wenc`, `wchannel`.*

* **Configure Wi-Fi Credentials via API:**
  ```bash
  curl -u admin:admin "http://<IP>/cgi-bin/hi3510/param.cgi?cmd=setwirelessattr&-wf_enable=1&-wf_ssid=YOUR_SSID&-wf_key=YOUR_PASSWORD&-wf_mode=0&-wf_auth=3&-wf_enc=1"
  ```
  *Parameter details:*
  * `-wf_enable=1`: Turn on Wi-Fi module
  * `-wf_mode=0`: Infrastructure / client mode
  * `-wf_auth=3`: WPA2-PSK (`0` = None, `1` = WEP, `2` = WPA-PSK, `3` = WPA2-PSK)
  * `-wf_enc=1`: AES (`0` = TKIP, `1` = AES)

### 4.3 Motorized PTZ (Pan / Tilt) Controls

Endpoint: `/cgi-bin/hi3510/ptzctrl.cgi`

| Action | HTTP Request |
| :--- | :--- |
| **Move Left** | `GET /cgi-bin/hi3510/ptzctrl.cgi?-step=0&-act=left` |
| **Move Right** | `GET /cgi-bin/hi3510/ptzctrl.cgi?-step=0&-act=right` |
| **Move Up** | `GET /cgi-bin/hi3510/ptzctrl.cgi?-step=0&-act=up` |
| **Move Down** | `GET /cgi-bin/hi3510/ptzctrl.cgi?-step=0&-act=down` |
| **Stop Motor** | `GET /cgi-bin/hi3510/ptzctrl.cgi?-step=0&-act=stop` |
| **Center / Home** | `GET /cgi-bin/hi3510/ptzctrl.cgi?-step=0&-act=home` |
| **Horizontal Patrol** | `GET /cgi-bin/hi3510/ptzctrl.cgi?-step=0&-act=hscan` |
| **Vertical Patrol** | `GET /cgi-bin/hi3510/ptzctrl.cgi?-step=0&-act=vscan` |

### 4.4 Live Video & Snapshots

* **Direct HTTP Snapshot (1280x720 JPEG):**
  ```bash
  curl -u admin:admin "http://<IP>/tmpfs/auto.jpg" -o snapshot.jpg
  ```
* **RTSP Streams:**
  * **Main Stream (1280x720 @ 25fps):** `rtsp://admin:admin@<IP>:554/11`
  * **Sub Stream (640x352 @ 25fps):** `rtsp://admin:admin@<IP>:554/12`
  * *Alternative standard URL:* `rtsp://admin:admin@<IP>:554/user=admin&password=admin&channel=1&stream=0.sdp?`

---

## 5. Web Interface Architecture

* `/` -> 302 redirects to `/web/admin.html`
* `/web/admin.html` -> Frameset wrapping `mainpage.html`
* `/web/config.html` -> Settings frameset:
  * `menu.html`: Sidebar navigation
  * `wifi.html`: Wireless network configuration
  * `video.html`: Video encoding bitrate, framerate, resolution
  * `hiplatform.html`: P2P cloud service configuration
  * `user.html`: User accounts and passwords
  * `deviceinfo.html`: Firmware version, hardware model, software build timestamp

---

## 6. Hardware UART Serial Console (Header `J3`)

The board provides an unpopulated 4-pin debug port at header **`J3`** (located near the Realtek Wi-Fi chip):

```text
  [ 1 ]    ( 2 )    ( 3 )    ( 4 )
┌──────┐
│  TX  │    GND      RX      VCC / NC
└──────┘
(Square)
```

### 6.1 Verified Pinout & Physical Identification

| Pin | Function | Visual Identification | Connect to Raspberry Pi |
| :--- | :--- | :--- | :--- |
| **Pin 1** | **`TX` (Camera Transmit)** | **Square pad with white silk box**; isolated copper trace | **Pin 10** (GPIO 15 / RXD) |
| **Pin 2** | **`GND` (Ground)** | Round pad with **4 thermal-relief spokes** tied to ground plane | **Pin 6** (GND) |
| **Pin 3** | **`RX` (Camera Receive)** | Round pad; isolated trace with circular clearance moat | **Pin 8** (GPIO 14 / TXD) |
| **Pin 4** | **`VCC (3.3V)` / NC** | Round pad | **DO NOT CONNECT** (Camera powers itself) |

* **Serial Parameters:** `115200 baud, 8 data bits, no parity, 1 stop bit (8N1)`
* **Logic Level:** **3.3V TTL** (Directly compatible with Raspberry Pi 3.3V GPIOs)

### 6.2 Raspberry Pi Hookup & Bootloader Break-in

1. **Configure Raspberry Pi Serial Port:**
   ```bash
   sudo raspi-config
   # Interface Options -> Serial Port:
   # Login shell accessible over serial? -> NO
   # Serial port hardware enabled?       -> YES
   sudo apt update && sudo apt install -y picocom
   ```

2. **Wiring (3 Wires Only):**
   * Camera `J3` Pin 2 (GND) $\rightarrow$ Raspberry Pi Pin 6 (GND)
   * Camera `J3` Pin 1 (TX)  $\rightarrow$ Raspberry Pi Pin 10 (GPIO 15 / RXD)
   * Camera `J3` Pin 3 (RX)  $\rightarrow$ Raspberry Pi Pin 8 (GPIO 14 / TXD)

3. **Open Terminal & Catch U-Boot:**
   ```bash
   picocom -b 115200 /dev/serial0
   ```
   * Power on the camera with its 5V adapter.
   * As soon as text starts streaming, mash `Space`, `Enter`, or `Ctrl+C` to halt autoboot.
   * Expected prompt: `GK7102 #`

---

## 7. Bespoke Rust Firmware Vision (`escamd`)

The long-term objective of this reverse-engineering effort is to replace the bloated, closed-source, 10-year-old vendor C stack with a high-performance, memory-safe, static Rust daemon.

### 7.1 Architecture & Components
* **Compilation Target:** `arm-unknown-linux-musleabi` (ARM1176JZF-S / ARMv6, static binary, zero glibc dependencies).
* **Footprint:** Single binary $< 5$ MB on SPI flash, $< 10$ MB runtime RAM usage (leaving 54+ MB RAM free).
* **Low-Latency Streaming:** Native WebRTC server ([`webrtc-rs`](https://github.com/webrtc-rs/webrtc)) reading hardware H.264 NAL units from `/dev/venc` for $< 100$ms browser playback with zero plugins.
* **Smooth PTZ Stepper Motor Control:** Tokio async motor task generating smooth sinusoidal acceleration/deceleration S-curves across the 8 `JULN2803AG` Darlington GPIO channels.
* **Hardened Web Server:** Embedded Axum / Actix-web service with `rustls` (TLS 1.3) and modern WebAssembly/Preact UI. Zero cloud telemetry.
* **Instant Boot:** Stripped Linux kernel + Rust `init` booting in $< 2$ seconds.

---

## 8. Related Documentation

* **[`ASTRO_SPECS.md`](./ASTRO_SPECS.md):** Detailed optical calculations, sensor geometry (SC1135 vs GC1034), telescope M12-to-1.25" adaptation, and astrophotography / planetary imaging use cases.
* **[`escam.py`](./escam.py):** Python CLI utility for PTZ motor driving, snapshots, and RTSP stream testing over LAN.
