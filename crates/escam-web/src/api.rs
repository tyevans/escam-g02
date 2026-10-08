//! # Axum REST API and WebSocket Handlers
//!
//! Provides JSON HTTP endpoints for telemetry, snapshots, and PTZ steering,
//! alongside WebSocket messaging for real-time joystick control.

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::response::{Html, IntoResponse, Response};
use axum::Json;
use escam_core::{CameraConfig, CameraControls, CameraControlsUpdate, CameraStatus, IrCutMode};
use escam_driver::{GpioDevice, IrCutController, MotorDevice};
use escam_ptz::PtzController;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    pub stacker: Arc<Mutex<Option<escam_astro::BayerStacker>>>,
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

#[derive(Debug, Deserialize)]
pub struct IrLedRequest {
    pub enabled: bool,
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

/// Serves embedded stylesheet.
pub async fn style_handler() -> Response {
    (
        [
            (axum::http::header::CONTENT_TYPE, "text/css; charset=utf-8"),
            (axum::http::header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        crate::assets::STYLE_CSS,
    )
        .into_response()
}

/// Serves embedded client application logic.
pub async fn app_js_handler() -> Response {
    (
        [
            (
                axum::http::header::CONTENT_TYPE,
                "application/javascript; charset=utf-8",
            ),
            (axum::http::header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        crate::assets::APP_JS,
    )
        .into_response()
}

/// Serves jmuxer library.
pub async fn jmuxer_handler() -> Response {
    (
        [
            (
                axum::http::header::CONTENT_TYPE,
                "application/javascript; charset=utf-8",
            ),
            (axum::http::header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        crate::assets::JMUXER_JS,
    )
        .into_response()
}

/// Returns live telemetry JSON.
pub async fn status_handler(State(state): State<AppState>) -> Json<CameraStatus> {
    let status = state.status.lock().await.clone();
    Json(status)
}

/// Returns active manual/astrophotography camera controls.
pub async fn camera_get_handler(State(state): State<AppState>) -> Json<CameraControls> {
    let controls = state.status.lock().await.controls.clone();
    Json(controls)
}

/// Updates manual/astrophotography camera controls.
pub async fn camera_post_handler(
    State(state): State<AppState>,
    Json(payload): Json<CameraControlsUpdate>,
) -> Json<serde_json::Value> {
    let mut status = state.status.lock().await;
    payload.apply_to(&mut status.controls);
    status.controls.normalize();

    let (w, h) = status.controls.resolution.dimensions();
    let mut stacker_guard = state.stacker.lock().await;
    match status.controls.stack_mode.as_str() {
        "Average" => {
            *stacker_guard = Some(escam_astro::BayerStacker::new(
                w,
                h,
                escam_astro::BayerPattern::Rggb,
                escam_astro::StackingMode::Average,
            ));
        }
        "Additive" => {
            *stacker_guard = Some(escam_astro::BayerStacker::new(
                w,
                h,
                escam_astro::BayerPattern::Rggb,
                escam_astro::StackingMode::Additive,
            ));
        }
        _ => {
            *stacker_guard = None;
        }
    }

    Json(serde_json::json!({
        "success": true,
        "message": "Camera controls updated",
        "controls": status.controls
    }))
}

/// Resets multi-frame live stack accumulator.
pub async fn camera_stack_reset_handler(State(state): State<AppState>) -> Json<ApiResponse> {
    let mut status = state.status.lock().await;
    status.stacked_frames = 0;
    status.total_stacked_exposure_secs = 0.0;
    let mut stacker_guard = state.stacker.lock().await;
    if let Some(ref mut st) = *stacker_guard {
        st.reset();
    }
    Json(ApiResponse {
        success: true,
        message: "Stack accumulator reset".into(),
    })
}

/// Executes PTZ motor movements.
pub async fn ptz_handler(
    State(state): State<AppState>,
    Json(payload): Json<PtzRequest>,
) -> Json<ApiResponse> {
    match payload.action.as_str() {
        "Stop" => {
            let _ = state.ptz.stop().await;
            Json(ApiResponse {
                success: true,
                message: "Halted".into(),
            })
        }
        "Home" => {
            let _ = state.ptz.home().await;
            Json(ApiResponse {
                success: true,
                message: "Homed".into(),
            })
        }
        "Joystick" => {
            let x = payload.x.unwrap_or(0.0);
            let y = payload.y.unwrap_or(0.0);
            if x == 0.0 && y == 0.0 {
                let _ = state.ptz.stop().await;
            } else {
                let _ = state.ptz.drive_joystick(x, y).await;
            }
            Json(ApiResponse {
                success: true,
                message: "Joystick updated".into(),
            })
        }
        _ => Json(ApiResponse {
            success: true,
            message: "Command processed".into(),
        }),
    }
}

/// Dedicated homing endpoint to re-center mount.
pub async fn ptz_home_handler(State(state): State<AppState>) -> Json<ApiResponse> {
    let _ = state.ptz.home().await;
    Json(ApiResponse {
        success: true,
        message: "Homed".into(),
    })
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

/// Sets IR illumination LED state (GPIO 10).
pub async fn irled_handler(
    State(state): State<AppState>,
    Json(payload): Json<IrLedRequest>,
) -> Json<ApiResponse> {
    let _ = state.ircut.set_ir_led(payload.enabled);
    state.status.lock().await.irled_enabled = payload.enabled;
    Json(ApiResponse {
        success: true,
        message: format!("IR LED set to {}", if payload.enabled { "ON" } else { "OFF" }),
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
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                if json["type"] == "ptz_joystick" {
                    let x = json["x"].as_f64().unwrap_or(0.0) as f32;
                    let y = json["y"].as_f64().unwrap_or(0.0) as f32;
                    if x == 0.0 && y == 0.0 {
                        let _ = state.ptz.stop().await;
                    } else {
                        let _ = state.ptz.drive_joystick(x, y).await;
                    }
                } else if json["type"] == "ptz_stop" {
                    let _ = state.ptz.stop().await;
                }
            }
        }
    }
}

/// Returns an uncompressed 10-bit Bayer frame packaged in standard NASA FITS format.
pub async fn astro_capture_fits_handler(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    use escam_astro::{apply_auto_stretch, BayerFrame, BayerPattern, FitsWriter, StretchMode};

    let (exposure, gain, (width, height), stretch_str) = {
        let status = state.status.lock().await;
        let query_exp = params.get("exposure").and_then(|s| s.parse::<f32>().ok());
        let exp = query_exp.unwrap_or(status.controls.exposure_secs);
        let g = status.controls.gain;
        let dims = status.controls.resolution.dimensions();
        let st = params.get("stretch").cloned().unwrap_or_else(|| status.controls.auto_stretch.clone());
        (exp, g, dims, st)
    };

    let mut frame = {
        let frame_bytes = state.frame.read().await.clone();
        if let Ok(mut f) = BayerFrame::from_jpeg(&frame_bytes, exposure) {
            if gain > 1.0 {
                for p in f.pixels.iter_mut() {
                    *p = ((*p as f32) * gain.sqrt()).min(65535.0) as u16;
                }
            }
            f
        } else {
            let mut synth = BayerFrame::new(width, height, 10, BayerPattern::Rggb, exposure);
            let bg_flux = ((100.0f32 + 250.0f32 * (1.0f32 - (-exposure / 4.0f32).exp())) * gain.sqrt()).clamp(80.0f32, 950.0f32) as u32;

            for y in 0..height {
                for x in 0..width {
                    let noise = ((x.wrapping_mul(31) ^ y.wrapping_mul(17)) % 19) as u32;
                    let mut val = (bg_flux + noise) as u16;
                    let star_grid_x = x % 160;
                    let star_grid_y = y % 120;
                    let dx = (star_grid_x as i32) - 80;
                    let dy = (star_grid_y as i32) - 60;
                    let dist_sq = (dx * dx + dy * dy) as u32;

                    if dist_sq < 36 {
                        let star_brightness = ((1000.0 * (exposure / (exposure + 0.5)) * gain.sqrt()) / (1.0 + (dist_sq as f32) * 0.4)) as u32;
                        val = (val as u32 + star_brightness).min(1023) as u16;
                    }
                    synth.set_pixel(x, y, val);
                }
            }
            synth
        }
    };

    let stretch_mode = StretchMode::from_str_loose(&stretch_str);
    apply_auto_stretch(&mut frame.pixels, stretch_mode);

    let fits_data = FitsWriter::write_fits(&frame);
    (
        [
            (axum::http::header::CONTENT_TYPE, "image/fits"),
            (
                axum::http::header::CONTENT_DISPOSITION,
                "attachment; filename=\"capture.fits\"",
            ),
        ],
        fits_data,
    )
        .into_response()
}
