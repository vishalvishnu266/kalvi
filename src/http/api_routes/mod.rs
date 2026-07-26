use crate::http::AppState;
use axum::{
    routing::{get, post},
    Router,
};

use crate::api::{academic as ac, admin as adm, auth as ath, people as pp};

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
        // ── academic ────────────────────────────────────────────
        .route("/academic/years", get(ac::list_years).post(ac::create_year))
        .route("/academic/years/current", get(ac::current_year))
        .route("/academic/years/{id}/activate", post(ac::activate_year))
        .route(
            "/academic/years/{id}/terms",
            get(ac::list_terms).post(ac::create_term),
        )
        .route("/academic/grades", get(ac::list_grades))
        .route("/academic/sections", get(ac::list_sections))
        .route("/academic/rooms", get(ac::list_rooms).post(ac::create_room))
        .route(
            "/academic/subjects",
            get(ac::list_subjects).post(ac::create_subject),
        )
        // ── people (staff) ──────────────────────────────────────
        .route("/people/staff", get(pp::list_staff).post(pp::hire))
        .route(
            "/people/staff/{id}",
            get(pp::get_staff).put(pp::update_staff),
        )
        .route("/people/staff/{id}/terminate", post(pp::terminate))
}
