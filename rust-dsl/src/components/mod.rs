//! Typed builders for each `<ui-*>` Lit web component.
//!
//! Only a small representative slice is included in the initial cut:
//!
//! * [`button`] — `<ui-button>`
//! * [`input`]  — `<ui-input>`
//! * [`card`]   — `<ui-card>`
//! * [`page`]   — a helper that wraps everything in a boilerplate HTML shell
//!   that loads the Lit component kit.
//!
//! Every builder follows the same Vaadin-style pattern:
//!
//! * A free function (lowercase) returns an empty instance:
//!   `button()`, `input()`, `card()`.
//! * Attribute setters take `impl Into<String>` (or an enum, for enums) and
//!   return `Self` for chaining.
//! * Containers have `.add(child)` and `.children(iter)` to insert children.

pub mod button;
pub mod input;
pub mod card;
pub mod page;
