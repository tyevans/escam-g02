//! # Axum HTTP & WebSocket Server Router
//!
//! Configures routes, CORS, compression, and error boundaries for the embedded web daemon.

use crate::api::{
    app_js_handler, astro_capture_fits_handler, camera_get_handler, camera_post_handler,
    camera_stack_reset_handler, index_handler, ircut_handler, irled_handler, jmuxer_handler,
    ptz_handler, ptz_home_handler, snapshot_handler, status_handler, stream_handler, style_handler, ws_handler,
    AppState,
};
use axum::routing::{get, post};
use axum::Router;

pub fn build_router(state: AppState) -> Router {
    eprintln!("[build_router] Initializing router...");
    let router = Router::new()
        .route("/", get(index_handler))
        .route("/style.css", get(style_handler))
        .route("/app.js", get(app_js_handler))
        .route("/jmuxer.min.js", get(jmuxer_handler))
        .route("/api/v1/status", get(status_handler))
        .route("/api/v1/camera", get(camera_get_handler).post(camera_post_handler))
        .route("/api/v1/camera/stack/reset", post(camera_stack_reset_handler))
        .route("/api/v1/ptz", post(ptz_handler))
        .route("/api/v1/ptz/home", post(ptz_home_handler))
        .route("/api/v1/ircut", post(ircut_handler))
        .route("/api/v1/irled", post(irled_handler))
        .route("/api/v1/snapshot", get(snapshot_handler))
        .route("/api/v1/stream", get(stream_handler))
        .route("/api/v1/stream.mjpg", get(stream_handler))
        .route("/api/v1/astro/capture.fits", get(astro_capture_fits_handler))
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

        let frame = Arc::new(tokio::sync::RwLock::new(
            crate::assets::LIVE_FRAME_JPEG.to_vec(),
        ));
        let (video_broadcast, _) = tokio::sync::broadcast::channel(16);
        let stacker = Arc::new(Mutex::new(None));

        AppState {
            config,
            status,
            ptz,
            ircut,
            frame,
            video_broadcast,
            stacker,
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
    async fn test_style_and_script_routes() {
        let app = build_router(create_test_state());
        let css_resp = app
            .clone()
            .oneshot(Request::builder().uri("/style.css").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(css_resp.status(), StatusCode::OK);

        let js_resp = app
            .oneshot(Request::builder().uri("/app.js").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(js_resp.status(), StatusCode::OK);
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
    async fn test_camera_controls_routes_get_and_post() {
        use http_body_util::BodyExt;
        let app = build_router(create_test_state());

        // GET current controls
        let get_resp = app
            .clone()
            .oneshot(Request::builder().uri("/api/v1/camera").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(get_resp.status(), StatusCode::OK);

        // POST update with 10s exposure and 16x gain
        let payload = r#"{"exposure_secs": 10.0, "gain": 16.0, "resolution": "Astro1280x960", "stack_mode": "Average"}"#;
        let post_resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/camera")
                    .header("content-type", "application/json")
                    .body(Body::from(payload))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(post_resp.status(), StatusCode::OK);

        let bytes = post_resp.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["success"], true);
        assert_eq!(json["controls"]["exposure_secs"], 10.0);
        assert_eq!(json["controls"]["gain"], 16.0);
        assert!((json["controls"]["target_fps"].as_f64().unwrap() - 0.1).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_camera_stack_reset_route() {
        let app = build_router(create_test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/camera/stack/reset")
                    .body(Body::empty())
                    .unwrap(),
            )
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

    #[tokio::test]
    async fn test_astro_capture_fits_route_returns_valid_fits() {
        use http_body_util::BodyExt;
        let app = build_router(create_test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/astro/capture.fits?exposure=10.0")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let content_type = response.headers().get("content-type").unwrap().to_str().unwrap();
        assert_eq!(content_type, "image/fits");

        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body_bytes.len() % 2880, 0);
        let header_start = String::from_utf8_lossy(&body_bytes[..80]);
        assert!(header_start.starts_with("SIMPLE  = T"));
    }

    #[tokio::test]
    async fn test_irled_route_toggles_state() {
        let app = build_router(create_test_state());
        let payload = r#"{"enabled": true}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/irled")
                    .header("content-type", "application/json")
                    .body(Body::from(payload))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}

