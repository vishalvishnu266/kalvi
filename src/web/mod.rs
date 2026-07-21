//! Server-rendered web UI (Hotwire + Askama).
//!
//! This module is intentionally **decoupled** from the JSON API in
//! [`crate::api`]. It serves an HTML SPA-like experience powered by Turbo
//! (from Hotwire), so navigation feels instant without shipping a JS
//! framework.
//!
//! ## URL layout
//! * Public: `/login`, `/logout`, `/assets/*`.
//! * Per-tenant app shell: `/{tenant}/…` (e.g. `/acme/`, `/acme/students`).
//!   Tenant id is a path parameter — no cookie or header plumbing.
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
/// * Public routes: `/login`, `/logout`, `/assets/*`.
/// * Tenant-scoped app shell: nested under `/{tenant}/…`. The tenant id is
///   pulled from the path parameter, so the middleware needs no cookie or
///   header. A lightweight session cookie (set at login) is still consulted
///   by [`auth::require_session`] to keep the URL from being spoofed.
pub fn build_web_router(tenants: TenantRegistry) -> Router {
    // Tenant id comes from the `{tenant}` path parameter captured by the
    // `.nest("/{tenant}", ...)` mount point below.
    let tenant_state = TenantScopeState::new(tenants)
        .with_source(TenantSource::path_param("tenant"));

    // App-shell (tenant-scoped) routes.
    let app_routes = Router::new()
        .merge(dashboard::routes())
        .merge(students::routes())
        .layer(axum::middleware::from_fn(auth::require_session))
        .layer(axum::middleware::from_fn_with_state(
            tenant_state.clone(),
            tenant_scope,
        ))
        .with_state(tenant_state.clone());

    Router::new()
        .merge(assets::routes())
        .merge(auth::public_routes(tenant_state))
        .nest("/{tenant}", app_routes)
}
