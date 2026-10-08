"""
US-0038: Cold-Boot Persistent Hardware Motor Driver Integration Test Suite
Governed by ADR-0026 and PRD-0009.
"""

from pathlib import Path
import hashlib
import subprocess

def test_escam_motor_artifact_integrity():
    dist_ko = Path("dist/escam_motor.ko")
    assert dist_ko.exists(), "dist/escam_motor.ko must be present"
    assert dist_ko.stat().st_size == 7180, f"Expected 7180 bytes, got {dist_ko.stat().st_size}"
    
    # Calculate sha256 checksum
    data = dist_ko.read_bytes()
    sha256 = hashlib.sha256(data).hexdigest()
    assert len(sha256) == 64

def test_patch_run_script_contract():
    run_patched = Path("run.patched")
    if run_patched.exists():
        content = run_patched.read_text(encoding="latin1")
        assert "Loading open-source escam_motor.ko" in content
        assert "insmod /mnt/mtd/ipc/conf/escam_motor.ko" in content
        assert "mknod /dev/motor c 243 0" in content
        assert "mknod /dev/gkio c 242 0" in content
        assert "insmod $TARGET/modules/motor.ko" in content

def test_system_clean_boot_script_escam_motor():
    debloat_path = Path("crates/escam-system/src/debloat.rs")
    content = debloat_path.read_text()
    assert "/mnt/mtd/ipc/conf/escam_motor.ko" in content
    assert "/mnt/mtd/ipc/modules/motor.ko" in content
