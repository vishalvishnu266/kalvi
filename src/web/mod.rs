//! Server-rendered web UI (Hotwire + Askama).
//!
//! This module is intentionally **decoupled** from the JSON API in
//! [`crate::api`]. It mounts at `/` (root) and serves an HTML SPA-like
//! experience powered by Turbo (from Hotwire), so navigation feels instant
//! without shipping a JS framework.
//!
//! ## Design goals
//! * **Server-rendered** — no build step, no frontend framework, no JSON hydration.
//! * **Hotwire-ready** — `<turbo-frame>` and `data-turbo-*` attributes are
//!   used throughout so the same HTML powers Hotwire Native on iOS/Android.
//! * **Responsive** — desktop shows a sidebar, mobile shows a bottom tab bar.
//! * **Dark mode** — `class="dark"` toggled via a small inline script; the
//!   choice is stored in `localStorage` and applied *before paint* to avoid
//!   the flash of unstyled content.
//! * **macOS-like typography** — the system font stack (`-apple-system`,
//!   `SF Pro`, `Segoe UI`) gives the app a native feel across platforms.

use axum::Router;

use crate::http::middleware::{TenantScopeState, TenantSource, tenant_scope};
use crate::tenancy::TenantRegistry;

pub mod assets;
pub mod auth;
pub mod dashboard;
pub mod error;
pub mod filters;
pub mod layout;
pub mod students;

/// Build the web router mounted at `/`.
///
/// Public (non-tenant-scoped) routes: `/login`, `/logout`, `/assets/*`.
///
/// Tenant-scoped routes: everything else. They read the tenant from a
/// cookie (`tenant_id`) that's set at login. If no cookie is present we
/// bounce the user to `/login`.
pub fn build_web_router(tenants: TenantRegistry) -> Router {
    // The web layer reads the tenant from a cookie set at login rather than
    // from a header (the API's default). We still reuse the same
    // `tenant_scope` middleware, just with a different `TenantSource`.
    let tenant_state = TenantScopeState::new(tenants)
        .with_source(TenantSource::Header("x-tenant-id".into()));

    // App-shell (tenant-scoped) routes.
    let app_routes = Router::new()
        .merge(dashboard::routes())
        .merge(students::routes())
        .layer(axum::middleware::from_fn(auth::require_session))
        .layer(axum::middleware::from_fn_with_state(
            tenant_state.clone(),
            tenant_scope,
        ))
        .layer(axum::middleware::from_fn(auth::cookie_to_tenant_header))
        .with_state(tenant_state.clone());

    Router::new()
        .merge(assets::routes())
        .merge(auth::public_routes(tenant_state))
        .merge(app_routes)
}
