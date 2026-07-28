use axum::{
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use crate::infrastructure::readiness_probe::ReadinessProbe;

pub async fn health() -> &'static str {
    "ok"
}

pub async fn live() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({ "status": "alive" })),
    )
}

pub async fn ready(Extension(r): Extension<ReadinessProbe>) -> impl IntoResponse {
    if r.is_ready() {
        (
            StatusCode::OK,
            Json(serde_json::json!({ "status": "ready" })),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "status": "draining" })),
        )
    }
}
