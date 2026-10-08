//! # Minimalist Localhost RTSP H.264 Ingestion Client
//!
//! Connects to the local Goke RTSP server over TCP, negotiates interleaved RTP,
//! and continuously streams Annex-B H.264 NALUs for WebRTC/WebSocket broadcast.

use crate::rtp::RtpDepacketizer;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tracing::{info, warn};

/// Computes standard MD5 hex digest (RFC 1321) with zero external dependencies.
pub fn md5_hex(data: &[u8]) -> String {
    let mut a: u32 = 0x67452301;
    let mut b: u32 = 0xefcdab89;
    let mut c: u32 = 0x98badcfe;
    let mut d: u32 = 0x10325476;

    let orig_len_bits = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&orig_len_bits.to_le_bytes());

    #[rustfmt::skip]
    let s: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    #[rustfmt::skip]
    let k: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    for chunk in msg.chunks_exact(64) {
        let mut m = [0u32; 16];
        for (i, c) in chunk.chunks_exact(4).enumerate() {
            m[i] = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
        }
        let (mut aa, mut bb, mut cc, mut dd) = (a, b, c, d);
        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((bb & cc) | (!bb & dd), i),
                16..=31 => ((dd & bb) | (!dd & cc), (5 * i + 1) % 16),
                32..=47 => (bb ^ cc ^ dd, (3 * i + 5) % 16),
                _ => (cc ^ (bb | !dd), (7 * i) % 16),
            };
            let temp = dd;
            dd = cc;
            cc = bb;
            bb = bb.wrapping_add((aa.wrapping_add(f).wrapping_add(k[i]).wrapping_add(m[g])).rotate_left(s[i]));
            aa = temp;
        }
        a = a.wrapping_add(aa);
        b = b.wrapping_add(bb);
        c = c.wrapping_add(cc);
        d = d.wrapping_add(dd);
    }

    let mut res = String::with_capacity(32);
    for val in [a, b, c, d] {
        for byte in val.to_le_bytes() {
            use std::fmt::Write;
            let _ = write!(res, "{:02x}", byte);
        }
    }
    res
}

fn build_digest_auth(method: &str, uri: &str, realm: &str, nonce: &str) -> String {
    let ha1 = md5_hex(format!("admin:{}:admin", realm).as_bytes());
    let ha2 = md5_hex(format!("{}:{}", method, uri).as_bytes());
    let response = md5_hex(format!("{}:{}:{}", ha1, nonce, ha2).as_bytes());
    format!(
        "Digest username=\"admin\", realm=\"{}\", nonce=\"{}\", uri=\"{}\", response=\"{}\"",
        realm, nonce, uri, response
    )
}

pub struct RtspClient {
    rtsp_addr: String,
    stream_path: String,
    sender: broadcast::Sender<Vec<u8>>,
}

impl RtspClient {
    pub fn new(rtsp_addr: &str, stream_path: &str, sender: broadcast::Sender<Vec<u8>>) -> Self {
        Self {
            rtsp_addr: rtsp_addr.to_string(),
            stream_path: stream_path.to_string(),
            sender,
        }
    }

    pub async fn run_loop(&self) {
        loop {
            if let Err(e) = self.stream_session().await {
                warn!("[rtsp_client] Stream error ({:?}), retrying in 2s...", e);
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    async fn stream_session(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[rtsp_client] Connecting to RTSP at {}...", self.rtsp_addr);
        let mut sock = TcpStream::connect(&self.rtsp_addr).await?;
        let _ = sock.set_nodelay(true);
        let base_uri = format!("rtsp://{}/{}", self.rtsp_addr, self.stream_path);

        // 1. Initial DESCRIBE to fetch 401 challenge (realm and nonce)
        let desc_req = format!("DESCRIBE {} RTSP/1.0\r\nCSeq: 1\r\nAccept: application/sdp\r\n\r\n", base_uri);
        sock.write_all(desc_req.as_bytes()).await?;

        let mut buf = vec![0u8; 4096];
        let n = sock.read(&mut buf).await?;
        let resp = String::from_utf8_lossy(&buf[..n]);

        let realm = extract_between(&resp, "realm=\"", "\"").unwrap_or_else(|| "Hipcam RealServer/V1.0".to_string());
        let nonce = extract_between(&resp, "nonce=\"", "\"").ok_or("Missing RTSP nonce")?;

        // 2. Authenticated DESCRIBE
        let auth = build_digest_auth("DESCRIBE", &base_uri, &realm, &nonce);
        let desc_auth = format!(
            "DESCRIBE {} RTSP/1.0\r\nCSeq: 2\r\nAuthorization: {}\r\nAccept: application/sdp\r\n\r\n",
            base_uri, auth
        );
        sock.write_all(desc_auth.as_bytes()).await?;
        let _ = sock.read(&mut buf).await?;

        // 3. SETUP Video Track (trackID=0)
        let track_uri = format!("{}/trackID=0", base_uri);
        let auth_setup = build_digest_auth("SETUP", &track_uri, &realm, &nonce);
        let setup_req = format!(
            "SETUP {} RTSP/1.0\r\nCSeq: 3\r\nAuthorization: {}\r\nTransport: RTP/AVP/TCP;unicast;interleaved=0-1\r\n\r\n",
            track_uri, auth_setup
        );
        sock.write_all(setup_req.as_bytes()).await?;
        let n = sock.read(&mut buf).await?;
        let setup_resp = String::from_utf8_lossy(&buf[..n]);
        let session = extract_between(&setup_resp, "Session: ", "\r").or_else(|| extract_between(&setup_resp, "Session: ", ";")).ok_or("Missing RTSP session")?;

        // 4. PLAY
        let play_uri = base_uri.clone();
        let auth_play = build_digest_auth("PLAY", &play_uri, &realm, &nonce);
        let play_req = format!(
            "PLAY {} RTSP/1.0\r\nCSeq: 4\r\nSession: {}\r\nAuthorization: {}\r\nRange: npt=0.000-\r\n\r\n",
            play_uri, session, auth_play
        );
        sock.write_all(play_req.as_bytes()).await?;

        // Read PLAY response byte-by-byte until \r\n\r\n so no interleaved RTP frames are eaten
        let mut play_resp = Vec::with_capacity(512);
        let mut b = [0u8; 1];
        while !play_resp.ends_with(b"\r\n\r\n") {
            sock.read_exact(&mut b).await?;
            play_resp.push(b[0]);
        }
        info!("[rtsp_client] RTSP PLAY initiated successfully on {}", self.stream_path);

        // 5. Ingestion Loop reading interleaved RTP packets
        let mut depacketizer = RtpDepacketizer::new();
        let mut rest = [0u8; 3];

        loop {
            sock.read_exact(&mut b).await?;
            if b[0] != b'$' {
                continue;
            }
            sock.read_exact(&mut rest).await?;
            let chan = rest[0];
            let len = ((rest[1] as usize) << 8) | (rest[2] as usize);

            let mut payload = vec![0u8; len];
            sock.read_exact(&mut payload).await?;

            if chan == 0 && payload.len() > 12 {
                // Video channel: skip 12-byte RTP header
                if let Some(annex_b) = depacketizer.depacketize(&payload[12..]) {
                    let _ = self.sender.send(annex_b);
                }
            }
        }
    }
}

fn extract_between(s: &str, start_pat: &str, end_pat: &str) -> Option<String> {
    let start = s.find(start_pat)? + start_pat.len();
    let rest = &s[start..];
    let end = rest.find(end_pat)?;
    Some(rest[..end].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_rust_md5() {
        assert_eq!(
            md5_hex(b"admin:Hipcam RealServer/V1.0:admin"),
            "97634781b8490a35133f37214a052297"
        );
    }
}
