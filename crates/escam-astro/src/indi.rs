//! # Embedded INDI Protocol Server (TCP 7624)
//!
//! Exposes the camera and PTZ mount to astronomy suites (Ekos, KStars, NINA)
//! via the standard XML-based INDI (Instrument-Neutral Distributed Interface) protocol.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::info;

pub struct IndiServer {
    port: u16,
}

impl IndiServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    /// Handles a single incoming INDI client TCP stream.
    pub async fn handle_client(mut stream: tokio::net::TcpStream) -> Result<(), std::io::Error> {
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }

        let request = String::from_utf8_lossy(&buf[..n]);
        if request.contains("getProperties") {
            let response = concat!(
                "<defTextVector device='ESCAM G02 CCD' name='DEVICE_INFO' state='Idle' perm='ro'>\n",
                "  <defText name='MODEL'>ESCAM G02 Rust</defText>\n",
                "  <defText name='SENSOR'>GalaxyCore GC1034 / SmartSens SC1135</defText>\n",
                "</defTextVector>\n",
                "<defNumberVector device='ESCAM G02 CCD' name='CCD_EXPOSURE' state='Idle' perm='rw'>\n",
                "  <defNumber name='PERIOD' label='Exposure (s)' min='0.001' max='10.0' step='0.01' format='%g'>1.0</defNumber>\n",
                "</defNumberVector>\n",
                "<defSwitchVector device='ESCAM G02 CCD' name='FILTER_SLOT' state='Idle' perm='rw' rule='OneOfMany'>\n",
                "  <defSwitch name='DAY_VIS' label='IR-Cut ON (Day)'>On</defSwitch>\n",
                "  <defSwitch name='NIGHT_HA' label='IR-Cut OFF (H-Alpha/Astro)'>Off</defSwitch>\n",
                "</defSwitchVector>\n"
            );
            stream.write_all(response.as_bytes()).await?;
        }

        Ok(())
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        info!("INDI server listening on port {}", self.port);

        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(async move {
                    let _ = Self::handle_client(stream).await;
                });
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn test_indi_server_get_properties() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        tokio::spawn(async move {
            if let Ok((stream, _)) = listener.accept().await {
                let _ = IndiServer::handle_client(stream).await;
            }
        });

        let mut client = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
        client.write_all(b"<getProperties version='1.7'/>\n").await.unwrap();

        let mut buf = vec![0u8; 1024];
        let n = client.read(&mut buf).await.unwrap();
        let response = String::from_utf8_lossy(&buf[..n]);

        assert!(response.contains("ESCAM G02 CCD"));
        assert!(response.contains("CCD_EXPOSURE"));
        assert!(response.contains("FILTER_SLOT"));
    }
}
