//! # Vendor Software Ejection & Startup Init Script Generator
//!
//! Generates the cleanup commands and persistent run script to replace
//! the vendor C binaries with `escamd` on /mnt/mtd/ipc/conf/run.

pub struct EjectionManager;

impl EjectionManager {
    /// Generates shell command to safely kill vendor processes without kernel panic.
    pub fn generate_kill_command() -> &'static str {
        "killall -9 ipc_server chksock net_detect watchdog 2>/dev/null || true"
    }

    /// Generates the persistent run script for /mnt/mtd/ipc/conf/run.
    pub fn generate_run_script(binary_path: &str) -> String {
        format!(
            r#"#!/bin/sh
# ESCAM G02 - Pure Rust Firmware Startup Script
# Replaces vendor ipc_server with open-source escamd

echo "[ESCAM] Ejecting stock vendor software..."
{}

echo "[ESCAM] Launching pure Rust daemon..."
exec {}
"#,
            Self::generate_kill_command(),
            binary_path
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ejection_script_contains_kill_targets() {
        let script = EjectionManager::generate_run_script("/mnt/mtd/ipc/escamd");
        assert!(script.contains("ipc_server"));
        assert!(script.contains("chksock"));
        assert!(script.contains("net_detect"));
        assert!(script.contains("/mnt/mtd/ipc/escamd"));
    }
}
