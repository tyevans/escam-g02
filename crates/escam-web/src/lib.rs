//! # ESCAM Web Server and Snappy Modern SPA Crate
//!
//! Provides the embedded Axum HTTP server, REST endpoints, WebSocket signaling,
//! and embedded responsive dark-mode Single-Page Application (SPA).

pub mod api;
pub mod assets;
pub mod server;

pub use api::AppState;
pub use server::build_router;
