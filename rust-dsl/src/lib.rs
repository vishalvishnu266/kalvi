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
pub mod layout;
pub mod components;

/// Reusable full-page builders (Students, Fees, Attendance, Dashboard).
/// Framework-agnostic: each returns a [`crate::components::page::Page`] that
/// the caller can `.render()` to HTML.
pub mod pages;

/// Re-exports the everyday pieces you want in scope.
///
/// A single `use lit_ui::prelude::*;` brings in every free-function
/// constructor and every enum you need to compose real pages.
pub mod prelude {
    // Core
    pub use crate::core::{Component, Node, RenderExt};

    // Layout primitives
    pub use crate::layout::{
        container, row, column, grid, spacer, section,
        Container, Row, Column, Grid, Spacer, Section,
        Gap, Align, Justify,
    };

    // Original slice
    pub use crate::components::button::{button, Button, Variant, Size};
    pub use crate::components::card::{card, Card};
    pub use crate::components::input::{input, Input, InputType};
    pub use crate::components::page::{page, Page};

    // Tier-1 ports
    pub use crate::components::badge::{badge, Badge, Tone};
    pub use crate::components::icon::{icon, Icon};
    pub use crate::components::avatar::{avatar, Avatar, AvatarSize};
    pub use crate::components::stat::{stat, Stat, Trend};
    pub use crate::components::list_item::{list_item, ListItem};
    pub use crate::components::select::{select, Select, SelectOption};
    pub use crate::components::checkbox::{checkbox, Checkbox};
    pub use crate::components::radio::{radio, radio_group, Radio, RadioGroup};
    pub use crate::components::switch::{switch, Switch};
    pub use crate::components::form::{form, Form};
    pub use crate::components::table::{table, Table, Column as TableColumn, Align as TableAlign};
    pub use crate::components::data_table::{data_table, DataTable, ColOpts};
    pub use crate::components::pagination::{pagination, Pagination};
    pub use crate::components::breadcrumb::{breadcrumb, Breadcrumb, Crumb};

    // Tier-2 ports
    pub use crate::components::avatar_group::{avatar_group, AvatarGroup};
    pub use crate::components::tooltip::{tooltip, Tooltip, Placement};
    pub use crate::components::tab_bar::{tab_bar, TabBar, Tab};
    pub use crate::components::segmented::{segmented, Segmented, Segment};
    pub use crate::components::empty_state::{empty_state, EmptyState};
    pub use crate::components::skeleton::{skeleton, Skeleton, Shape as SkeletonShape};
    pub use crate::components::progress::{progress, Progress, ProgShape, ProgTone};
    pub use crate::components::drawer::{drawer, Drawer, DrawerPlacement, DrawerSize};
    pub use crate::components::dropdown_menu::{
        dropdown_menu, DropdownMenu, MenuAlign, MenuItem, MenuEntry,
    };
    pub use crate::components::modal::{modal, Modal};
    pub use crate::components::stepper::{stepper, Stepper, StepperOrientation};
    pub use crate::components::timeline::{
        timeline, timeline_item, Timeline, TimelineItem, TimelineTone,
    };
    pub use crate::components::kanban::{
        kanban, kanban_column, kanban_card, Kanban, KanbanColumn, KanbanCard,
    };

    // Tier-3 ports (specials)
    pub use crate::components::file_upload::{file_upload, FileUpload};
    pub use crate::components::datepicker::{datepicker, Datepicker};
    pub use crate::components::daterange::{date_range, DateRange};
    pub use crate::components::combobox::{combobox, Combobox, ComboOption};
    pub use crate::components::inline_edit::{inline_edit, InlineEdit, InlineKind};
    pub use crate::components::toast::{toast, toast_host, Toast, ToastHost, ToastTone};
    pub use crate::components::command::{command, command_item, Command, CommandItem};
}
