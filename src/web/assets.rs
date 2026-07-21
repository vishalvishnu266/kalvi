//! Embedded static assets, served from `/assets/*`.
//!
//! Uses `rust-embed` so the binary can be deployed as a single file. In
//! debug builds we pass `--features debug-embed` implicitly (via
//! `Cargo.toml`) so files are read from disk and hot-reloaded on refresh.

use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Static;

pub fn routes() -> Router {
    Router::new().route("/assets/{*path}", get(serve))
}

async fn serve(Path(path): Path<String>) -> Response {
    match Static::get(&path) {
        Some(file) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, mime.as_ref().to_string()),
                    // A hash-suffixed URL scheme would let us cache-forever;
                    // for now, 1-hour cache is a safe default.
                    (header::CACHE_CONTROL, "public, max-age=3600".to_string()),
                ],
                Body::from(file.data.into_owned()),
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "asset not found").into_response(),
    }
}
