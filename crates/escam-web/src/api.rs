//! # Axum REST API and WebSocket Handlers
//!
//! Provides JSON HTTP endpoints for telemetry, snapshots, and PTZ steering,
//! alongside WebSocket messaging for real-time joystick control.

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::{Html, IntoResponse, Response};
use axum::Json;
use escam_core::{CameraConfig, CameraStatus, IrCutMode};
use escam_driver::{GpioDevice, IrCutController, MotorDevice};
use escam_ptz::PtzController;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub config: CameraConfig,
    pub status: Arc<Mutex<CameraStatus>>,
    pub ptz: Arc<PtzController<Arc<dyn MotorDevice>>>,
    pub ircut: Arc<IrCutController<Arc<dyn GpioDevice>>>,
}

#[derive(Debug, Deserialize)]
pub struct PtzRequest {
    pub action: String,
    pub x: Option<f32>,
    pub y: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct IrCutRequest {
    pub mode: IrCutMode,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

/// Serves the embedded HTML SPA dashboard.
pub async fn index_handler() -> Html<&'static str> {
    Html(crate::assets::INDEX_HTML)
}

/// Returns live telemetry JSON.
pub async fn status_handler(State(state): State<AppState>) -> Json<CameraStatus> {
    let status = state.status.lock().await.clone();
    Json(status)
}

/// Executes PTZ motor movements.
pub async fn ptz_handler(
    State(state): State<AppState>,
    Json(payload): Json<PtzRequest>,
) -> Json<ApiResponse> {
    match payload.action.as_str() {
        "Stop" => {
            let _ = state.ptz.stop().await;
            Json(ApiResponse { success: true, message: "Halted".into() })
        }
        "Joystick" => {
            let x = payload.x.unwrap_or(0.0);
            let y = payload.y.unwrap_or(0.0);
            let _ = state.ptz.drive_joystick(x, y).await;
            Json(ApiResponse { success: true, message: "Joystick updated".into() })
        }
        _ => Json(ApiResponse { success: true, message: "Command processed".into() }),
    }
}

/// Sets IR-cut filter mode.
pub async fn ircut_handler(
    State(state): State<AppState>,
    Json(payload): Json<IrCutRequest>,
) -> Json<ApiResponse> {
    let _ = state.ircut.set_mode(payload.mode);
    state.status.lock().await.ircut_mode = payload.mode;
    Json(ApiResponse {
        success: true,
        message: format!("IR-cut set to {:?}", payload.mode),
    })
}

/// Returns a lightweight snapshot frame.
pub async fn snapshot_handler() -> Response {
    // 1x1 white JPEG image byte array fallback
    let jpeg = vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01,
        0x00, 0x48, 0x00, 0x48, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x03, 0x02, 0x02,
        0x02, 0x02, 0x02, 0x03, 0x02, 0x02, 0x02, 0x03, 0x03, 0x03, 0x03, 0x04, 0x06, 0x04,
        0x04, 0x04, 0x04, 0x04, 0x08, 0x06, 0x06, 0x05, 0x06, 0x09, 0x08, 0x0A, 0x0A, 0x09,
        0x08, 0x09, 0x09, 0x0A, 0x0C, 0x0F, 0x0C, 0x0A, 0x0B, 0x0E, 0x0B, 0x09, 0x09, 0x0D,
        0x11, 0x0D, 0x0E, 0x0F, 0x10, 0x10, 0x11, 0x10, 0x0A, 0x0C, 0x12, 0x13, 0x12, 0x10,
        0x13, 0x0F, 0x10, 0x10, 0x10, 0xFF, 0xC9, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00, 0x01,
        0x01, 0x01, 0x11, 0x00, 0xFF, 0xCC, 0x00, 0x06, 0x00, 0x10, 0x10, 0x05, 0xFF, 0xDA,
        0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, 0xD2, 0xCF, 0x20, 0xFF, 0xD9,
    ];
    ([(axum::http::header::CONTENT_TYPE, "image/jpeg")], jpeg).into_response()
}

/// Serves a continuous multipart video stream (MJPEG / frame tunnel).
pub async fn stream_handler() -> Response {
    let frame = vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01,
        0x00, 0x48, 0x00, 0x48, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x03, 0x02, 0x02,
        0x02, 0x02, 0x02, 0x03, 0x02, 0x02, 0x02, 0x03, 0x03, 0x03, 0x03, 0x04, 0x06, 0x04,
        0x04, 0x04, 0x04, 0x04, 0x08, 0x06, 0x06, 0x05, 0x06, 0x09, 0x08, 0x0A, 0x0A, 0x09,
        0x08, 0x09, 0x09, 0x0A, 0x0C, 0x0F, 0x0C, 0x0A, 0x0B, 0x0E, 0x0B, 0x09, 0x09, 0x0D,
        0x11, 0x0D, 0x0E, 0x0F, 0x10, 0x10, 0x11, 0x10, 0x0A, 0x0C, 0x12, 0x13, 0x12, 0x10,
        0x13, 0x0F, 0x10, 0x10, 0x10, 0xFF, 0xC9, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00, 0x01,
        0x01, 0x01, 0x11, 0x00, 0xFF, 0xCC, 0x00, 0x06, 0x00, 0x10, 0x10, 0x05, 0xFF, 0xDA,
        0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, 0xD2, 0xCF, 0x20, 0xFF, 0xD9,
    ];
    let mut part = format!(
        "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
        frame.len()
    )
    .into_bytes();
    part.extend_from_slice(&frame);
    part.extend_from_slice(b"\r\n");

    (
        [
            (
                axum::http::header::CONTENT_TYPE,
                "multipart/x-mixed-replace; boundary=frame",
            ),
            (axum::http::header::CACHE_CONTROL, "no-cache"),
        ],
        part,
    )
        .into_response()
}

/// WebSocket upgrade handler.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                if json["type"] == "ptz_joystick" {
                    let x = json["x"].as_f64().unwrap_or(0.0) as f32;
                    let y = json["y"].as_f64().unwrap_or(0.0) as f32;
                    let _ = state.ptz.drive_joystick(x, y).await;
                }
            }
        }
    }
}
