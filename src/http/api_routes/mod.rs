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


pub fn admin_api() -> Router<AppState> {
    Router::new()
}

pub fn tenant_api() -> Router<AppState> {
    Router::new()
}
