mod shared;

use axum::Router;
use ::shared::middleware::AppState;

// Student feature routes
// Note: Test endpoints removed. Reference implementation available in git history.
// Hotwire Turbo implementation pattern:
// - Turbo Frames: Use turbo-frame tags with src/target attributes
// - Turbo Streams: Return content-type "text/vnd.turbo-stream.html"
// - Actions: append, prepend, replace, update, remove, before, after
pub fn routes() -> Router<AppState> {
    Router::new()
    // Add real student management routes here
}
