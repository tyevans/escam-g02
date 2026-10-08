# How-to: Source and Build Open-Source Wi-Fi and Sensor Drivers

This guide covers building and loading community open-source replacements for proprietary vendor modules `8188fu.ko` (Realtek RTL8188FTV Wi-Fi) and `gc1034_ex.ko` (GalaxyCore GC1034 sensor) on the ESCAM G02.

---

## 1. Realtek RTL8188FTV Wi-Fi Driver (`rtl8188fu`)

The internal Wi-Fi adapter is connected to the Goke GK7102C internal USB 2.0 host controller:
* **USB Vendor ID:** `0x0bda` (Realtek Semiconductor)
* **USB Product ID:** `0xf179` (RTL8188FTV 802.11b/g/n)

### Cross-Compilation Steps
Run the automated build script [`scripts/build_rtl8188fu.sh`](../../scripts/build_rtl8188fu.sh):

```bash
export KDIR=/path/to/linux-kernel-build
export CROSS_COMPILE=arm-linux-musleabi-
export ARCH=arm

./scripts/build_rtl8188fu.sh
```

### Loading the Driver
1. Copy `8188fu.ko` to the device:
   ```bash
   scp dist/8188fu.ko root@10.75.2.93:/mnt/mtd/ipc/modules/8188fu.ko
   ```
2. Load module:
   ```bash
   insmod /mnt/mtd/ipc/modules/8188fu.ko
   ```
3. Verify interface enumeration:
   ```bash
   ifconfig wlan0 up
   iwlist wlan0 scan
   ```

---

## 2. GalaxyCore GC1034 Image Sensor Driver

The camera uses a GalaxyCore GC1034 720p CMOS sensor connected via I2C (`0x21`) and DVP parallel bus.

The open-source Linux V4L2 subdevice driver is located at [`drivers/gc1034_sensor.c`](../../drivers/gc1034_sensor.c):

### Build and Install
```bash
cd drivers
make
scp gc1034_sensor.ko root@10.75.2.93:/mnt/mtd/ipc/modules/gc1034_sensor.ko
```

### Register Mapping Parity
The open-source driver provides 100% parity with our Rust driver [`crates/escam-driver/src/sensor.rs`](../../crates/escam-driver/src/sensor.rs):

* **Chip ID Check:** Registers `0xF0` (`0x10`) and `0xF1` (`0x34`)
* **Exposure Duration:** Registers `0x03` (High [13:8]) and `0x04` (Low [7:0])
* **Analog PGA Gain:** Register `0xB6`
