//! # H.264 NAL Unit Parser & Annex-B Demuxer
//!
//! Parses raw bitstreams emitted by the Goke hardware video encoder (/dev/venc).

use bytes::Bytes;

/// Standard H.264 NAL Unit Types (RFC 6184)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NaluType {
    Unspecified,
    NonIdrSlice,
    DataPartitionA,
    DataPartitionB,
    DataPartitionC,
    IdrSlice,
    Sei,
    Sps,
    Pps,
    AccessUnitDelimiter,
    EndOfSequence,
    EndOfStream,
    FillerData,
    Other(u8),
}

impl From<u8> for NaluType {
    fn from(val: u8) -> Self {
        match val & 0x1F {
            0 => Self::Unspecified,
            1 => Self::NonIdrSlice,
            2 => Self::DataPartitionA,
            3 => Self::DataPartitionB,
            4 => Self::DataPartitionC,
            5 => Self::IdrSlice,
            6 => Self::Sei,
            7 => Self::Sps,
            8 => Self::Pps,
            9 => Self::AccessUnitDelimiter,
            10 => Self::EndOfSequence,
            11 => Self::EndOfStream,
            12 => Self::FillerData,
            other => Self::Other(other),
        }
    }
}

/// Represents a single discrete H.264 NAL Unit
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nalu {
    pub nalu_type: NaluType,
    pub ref_idc: u8,
    pub data: Bytes,
}

impl Nalu {
    pub fn parse_annex_b(data: &[u8]) -> Vec<Self> {
        let mut nalus = Vec::new();
        let len = data.len();
        let mut i = 0;

        while i < len {
            // Locate start code
            let (_start_code_len, prefix_end) = if i + 4 <= len && &data[i..i + 4] == [0, 0, 0, 1] {
                (4, i + 4)
            } else if i + 3 <= len && &data[i..i + 3] == [0, 0, 1] {
                (3, i + 3)
            } else {
                i += 1;
                continue;
            };

            // Find next start code or end of buffer
            let mut next_start = len;
            for j in prefix_end..len {
                if (j + 4 <= len && &data[j..j + 4] == [0, 0, 0, 1])
                    || (j + 3 <= len && &data[j..j + 3] == [0, 0, 1])
                {
                    next_start = j;
                    break;
                }
            }

            if prefix_end < next_start {
                let nalu_data = &data[prefix_end..next_start];
                let header = nalu_data[0];
                let ref_idc = (header >> 5) & 0x03;
                let nalu_type = NaluType::from(header);

                nalus.push(Self {
                    nalu_type,
                    ref_idc,
                    data: Bytes::copy_from_slice(nalu_data),
                });
            }

            i = next_start;
        }

        nalus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annex_b_parsing() {
        // Construct a synthetic bitstream with SPS (7), PPS (8), and IDR (5)
        let bitstream = vec![
            0x00, 0x00, 0x00, 0x01, 0x67, 0x42, 0x00, 0x1f, // 4-byte prefix SPS
            0x00, 0x00, 0x01, 0x68, 0xce, 0x3c, 0x80,       // 3-byte prefix PPS
            0x00, 0x00, 0x00, 0x01, 0x65, 0x88, 0x84, 0x00, // 4-byte prefix IDR
        ];

        let nalus = Nalu::parse_annex_b(&bitstream);
        assert_eq!(nalus.len(), 3);
        assert_eq!(nalus[0].nalu_type, NaluType::Sps);
        assert_eq!(nalus[1].nalu_type, NaluType::Pps);
        assert_eq!(nalus[2].nalu_type, NaluType::IdrSlice);
    }
}
