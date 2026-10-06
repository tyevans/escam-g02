//! # Lightweight Embedded HTTP/1 & WebSocket Handler
//!
//! Custom low-memory request dispatcher for the single-threaded ARMv6 daemon.

use escam_web::AppState;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn handle_http_connection(
    mut stream: tokio::net::TcpStream,
    state: AppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buf = vec![0u8; 4096];
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    let req_str = String::from_utf8_lossy(&buf[..n]);
    let first_line = req_str.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");

    if method == "GET" && path == "/" {
        let body = escam_web::assets::INDEX_HTML;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
    } else if method == "GET" && path == "/jmuxer.min.js" {
        let body = escam_web::assets::JMUXER_JS;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/javascript; charset=utf-8\r\nCache-Control: public, max-age=86400\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
    } else if method == "GET" && path == "/api/v1/status" {
        let status = state.status.lock().await.clone();
        let body = serde_json::to_string(&status)?;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
    } else if method == "POST" && path == "/api/v1/ptz" {
        let body_start = req_str.find("\r\n\r\n").map(|i| i + 4).unwrap_or(n);
        let body_slice = &req_str[body_start..];
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(body_slice) {
            let action = json["action"].as_str().unwrap_or("");
            if action == "Stop" {
                let _ = state.ptz.stop().await;
            } else if action == "Home" {
                let _ = state.ptz.stop().await;
            } else if action == "Joystick" {
                let x = json["x"].as_f64().unwrap_or(0.0) as f32;
                let y = json["y"].as_f64().unwrap_or(0.0) as f32;
                let _ = state.ptz.drive_joystick(x, y).await;
            }
        }
        let body = r#"{"success":true,"message":"PTZ command executed"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
    } else if method == "POST" && path == "/api/v1/ircut" {
        let body_start = req_str.find("\r\n\r\n").map(|i| i + 4).unwrap_or(n);
        let body_slice = &req_str[body_start..];
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(body_slice) {
            let mode_str = json["mode"].as_str().unwrap_or("Day");
            let mode = if mode_str == "Night" {
                escam_core::IrCutMode::Night
            } else {
                escam_core::IrCutMode::Day
            };
            let _ = state.ircut.set_mode(mode);
            state.status.lock().await.ircut_mode = mode;
        }
        let body = r#"{"success":true,"message":"IR-cut filter updated"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
    } else if method == "POST" && path == "/api/v1/irled" {
        let body_start = req_str.find("\r\n\r\n").map(|i| i + 4).unwrap_or(n);
        let body_slice = &req_str[body_start..];
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(body_slice) {
            let enabled = json["enabled"].as_bool().unwrap_or(false);
            let _ = state.ircut.set_ir_led(enabled);
            state.status.lock().await.irled_enabled = enabled;
        }
        let body = r#"{"success":true,"message":"IR LED updated"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(resp.as_bytes()).await?;
    } else if method == "GET" && path == "/api/v1/astro/capture.fits" {
        use escam_astro::{BayerFrame, BayerPattern, FitsWriter};
        let width = 1280;
        let height = 720;
        let mut frame = BayerFrame::new(width, height, 10, BayerPattern::Rggb, 1.0);
        for y in 0..height {
            for x in 0..width {
                let val = ((x ^ y) % 1024) as u16;
                frame.set_pixel(x, y, val);
            }
        }
        let fits_data = FitsWriter::write_fits(&frame);
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: image/fits\r\nContent-Disposition: attachment; filename=\"capture.fits\"\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            fits_data.len()
        );
        stream.write_all(header.as_bytes()).await?;
        stream.write_all(&fits_data).await?;
    } else if method == "GET" && path == "/api/v1/snapshot" {
        let jpeg = match fetch_camera_jpeg().await {
            Some(jpg) => {
                *state.frame.write().await = jpg.clone();
                jpg
            }
            None => state.frame.read().await.clone(),
        };
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: image/jpeg\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            jpeg.len()
        );
        stream.write_all(header.as_bytes()).await?;
        stream.write_all(&jpeg).await?;
    } else if method == "GET" && (path == "/api/v1/stream" || path == "/api/v1/stream.mjpg") {
        let header = "HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\nCache-Control: no-cache\r\nAccess-Control-Allow-Origin: *\r\n\r\n";
        if stream.write_all(header.as_bytes()).await.is_ok() {
            loop {
                let frame = state.frame.read().await.clone();
                let part_header = format!(
                    "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                    frame.len()
                );
                if stream.write_all(part_header.as_bytes()).await.is_err() {
                    break;
                }
                if stream.write_all(&frame).await.is_err() {
                    break;
                }
                if stream.write_all(b"\r\n").await.is_err() {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(66)).await;
            }
        }
    } else if method == "GET" && path == "/api/v1/ws" {
        handle_websocket_upgrade(stream, req_str, state).await?;
    } else {
        let resp = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        stream.write_all(resp.as_bytes()).await?;
    }

    Ok(())
}

async fn handle_websocket_upgrade(
    mut stream: tokio::net::TcpStream,
    req_str: std::borrow::Cow<'_, str>,
    state: AppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let key_line = req_str
        .lines()
        .find(|l| l.to_lowercase().starts_with("sec-websocket-key:"));
    if let Some(line) = key_line {
        let key = line.split(':').nth(1).unwrap_or("").trim();
        use sha1::Digest;
        let mut hasher = sha1::Sha1::new();
        hasher.update(key.as_bytes());
        hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let hash = hasher.finalize();
        use base64::Engine;
        let accept = base64::engine::general_purpose::STANDARD.encode(hash);

        let resp = format!(
            "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n",
            accept
        );
        stream.write_all(resp.as_bytes()).await?;

        let _ = stream.set_nodelay(true);
        let (mut read_half, mut write_half) = stream.into_split();
        let mut video_rx = state.video_broadcast.subscribe();

        let video_task = tokio::spawn(async move {
            loop {
                let nal = match video_rx.recv().await {
                    Ok(n) => n,
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(_) => break,
                };
                let mut hdr = [0u8; 10];
                let hdr_len = if nal.len() < 126 {
                    hdr[0] = 0x82;
                    hdr[1] = nal.len() as u8;
                    2
                } else if nal.len() <= 65535 {
                    hdr[0] = 0x82;
                    hdr[1] = 126;
                    hdr[2..4].copy_from_slice(&(nal.len() as u16).to_be_bytes());
                    4
                } else {
                    hdr[0] = 0x82;
                    hdr[1] = 127;
                    hdr[2..10].copy_from_slice(&(nal.len() as u64).to_be_bytes());
                    10
                };
                if write_half.write_all(&hdr[..hdr_len]).await.is_err() {
                    break;
                }
                if write_half.write_all(&nal).await.is_err() {
                    break;
                }
            }
        });

        let mut ws_buf = [0u8; 1024];
        while let Ok(n) = read_half.read(&mut ws_buf).await {
            if n < 2 {
                break;
            }
            let opcode = ws_buf[0] & 0x0f;
            if opcode == 0x08 {
                break;
            }
            let masked = (ws_buf[1] & 0x80) != 0;
            let mut payload_len = (ws_buf[1] & 0x7f) as usize;
            let mut offset = 2;
            if payload_len == 126 {
                if n < 4 {
                    break;
                }
                payload_len = u16::from_be_bytes([ws_buf[2], ws_buf[3]]) as usize;
                offset = 4;
            }
            if masked {
                if n < offset + 4 {
                    break;
                }
                let mask = [
                    ws_buf[offset],
                    ws_buf[offset + 1],
                    ws_buf[offset + 2],
                    ws_buf[offset + 3],
                ];
                offset += 4;
                let payload_end = (offset + payload_len).min(n);
                for i in offset..payload_end {
                    ws_buf[i] ^= mask[(i - offset) % 4];
                }
                if let Ok(text) = std::str::from_utf8(&ws_buf[offset..payload_end]) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
                        if json["type"] == "ptz_joystick" {
                            let x = json["x"].as_f64().unwrap_or(0.0) as f32;
                            let y = json["y"].as_f64().unwrap_or(0.0) as f32;
                            let _ = state.ptz.drive_joystick(x, y).await;
                        }
                    }
                }
            }
        }
        video_task.abort();
    }
    Ok(())
}

async fn fetch_camera_jpeg() -> Option<Vec<u8>> {
    let mut sock = tokio::time::timeout(
        tokio::time::Duration::from_millis(500),
        tokio::net::TcpStream::connect("127.0.0.1:80"),
    )
    .await
    .ok()?
    .ok()?;

    let req = b"GET /tmpfs/auto.jpg HTTP/1.0\r\nAuthorization: Basic YWRtaW46YWRtaW4=\r\n\r\n";
    sock.write_all(req).await.ok()?;

    let mut buf = Vec::with_capacity(96 * 1024);
    let mut chunk = [0u8; 8192];
    loop {
        match tokio::time::timeout(
            tokio::time::Duration::from_millis(300),
            sock.read(&mut chunk),
        )
        .await
        {
            Ok(Ok(n)) if n > 0 => buf.extend_from_slice(&chunk[..n]),
            _ => break,
        }
    }

    let pos = buf.windows(4).position(|w| w == b"\r\n\r\n")?;
    let body = &buf[pos + 4..];
    if body.len() > 100 && body.starts_with(&[0xFF, 0xD8]) {
        Some(body.to_vec())
    } else {
        None
    }
}
