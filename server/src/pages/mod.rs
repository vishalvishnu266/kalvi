//! All demo pages. Each submodule exports a single `handler` function
//! that:
//!
//! 1. Builds one or more `Fragment`s using the DSL.
//! 2. Runs them through [`ui_shell::negotiate`] with the app's
//!    [`crate::shell::chrome`] so the response is either a full page
//!    or a fragment-only body — the caller controls that via the
//!    `Accept` header.
//!
//! The pattern is deliberately repetitive. Copy-paste one when you
//! need a new page; it's the whole "how to add a page" contract.

pub mod admin;
pub mod apps;
pub mod dashboard;
pub mod landing;
pub mod reports;
pub mod settings;
pub mod users;
