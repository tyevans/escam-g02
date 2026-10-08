//! # ESCAM Web Server and Snappy Modern SPA Crate
//!
//! Provides the embedded Axum HTTP server, REST endpoints, WebSocket signaling,
//! embedded responsive dark-mode Single-Page Application (SPA), and OpenAPI 3.1 docs.

pub mod api;
pub mod assets;
pub mod openapi;
pub mod server;

pub use api::AppState;
pub use openapi::OpenApiRegistry;
pub use server::build_router;
