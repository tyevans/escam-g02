# Hardware Device Node & ioctl Reference

This reference documents the reverse-engineered Linux character device nodes on the ESCAM G02 (Goke GK7102C SoC).

---

## 1. Stepper Motor Interface (`/dev/motor`)

- **Kernel Driver**: `motor.ko`
- **Driver Circuitry**: `JULN2803AG` 8-channel Darlington transistor array
- **Physical Motors**: 2x unipolar 4-phase stepper motors (4 phases pan + 4 phases tilt)

### ioctl Commands

| Command Code | Constant | Payload | Description |
|:---|:---|:---|:---|
| `0xC0046D00` | `MOTOR_IOCTL_STOP` | None (0) | Immediately de-energizes all 8 Darlington coil phases |
| `0xC0046D01` | `MOTOR_IOCTL_RUN` | `struct MotorRun` (8 bytes) | Starts continuous stepping for pan and tilt axes |
| `0xC0046D02` | `MOTOR_IOCTL_CYCLE` | None (0) | Initiates patrol/cycle scan |
| `0xC0046D05` | `MOTOR_IOCTL_AUTO_CHECK` | None (0) | Executes homing and limit self-detection |
| `0xC0046D06` | `MOTOR_IOCTL_SPEED` | `struct MotorSpeed` (4 bytes) | Sets APB timer divisor for step frequency |
| `0xC0046D08` | `MOTOR_IOCTL_GET_STATUS`| Pointer to `u32` (4 bytes) | Returns 1 if moving, 0 if idle |
| `0xC0046D13` | `MOTOR_IOCTL_GET_POS` | Pointer to `[i32; 2]` (8 bytes) | Returns current `[pan_step, tilt_step]` |

### Data Structures

```c
// Note: In kernel module motor.ko, pandir is at offset 0 and titldir is at offset 4:
struct MotorRun {
    int pandir;   // 3 = Right (CW), 4 = Left (CCW), 0 = Stop (offset 0)
    int titldir;  // 1 = Up, 2 = Down, 0 = Stop (offset 4)
};

struct MotorSpeed {
    unsigned int speed; // Timer divisor
};
```

---

## 2. GPIO & IR-Cut Shutter (`/dev/gkio`)

- **Kernel Driver**: `gkio.ko`
- **Driver Circuitry**: UTC BA6208L Bi-directional H-Bridge
- **Mechanical Shutter**: Solenoid sliding the IR-cut glass over the CMOS sensor

### ioctl Commands

| Command Code | Constant | Payload | Description |
|:---|:---|:---|:---|
| `0xC0046200` | `GKIO_IOCTL_SET_VALUE` | `struct GpioVal` (8 bytes) | Sets GPIO output value |
| `0xC0046201` | `GKIO_IOCTL_GET_VALUE` | `struct GpioVal` (8 bytes) | Reads GPIO input value |

### Solenoid Pin Mapping

- **GPIO 14**: Forward drive (Day mode / IR-Cut ON)
- **GPIO 17**: Reverse drive (Night / Astro H-alpha mode / IR-Cut OFF)

Actuation protocol: Assert high (1) for 100ms, then assert low (0) to eliminate static current.

---

## 3. Hardware Watchdog (`/dev/watchdog`)

- **Kernel Driver**: `gk_wdt_v1_00` (Goke Microelectronics Watchdog Timer)
- **MMIO Base**: `0xf3006000`
- **Default Hardware Timeout**: 60 seconds

### Feeding Protocol
- **Keepalive Feed**: Writing any byte (e.g. `\0`) to `/dev/watchdog` resets the hardware counter. Must be called at least once every 10 seconds.
- **Safe Disarm (Magic Close)**: Writing character `'V'` (`0x56`) prior to closing the file descriptor safely disarms the watchdog timer without triggering an automatic SoC hardware reboot.

---

## 4. Video Sensor & Media Subsystem

- **Sensor Hardware**: GalaxyCore GC1034 1/4" 720p CMOS Image Sensor (I2C address `0x42` / `0x6c`)
- **Kernel Drivers**:
  - `hal.ko`: Hardware Abstraction Layer
  - `media.ko`: Goke Media Processing Platform (VENC / VI ring buffers)
  - `sensor.ko`: CMOS Sensor Interface
  - `gc1034_ex.ko`: GalaxyCore GC1034 kernel driver
- **Hardware Device Nodes**:
  - `/dev/gk_video` (major 248, minor 0): Video encoding ring buffer
  - `/dev/adc` (major 10, minor 11): Analog-to-digital converter / ambient photoresistor
- **Stock Stream Channels**:
  - Channel 11 (`/11`): Main Stream (1280x720 @ 30 FPS, 1536 kbps H.264, GOP 60)
  - Channel 12 (`/12`): Sub Stream (640x360 @ 15 FPS)

---

## 5. ARMv6 Architecture Constraints

- **Processor**: ARM1176JZF-S @ 600MHz (ARMv6l, CPU part `0xb76`)
- **Target Triple**: `arm-unknown-linux-musleabi` (soft-float ABI)
- **Hardware Limitations**:
  - **No NEON**: ARMv7 NEON SIMD instructions are strictly unsupported. Compression libraries (e.g. `simd-adler32`) must be disabled.
  - **No 64-bit Atomics**: Lacks `LDREXD`/`STREXD` instructions. Rust crates utilizing `AtomicU64` (e.g. HTTP/2 `h2`) will crash with illegal instruction/SIGSEGV.
  - **Unaligned Memory Access**: ARMv6 rotates unaligned 32-bit reads unless CP15 U-bit is set. Hash table lookups must use aligned data structures.

