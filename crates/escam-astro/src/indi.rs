//! # Embedded INDI Protocol Server (TCP 7624)
//!
//! Exposes the camera and PTZ mount to astronomy suites (Ekos, KStars, NINA, Stellarium)
//! via the standard XML-based INDI (Instrument-Neutral Distributed Interface) protocol.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

/// Direction for telescope mount movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountDirection {
    North,
    South,
    East,
    West,
}

/// Commands dispatched from INDI clients to the PTZ mount.
#[derive(Debug, Clone, PartialEq)]
pub enum IndiMountAction {
    Move(MountDirection),
    Stop,
    Park,
    SetCoords { az: f32, alt: f32 },
    SetTracking(bool),
}

/// Embedded INDI Protocol Server.
pub struct IndiServer {
    port: u16,
    mount_tx: Option<UnboundedSender<IndiMountAction>>,
}

impl IndiServer {
    /// Create a standalone INDI server without mount integration.
    pub fn new(port: u16) -> Self {
        Self {
            port,
            mount_tx: None,
        }
    }

    /// Create an INDI server with PTZ mount integration.
    pub fn with_mount(port: u16, mount_tx: UnboundedSender<IndiMountAction>) -> Self {
        Self {
            port,
            mount_tx: Some(mount_tx),
        }
    }

    /// XML response defining all device properties when <getProperties> is received.
    pub fn get_definitions_xml() -> &'static str {
        concat!(
            // Device 1: ESCAM G02 CCD Camera
            "<defTextVector device='ESCAM G02 CCD' name='DEVICE_INFO' state='Idle' perm='ro'>\n",
            "  <defText name='MODEL'>ESCAM G02 Rust</defText>\n",
            "  <defText name='SENSOR'>GalaxyCore GC1034 / SmartSens SC1135</defText>\n",
            "</defTextVector>\n",
            "<defNumberVector device='ESCAM G02 CCD' name='CCD_EXPOSURE' state='Idle' perm='rw'>\n",
            "  <defNumber name='PERIOD' label='Exposure (s)' min='0.001' max='60.0' step='0.01' format='%g'>1.0</defNumber>\n",
            "</defNumberVector>\n",
            "<defSwitchVector device='ESCAM G02 CCD' name='FILTER_SLOT' state='Idle' perm='rw' rule='OneOfMany'>\n",
            "  <defSwitch name='DAY_VIS' label='IR-Cut ON (Day)'>On</defSwitch>\n",
            "  <defSwitch name='NIGHT_HA' label='IR-Cut OFF (H-Alpha/Astro)'>Off</defSwitch>\n",
            "</defSwitchVector>\n",
            // Device 2: ESCAM G02 Mount (Telescope PTZ)
            "<defSwitchVector device='ESCAM G02 Mount' name='CONNECTION' label='Connection' group='Main Control' state='Ok' perm='rw' rule='OneOfMany'>\n",
            "  <defSwitch name='CONNECT' label='Connect'>On</defSwitch>\n",
            "  <defSwitch name='DISCONNECT' label='Disconnect'>Off</defSwitch>\n",
            "</defSwitchVector>\n",
            "<defNumberVector device='ESCAM G02 Mount' name='HORIZONTAL_COORD' label='Horizontal Coordinates' group='Main Control' state='Idle' perm='rw'>\n",
            "  <defNumber name='AZ' label='Azimuth (deg)' min='0.0' max='355.0' step='0.1' format='%3.2f'>177.5</defNumber>\n",
            "  <defNumber name='ALT' label='Altitude (deg)' min='0.0' max='90.0' step='0.1' format='%2.2f'>40.0</defNumber>\n",
            "</defNumberVector>\n",
            "<defSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_MOTION_NS' label='Motion N/S' group='Motion Control' state='Idle' perm='rw' rule='AtMostOne'>\n",
            "  <defSwitch name='MOTION_NORTH' label='North (Tilt Up)'>Off</defSwitch>\n",
            "  <defSwitch name='MOTION_SOUTH' label='South (Tilt Down)'>Off</defSwitch>\n",
            "</defSwitchVector>\n",
            "<defSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_MOTION_WE' label='Motion W/E' group='Motion Control' state='Idle' perm='rw' rule='AtMostOne'>\n",
            "  <defSwitch name='MOTION_WEST' label='West (Pan Left)'>Off</defSwitch>\n",
            "  <defSwitch name='MOTION_EAST' label='East (Pan Right)'>Off</defSwitch>\n",
            "</defSwitchVector>\n",
            "<defSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_ABORT_MOTION' label='Abort Motion' group='Motion Control' state='Idle' perm='rw' rule='AtMostOne'>\n",
            "  <defSwitch name='ABORT' label='Abort'>Off</defSwitch>\n",
            "</defSwitchVector>\n",
            "<defSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_PARK' label='Park' group='Main Control' state='Idle' perm='rw' rule='OneOfMany'>\n",
            "  <defSwitch name='PARK' label='Park (Home)'>Off</defSwitch>\n",
            "  <defSwitch name='UNPARK' label='Unpark'>On</defSwitch>\n",
            "</defSwitchVector>\n",
            "<defSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_TRACK_RATE' label='Tracking Rate' group='Motion Control' state='Idle' perm='rw' rule='OneOfMany'>\n",
            "  <defSwitch name='TRACK_SIDEREAL' label='Sidereal'>Off</defSwitch>\n",
            "  <defSwitch name='TRACK_LUNAR' label='Lunar'>Off</defSwitch>\n",
            "  <defSwitch name='TRACK_SOLAR' label='Solar'>Off</defSwitch>\n",
            "  <defSwitch name='TRACK_OFF' label='Off'>On</defSwitch>\n",
            "</defSwitchVector>\n"
        )
    }

    /// Handles incoming INDI client messages and dispatches mount actions.
    pub async fn process_indi_message(
        request: &str,
        mount_tx: &Option<UnboundedSender<IndiMountAction>>,
    ) -> Option<String> {
        if request.contains("getProperties") {
            return Some(Self::get_definitions_xml().to_string());
        }

        // Handle Switch Vectors
        if request.contains("<newSwitchVector") {
            if request.contains("TELESCOPE_MOTION_NS") {
                if let Some(tx) = mount_tx {
                    if request.contains("<oneSwitch name='MOTION_NORTH'>On</oneSwitch>") {
                        let _ = tx.send(IndiMountAction::Move(MountDirection::North));
                    } else if request.contains("<oneSwitch name='MOTION_SOUTH'>On</oneSwitch>") {
                        let _ = tx.send(IndiMountAction::Move(MountDirection::South));
                    } else if request.contains("Off</oneSwitch>") {
                        let _ = tx.send(IndiMountAction::Stop);
                    }
                }
                return Some("<setSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_MOTION_NS' state='Ok'/>\n".to_string());
            }

            if request.contains("TELESCOPE_MOTION_WE") {
                if let Some(tx) = mount_tx {
                    if request.contains("<oneSwitch name='MOTION_WEST'>On</oneSwitch>") {
                        let _ = tx.send(IndiMountAction::Move(MountDirection::West));
                    } else if request.contains("<oneSwitch name='MOTION_EAST'>On</oneSwitch>") {
                        let _ = tx.send(IndiMountAction::Move(MountDirection::East));
                    } else if request.contains("Off</oneSwitch>") {
                        let _ = tx.send(IndiMountAction::Stop);
                    }
                }
                return Some("<setSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_MOTION_WE' state='Ok'/>\n".to_string());
            }

            if request.contains("TELESCOPE_ABORT_MOTION") {
                if let Some(tx) = mount_tx {
                    let _ = tx.send(IndiMountAction::Stop);
                }
                return Some("<setSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_ABORT_MOTION' state='Ok'/>\n".to_string());
            }

            if request.contains("TELESCOPE_PARK") {
                if let Some(tx) = mount_tx {
                    if request.contains("<oneSwitch name='PARK'>On</oneSwitch>") {
                        let _ = tx.send(IndiMountAction::Park);
                    }
                }
                return Some("<setSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_PARK' state='Ok'/>\n".to_string());
            }

            if request.contains("TELESCOPE_TRACK_RATE") {
                if let Some(tx) = mount_tx {
                    if request.contains("<oneSwitch name='TRACK_OFF'>On</oneSwitch>") {
                        let _ = tx.send(IndiMountAction::SetTracking(false));
                    } else {
                        let _ = tx.send(IndiMountAction::SetTracking(true));
                    }
                }
                return Some("<setSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_TRACK_RATE' state='Ok'/>\n".to_string());
            }
        }

        // Handle Coordinate Slews
        if request.contains("<newNumberVector") && request.contains("HORIZONTAL_COORD") {
            let az = parse_tag_float(request, "AZ").unwrap_or(177.5);
            let alt = parse_tag_float(request, "ALT").unwrap_or(40.0);
            if let Some(tx) = mount_tx {
                let _ = tx.send(IndiMountAction::SetCoords { az, alt });
            }
            return Some(format!(
                "<setNumberVector device='ESCAM G02 Mount' name='HORIZONTAL_COORD' state='Ok'>\n  <oneNumber name='AZ'>{:.2}</oneNumber>\n  <oneNumber name='ALT'>{:.2}</oneNumber>\n</setNumberVector>\n",
                az, alt
            ));
        }

        None
    }

    /// Handles a single incoming INDI client TCP stream.
    pub async fn handle_client(
        mut stream: tokio::net::TcpStream,
        mount_tx: Option<UnboundedSender<IndiMountAction>>,
    ) -> Result<(), std::io::Error> {
        let mut buf = [0u8; 2048];
        loop {
            let n = stream.read(&mut buf).await?;
            if n == 0 {
                return Ok(());
            }

            let request = String::from_utf8_lossy(&buf[..n]);
            if let Some(response) = Self::process_indi_message(&request, &mount_tx).await {
                stream.write_all(response.as_bytes()).await?;
            }
        }
    }

    /// Launch the background INDI server.
    pub async fn run(&self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        info!("INDI server listening on port {}", self.port);
        let mount_tx = self.mount_tx.clone();

        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let tx = mount_tx.clone();
                tokio::spawn(async move {
                    let _ = Self::handle_client(stream, tx).await;
                });
            }
        });

        Ok(())
    }
}

fn parse_tag_float(xml: &str, tag_name: &str) -> Option<f32> {
    let pattern = format!("name='{}'>", tag_name);
    let start = xml.find(&pattern)? + pattern.len();
    let end = xml[start..].find('<')? + start;
    xml[start..end].trim().parse::<f32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc::unbounded_channel;

    #[tokio::test]
    async fn test_indi_server_get_properties() {
        let (tx, _rx) = unbounded_channel();
        let resp = IndiServer::process_indi_message("<getProperties version='1.7'/>\n", &Some(tx))
            .await
            .unwrap();

        assert!(resp.contains("ESCAM G02 CCD"));
        assert!(resp.contains("ESCAM G02 Mount"));
        assert!(resp.contains("HORIZONTAL_COORD"));
        assert!(resp.contains("TELESCOPE_MOTION_NS"));
        assert!(resp.contains("TELESCOPE_MOTION_WE"));
        assert!(resp.contains("TELESCOPE_PARK"));
    }

    #[tokio::test]
    async fn test_indi_motion_commands() {
        let (tx, mut rx) = unbounded_channel();
        let north_cmd = "<newSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_MOTION_NS'><oneSwitch name='MOTION_NORTH'>On</oneSwitch></newSwitchVector>\n";
        let resp = IndiServer::process_indi_message(north_cmd, &Some(tx.clone()))
            .await
            .unwrap();
        assert!(resp.contains("TELESCOPE_MOTION_NS"));
        assert_eq!(rx.recv().await.unwrap(), IndiMountAction::Move(MountDirection::North));

        let stop_cmd = "<newSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_MOTION_NS'><oneSwitch name='MOTION_NORTH'>Off</oneSwitch></newSwitchVector>\n";
        let _ = IndiServer::process_indi_message(stop_cmd, &Some(tx.clone())).await;
        assert_eq!(rx.recv().await.unwrap(), IndiMountAction::Stop);

        let abort_cmd = "<newSwitchVector device='ESCAM G02 Mount' name='TELESCOPE_ABORT_MOTION'><oneSwitch name='ABORT'>On</oneSwitch></newSwitchVector>\n";
        let _ = IndiServer::process_indi_message(abort_cmd, &Some(tx.clone())).await;
        assert_eq!(rx.recv().await.unwrap(), IndiMountAction::Stop);
    }

    #[tokio::test]
    async fn test_indi_coordinate_slews() {
        let (tx, mut rx) = unbounded_channel();
        let slew_cmd = "<newNumberVector device='ESCAM G02 Mount' name='HORIZONTAL_COORD'><oneNumber name='AZ'>245.50</oneNumber><oneNumber name='ALT'>62.30</oneNumber></newNumberVector>\n";
        let resp = IndiServer::process_indi_message(slew_cmd, &Some(tx)).await.unwrap();

        assert!(resp.contains("AZ"));
        assert!(resp.contains("245.50"));
        match rx.recv().await.unwrap() {
            IndiMountAction::SetCoords { az, alt } => {
                assert!((az - 245.50).abs() < 1e-2);
                assert!((alt - 62.30).abs() < 1e-2);
            }
            other => panic!("Expected SetCoords, got {:?}", other),
        }
    }
}
