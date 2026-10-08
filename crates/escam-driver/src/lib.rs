//! # ESCAM Hardware Driver Crate
//!
//! Provides type-safe hardware abstraction wrappers for the reverse-engineered
//! Goke GK7102C Linux kernel character device nodes:
//! - `/dev/motor`: 2-axis PTZ stepper motor driver via `motor.ko`
//! - `/dev/gkio`: General purpose I/O and IR-cut H-bridge driver via `gkio.ko`
//! - Sensor I2C: Direct CMOS register access for GC1034 / SC1135

pub mod gpio;
pub mod motor;
pub mod sensor;
pub mod userspace_motor;
pub mod vpu;

pub use gpio::{GpioDevice, GpioVal, IrCutController, LinuxGpioDevice, MockGpioDevice};
pub use motor::{LinuxMotorDevice, MockMotorDevice, MotorDevice, MotorRun, MotorSpeed};
pub use sensor::{MockSensorBus, SensorBus, SensorI2cDriver, SensorRegisters, GC1034_CHIP_ID, SC1135_CHIP_ID};
pub use userspace_motor::{UserspaceMotorController, HALF_STEP_TABLE, PAN_PINS, TILT_PINS};
pub use vpu::{GkEncStreamHeader, GkViConfig, VpuStreamPump, GK_ENC_IOC_GET_STREAM, MMZ_PHYSICAL_BASE, MMZ_PHYSICAL_SIZE};
