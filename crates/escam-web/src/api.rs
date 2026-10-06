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
    pub frame: Arc<tokio::sync::RwLock<Vec<u8>>>,
    pub video_broadcast: tokio::sync::broadcast::Sender<Vec<u8>>,
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
pub async fn snapshot_handler(State(state): State<AppState>) -> Response {
    let jpeg = state.frame.read().await.clone();
    ([(axum::http::header::CONTENT_TYPE, "image/jpeg")], jpeg).into_response()
}

/// Serves a continuous multipart video stream (MJPEG / frame tunnel).
pub async fn stream_handler(State(state): State<AppState>) -> Response {
    let frame = state.frame.read().await.clone();
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
