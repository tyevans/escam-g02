//! # ESCAM High-Performance Media & Streaming Crate
//!
//! Handles H.264 Annex-B NAL unit ingestion, RFC 6184 RTP packetization,
//! WebRTC peer session coordination, and live stream delivery.

pub mod nalu;
pub mod rtp;
pub mod webrtc;

pub use nalu::{Nalu, NaluType};
pub use rtp::{RtpPacket, RtpPacketizer};
pub use webrtc::{SignalingMessage, WebRtcError, WebRtcSession};
