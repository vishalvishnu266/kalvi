//! # ui-shell — server-side runtime for the `<ui-app-shell>` fragment protocol
//!
//! The browser hosts a thin custom element (`<ui-app-shell>`) with six
//! named regions ("islands"):
//!
//! | target      | slot name in the shell |
//! |-------------|------------------------|
//! | [`TARGET_MAIN`]     | `main`     |
//! | [`TARGET_COPILOT`]  | `copilot`  |
//! | [`TARGET_TOPBAR`]   | `topbar`   |
//! | [`TARGET_SIDEBAR`]  | `sidebar`  |
//! | [`TARGET_TOAST`]    | `toast`    |
//! | [`TARGET_MODAL`]    | `modal`    |
//!
//! Every server response that wants to update an island wraps the DSL-
//! rendered HTML in a `<ui-fragment target=".." action="..">` envelope.
//! The client applies each envelope it sees; the same wire format works
//! for a single-response POST *and* for a streaming SSE agent turn.
//!
//! ## Wire format
//!
//! ```html
//! <ui-fragment target="main" action="replace">
//!   <!-- any HTML built by rust-dsl -->
//! </ui-fragment>
//! ```
//!
//! Multiple envelopes may be concatenated in one response body.
//!
//! ## Content negotiation
//!
//! * `Accept: text/vnd.ui-fragments+html` → the handler returns fragments
//!   only (no shell). This is what the client runtime sends for
//!   intercepted navigation and form submits.
//! * Anything else → the handler returns a full HTML document built by
//!   [`respond`], with the fragments already inlined into their slots.
//!
//! ## Example
//!
//! ```ignore
//! use ui_shell::{Fragment, Target, Action, Fragments, negotiate, Render};
//! use lit_ui::prelude::*;
//!
//! async fn dashboard(headers: http::HeaderMap) -> impl axum::response::IntoResponse {
//!     let main = card().title("Dashboard").add(stat().label("Users").value("42"));
//!     let frags = Fragments::new()
//!         .push(Fragment::replace(Target::Main, main));
//!     negotiate(&headers, frags, shell_chrome)
//! }
//! ```

pub mod fragment;
pub mod negotiation;
pub mod sse;

pub use fragment::{Action, Fragment, Fragments, Render, Target};
pub use negotiation::{negotiate, ChromeFn, FRAGMENTS_MIME};
pub use sse::fragments_sse;

// -----------------------------------------------------------------------------
// Canonical target names (also used by the JS runtime).
// Keeping these as `pub const &str` lets DSL/route code refer to them by
// symbol so typos are compile errors, while still exposing plain strings
// for the wire format.
// -----------------------------------------------------------------------------

/// The primary content island. Default swap target for link navigation.
pub const TARGET_MAIN: &str = "main";
/// The copilot pane island (right side on desktop, bottom sheet on mobile).
pub const TARGET_COPILOT: &str = "copilot";
/// Persistent topbar island (breadcrumbs, actions, theme toggle).
pub const TARGET_TOPBAR: &str = "topbar";
/// Persistent sidebar island (primary nav on desktop).
pub const TARGET_SIDEBAR: &str = "sidebar";
/// Toast host island. Append fragments here to show transient toasts.
pub const TARGET_TOAST: &str = "toast";
/// Modal host island. Replace to open, remove to close.
pub const TARGET_MODAL: &str = "modal";
