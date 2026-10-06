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
}
