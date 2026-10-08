# How-To: Serial UART Console Rescue and Configuration Recovery

This guide describes how to connect to the physical serial UART console on the ESCAM G02 (Goke GK7102C) to recover userspace boot scripts or debug early kernel initialization without touching SPI flash chips.

---

## 1. Problem Overview & Scope

During driver modernization and persistent boot experimentation, modifications to `/mnt/mtd/ipc/conf/run` (located on the read-write JFFS2 partition `mtd5`) may stall userspace execution before network daemons (`loadNetwork`, `dhcp.sh`, `telnetd`) start.

Because the bootloader (`mtd0` U-Boot), kernel (`mtd1`), and rootfs (`mtd2`) remain 100% stock and uncorrupted, hardware recovery is achieved in seconds via the onboard 3.3V UART header.

---

## 2. Hardware Requirements

- **3.3V USB-to-UART Serial Adapter** (FTDI FT232R, Silicon Labs CP2102, CH340G, or Raspberry Pi GPIO).
- **3 Female-to-Male or Female-to-Female Jumper Wires**.
- Terminal emulator (`screen`, `minicom`, or `picocom`).

> [!CAUTION]
> The SoC logic levels are **3.3V TTL**. Do **NOT** connect a 5V or RS-232 (+/-12V) serial cable directly to the board, as this will damage the SoC I/O pads.

---

## 3. PCB UART Header Pinout

On the main ESCAM G02 board (near the GK7102C SoC), find the 3-pin unpopulated header / test pads:

| Pin Label | Function | Adapter Connection |
|:---|:---|:---|
| **GND** | Ground | Connect to USB-UART **GND** |
| **TX** | SoC Transmit (Output) | Connect to USB-UART **RX** |
| **RX** | SoC Receive (Input) | Connect to USB-UART **TX** |

*(Note: Baud rate: **115200**, Data bits: **8**, Parity: **None**, Stop bits: **1**, Flow control: **None**)*.

---

## 4. Connecting and Launching Serial Terminal

On the host Linux workstation:

```bash
picocom -b 115200 /dev/ttyUSB0
```
*(Or `screen /dev/ttyUSB0 115200`)*.

Power cycle the camera (connect 5V micro-USB). You will immediately see the U-Boot bootloader banner:

```text
U-Boot 2014.07 (Aug 13 2017 - 14:02:11)
CPU: GK7102C
DRAM: 64 MiB
Hit any key to stop autoboot:  1
```

---

## 5. Restoration Procedure

### Option A: From Linux Root Shell (Recommended)

Allow Linux to continue booting. When the kernel completes init, press `<Enter>` to receive the BusyBox ash shell prompt:

```text
Please press Enter to activate this console.
/ #
```

Run the following commands to restore the verified factory-backed startup script:

```sh
# 1. Restore verified backup
cp /mnt/mtd/ipc/conf/run.bak /mnt/mtd/ipc/conf/run

# 2. Ensure execution permissions
chmod +x /mnt/mtd/ipc/conf/run

# 3. Flush buffers to JFFS2 flash
sync

# 4. Reboot
reboot
```

Upon reboot, `run` will execute the clean factory script, load vendor drivers, associate with Wi-Fi, and bring `10.75.2.93:2323` and `escamd` back online.

---

### Option B: From U-Boot Console (If Linux Hangs)

If the Linux kernel itself hangs or panics before spawning a shell:

1. Press any key during the 1-second countdown to stop autoboot:
   ```text
   Hit any key to stop autoboot: 0
   gk7102c #
   ```
2. You can boot a rescue kernel and initrd over TFTP or inspect flash:
   ```text
   printenv
   help
   ```

---

## 6. Diagnostic Takeaways & Root Cause

1. **Failure Mode**: In `loadmotor()`, checking `if [ -f /mnt/mtd/ipc/conf/escam_motor.ko ]` rather than checking whether `insmod` returned exit code 0 prevented automatic fallback to vendor `motor.ko` when `escam_motor.ko` encountered a conflict with GPIO 1 (which was pre-exported by `reset_sensor.sh`).
2. **Safety Rule**: Future boot modifications must use unconditional exit-code traps:
   ```sh
   insmod /mnt/mtd/ipc/conf/escam_motor.ko || insmod $TARGET/modules/motor.ko ...
   ```
