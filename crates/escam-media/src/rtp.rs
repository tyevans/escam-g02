//! # RFC 6184 RTP Packetizer for H.264
//!
//! Handles encapsulation of H.264 NALUs into RTP packets with FU-A fragmentation.

use bytes::{Bytes, BytesMut};

pub const DEFAULT_MTU: usize = 1400;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtpPacket {
    pub sequence_number: u16,
    pub timestamp: u32,
    pub payload: Bytes,
    pub marker: bool,
}

pub struct RtpPacketizer {
    mtu: usize,
    sequence_number: u16,
    timestamp: u32,
}

impl RtpPacketizer {
    pub fn new(mtu: usize) -> Self {
        Self {
            mtu,
            sequence_number: 1000,
            timestamp: 0,
        }
    }

    /// Advances 90kHz timestamp for the next video frame at 25 fps.
    pub fn advance_frame(&mut self) {
        self.timestamp = self.timestamp.wrapping_add(3600); // 90000 / 25 = 3600 ticks
    }

    /// Packetizes a single NAL unit into one or more RTP packets.
    pub fn packetize(&mut self, nalu_data: &[u8]) -> Vec<RtpPacket> {
        let mut packets = Vec::new();
        if nalu_data.is_empty() {
            return packets;
        }

        if nalu_data.len() <= self.mtu {
            // Single NAL unit packet
            self.sequence_number = self.sequence_number.wrapping_add(1);
            packets.push(RtpPacket {
                sequence_number: self.sequence_number,
                timestamp: self.timestamp,
                payload: Bytes::copy_from_slice(nalu_data),
                marker: true,
            });
        } else {
            // FU-A Fragmentation
            let nalu_header = nalu_data[0];
            let nri = nalu_header & 0x60;
            let nalu_type = nalu_header & 0x1F;
            let fu_indicator = nri | 28; // Type 28 = FU-A

            let payload_data = &nalu_data[1..];
            let max_chunk = self.mtu - 2; // Subtract 2 bytes for FU indicator & header
            let total_chunks = (payload_data.len() + max_chunk - 1) / max_chunk;

            for (i, chunk) in payload_data.chunks(max_chunk).enumerate() {
                let is_first = i == 0;
                let is_last = i == (total_chunks - 1);

                let mut fu_header = nalu_type;
                if is_first {
                    fu_header |= 0x80; // S bit
                }
                if is_last {
                    fu_header |= 0x40; // E bit
                }

                let mut packet_buf = BytesMut::with_capacity(2 + chunk.len());
                packet_buf.extend_from_slice(&[fu_indicator, fu_header]);
                packet_buf.extend_from_slice(chunk);

                self.sequence_number = self.sequence_number.wrapping_add(1);
                packets.push(RtpPacket {
                    sequence_number: self.sequence_number,
                    timestamp: self.timestamp,
                    payload: packet_buf.freeze(),
                    marker: is_last,
                });
            }
        }

        packets
    }
}

/// Depacketizer for RFC 6184 H.264 RTP payloads into Annex-B NAL units.
#[derive(Debug, Default)]
pub struct RtpDepacketizer {
    fu_buffer: Vec<u8>,
}

impl RtpDepacketizer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Ingests an RTP packet payload (skipping 12-byte RTP header) and returns an Annex-B NAL unit if complete.
    pub fn depacketize(&mut self, payload: &[u8]) -> Option<Vec<u8>> {
        if payload.is_empty() {
            return None;
        }

        let nalu_header = payload[0];
        let nalu_type = nalu_header & 0x1F;

        if nalu_type == 28 {
            if payload.len() < 2 {
                return None;
            }
            let fu_header = payload[1];
            let start = (fu_header & 0x80) != 0;
            let end = (fu_header & 0x40) != 0;
            let actual_type = fu_header & 0x1F;
            let nri = nalu_header & 0x60;

            if start {
                self.fu_buffer.clear();
                self.fu_buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
                self.fu_buffer.push(nri | actual_type);
                self.fu_buffer.extend_from_slice(&payload[2..]);
            } else if !self.fu_buffer.is_empty() {
                self.fu_buffer.extend_from_slice(&payload[2..]);
            }

            if end && !self.fu_buffer.is_empty() {
                let complete = std::mem::take(&mut self.fu_buffer);
                return Some(complete);
            }
            None
        } else if (1..=23).contains(&nalu_type) {
            let mut complete = Vec::with_capacity(4 + payload.len());
            complete.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
            complete.extend_from_slice(payload);
            Some(complete)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_nalu_packetization() {
        let mut packetizer = RtpPacketizer::new(1400);
        let nalu = vec![0x67, 0x42, 0x00, 0x1f];
        let packets = packetizer.packetize(&nalu);

        assert_eq!(packets.len(), 1);
        assert!(packets[0].marker);
        assert_eq!(packets[0].payload.as_ref(), &nalu[..]);
    }

    #[test]
    fn test_fu_a_fragmentation() {
        let mut packetizer = RtpPacketizer::new(500);
        let mut nalu = vec![0x65]; // IDR header
        nalu.extend(vec![0xAA; 1200]); // 1201 bytes total > 500 MTU

        let packets = packetizer.packetize(&nalu);
        assert_eq!(packets.len(), 3);

        // Check FU-A indicator (0x65 -> NRI 0x60, type 28 -> 0x7C)
        assert_eq!(packets[0].payload[0], 0x60 | 28);
        // Start bit set
        assert_eq!(packets[0].payload[1] & 0x80, 0x80);
        assert!(!packets[0].marker);

        // End bit set on last packet
        assert_eq!(packets[2].payload[1] & 0x40, 0x40);
        assert!(packets[2].marker);
    }

    #[test]
    fn test_depacketizer_roundtrip() {
        let mut packetizer = RtpPacketizer::new(500);
        let mut depacketizer = RtpDepacketizer::new();

        // 1. Test single NALU (e.g. SPS)
        let sps = vec![0x67, 0x42, 0x00, 0x1f];
        let pkts = packetizer.packetize(&sps);
        assert_eq!(pkts.len(), 1);
        let depkt = depacketizer.depacketize(&pkts[0].payload).unwrap();
        assert_eq!(&depkt[..4], &[0x00, 0x00, 0x00, 0x01]);
        assert_eq!(&depkt[4..], &sps[..]);

        // 2. Test FU-A fragmented NALU
        let mut idr = vec![0x65];
        idr.extend(vec![0x42; 1100]);
        let pkts2 = packetizer.packetize(&idr);
        assert!(pkts2.len() > 1);

        let mut reconstructed = None;
        for pkt in pkts2 {
            if let Some(res) = depacketizer.depacketize(&pkt.payload) {
                reconstructed = Some(res);
            }
        }
        let full = reconstructed.expect("Failed to reconstruct fragmented NALU");
        assert_eq!(&full[..4], &[0x00, 0x00, 0x00, 0x01]);
        assert_eq!(&full[4..], &idr[..]);
    }
}
