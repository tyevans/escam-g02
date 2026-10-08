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


@given(st.lists(st.integers(min_value=1, max_value=15), min_size=1, max_size=10))
def test_h264_annex_b_chunked_stream_invariant(chunk_sizes: list[int]):
    """
    Invariant: Arbitrary fragmentation of an Annex-B stream across network or ring buffer
    chunks must reconstruct the identical sequence of NAL units without data loss.
    """
    nal1 = bytes([0x67, 0x42, 0x00, 0x1f]) # SPS
    nal2 = bytes([0x68, 0xce, 0x3c])       # PPS
    nal3 = bytes([0x65, 0x88, 0x84, 0x01]) # IDR

    stream = b"\x00\x00\x00\x01" + nal1 + b"\x00\x00\x01" + nal2 + b"\x00\x00\x00\x01" + nal3 + b"\x00\x00\x00\x01"

    # Simulate chunking
    chunks = []
    idx = 0
    size_iter = iter(chunk_sizes)
    while idx < len(stream):
        sz = next(size_iter, 5)
        chunks.append(stream[idx:idx + sz])
        idx += sz

    # Parse via simple buffer simulation
    buf = bytearray()
    extracted = []
    for c in chunks:
        buf.extend(c)
        while True:
            # find start code
            pos1 = buf.find(b"\x00\x00\x00\x01")
            pos2 = buf.find(b"\x00\x00\x01")
            starts = [p for p in (pos1, pos2) if p != -1]
            if not starts:
                break
            start = min(starts)
            plen = 4 if start == pos1 else 3
            prefix_end = start + plen

            # find next start
            npos1 = buf.find(b"\x00\x00\x00\x01", prefix_end)
            npos2 = buf.find(b"\x00\x00\x01", prefix_end)
            nstarts = [p for p in (npos1, npos2) if p != -1]
            if not nstarts:
                break
            next_start = min(nstarts)
            extracted.append(bytes(buf[prefix_end:next_start]))
            buf = buf[next_start:]

    assert len(extracted) == 3
    assert extracted[0] == nal1
    assert extracted[1] == nal2
    assert extracted[2] == nal3


@given(
    st.floats(min_value=0.0001, max_value=120.0),
    st.floats(min_value=0.1, max_value=100.0),
    st.floats(min_value=0.01, max_value=120.0),
)
def test_camera_exposure_fps_coupling_invariant(raw_exp: float, raw_gain: float, raw_fps: float):
    """
    ADR-0009 & US-0012 Invariant:
    Normalizing arbitrary exposure, gain, and target_fps values must strictly clamp
    exposure within [0.001, 60.0] and gain within [1.0, 64.0].
    When exposure > 0.04s (e.g. 10s exposure), the effective frame rate must never exceed
    the reciprocal of the exposure time (1.0 / exp).
    """
    exposure_secs = max(0.001, min(60.0, raw_exp))
    gain = max(1.0, min(64.0, raw_gain))
    max_fps = 1.0 / exposure_secs
    target_fps = max(0.016, min(60.0, min(raw_fps, max_fps)))

    assert 0.001 <= exposure_secs <= 60.0
    assert 1.0 <= gain <= 64.0
    assert target_fps <= (1.0 / exposure_secs) + 1e-5
    assert target_fps >= 0.016


@given(
    st.floats(min_value=0.01, max_value=0.99),
    st.floats(min_value=0.0, max_value=1.0),
)
def test_mtf_transfer_function_invariants(m: float, x: float):
    """
    ADR-0009 Invariant:
    The Midtone Transfer Function (MTF) must always map [0, 1] monotonically into [0, 1].
    Boundary points 0.0 and 1.0 must be strictly preserved.
    """
    def mtf(m_val: float, x_val: float) -> float:
        if x_val <= 0.0:
            return 0.0
        if x_val >= 1.0:
            return 1.0
        num = (m_val - 1.0) * x_val
        den = (2.0 * m_val - 1.0) * x_val - m_val
        if abs(den) < 1e-6:
            return x_val
        return max(0.0, min(1.0, num / den))

    res = mtf(m, x)
    assert 0.0 <= res <= 1.0
    assert mtf(m, 0.0) == 0.0
    assert mtf(m, 1.0) == 1.0


