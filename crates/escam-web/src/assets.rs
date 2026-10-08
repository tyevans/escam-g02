//! # Embedded Static Assets for Snappy Modern SPA
//!
//! Provides embedded single-page application loaded directly
//! from camera memory without external CDN requests.
//!
//! Features dual theming:
//! - Surrealist Dada Mode (Max Ernst 'L'Oeil Céleste', decalcomania & brass)
//! - FNAF Security Station Mode (Five Nights at Freddy's CRT surveillance monitor)

pub const INDEX_HTML: &str = include_str!("index.html");
pub const STYLE_CSS: &str = include_str!("style.css");
pub const APP_JS: &str = include_str!("app.js");
pub const JMUXER_JS: &str = include_str!("jmuxer.min.js");
pub const LIVE_FRAME_JPEG: &[u8] = include_bytes!("frame.jpg");
