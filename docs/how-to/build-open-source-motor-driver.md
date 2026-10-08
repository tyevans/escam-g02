# How-to: Build and Deploy the Open-Source Stepper Motor & GPIO Driver

This guide explains how to compile and load the pure open-source C kernel module [`drivers/escam_motor.c`](../../drivers/escam_motor.c) to replace proprietary vendor modules `motor.ko` and `gkio.ko` on the ESCAM G02.

---

## 1. Overview

The vendor modules `motor.ko` and `gkio.ko` are closed binary blobs with `vermagic: 3.4.43-Goke`. By providing our own open-source driver matching the same character device contracts:
* `/dev/motor` (`MOTOR_IOCTL_RUN: 0xC004_6D01`, `MOTOR_IOCTL_STOP: 0xC004_6D00`)
* `/dev/gkio` (`GKIO_IOCTL_SET_VALUE: 0xC004_6200`, `GKIO_IOCTL_GET_VALUE: 0xC004_6201`)

We maintain 100% binary interface compatibility with `crates/escam-driver` and the `escamd` daemon, while gaining the ability to recompile for newer Linux kernels (4.9, 5.4, 6.x).

---

## 2. Cross-Compilation

From the repository root:

```bash
# Set your target kernel tree build directory
export KDIR=/path/to/linux-kernel-build
export CROSS_COMPILE=arm-linux-musleabi-
export ARCH=arm

# Build the module
cd drivers
make
```

The build produces `escam_motor.ko`.

---

## 3. Deployment & Loading

1. Transfer `escam_motor.ko` to the camera:
   ```bash
   scp drivers/escam_motor.ko root@10.75.2.93:/mnt/mtd/ipc/modules/escam_motor.ko
   ```
2. Unload legacy vendor modules:
   ```bash
   rmmod motor 2>/dev/null || true
   rmmod gkio 2>/dev/null || true
   ```
3. Load the open-source replacement:
   ```bash
   insmod /mnt/mtd/ipc/modules/escam_motor.ko
   ```
4. Verify device nodes:
   ```bash
   ls -la /dev/motor /dev/gkio
   ```

---

## 4. Alternative: Userspace Direct-GPIO Stepping

If running a kernel without module loading support, `crates/escam-driver` also provides [`UserspaceMotorController`](../../crates/escam-driver/src/userspace_motor.rs). It drives the GPIO lines directly via standard Linux `gpio-sysfs` or `libgpiod`, providing zero-kernel-module operation.
