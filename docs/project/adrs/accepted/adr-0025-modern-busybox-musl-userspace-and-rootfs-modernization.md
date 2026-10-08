---
id: '0025'
title: Modern Musl-Linked BusyBox Userspace and RootFS Modernization
status: Accepted
date: 2026-10-07
deciders:
  - Elena
  - Alex
---

# ADR-0025: Modern Musl-Linked BusyBox Userspace and RootFS Modernization

## Status
Accepted

## Context
The stock ESCAM G02 rootfs (`mtd3_rootfs.bin`) contains an outdated BusyBox v1.18.1 compiled in 2010 dynamically linked against `uClibc-0.9.33.2`. This ancient userland lacks modern POSIX tool flags, modern shell features, and security patches, and ties system utilities to obsolete C library symbols.

A common assumption is that updating BusyBox requires first replacing the proprietary vendor kernel modules (`media.ko`, `gkio.ko`, etc.). However, Linux userspace utilities interact with the kernel exclusively through the standard POSIX system call interface (such as `init_module`, `ioctl`, `fork`, `execve`). The syscall ABI on Linux 3.4.43 is fully forward-compatible with modern BusyBox releases.

## Decision
We decouple userland modernization from kernel module replacement:
1. **Static Musl Toolchain**: Cross-compile modern BusyBox (1.36+) statically using `arm-unknown-linux-musleabihf` / `arm-linux-musleabi` (ARMv6, ARM1176JZF-S).
2. **Immediate Deployment**: Deploy modern BusyBox directly onto the existing Linux 3.4.43-Goke kernel.
3. **Dual Deployment Paths**:
   - *Non-destructive overlay*: Place the static BusyBox binary in `/mnt/mtd/ipc/conf/bin/busybox` or symlink into `/bin` during clean init.
   - *SquashFS RootFS Rebuild*: Generate a refreshed `mtd3_rootfs.bin` containing only modern BusyBox, basic `/etc` configs, and standard symlinks, reclaiming ~1MB of flash.
4. **Applet Selection**: Include essential system applets (`ash`, `ls`, `ps`, `cat`, `grep`, `awk`, `sed`, `tar`, `gzip`, `insmod`, `rmmod`, `lsmod`, `netstat`, `ifconfig`, `route`, `udhcpc`, `telnetd`) while excluding bloated/unnecessary networking daemons replaced by `escamd`.

## Consequences
### Positive
- Modern, secure, bug-fixed shell and POSIX utility environment.
- Zero dynamic linking dependencies or uClibc version locks.
- Can be tested, verified, and shipped immediately without waiting for kernel driver reverse-engineering.

### Negative
- Static linking adds slightly larger binary footprint per executable (~800KB vs ~400KB dynamically linked), but easily accommodated within the 1.9MB `mtd3` partition.
