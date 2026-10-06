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
struct MotorRun {
    int pandir;   // 1 = Right (CW), 2 = Left (CCW), 0 = Stop
    int titldir;  // 3 = Up, 4 = Down, 0 = Stop
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
