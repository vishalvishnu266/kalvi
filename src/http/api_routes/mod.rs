//! Tenant-scoped and admin JSON API route trees.
//!
//! Business modules have been stripped. The tenant router now
//! carries only:
//!   * `auth` — register / login / whoami / change-password.
//!   * `demo` — a tiny reference module showing how to wire a new
//!     domain (see `src/api/demo.rs` + `src/services/demo.rs`).

use crate::http::AppState;
use axum::{
    routing::{get, post},
    Router,
};

use crate::api::{admin as adm, auth as ath, demo as dm};

pub fn admin_api() -> Router<AppState> {
    Router::new()
        .route("/tenants", get(adm::list).post(adm::create))
        .route(
            "/tenants/{tenant_id}",
            get(adm::get_one).put(adm::update).delete(adm::soft_delete),
        )
        .route("/tenants/{tenant_id}/enable", post(adm::enable))
        .route("/tenants/{tenant_id}/disable", post(adm::disable))
}

pub fn tenant_api() -> Router<AppState> {
    Router::new()
        // ── auth ────────────────────────────────────────────────
        .route("/auth/register", post(ath::register))
        .route("/auth/login", post(ath::login))
        .route("/auth/change-password", post(ath::change_password))
        .route("/auth/whoami", get(ath::whoami))
        // ── demo (reference wiring — delete once real modules land) ─
        .route("/demo/ping", get(dm::ping))
        .route("/demo/messages", get(dm::list))
        .route("/demo/echo", post(dm::echo))
}
