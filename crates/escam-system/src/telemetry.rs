//! # System Telemetry and Memory Footprint Auditor
//!
//! Inspects Linux /proc memory statistics and verifies that total runtime RAM
//! consumption remains strictly below the 8 MB (8,192 kB) invariant.

use std::fs;
use std::time::Instant;

pub const MAX_ALLOWED_RSS_KB: u64 = 8192; // 8 MB Hard limit

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessMemory {
    pub vmrss_kb: u64,
    pub vmsize_kb: u64,
}

pub struct TelemetryAuditor {
    start_time: Instant,
}

impl TelemetryAuditor {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Reads resident set size (VmRSS) from /proc/self/status.
    pub fn read_current_memory() -> Result<ProcessMemory, std::io::Error> {
        let status = fs::read_to_string("/proc/self/status")?;
        let mut vmrss = 0;
        let mut vmsize = 0;

        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    vmrss = parts[1].parse().unwrap_or(0);
                }
            } else if line.starts_with("VmSize:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    vmsize = parts[1].parse().unwrap_or(0);
                }
            }
        }

        Ok(ProcessMemory {
            vmrss_kb: vmrss,
            vmsize_kb: vmsize,
        })
    }

    /// Validates that RSS complies with the <8MB system constraint.
    pub fn verify_rss_limit(memory: &ProcessMemory) -> bool {
        memory.vmrss_kb <= MAX_ALLOWED_RSS_KB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_proc_memory() {
        if let Ok(mem) = TelemetryAuditor::read_current_memory() {
            // Memory read succeeds on Linux
            assert!(mem.vmrss_kb > 0);
        }
    }

    #[test]
    fn test_rss_limit_validation() {
        let good_mem = ProcessMemory {
            vmrss_kb: 4096, // 4 MB
            vmsize_kb: 16384,
        };
        assert!(TelemetryAuditor::verify_rss_limit(&good_mem));

        let bad_mem = ProcessMemory {
            vmrss_kb: 10240, // 10 MB > 8 MB
            vmsize_kb: 32768,
        };
        assert!(!TelemetryAuditor::verify_rss_limit(&bad_mem));
    }
}
