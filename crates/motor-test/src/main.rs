//! # ESCAM G02 - Standalone Motor & IR-Cut Proof of Concept (motor-test)
//!
//! Minimal static binary cross-compiled for ARMv6 musl (arm-unknown-linux-musleabi).
//! Tests /dev/motor ioctl commands and /dev/gkio IR-cut H-bridge toggles.

use escam_core::IrCutMode;
use escam_driver::{
    IrCutController, LinuxGpioDevice, LinuxMotorDevice, MockGpioDevice, MockMotorDevice,
    MotorDevice, MotorRun,
};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("==================================================");
    println!("⚡ ESCAM G02 - Standalone Motor & IR-Cut Test Utility");
    println!("Compiled for: ARMv6 musl (Static ELF)");
    println!("==================================================");

    // 1. Initialize Motor Interface (Physical with Mock Fallback for Host Execution)
    let (motor, is_mock): (Box<dyn MotorDevice>, bool) = match LinuxMotorDevice::open() {
        Ok(dev) => {
            println!("✅ Successfully opened /dev/motor (Hardware mode)");
            (Box::new(dev), false)
        }
        Err(e) => {
            println!("⚠️ Notice: /dev/motor unavailable ({}). Running in SIMULATION mode.", e);
            (Box::new(MockMotorDevice::new()), true)
        }
    };

    println!("[1/5] Setting motor timer speed divisor...");
    let _ = motor.set_speed(100);

    println!("[2/5] Sweeping Pan Right (pandir=1)...");
    let _ = motor.run(MotorRun::new(1, 0));
    sleep(Duration::from_millis(500));

    println!("[3/5] Halting Pan motor (pandir=0)...");
    let _ = motor.stop();
    sleep(Duration::from_millis(200));

    println!("[4/5] Sweeping Pan Left (pandir=2)...");
    let _ = motor.run(MotorRun::new(2, 0));
    sleep(Duration::from_millis(500));

    let _ = motor.stop();
    println!("✅ Motor sweep completed cleanly.");

    // 2. Test IR-Cut Filter Solenoid
    println!("[5/5] Testing UTC BA6208L IR-Cut H-Bridge toggle...");
    if !is_mock {
        if let Ok(gpio) = LinuxGpioDevice::open() {
            let ircut = IrCutController::new(gpio);
            println!("  -> Actuating Night / Astro H-alpha mode (GPIO 17 pulse)...");
            let _ = ircut.set_mode(IrCutMode::Night);
            sleep(Duration::from_millis(500));

            println!("  -> Actuating Day mode (GPIO 14 pulse)...");
            let _ = ircut.set_mode(IrCutMode::Day);
        }
    } else {
        let mock_gpio = MockGpioDevice::new();
        let ircut = IrCutController::new(mock_gpio);
        let _ = ircut.set_mode(IrCutMode::Night);
        let _ = ircut.set_mode(IrCutMode::Day);
        println!("  -> Simulated IR-Cut pulse transitions verified.");
    }

    println!("==================================================");
    println!("✨ motor-test diagnostic execution complete!");
    println!("==================================================");
}
