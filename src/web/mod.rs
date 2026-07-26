//! Server-rendered web (HTML) handlers.
//!
//! Business module screens (staff, timetable, fees, ...) have been
//! stripped. The remaining files provide the shell:
//!   * `landing` — public marketing page.
//!   * `login`  — cookie-based login flow (see `auth`).
//!   * `admin`  — control-plane tenant management.
//!   * `dashboard` — post-login home with the launcher grid.
//!   * `portal` — cross-tenant parent/student portal shell.
//!   * `demo`   — placeholder module showing the wiring pattern.

pub mod admin;
pub mod assets;
pub mod auth;
pub mod dashboard;
pub mod demo;
pub mod error;
pub mod filters;
pub mod landing;
pub mod layout;
pub mod portal;
