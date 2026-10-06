//! # WebRTC Session & Signaling Coordinator
//!
//! Manages WebRTC SDP offer/answer negotiation and ICE candidate exchange
//! over WebSocket signaling channels for sub-100ms video streaming.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Error, Debug)]
pub enum WebRtcError {
    #[error("Invalid SDP format")]
    InvalidSdp,
    #[error("Session already active")]
    SessionAlreadyActive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SignalingMessage {
    #[serde(rename = "offer")]
    Offer { sdp: String },
    #[serde(rename = "answer")]
    Answer { sdp: String },
    #[serde(rename = "candidate")]
    Candidate { candidate: String, sdp_mid: Option<String> },
}

pub struct WebRtcSession {
    session_id: String,
    is_connected: AtomicBool,
    cached_candidates: Arc<Mutex<Vec<String>>>,
}

impl WebRtcSession {
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            is_connected: AtomicBool::new(false),
            cached_candidates: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn is_active(&self) -> bool {
        self.is_connected.load(Ordering::SeqCst)
    }

    /// Processes an incoming SDP offer and generates the corresponding SDP answer.
    pub async fn handle_offer(&self, offer_sdp: &str) -> Result<SignalingMessage, WebRtcError> {
        if !offer_sdp.contains("m=video") && !offer_sdp.contains("v=0") {
            return Err(WebRtcError::InvalidSdp);
        }

        self.is_connected.store(true, Ordering::SeqCst);

        // Synthesize standard H.264 SDP Answer for local camera RTP endpoint
        let answer_sdp = concat!(
            "v=0\r\n",
            "o=- 0 0 IN IP4 127.0.0.1\r\n",
            "s=ESCAM G02 WebRTC\r\n",
            "t=0 0\r\n",
            "m=video 9 UDP/TLS/RTP/SAVPF 96\r\n",
            "c=IN IP4 0.0.0.0\r\n",
            "a=rtpmap:96 H264/90000\r\n",
            "a=fmtp:96 packetization-mode=1;profile-level-id=42e01f\r\n",
            "a=sendonly\r\n"
        );

        Ok(SignalingMessage::Answer {
            sdp: answer_sdp.to_string(),
        })
    }

    pub async fn add_ice_candidate(&self, candidate: &str) {
        self.cached_candidates.lock().await.push(candidate.to_string());
    }

    pub async fn close(&self) {
        self.is_connected.store(false, Ordering::SeqCst);
        self.cached_candidates.lock().await.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sdp_offer_answer_negotiation() {
        let session = WebRtcSession::new("test-peer-1");
        assert!(!session.is_active());

        let fake_offer = "v=0\r\nm=video 9 UDP/TLS/RTP/SAVPF 96\r\n";
        let answer = session.handle_offer(fake_offer).await.unwrap();

        match answer {
            SignalingMessage::Answer { sdp } => {
                assert!(sdp.contains("H264/90000"));
                assert!(sdp.contains("ESCAM G02 WebRTC"));
            }
            _ => panic!("Expected SDP Answer message"),
        }

        assert!(session.is_active());
        session.close().await;
        assert!(!session.is_active());
    }
}
