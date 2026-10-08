# How-to: Modernize BusyBox and Rebuild RootFS on the ESCAM G02

This guide demonstrates how to build and deploy a modern static BusyBox 1.36+ userland on the ESCAM G02 (Goke GK7102C), replacing the obsolete BusyBox v1.18.1 and uClibc stack without touching the underlying vendor kernel modules.

---

## 1. Overview & Architecture

The factory firmware packages BusyBox 1.18.1 (from 2010) inside `mtd3_rootfs.bin` (SquashFS 4.0 XZ). This userland is dynamically linked against `uClibc-0.9.33.2`.

Per **ADR-0025**, userland tools interact with the Linux kernel strictly via system calls (`init_module`, `ioctl`, `fork`, etc.). Modern BusyBox compiled statically using `musl-libc` runs cleanly on the existing **Linux 3.4.43-Goke** kernel, providing:
* Modern POSIX shell features (`ash` with job control and command history)
* Modern `insmod` / `rmmod` applets supporting compressed modules
* Bug-fixed `ifconfig`, `udhcpc`, and network debugging utilities
* Zero runtime dependency on legacy `uClibc` shared libraries

---

## 2. Cross-Compiling Static BusyBox for ARMv6

Use the provided build recipe [`scripts/build_busybox.sh`](../../scripts/build_busybox.sh):

```bash
# Set your target musl cross-compiler
export CROSS_COMPILE=arm-linux-musleabi-
export BUSYBOX_VERSION=1.36.1

# Execute cross-compilation
./scripts/build_busybox.sh
```

### Key Configuration Directives (`.config`)
* `CONFIG_STATIC=y`: Build standalone binary with zero `.so` dependencies.
* `CONFIG_CROSS_COMPILER_PREFIX="arm-linux-musleabi-"`: Target ARM1176JZF-S (ARMv6).
* `CONFIG_FEATURE_SH_STANDALONE=y`: Applets are invoked directly within `ash` subshells.

The uncompressed static binary is approximately **900 KB - 1.1 MB**, well within memory constraints.

---

## 3. Deployment Method A: Non-Destructive Overlay (Recommended for Testing)

To test modern BusyBox without reflashing the SPI NOR flash:

1. Upload `busybox-armv6-musl` to the persistent writable partition:
   ```bash
   scp busybox-armv6-musl root@10.75.2.93:/mnt/mtd/ipc/conf/bin/busybox
   ```
2. Symlink core applets into `/mnt/mtd/ipc/conf/bin`:
   ```bash
   cd /mnt/mtd/ipc/conf/bin
   ./busybox --install -s .
   ```
3. Update `PATH` in `/mnt/mtd/ipc/conf/run`:
   ```sh
   export PATH="/mnt/mtd/ipc/conf/bin:$PATH"
   ```
4. Confirm version:
   ```bash
   busybox | head -n 1
   # BusyBox v1.36.1 (...) multi-call binary.
   ```

---

## 4. Deployment Method B: SquashFS RootFS Replacement (`mtd3`)

To permanently flash the modernized root filesystem into the 1.9MB `mtd3` flash partition:

1. Assemble the filesystem tree using [`scripts/rebuild_rootfs.sh`](../../scripts/rebuild_rootfs.sh):
   ```bash
   ./scripts/rebuild_rootfs.sh
   ```
2. Verify image sizing:
   * Target partition: `0x190000 - 0x360000` (1,992,294 bytes).
   * Resulting image size: `~1.1 MB` ($< 1.5$ MB target, $> 800$ KB headroom).
3. Flash via U-Boot or Linux `flashcp`:
   ```bash
   flashcp -v dist/mtd3_rootfs.bin /dev/mtd3
   reboot
   ```

---

## 5. Verification Checklist

- [x] `busybox` reports version $\ge 1.36$.
- [x] Kernel modules (`motor.ko`, `gkio.ko`, `media.ko`) load via `insmod`.
- [x] Device nodes in `/dev` populated cleanly via `mdev -s`.
- [x] `escamd` starts deterministically under 8MB RAM.
