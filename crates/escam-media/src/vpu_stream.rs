//! # Zero-Copy Unix Domain Socket VPU Stream Ingestion
//!
//! Ingests raw Annex-B H.264 streams directly from the headless `gk-vpu` micro-streamer
//! over a local Unix domain socket (e.g. `/tmp/venc.sock`). Eliminates RTSP/RTP networking
//! and replaces the legacy `ipc_server` vendor daemon.

use crate::nalu::H264StreamReader;
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::UnixStream;
use tokio::sync::broadcast;
use tracing::{info, warn};

#[derive(Error, Debug)]
pub enum VpuStreamError {
    #[error("I/O error connecting to VPU socket: {0}")]
    Io(#[from] std::io::Error),
    #[error("VPU socket closed prematurely")]
    Closed,
}

pub struct VpuUnixStreamReader {
    socket_path: PathBuf,
}

impl VpuUnixStreamReader {
    pub fn new<P: AsRef<Path>>(socket_path: P) -> Self {
        Self {
            socket_path: socket_path.as_ref().to_path_buf(),
        }
    }

    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Connects to the local VPU Unix domain socket.
    pub async fn connect(&self) -> Result<UnixStream, VpuStreamError> {
        let stream = UnixStream::connect(&self.socket_path).await?;
        Ok(stream)
    }

    /// Resilient connection loop that repeatedly pumps from the VPU socket.
    pub async fn run_loop(&self, tx: broadcast::Sender<Vec<u8>>) {
        loop {
            if let Err(e) = self.pump_to_broadcast(tx.clone()).await {
                warn!(
                    "[vpu_stream] Disconnected from VPU socket {:?} ({:?}), retrying in 2s...",
                    self.socket_path, e
                );
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    /// Continuous pump reading Annex-B frames from the socket and broadcasting to `tx`.
    pub async fn pump_to_broadcast(
        &self,
        tx: broadcast::Sender<Vec<u8>>,
    ) -> Result<(), VpuStreamError> {
        let mut stream = self.connect().await?;
        info!("Connected to local VPU stream at {:?}", self.socket_path);

        let mut reader = H264StreamReader::new();
        let mut buf = [0u8; 16384];

        loop {
            let n = match stream.read(&mut buf).await {
                Ok(0) => return Err(VpuStreamError::Closed),
                Ok(n) => n,
                Err(e) => return Err(VpuStreamError::Io(e)),
            };

            let nalus = reader.ingest(&buf[..n]);
            for nalu in nalus {
                let mut packet = Vec::with_capacity(4 + nalu.data.len());
                packet.extend_from_slice(&[0, 0, 0, 1]);
                packet.extend_from_slice(&nalu.data);
                let _ = tx.send(packet);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
    use tokio::net::UnixListener;

    #[tokio::test]
    async fn test_vpu_unix_stream_ingest_and_broadcast() {
        let sock_path = format!("/tmp/test_vpu_{}.sock", std::process::id());
        let _ = std::fs::remove_file(&sock_path);

        let listener = UnixListener::bind(&sock_path).unwrap();
        let (tx, mut rx) = broadcast::channel(16);

        // Spawn mock VPU server
        tokio::spawn(async move {
            if let Ok((mut sock, _)) = listener.accept().await {
                let sample_nalu = [
                    0x00, 0x00, 0x00, 0x01, 0x67, 0x42, 0x00, 0x1f, // SPS
                    0x00, 0x00, 0x00, 0x01, 0x68, 0xce, 0x3c, 0x80, // PPS
                    0x00, 0x00, 0x00, 0x01, 0x65, 0x88, 0x84, 0x00, // IDR
                ];
                let _ = sock.write_all(&sample_nalu).await;
            }
        });

        let reader = VpuUnixStreamReader::new(&sock_path);
        let pump_tx = tx.clone();

        tokio::spawn(async move {
            let _ = reader.pump_to_broadcast(pump_tx).await;
        });

        // Receive framed NALUs
        let mut received = 0;
        let timeout = tokio::time::timeout(std::time::Duration::from_millis(500), async {
            while let Ok(frame) = rx.recv().await {
                assert!(frame.starts_with(&[0, 0, 0, 1]));
                received += 1;
                if received >= 2 {
                    break;
                }
            }
        })
        .await;

        let _ = std::fs::remove_file(&sock_path);
        assert!(timeout.is_ok(), "Timed out waiting for NALU broadcast");
        assert!(received >= 2);
    }
}
