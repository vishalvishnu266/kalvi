//! Server-rendered web UI (Hotwire + Askama).
//!
//! This module is intentionally **decoupled** from the JSON API in
//! [`crate::api`]. It serves an HTML SPA-like experience powered by Turbo
//! (from Hotwire), so navigation feels instant without shipping a JS
//! framework.
//!
//! ## URL layout
//! * `GET  /`                        — SaaS landing page (or redirect if
//!   the visitor already has a session cookie).
//! * `GET  /assets/*`                — embedded static assets.
//! * `GET  /web/login`               — global login form.
//! * `POST /web/login`               — submit login.
//! * `POST /web/logout`              — sign out.
//! * `GET  /web/{tenant}/login`      — tenant-specific login form.
//! * `/web/{tenant}/…`               — authenticated app shell (dashboard,
//!   students, …). Tenant id is a path parameter — no cookie or header
//!   plumbing for DB routing.
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
pub mod landing;
pub mod layout;
pub mod modules;
pub mod students;

/// Build the web router.
///
/// URL layout:
/// * `GET  /`                       — SaaS landing page (or redirect to the
///   user's tenant shell if a session cookie is present).
/// * `GET  /assets/*`               — embedded static assets.
/// * `GET  /web/login`              — global login form (user picks tenant).
/// * `POST /web/login`              — submit login form.
/// * `POST /web/logout`             — clear session.
/// * `GET  /web/{tenant}/login`     — tenant-specific login (tenant pre-filled).
/// * `/web/{tenant}/…`              — authenticated app shell (dashboard,
///   students, …). Requires a matching session cookie.
///
/// Tenant DB routing is entirely path-based — [`TenantSource::path_param`]
/// pulls the id from the `{tenant}` URL segment.
pub fn build_web_router(tenants: TenantRegistry) -> Router {
    let tenant_state = TenantScopeState::new(tenants)
        .with_source(TenantSource::path_param("tenant"));

    // Tenant-scoped app shell (authenticated).
    //
    // `students::routes()` is a real, wired-up module. Every other module
    // is served by `modules::routes()` as a "Coming soon" placeholder so
    // every tile on the launcher leads somewhere.
    //
    // Trailing-slash handling: `/web/{tenant}` and `/web/{tenant}/` both
    // resolve to the dashboard, thanks to the `NormalizePathLayer` wrapped
    // around the whole app in [`crate::api::build_router`]. No per-route
    // aliases needed here.
    let app_routes = Router::new()
        .merge(dashboard::routes())
        .merge(students::routes())
        .merge(modules::routes())
        .layer(axum::middleware::from_fn(auth::require_session))
        .layer(axum::middleware::from_fn_with_state(
            tenant_state.clone(),
            tenant_scope,
        ))
        .with_state(tenant_state.clone());

    // /web/* subtree: public login routes + tenant-scoped app shell.
    let web_routes = Router::new()
        .merge(auth::public_routes(tenant_state.clone()))
        .nest("/{tenant}", app_routes);

    Router::new()
        .merge(landing::routes())
        .merge(assets::routes())
        .nest("/web", web_routes)
}
