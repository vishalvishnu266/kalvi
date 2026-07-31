//! # lit-ui — a macro-free Rust DSL over the Lit web-component kit.
//!
//! Every UI concept is a plain Rust struct. You configure it through
//! **method-chaining** (Vaadin-style), then call [`Component::render`] to get
//! the equivalent HTML string.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! let markup = card()
//!     .title("Add student")
//!     .padded()
//!     .add(input().label("Full name").required())
//!     .add(button().label("Save").variant(Variant::Primary))
//!     .render();
//!
//! assert!(markup.starts_with("<ui-card"));
//! ```
//!
//! ## Design principles
//!
//! * **No macros** — everything is normal Rust code. That means great IDE
//!   support, no `proc-macro` compile hit, and readable error messages.
//! * **String rendering** — [`Component::render`] returns an owned `String`.
//!   Perfect for Axum / Askama / minijinja handlers or Hotwire streams.
//! * **Zero runtime dependencies** — the crate is just data structures + a
//!   tiny HTML-escaper.
//! * **Web-component parity** — every attribute maps 1:1 to an HTML attribute
//!   read by the corresponding `<ui-*>` Lit element. Rust code is a *typed
//!   view* over the same DSL that HTML uses.
//!
//! ## Children API
//!
//! Every container exposes:
//!
//! * `.add(child)` — one child at a time. Great for readable chains.
//! * `.children(iter)` — many children in one call, from any `IntoIterator`.
//!
//! Children are stored as `Box<dyn Component>`, so you can freely mix
//! different component types.

pub mod core;
pub mod components;

/// Re-exports the everyday pieces you want in scope.
pub mod prelude {
    pub use crate::core::{Component, Node, RenderExt};
    pub use crate::components::button::{button, Button, Variant, Size};
    pub use crate::components::card::{card, Card};
    pub use crate::components::input::{input, Input, InputType};
    pub use crate::components::page::{page, Page};
}
