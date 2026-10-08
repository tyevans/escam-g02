import os
import subprocess
from pathlib import Path
import pytest

def test_escam_motor_ko_built_and_vermagic():
    dist_ko = Path("dist/escam_motor.ko")
    assert dist_ko.exists(), "dist/escam_motor.ko must be built"
    assert dist_ko.stat().st_size > 0, "dist/escam_motor.ko must not be empty"
    assert dist_ko.stat().st_size < 100_000, "dist/escam_motor.ko must be lightweight (<100KB)"

    # Inspect modinfo if available
    res = subprocess.run(["modinfo", str(dist_ko)], capture_output=True, text=True)
    if res.returncode == 0:
        assert "3.4.43-Goke" in res.stdout, "vermagic must match 3.4.43-Goke"
        assert "GPL" in res.stdout, "license must be GPL"

def test_clean_boot_script_escam_motor_conditional():
    # Verify clean boot generation from escam-system
    res = subprocess.run(["cargo", "test", "-p", "escam-system", "--lib"], capture_output=True, text=True)
    assert res.returncode == 0, f"escam-system tests failed: {res.stderr}"

    debloat_path = Path("crates/escam-system/src/debloat.rs")
    content = debloat_path.read_text()
    assert "/mnt/mtd/ipc/conf/escam_motor.ko" in content, "Clean boot must check for escam_motor.ko"
    assert "/mnt/mtd/ipc/modules/motor.ko" in content, "Clean boot must maintain fallback to vendor motor.ko"
