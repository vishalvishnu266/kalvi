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

// Tier-1 ports
pub mod badge;
pub mod icon;
pub mod avatar;
pub mod stat;
pub mod list_item;
pub mod select;
pub mod checkbox;
pub mod radio;
pub mod switch;
pub mod form;
pub mod form_section;
pub mod hidden;
pub mod table;
pub mod data_table;
pub mod pagination;
pub mod breadcrumb;

// Tier-2 ports
pub mod avatar_group;
pub mod tooltip;
pub mod tab_bar;
pub mod segmented;
pub mod empty_state;
pub mod skeleton;
pub mod progress;
pub mod drawer;
pub mod dropdown_menu;
pub mod modal;
pub mod stepper;
pub mod timeline;
pub mod kanban;

// Tier-3 ports (specials)
pub mod file_upload;
pub mod datepicker;
pub mod daterange;
pub mod inline_edit;
pub mod toast;
pub mod command;

// Error-UX surfaces (see also: input().error(), toast()).
pub mod form_banner;
pub mod alert;
pub mod ack_panel;

// Agentic AI chat window — see `docs` in the module for architecture.
pub mod copilot;
// Reusable primitives extracted from <ui-copilot> — step bars, tool cards,
// mention popovers — so any page (not just Copilot) can drop them in.
pub mod copilot_primitives;

// ---------------------------------------------------------------------------
// Framework primitives (added for the backend-driven fragment protocol).
//
// * `app_shell` — `<ui-app-shell>` with six named slots (islands).
// * `fragment`  — `<ui-fragment target action>` envelope for island swaps.
//
// These are ADDITIVE — no existing component was changed to introduce them.
// ---------------------------------------------------------------------------
pub mod app_shell;
pub mod fragment;
