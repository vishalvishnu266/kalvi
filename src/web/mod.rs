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
//! * `GET  /web/{tenant}/login`     — tenant-specific login form.
//! * `/web/{tenant}/…`              — authenticated app shell. Tenant id is
//!   a path parameter — no cookie/header plumbing for DB routing.

pub mod assets;
pub mod auth;
pub mod dashboard;
pub mod error;
pub mod filters;
pub mod landing;
pub mod layout;
pub mod modules;
pub mod students;
