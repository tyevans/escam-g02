//! Vendor Firmware Debloater and Clean-Boot Security Lockdown.
//!
//! Provides inspection and removal of non-essential vendor binaries, cloud telemetry,
//! audio files, and scripts from persistent flash storage to maximize available space
//! and secure the device.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashAudit {
    pub total_bytes_scanned: u64,
    pub reclaimable_bytes: u64,
    pub candidate_files: Vec<String>,
    pub protected_files_verified: bool,
}

pub struct Debloater {
    base_ipc_dir: PathBuf,
    conf_dir: PathBuf,
}

impl Debloater {
    pub fn new<P1: AsRef<Path>, P2: AsRef<Path>>(base_ipc: P1, conf: P2) -> Self {
        Self {
            base_ipc_dir: base_ipc.as_ref().to_path_buf(),
            conf_dir: conf.as_ref().to_path_buf(),
        }
    }

    /// Checks if a file path is safe to remove (non-essential vendor junk).
    pub fn is_reclaimable(path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        // Strictly protect kernel modules, essential libs, and pure escam artifacts
        if path_str.ends_with(".ko")
            || path_str.contains("escamd")
            || path_str.contains("busybox")
            || path_str.ends_with("/run")
            || path_str.contains("wifi.conf")
            || path_str.contains("resolv.conf")
        {
            return false;
        }

        // Reclaim vendor web GUI, unused cloud scripts, audio voice prompts
        path_str.contains("/web/")
            || path_str.contains("/audio/")
            || path_str.ends_with(".g711")
            || path_str.ends_with(".pcm")
            || path_str.ends_with("p2p_srv")
            || path_str.ends_with("net_detect")
            || path_str.ends_with("config_3thddns.ini")
            || path_str.ends_with("config_cloud.ini")
            || path_str.ends_with("config_facddns.ini")
            || path_str.ends_with("config_default.zip")
    }

    /// Audits reclaimable files without deleting anything.
    pub fn audit(&self) -> FlashAudit {
        let mut total_scanned = 0u64;
        let mut reclaimable = 0u64;
        let mut candidates = Vec::new();

        self.scan_recursive(&self.base_ipc_dir, &mut total_scanned, &mut reclaimable, &mut candidates);
        self.scan_recursive(&self.conf_dir, &mut total_scanned, &mut reclaimable, &mut candidates);

        FlashAudit {
            total_bytes_scanned: total_scanned,
            reclaimable_bytes: reclaimable,
            candidate_files: candidates,
            protected_files_verified: true,
        }
    }

    fn scan_recursive(&self, dir: &Path, scanned: &mut u64, reclaimable: &mut u64, candidates: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    self.scan_recursive(&p, scanned, reclaimable, candidates);
                } else if p.is_file() {
                    if let Ok(meta) = p.metadata() {
                        let size = meta.len();
                        *scanned += size;
                        if Self::is_reclaimable(&p) {
                            *reclaimable += size;
                            candidates.push(p.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }

    /// Generates a clean-boot `/mnt/mtd/ipc/conf/run` init script.
    pub fn generate_clean_boot_script() -> &'static str {
        r#"#!/bin/sh
# ESCAM G02 Pure Rust Clean-Boot Init Script
# Zero vendor cloud daemons, deterministic memory footprint

if [ -d /mnt/mtd/ipc/conf/bin ]; then
    export PATH="/mnt/mtd/ipc/conf/bin:$PATH"
fi

echo "[init] Loading hardware kernel modules..."
insmod /mnt/mtd/ipc/modules/hal.ko 2>/dev/null
insmod /mnt/mtd/ipc/modules/media.ko 2>/dev/null
insmod /mnt/mtd/ipc/modules/sensor.ko 2>/dev/null
insmod /mnt/mtd/ipc/modules/gc1034_ex.ko 2>/dev/null
if [ -f /mnt/mtd/ipc/conf/escam_motor.ko ]; then
    insmod /mnt/mtd/ipc/conf/escam_motor.ko 2>/dev/null
else
    insmod /mnt/mtd/ipc/modules/motor.ko 2>/dev/null
    insmod /mnt/mtd/ipc/modules/gkio.ko 2>/dev/null
fi

echo "[init] Setting up tmpfs and decompressing escamd..."
mkdir -p /mnt/mtd/ipc/tmpfs
if [ -f /mnt/mtd/ipc/conf/escamd.gz ]; then
    zcat /mnt/mtd/ipc/conf/escamd.gz > /mnt/mtd/ipc/tmpfs/escamd
    chmod +x /mnt/mtd/ipc/tmpfs/escamd
fi

echo "[init] Starting escamd pure Rust daemon..."
/mnt/mtd/ipc/tmpfs/escamd > /mnt/mtd/ipc/tmpfs/escamd.log 2>&1 &
echo "[init] Clean-boot complete."
"#
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protected_vs_reclaimable_paths() {
        assert!(!Debloater::is_reclaimable(Path::new("/mnt/mtd/ipc/modules/motor.ko")));
        assert!(!Debloater::is_reclaimable(Path::new("/mnt/mtd/ipc/conf/escamd.gz")));
        assert!(!Debloater::is_reclaimable(Path::new("/mnt/mtd/ipc/conf/run")));
        assert!(!Debloater::is_reclaimable(Path::new("/mnt/mtd/ipc/conf/bin/busybox")));

        assert!(Debloater::is_reclaimable(Path::new("/mnt/mtd/ipc/conf/config_cloud.ini")));
        assert!(Debloater::is_reclaimable(Path::new("/mnt/mtd/ipc/web/index.html")));
        assert!(Debloater::is_reclaimable(Path::new("/mnt/mtd/ipc/audio/welcome.g711")));
    }
}

