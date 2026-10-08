"""
US-0040: Cross-Compilation and Verification of Open-Source GC1034 Sensor Driver
Governed by ADR-0026 and PRD-0009.
"""

from pathlib import Path
import subprocess

def test_gc1034_sensor_ko_built_and_modinfo():
    dist_ko = Path("dist/gc1034_sensor.ko")
    assert dist_ko.exists(), "dist/gc1034_sensor.ko must be built"
    assert dist_ko.stat().st_size > 0, "dist/gc1034_sensor.ko must not be empty"
    assert dist_ko.stat().st_size < 50_000, "dist/gc1034_sensor.ko must be lightweight (<50KB)"

    res = subprocess.run(["modinfo", str(dist_ko)], capture_output=True, text=True)
    assert res.returncode == 0
    info = res.stdout

    assert "3.4.43-Goke" in info, "vermagic must match 3.4.43-Goke"
    assert "license:        GPL" in info, "license must be GPL"
    assert "alias:          i2c:gc1034" in info, "must declare i2c:gc1034 alias"
    assert "Elena" in info, "author must match"

def test_gc1034_source_and_register_contracts():
    src_path = Path("drivers/gc1034_sensor.c")
    content = src_path.read_text()

    assert "0x21" in content, "GC1034 I2C address must be 0x21"
    assert "0x1034" in content, "GC1034 Chip ID must be 0x1034"
    assert "GC1034_REG_EXP_H" in content, "Exposure register must be defined"
    assert "GC1034_REG_ANALOG_GAIN" in content, "Gain register must be defined"
    assert "v4l2_i2c_subdev_init" in content, "Must initialize V4L2 I2C subdevice"
