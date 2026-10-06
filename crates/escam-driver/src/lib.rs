//! # ESCAM Hardware Driver Crate
//!
//! Provides type-safe hardware abstraction wrappers for the reverse-engineered
//! Goke GK7102C Linux kernel character device nodes:
//! - `/dev/motor`: 2-axis PTZ stepper motor driver via `motor.ko`
//! - `/dev/gkio`: General purpose I/O and IR-cut H-bridge driver via `gkio.ko`

pub mod gpio;
pub mod motor;

pub use gpio::{GpioDevice, GpioVal, IrCutController, LinuxGpioDevice, MockGpioDevice};
pub use motor::{LinuxMotorDevice, MockMotorDevice, MotorDevice, MotorRun, MotorSpeed};
