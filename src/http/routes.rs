use crate::health_probes::Readiness;
use crate::services::ServiceError;
use crate::AppState;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::http::api_routes;
use crate::middleware::tracing as wtr;
use crate::web::{agent as wag, assets as wa, copilot as wc, dsl as wd};

pub type ServiceHttpError = ServiceError;

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            ServiceError::NotFound => (StatusCode::NOT_FOUND, "not_found"),
            ServiceError::Validation(_) => (StatusCode::BAD_REQUEST, "validation_error"),
            ServiceError::Conflict(_) => (StatusCode::CONFLICT, "conflict"),
            ServiceError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            ServiceError::Forbidden(_) => (StatusCode::FORBIDDEN, "forbidden"),
            ServiceError::Hash(_) => (StatusCode::INTERNAL_SERVER_ERROR, "hash_error"),
            ServiceError::Repo(_) => (StatusCode::INTERNAL_SERVER_ERROR, "repo_error"),
            ServiceError::Sqlx(_) => (StatusCode::INTERNAL_SERVER_ERROR, "db_error"),
        };
        let body = Json(serde_json::json!({
            "error":   code,
            "message": self.to_string(),
        }));
        (status, body).into_response()
    }
}

pub fn build_router(state: AppState, readiness: Readiness) -> Router {
    tracing::debug!("build_router: assembling application router");

    let tenant_api = api_routes::tenant_api();

    let global = Router::new()
        .route("/assets/{*path}", get(wa::serve))
        // lit-components/ folder served straight from disk (dev-friendly).
        .route(
            "/lit-components/{*path}",
            get(wa::serve_lit_components),
        )
        // Rust-DSL rendered demo pages (mock data — no services).
        .route("/dsl",            get(wd::index))
        .route("/dsl/students",   get(wd::students_page))
        .route("/dsl/fees",       get(wd::fees_page))
        .route("/dsl/attendance", get(wd::attendance_page))
        .route("/dsl/dashboard",  get(wd::dashboard_page))
        .route("/dsl/icons",      get(wd::icons_page))
        .route("/dsl/layouts",    get(wd::layouts_page))
        .route("/dsl/components", get(wd::components_page))
        .route("/dsl/copilot",    get(wd::copilot_demo_page))
        .route("/dsl/copilot-v2", get(wd::copilot_v2_page))
        .route("/dsl/errors",           get(wd::errors_page))
        .route("/dsl/errors/combos",    get(wd::errors_combos_page))
        .route("/dsl/errors/roundtrip",
               get(wd::errors_roundtrip_get).post(wd::errors_roundtrip_post))
        .route("/dsl/errors/validator",
               get(wd::errors_validator_get).post(wd::errors_validator_post));

    let web_global = crate::http::web::global_routes();
    let portal_global = crate::http::portal::global_routes();
    let admin = crate::http::admin::routes();
    let web_tenant = crate::http::web::routes(state.clone());
    let portal_tenant = crate::http::portal::routes(state.clone());

    // Agentic Copilot mock backend (`/copilot/session`, `/message`, `/history/:id`).
    // In-memory only; safe to construct fresh here — no DB, no LLM.
    let copilot = wc::router(wc::CopilotState::new());

    // Second-generation agent surface: /agent/suggest, /agent/complete,
    // /agent/schema, /agent/invoke — powers <ui-copilot-v2>.
    // All state-less mocks; safe to build inline.
    let agent = wag::router::<AppState>();

    tracing::debug!("build_router: finalizing assembly and adding middleware");
    let router = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/live", get(probe_live))
        .route("/api/ready", get(probe_ready))
        .nest("/admin", admin)
        .nest("/api/{tenant}", tenant_api)
        .nest("/web/{tenant}", web_tenant)
        .nest("/portal/{tenant}", portal_tenant)
        .nest("/web", web_global)
        .nest("/portal", portal_global)
        .merge(copilot)
        .merge(agent)
        .merge(global)
        .with_state(state)
        .layer(axum::Extension(readiness))
        .layer(axum::middleware::from_fn(wtr::trace_request));

    tracing::debug!("build_router: router assembly complete");
    router
}

async fn probe_live() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({ "status": "alive" })),
    )
}

async fn probe_ready(axum::Extension(r): axum::Extension<Readiness>) -> impl IntoResponse {
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
