import os
import subprocess
from pathlib import Path
import pytest

def test_flash_script_exists_and_executable():
    script = Path("scripts/flash_rootfs.sh")
    assert script.exists(), "scripts/flash_rootfs.sh must exist"
    assert os.access(script, os.X_OK), "scripts/flash_rootfs.sh must be executable"

def test_flash_script_check_mode():
    res = subprocess.run(["./scripts/flash_rootfs.sh", "--check"], capture_output=True, text=True)
    assert res.returncode == 0, f"--check failed: {res.stderr}"
    assert "SquashFS 4.0 Superblock Magic verified" in res.stdout
    assert "SHA256" in res.stdout
    assert "Margin" in res.stdout

def test_flash_script_dry_run_mode():
    ping_check = subprocess.run(["ping", "-c", "1", "-W", "1", "10.75.2.93"], capture_output=True)
    if ping_check.returncode != 0:
        pytest.skip("Hardware camera at 10.75.2.93 is currently unreachable; skipping live dry-run")
    res = subprocess.run(["./scripts/flash_rootfs.sh", "--dry-run"], capture_output=True, text=True)
    assert res.returncode == 0, f"--dry-run failed: {res.stderr}"
    assert "Target /dev/mtd3 (rootfs) confirmed" in res.stdout
    assert "Dry-run preflight checks successfully completed" in res.stdout

