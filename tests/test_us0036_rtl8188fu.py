import os
import subprocess
from pathlib import Path
import pytest

def test_8188fu_ko_built_and_modinfo():
    dist_ko = Path("dist/8188fu.ko")
    assert dist_ko.exists(), "dist/8188fu.ko must be built"
    assert dist_ko.stat().st_size > 500_000, "dist/8188fu.ko must be a complete driver (>500KB)"

    res = subprocess.run(["modinfo", str(dist_ko)], capture_output=True, text=True)
    if res.returncode == 0:
        assert "3.4.43-Goke" in res.stdout, "vermagic must match 3.4.43-Goke"
        assert "GPL" in res.stdout, "license must be GPL"
        assert "0BDApF179" in res.stdout, "USB alias 0BDA:F179 must be supported"

def test_build_script_executable():
    script = Path("scripts/build_rtl8188fu.sh")
    assert script.exists(), "scripts/build_rtl8188fu.sh must exist"
    assert os.access(script, os.X_OK), "scripts/build_rtl8188fu.sh must be executable"
