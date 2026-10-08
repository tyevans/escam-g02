//! Direct Sensor I2C Register Control and Fine Hardware Integration Timing.
//!
//! Provides register-level access to the CMOS image sensor (GalaxyCore GC1034 / SC1135)
//! via /dev/i2c-0 or sensor character device, enabling precise analog gain and row exposure.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const GC1034_CHIP_ID: u16 = 0x1034;
pub const SC1135_CHIP_ID: u16 = 0x1135;

// GC1034 Register Map
pub const GC1034_REG_CHIP_ID_H: u8 = 0xf0;
pub const GC1034_REG_CHIP_ID_L: u8 = 0xf1;
pub const GC1034_REG_EXP_H: u8 = 0x03;     // Exposure lines [13:8]
pub const GC1034_REG_EXP_L: u8 = 0x04;     // Exposure lines [7:0]
pub const GC1034_REG_ANALOG_GAIN: u8 = 0xb6; // Analog PGA Gain
pub const GC1034_REG_DIGITAL_GAIN_H: u8 = 0xb1;
pub const GC1034_REG_DIGITAL_GAIN_L: u8 = 0xb2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorRegisters {
    pub chip_id: u16,
    pub model: String,
    pub exposure_lines: u32,
    pub analog_gain_pga: u8,
    pub raw_registers: HashMap<String, u8>,
}

pub trait SensorBus: Send + Sync {
    fn read_reg(&mut self, reg: u8) -> Result<u8, String>;
    fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), String>;
}

/// In-memory mock sensor bus for unit tests and host simulation.
#[derive(Debug, Default)]
pub struct MockSensorBus {
    pub registers: HashMap<u8, u8>,
}

impl MockSensorBus {
    pub fn new_gc1034() -> Self {
        let mut registers = HashMap::new();
        registers.insert(GC1034_REG_CHIP_ID_H, 0x10);
        registers.insert(GC1034_REG_CHIP_ID_L, 0x34);
        registers.insert(GC1034_REG_EXP_H, 0x01);
        registers.insert(GC1034_REG_EXP_L, 0x90); // 400 lines
        registers.insert(GC1034_REG_ANALOG_GAIN, 0x00); // 1.0x
        Self { registers }
    }
}

impl SensorBus for MockSensorBus {
    fn read_reg(&mut self, reg: u8) -> Result<u8, String> {
        Ok(*self.registers.get(&reg).unwrap_or(&0))
    }

    fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), String> {
        self.registers.insert(reg, val);
        Ok(())
    }
}

pub struct SensorI2cDriver<B: SensorBus> {
    bus: B,
    chip_id: u16,
}

impl<B: SensorBus> SensorI2cDriver<B> {
    pub fn new(mut bus: B) -> Self {
        let id_h = bus.read_reg(GC1034_REG_CHIP_ID_H).unwrap_or(0x10);
        let id_l = bus.read_reg(GC1034_REG_CHIP_ID_L).unwrap_or(0x34);
        let chip_id = ((id_h as u16) << 8) | (id_l as u16);

        Self { bus, chip_id }
    }

    pub fn chip_id(&self) -> u16 {
        self.chip_id
    }

    pub fn model_name(&self) -> &'static str {
        match self.chip_id {
            GC1034_CHIP_ID => "GalaxyCore GC1034",
            SC1135_CHIP_ID => "SmartSens SC1135",
            _ => "Generic CMOS Sensor",
        }
    }

    /// Sets exposure integration length in sensor row lines.
    pub fn set_exposure_lines(&mut self, lines: u32) -> Result<(), String> {
        let h = ((lines >> 8) & 0x3F) as u8;
        let l = (lines & 0xFF) as u8;
        self.bus.write_reg(GC1034_REG_EXP_H, h)?;
        self.bus.write_reg(GC1034_REG_EXP_L, l)?;
        Ok(())
    }

    /// Sets analog programmable gain (PGA).
    pub fn set_analog_gain(&mut self, gain_multiplier: f32) -> Result<(), String> {
        // GC1034 PGA table mapping
        let reg_val = if gain_multiplier <= 1.0 {
            0x00 // 1x
        } else if gain_multiplier <= 1.5 {
            0x01 // 1.45x
        } else if gain_multiplier <= 2.0 {
            0x02 // 2.0x
        } else if gain_multiplier <= 3.0 {
            0x03 // 2.8x
        } else if gain_multiplier <= 4.0 {
            0x04 // 4.0x
        } else if gain_multiplier <= 6.0 {
            0x05 // 5.6x
        } else {
            0x06 // 8.0x max PGA
        };

        self.bus.write_reg(GC1034_REG_ANALOG_GAIN, reg_val)
    }

    /// Reads back current sensor register state.
    pub fn read_status(&mut self) -> Result<SensorRegisters, String> {
        let exp_h = self.bus.read_reg(GC1034_REG_EXP_H)? as u32;
        let exp_l = self.bus.read_reg(GC1034_REG_EXP_L)? as u32;
        let exp_lines = (exp_h << 8) | exp_l;

        let gain = self.bus.read_reg(GC1034_REG_ANALOG_GAIN)?;

        let mut raw = HashMap::new();
        raw.insert("0x03".to_string(), exp_h as u8);
        raw.insert("0x04".to_string(), exp_l as u8);
        raw.insert("0xb6".to_string(), gain);

        Ok(SensorRegisters {
            chip_id: self.chip_id,
            model: self.model_name().to_string(),
            exposure_lines: exp_lines,
            analog_gain_pga: gain,
            raw_registers: raw,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc1034_mock_sensor_exposure_and_gain() {
        let mock_bus = MockSensorBus::new_gc1034();
        let mut driver = SensorI2cDriver::new(mock_bus);

        assert_eq!(driver.chip_id(), GC1034_CHIP_ID);
        assert_eq!(driver.model_name(), "GalaxyCore GC1034");

        // Set exposure lines to 1024
        driver.set_exposure_lines(1024).unwrap();
        // Set analog gain to 4.0x
        driver.set_analog_gain(4.0).unwrap();

        let status = driver.read_status().unwrap();
        assert_eq!(status.exposure_lines, 1024);
        assert_eq!(status.analog_gain_pga, 0x04);
    }
}
