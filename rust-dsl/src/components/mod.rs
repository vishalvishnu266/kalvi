//! Typed builders for each `<ui-*>` **primitive** Lit web component.
//!
//! Everything here is a leaf building block: form inputs (button, input,
//! select, checkbox, radio, switch, datepicker, daterange) plus atomic
//! display atoms (icon, badge, avatar, tooltip, skeleton). Higher-level
//! containers (form, card, modal, kanban, data-table, …) were removed
//! from this crate — layout and page-shell responsibilities now live in
//! the `<ui-columns>` / `<ui-stack>` / `<ui-grid>` / `<ui-sidebar>` /
//! `<ui-center>` / `<ui-cluster>` web components on the Lit side.
//!
//! Every builder follows the same shape thanks to `#[derive(UiComponent)]`
//! (see the `lit-ui-macros` crate):
//!
//! * A free function (lowercase) returns an empty instance:
//!   `button()`, `input()`, `checkbox("Agree")`.
//! * Setters take `impl Into<String>` (or an enum, for enums) and return
//!   `Self` for chaining.
//! * `.render()` (from `Component`) turns the builder into a byte-exact
//!   HTML string.

// Form inputs
pub mod button;
pub mod input;
pub mod select;
pub mod checkbox;
pub mod radio;
pub mod switch;
pub mod datepicker;
pub mod daterange;

// Atomic display
pub mod icon;
pub mod badge;
pub mod avatar;
pub mod tooltip;
pub mod skeleton;
