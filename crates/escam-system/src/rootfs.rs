//! RootFS Modernization and Static Musl BusyBox Verifier.
//!
//! Governed by ADR-0025 and TASK-0039.
//! Validates modern BusyBox applet requirements, filesystem skeleton, and flash budget
//! for replacing the ancient 2010 uClibc BusyBox v1.18.1 on the ESCAM G02.

use serde::{Deserialize, Serialize};

/// Maximum partition size for mtd3 (RootFS): 1.9 MB (1,992,294 bytes).
pub const ROOTFS_PARTITION_LIMIT_BYTES: u64 = 1_992_294;

/// Target maximum compressed size for modern SquashFS image (leaves >400KB flash margin).
pub const ROOTFS_TARGET_MAX_BYTES: u64 = 1_572_864; // 1.5 MB

/// Minimum core applets required for ESCAM G02 clean-boot and networking.
pub const REQUIRED_APPLETS: &[&str] = &[
    "ash", "sh", "ls", "ps", "cat", "grep", "awk", "sed",
    "insmod", "rmmod", "lsmod", "ifconfig", "udhcpc", "telnetd",
    "mkdir", "mount", "umount", "mknod", "kill", "killall",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusyboxMetadata {
    pub version: String,
    pub is_static: bool,
    pub target_arch: String,
    pub applets: Vec<String>,
}

pub struct RootfsVerifier;

impl RootfsVerifier {
    /// Validates that all required core applets are provided by the BusyBox binary.
    pub fn verify_applets(available: &[String]) -> Result<(), Vec<String>> {
        let missing: Vec<String> = REQUIRED_APPLETS
            .iter()
            .filter(|req| !available.iter().any(|a| a == *req))
            .map(|s| s.to_string())
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }

    /// Verifies that the compressed rootfs fits within the 1.9MB partition limit
    /// and returns the remaining free headroom in bytes.
    pub fn verify_flash_budget(squashfs_bytes: u64) -> Result<u64, String> {
        if squashfs_bytes > ROOTFS_PARTITION_LIMIT_BYTES {
            return Err(format!(
                "SquashFS image size ({} bytes) exceeds mtd3 partition capacity ({} bytes)",
                squashfs_bytes, ROOTFS_PARTITION_LIMIT_BYTES
            ));
        }
        let headroom = ROOTFS_PARTITION_LIMIT_BYTES - squashfs_bytes;
        Ok(headroom)
    }

    /// Generates standard minimal `/etc/inittab` for clean BusyBox init.
    pub fn generate_inittab() -> &'static str {
        r#"::sysinit:/etc/init.d/rcS
::respawn:/bin/cttyhack /bin/sh
::ctrlaltdel:/sbin/reboot
::shutdown:/etc/init.d/rcK
::shutdown:/bin/umount -a -r
"#
    }

    /// Generates modern `/etc/mdev.conf` for hotplug device nodes.
    pub fn generate_mdev_conf() -> &'static str {
        r#"# Support dynamic device node population
null 0:0 666
zero 0:0 666
urandom 0:0 444
console 0:0 600
tty 0:0 666

# Camera hardware device nodes
gkio 0:0 660
motor 0:0 660
gk_video 0:0 660
watchdog 0:0 660
"#
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_applet_verification_success() {
        let available: Vec<String> = REQUIRED_APPLETS.iter().map(|s| s.to_string()).collect();
        assert!(RootfsVerifier::verify_applets(&available).is_ok());
    }

    #[test]
    fn test_applet_verification_missing_fails() {
        let available = vec!["ls".to_string(), "ps".to_string()];
        let res = RootfsVerifier::verify_applets(&available);
        assert!(res.is_err());
        let missing = res.unwrap_err();
        assert!(missing.contains(&"ash".to_string()));
        assert!(missing.contains(&"insmod".to_string()));
    }

    #[test]
    fn test_flash_budget_check() {
        let small_image = 1_200_000u64; // 1.2 MB
        let headroom = RootfsVerifier::verify_flash_budget(small_image).unwrap();
        assert!(headroom > 700_000);

        let oversized_image = 2_500_000u64; // 2.5 MB
        assert!(RootfsVerifier::verify_flash_budget(oversized_image).is_err());
    }
}
