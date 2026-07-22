//! Server-rendered web UI (Hotwire + Askama).
//!
//! Every submodule here only contains `pub async fn` handlers (and any
//! templates / helpers they need). The router that wires URLs to these
//! handlers lives in [`crate::http::routes`].
//!
//! ## URL layout (see [`crate::http::routes::build_router`] for the full map)
//! * `GET  /`                       — landing page (or redirect if signed in).
//! * `GET  /assets/{*path}`         — embedded static assets.
//! * `GET  /web/login`              — global login form.
//! * `POST /web/login`              — submit login.
//! * `POST /web/logout`             — sign out.
//! * `/web/{tenant}/…`              — tenant-scoped app shell (dashboard,
//!   students, module stubs, tenant-specific login). The `{tenant}` segment
//!   is factored out via a single `.nest("/web/{tenant}", …)`, mirroring
//!   how `/api/{tenant}` works — individual routes inside the nest are
//!   written **without** repeating `{tenant}`, and handlers pick the tenant
//!   up via the [`crate::http::TenantScope`] extractor.

pub mod assets;
pub mod auth;
pub mod dashboard;
pub mod error;
pub mod filters;
pub mod landing;
pub mod layout;
pub mod modules;
pub mod students;
