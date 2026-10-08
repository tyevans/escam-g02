# Reference: ESCAM G02 Hardware GPIO Pinout & Peripherals

This specification documents the physical GPIO pin routing and peripheral interfaces on the ESCAM G02 mainboard (YuanTe `YT_710XC_M_2.1`), reverse-engineered from vendor drivers `motor.ko` and `gkio.ko`.

---

## 1. GPIO Pin Assignment Summary

| GPIO Line | Direction | Connected Peripheral | IC / Component | Function / Notes |
|:---|:---|:---|:---|:---|
| **GPIO 0** | Output | Pan Phase A | `JULN2803AG` Ch 1 | Unipolar stepper coil 1 |
| **GPIO 1** | Output | Pan Phase B | `JULN2803AG` Ch 2 | Unipolar stepper coil 2 |
| **GPIO 2** | Output | Pan Phase C | `JULN2803AG` Ch 3 | Unipolar stepper coil 3 |
| **GPIO 3** | Output | Pan Phase D | `JULN2803AG` Ch 4 | Unipolar stepper coil 4 |
| **GPIO 4** | Output | Tilt Phase A | `JULN2803AG` Ch 5 | Unipolar stepper coil 5 |
| **GPIO 5** | Output | Tilt Phase B | `JULN2803AG` Ch 6 | Unipolar stepper coil 6 |
| **GPIO 6** | Output | Tilt Phase C | `JULN2803AG` Ch 7 | Unipolar stepper coil 7 |
| **GPIO 7** | Output | Tilt Phase D | `JULN2803AG` Ch 8 | Unipolar stepper coil 8 |
| **GPIO 10** | Output | IR Illuminator | Discrete NPN switch | 850nm night IR LEDs (High = ON) |
| **GPIO 14** | Output | IR-Cut Filter FWD | `UTC BA6208L` Pin 2 | Moves filter into optical path (Day/Visible) |
| **GPIO 17** | Output | IR-Cut Filter REV | `UTC BA6208L` Pin 4 | Moves filter out of optical path (Night/Hα) |
| **GPIO 12** | Input | CDS Light Sensor | Voltage Divider | Day/Night ambient threshold detector |

---

## 2. Stepper Motor Phase Sequence

The two unipolar stepper motors (`28BYJ-48` equivalent, 5V, 64:1 reduction) use standard 8-phase half-stepping to ensure smooth motion and minimal acoustic resonance:

```
Step | Coil 1 | Coil 2 | Coil 3 | Coil 4 | Hex Mask
-----+--------+--------+--------+--------+---------
  0  |   1    |   0    |   0    |   0    |  0x01
  1  |   1    |   1    |   0    |   0    |  0x03
  2  |   0    |   1    |   0    |   0    |  0x02
  3  |   0    |   1    |   1    |   0    |  0x06
  4  |   0    |   0    |   1    |   0    |  0x04
  5  |   0    |   0    |   1    |   1    |  0x0C
  6  |   0    |   0    |   0    |   1    |  0x08
  7  |   1    |   0    |   0    |   1    |  0x09
```

When motion stops, all coil lines must be set to `0` to prevent steady-state heating of the Darlington transistors and motor windings.

---

## 3. UTC BA6208L H-Bridge Solenoid Actuation

The optical IR-cut filter solenoid operates via bi-directional current pulses:
* **Day Mode (Filter IN)**: Set GPIO 14 = High, GPIO 17 = Low for **100 ms**, then return both to Low.
* **Night / Hα Mode (Filter OUT)**: Set GPIO 14 = Low, GPIO 17 = High for **100 ms**, then return both to Low.
* **Idle State**: Both pins must remain Low to avoid draining battery/DC power or damaging the solenoid.
