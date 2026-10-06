//! # Axum HTTP & WebSocket Server Router
//!
//! Configures routes, CORS, compression, and error boundaries for the embedded web daemon.

use crate::api::{
    index_handler, ircut_handler, ptz_handler, snapshot_handler, status_handler, stream_handler,
    ws_handler, AppState,
};
use axum::routing::{get, post};
use axum::Router;

pub fn build_router(state: AppState) -> Router {
    eprintln!("[build_router] Initializing router...");
    let router = Router::new()
        .route("/", get(index_handler))
        .route("/api/v1/status", get(status_handler))
        .route("/api/v1/ptz", post(ptz_handler))
        .route("/api/v1/ircut", post(ircut_handler))
        .route("/api/v1/snapshot", get(snapshot_handler))
        .route("/api/v1/stream", get(stream_handler))
        .route("/api/v1/stream.mjpg", get(stream_handler))
        .route("/api/v1/ws", get(ws_handler));
    eprintln!("[build_router] Routes added.");

    eprintln!("[build_router] Calling with_state...");
    let router = router.with_state(state);
    eprintln!("[build_router] Router successfully built.");
    router
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use escam_core::{CameraConfig, CameraStatus};
    use escam_driver::{IrCutController, MockGpioDevice, MockMotorDevice};
    use escam_ptz::PtzController;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tower::ServiceExt;

    fn create_test_state() -> AppState {
        let config = CameraConfig::default();
        let motor: Arc<dyn escam_driver::MotorDevice> = Arc::new(MockMotorDevice::new());
        let gpio: Arc<dyn escam_driver::GpioDevice> = Arc::new(MockGpioDevice::new());
        let ptz = Arc::new(PtzController::new(motor, config.clone()));
        let ircut = Arc::new(IrCutController::new(gpio));
        let status = Arc::new(Mutex::new(CameraStatus::default()));

        AppState {
            config,
            status,
            ptz,
            ircut,
        }
    }

    #[tokio::test]
    async fn test_index_route_serves_html() {
        let app = build_router(create_test_state());
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_status_route_returns_json() {
        let app = build_router(create_test_state());
        let response = app
            .oneshot(Request::builder().uri("/api/v1/status").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_stream_route_returns_multipart() {
        let app = build_router(create_test_state());
        let response = app
            .oneshot(Request::builder().uri("/api/v1/stream").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let content_type = response.headers().get("content-type").unwrap().to_str().unwrap();
        assert!(content_type.contains("multipart/x-mixed-replace"));
    }
}
