use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Static;

pub async fn serve(Path(path): Path<String>) -> Response {
    match Static::get(&path) {
        Some(file) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, mime.as_ref().to_string()),
                    (header::CACHE_CONTROL, "public, max-age=3600".to_string()),
                ],
                Body::from(file.data.into_owned()),
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "asset not found").into_response(),
    }
}

/// Serve the `lit-components/` folder off disk (dev-friendly — no rebuild
/// needed when JS/CSS changes). Path is resolved via `CARGO_MANIFEST_DIR`
/// so it works regardless of the current working directory.
///
/// Includes a defensive `..` check to prevent path-traversal escapes.
pub async fn serve_lit_components(Path(path): Path<String>) -> Response {
    // Reject any component that could escape the root directory.
    if path.split('/').any(|seg| seg == ".." || seg.contains('\0')) {
        return (StatusCode::BAD_REQUEST, "invalid path").into_response();
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lit-components");
    let mut full = root.clone();
    for seg in path.split('/') { full.push(seg); }

    // Defence-in-depth: canonicalise if possible, and refuse anything outside `root`.
    if let (Ok(rc), Ok(fc)) = (root.canonicalize(), full.canonicalize()) {
        if !fc.starts_with(&rc) {
            return (StatusCode::BAD_REQUEST, "invalid path").into_response();
        }
    }

    match tokio::fs::read(&full).await {
        Ok(bytes) => {
            let mime = mime_guess::from_path(&full).first_or_octet_stream();
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, mime.as_ref().to_string()),
                    // JS modules must be served with the JS MIME; mime_guess handles that.
                    // Cache short — we're editing components live in dev.
                    (header::CACHE_CONTROL, "public, max-age=60".to_string()),
                ],
                Body::from(bytes),
            )
                .into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}
