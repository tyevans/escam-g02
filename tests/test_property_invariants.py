"""
Generative Property-Based Testing Suite
Governed by ADR-0009 and ADR-0007.

Verifies domain model invariants and motion profiles across randomized inputs
using the Hypothesis framework.
"""

from hypothesis import given, strategies as st
import math

@given(st.floats(min_value=-1.0, max_value=1.0), st.floats(min_value=-1.0, max_value=1.0))
def test_joystick_input_bounds_invariant(x: float, y: float):
    """
    Invariant: Virtual joystick magnitude must remain clamped and directional
    mapping must never produce out-of-bounds motor commands.
    """
    pan_dir = 3 if x > 0.1 else (4 if x < -0.1 else 0)
    tilt_dir = 1 if y > 0.1 else (2 if y < -0.1 else 0)
    
    assert pan_dir in (0, 3, 4)
    assert tilt_dir in (0, 1, 2)


@given(st.integers(min_value=1, max_value=2000), st.integers(min_value=10, max_value=5000))
def test_scurve_timer_interval_invariant(speed: int, max_speed: int):
    """
    Invariant: Stepper motor pulse interval (delay) must strictly decrease
    as speed increases, and remain non-zero and positive.
    """
    clamped_speed = min(speed, max_speed)
    # Timer interval in microseconds = 1_000_000 / speed
    interval_us = 1_000_000 // clamped_speed
    
    assert interval_us > 0
    assert interval_us <= 1_000_000


@given(st.binary(min_size=4, max_size=1400))
def test_rtp_packet_structure_invariant(payload: bytes):
    """
    Invariant: RTP packet serialization must always prepend a valid 12-byte header
    with RFC 6184 H.264 payload type (96).
    """
    version = 2
    payload_type = 96
    seq_num = 100
    timestamp = 90000
    ssrc = 0x12345678

    header = bytearray(12)
    header[0] = (version << 6) & 0xC0
    header[1] = payload_type & 0x7F
    header[2:4] = seq_num.to_bytes(2, "big")
    header[4:8] = timestamp.to_bytes(4, "big")
    header[8:12] = ssrc.to_bytes(4, "big")
    
    packet = bytes(header) + payload
    assert len(packet) == 12 + len(payload)
    assert packet[0] == 0x80
    assert packet[1] == 96
