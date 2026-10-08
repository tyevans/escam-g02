"""
US-0039: In-Vivo Verification and Testing of Open-Source RTL8188FU Wi-Fi Driver
Governed by ADR-0026 and PRD-0009.
"""

from pathlib import Path
import subprocess

def test_rtl8188fu_binary_and_modinfo():
    dist_ko = Path("dist/8188fu.ko")
    assert dist_ko.exists(), "dist/8188fu.ko must exist"
    assert dist_ko.stat().st_size > 0, "dist/8188fu.ko must not be empty"

    res = subprocess.run(["modinfo", str(dist_ko)], capture_output=True, text=True)
    assert res.returncode == 0
    info = res.stdout

    assert "usb:v0BDApF179" in info, "Must match Realtek RTL8188FTV USB ID 0bda:f179"
    assert "3.4.43-Goke" in info, "Vermagic must match target kernel 3.4.43-Goke"
    assert "Realtek Semiconductor Corp." in info
    assert "parm:           ifname" in info, "Must support ifname parameter"

def test_rtl8188fu_zero_external_dependencies():
    dist_ko = Path("dist/8188fu.ko")
    res = subprocess.run(["modinfo", "-F", "depends", str(dist_ko)], capture_output=True, text=True)
    assert res.returncode == 0
    # Must have no proprietary dependencies (like vendor rtkm)
    assert res.stdout.strip() == "", f"Expected 0 dependencies, got '{res.stdout.strip()}'"
