//! Goke GK7102C Hardware Video Processing Unit (VPU) & MMZ Abstractions.
//!
//! Governed by ADR-0020, ADR-0026, and TASK-0042.
//! Maps out the reverse-engineered `/dev/gk_video` ioctl commands,
//! Media Memory Zone (MMZ) DMA buffer allocation, and VPU stream pump contracts.

use serde::{Deserialize, Serialize};

/// Base physical address of MMZ (Media Memory Zone): 24 MB reserved.
pub const MMZ_PHYSICAL_BASE: u32 = 0x0000_0000;
pub const MMZ_PHYSICAL_SIZE: u32 = 0x0180_0000; // 24 MB
pub const SYSTEM_RAM_BASE: u32 = 0x0180_0000;
pub const SYSTEM_RAM_SIZE: u32 = 0x0280_0000;   // 40 MB (Total 64 MB SiP)

/// Reverse-engineered /dev/gk_video ioctl command definitions (magic 'v' = 0x76)
pub const GK_ENC_IOC_GET_STREAM: u32 = 0x8004_6537;
pub const GK_VI_IOC_ENABLE: u32     = 0x8004_7670;
pub const GK_MMZ_IOC_MAP: u32       = 0x8004_6D00;
pub const GK_ISP_IOC_INIT: u32      = 0x8004_6901;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GkViConfig {
    pub width: u32,
    pub height: u32,
    pub framerate: u32,
    pub raw_bayer_format: u32, // 10-bit RGGB = 0x01
}

impl Default for GkViConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            framerate: 25,
            raw_bayer_format: 0x01,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GkEncStreamHeader {
    pub nal_type: u8,
    pub payload_size: u32,
    pub timestamp_us: u64,
    pub is_keyframe: bool,
}

#[derive(Debug, Clone)]
pub struct VpuStreamPump {
    pub is_initialized: bool,
    pub current_config: GkViConfig,
    pub total_frames_pumped: u64,
}

impl Default for VpuStreamPump {
    fn default() -> Self {
        Self::new()
    }
}

impl VpuStreamPump {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            current_config: GkViConfig::default(),
            total_frames_pumped: 0,
        }
    }

    /// Initializes VI, ISP, and VENC subsystems with target resolution.
    pub fn initialize(&mut self, config: GkViConfig) -> Result<(), String> {
        if config.width == 0 || config.height == 0 {
            return Err("Invalid resolution dimensions".to_string());
        }
        self.current_config = config;
        self.is_initialized = true;
        Ok(())
    }

    /// Simulates polling the next frame from the VPU bitstream ring buffer.
    pub fn poll_next_nalu(&mut self) -> Result<GkEncStreamHeader, String> {
        if !self.is_initialized {
            return Err("VPU hardware pipeline not started".to_string());
        }
        self.total_frames_pumped += 1;
        let is_key = (self.total_frames_pumped % 50) == 1;
        Ok(GkEncStreamHeader {
            nal_type: if is_key { 5 } else { 1 },
            payload_size: if is_key { 32768 } else { 4096 },
            timestamp_us: self.total_frames_pumped * 40000, // 25 fps = 40ms
            is_keyframe: is_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mmz_memory_split_bounds() {
        assert_eq!(MMZ_PHYSICAL_BASE + MMZ_PHYSICAL_SIZE, SYSTEM_RAM_BASE);
        assert_eq!(SYSTEM_RAM_BASE + SYSTEM_RAM_SIZE, 0x0400_0000); // 64 MB Total
    }

    #[test]
    fn test_vpu_stream_pump_lifecycle() {
        let mut pump = VpuStreamPump::new();
        assert!(pump.poll_next_nalu().is_err());

        assert!(pump.initialize(GkViConfig::default()).is_ok());
        let frame1 = pump.poll_next_nalu().unwrap();
        assert!(frame1.is_keyframe);
        assert_eq!(frame1.nal_type, 5);

        let frame2 = pump.poll_next_nalu().unwrap();
        assert!(!frame2.is_keyframe);
        assert_eq!(frame2.nal_type, 1);
    }
}
