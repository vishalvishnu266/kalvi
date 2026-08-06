//! Rust builders for the layout web components in
//! `lit-components/components/layout/*.js`.
//!
//! Six primitives, one file:
//! * `columns()`  — explicit N-col with optional ratios
//! * `stack()`    — vertical rhythm
//! * `cluster()`  — wrapping inline row
//! * `grid()`     — auto-fit responsive grid
//! * `sidebar()`  — fixed sidebar + fluid main
//! * `center()`   — centered readable column
//!
//! Every builder follows the same shape as the primitive derives — same
//! `.gap()`, same `.add()` / `.children()` from `#[ui(children)]`,
//! same setters-return-Self ergonomics. No surprises.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! stack().gap(Gap::Md)
//!   .add(heading("Buttons").h2())
//!   .add(columns().ratios("1 3").gap(Gap::Md)
//!         .add(badge("primary"))
//!         .add(button().label("Save")))
//! ```

#[allow(unused_imports)]
use crate::core::{Child, Component};
use lit_ui_macros::{AttrEnum, UiComponent};

// ── Shared enums ───────────────────────────────────────────────────────

/// Space-token preset shared by every layout primitive that has a gap.
/// Mirrors the presets in `layout/*.js` CSS.
#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gap {
    #[attr("none")] None,
    #[attr("xs")]   Xs,
    #[attr("sm")]   Sm,
    #[attr("md")]   #[attr_enum(default)] Md,
    #[attr("lg")]   Lg,
    #[attr("xl")]   Xl,
}

/// Cross-axis alignment for stack / cluster.
#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    #[attr("start")]    Start,
    #[attr("center")]   Center,
    #[attr("end")]      End,
    #[attr("stretch")]  #[attr_enum(default)] Stretch,
    #[attr("baseline")] Baseline,
}

/// Main-axis distribution.
#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distribute {
    #[attr("start")]   #[attr_enum(default)] Start,
    #[attr("center")]  Center,
    #[attr("end")]     End,
    #[attr("between")] Between,
    #[attr("around")]  Around,
    #[attr("evenly")]  Evenly,
}

/// `justify-content` for cluster (a small subset of `Distribute`).
#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    #[attr("start")]   #[attr_enum(default)] Start,
    #[attr("center")]  Center,
    #[attr("end")]     End,
    #[attr("between")] Between,
}

/// Sidebar side.
#[derive(AttrEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    #[attr("start")] #[attr_enum(default)] Start,
    #[attr("end")]   End,
}

// ── ui-columns ─────────────────────────────────────────────────────────

#[derive(UiComponent)]
#[ui(tag = "ui-columns")]
pub struct Columns {
    #[ui(attr = "cols", default = "2", skip_if = "self.cols == 2")] pub cols: u32,
    #[ui(attr = "ratios")]                                          pub ratios: Option<String>,
    #[ui(enum_attr = "gap", skip_if = "self.gap == Gap::Md")]       pub gap: Gap,
    #[ui(enum_attr = "align", skip_if = "self.align == Align::Stretch")]
                                                                    pub align: Align,
    #[ui(attr = "collapse-at", default = "768", skip_if = "self.collapse_at == 768")]
                                                                    pub collapse_at: u32,
    #[ui(children)]                                                 pub children: Vec<Child>,
}

// ── ui-stack ───────────────────────────────────────────────────────────

#[derive(UiComponent)]
#[ui(tag = "ui-stack")]
pub struct Stack {
    #[ui(enum_attr = "gap",   skip_if = "self.gap == Gap::Md")]           pub gap: Gap,
    #[ui(enum_attr = "align", skip_if = "self.align == Align::Stretch")]  pub align: Align,
    #[ui(enum_attr = "distribute", skip_if = "self.distribute == Distribute::Start")]
                                                                          pub distribute: Distribute,
    #[ui(flag = "inline")]                                                pub inline: bool,
    #[ui(flag = "wrap")]                                                  pub wrap: bool,
    #[ui(children)]                                                       pub children: Vec<Child>,
}

// ── ui-cluster ─────────────────────────────────────────────────────────

#[derive(UiComponent)]
#[ui(tag = "ui-cluster")]
pub struct Cluster {
    #[ui(enum_attr = "gap",     skip_if = "self.gap == Gap::Sm")]           pub gap: Gap,
    #[ui(enum_attr = "align",   skip_if = "self.align == Align::Center")]   pub align: Align,
    #[ui(enum_attr = "justify", skip_if = "self.justify == Justify::Start")] pub justify: Justify,
    #[ui(flag = "nowrap")]                                                  pub nowrap: bool,
    #[ui(children)]                                                         pub children: Vec<Child>,
}
// Cluster's default gap is `sm` (matches the JS `--space-3`), and its
// default align is `center`. Override those defaults on `Default::default`
// via `impl` because the derive uses the enum's `#[attr_enum(default)]`
// (which is Md / Stretch for `Gap` / `Align`).
impl Cluster {
    /// Override the default constructor to match the JS component's
    /// declared defaults (`gap=sm`, `align=center`).
    pub fn new() -> Self {
        Self { gap: Gap::Sm, align: Align::Center, justify: Justify::Start, nowrap: false, children: Vec::new() }
    }
}
// Free constructor that uses `Cluster::new()` (the derive already emits
// one that calls `Default::default()`, so we override it to keep the
// cluster-specific defaults intact).
pub fn cluster_row() -> Cluster { Cluster::new() }

// ── ui-grid ────────────────────────────────────────────────────────────

#[derive(UiComponent)]
#[ui(tag = "ui-grid")]
pub struct Grid {
    #[ui(attr = "min-col", default = "240", skip_if = "self.min_col == 240")]
                                                                     pub min_col: u32,
    // `cols == 0` means "auto-fit mode wins" — the JS component treats
    // any positive cols value as an explicit fixed-column override.
    #[ui(attr = "cols", skip_if = "self.cols == 0")]                 pub cols: u32,
    #[ui(enum_attr = "gap", skip_if = "self.gap == Gap::Md")]        pub gap: Gap,
    #[ui(flag = "dense")]                                            pub dense: bool,
    #[ui(children)]                                                  pub children: Vec<Child>,
}

// ── ui-sidebar ─────────────────────────────────────────────────────────

#[derive(UiComponent)]
#[ui(tag = "ui-sidebar")]
pub struct Sidebar {
    #[ui(enum_attr = "side", skip_if = "self.side == Side::Start")]  pub side: Side,
    #[ui(attr = "width", default = "\"240px\".to_string()", skip_if = "self.width == \"240px\"")]
                                                                     pub width: String,
    #[ui(attr = "content-min", default = "\"50%\".to_string()", skip_if = "self.content_min == \"50%\"")]
                                                                     pub content_min: String,
    #[ui(enum_attr = "gap", skip_if = "self.gap == Gap::Md")]        pub gap: Gap,
    #[ui(attr = "collapse-at", default = "768", skip_if = "self.collapse_at == 768")]
                                                                     pub collapse_at: u32,
    #[ui(children)]                                                  pub children: Vec<Child>,
}

// ── ui-center ──────────────────────────────────────────────────────────

#[derive(UiComponent)]
#[ui(tag = "ui-center")]
pub struct Center {
    #[ui(attr = "max-w", default = "\"72ch\".to_string()", skip_if = "self.max_w == \"72ch\"")]
                                                                     pub max_w: String,
    #[ui(flag = "padded")]                                           pub padded: bool,
    #[ui(flag = "intrinsic")]                                        pub intrinsic: bool,
    #[ui(children)]                                                  pub children: Vec<Child>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::badge::badge;

    #[test]
    fn columns_defaults_emit_empty_tag() {
        // cols=2, gap=md, align=stretch, collapse-at=768 all skipped as defaults.
        assert_eq!(columns().render(), "<ui-columns></ui-columns>");
    }

    #[test]
    fn columns_ratios_and_children() {
        let html = columns().ratios("1 2 1").gap(Gap::Lg)
            .add(badge("a")).add(badge("b")).add(badge("c")).render();
        assert_eq!(
            html,
            r#"<ui-columns ratios="1 2 1" gap="lg"><ui-badge>a</ui-badge><ui-badge>b</ui-badge><ui-badge>c</ui-badge></ui-columns>"#
        );
    }

    #[test]
    fn stack_defaults_and_gap_override() {
        assert_eq!(stack().render(), "<ui-stack></ui-stack>");
        assert_eq!(stack().gap(Gap::Sm).render(), r#"<ui-stack gap="sm"></ui-stack>"#);
    }

    #[test]
    fn cluster_uses_sm_default_gap() {
        // Cluster's own default gap is `sm`, not `md`.
        let c = Cluster::new();
        assert_eq!(c.render(), "<ui-cluster></ui-cluster>");
        // Bumping to md now renders because it's no longer the default.
        assert_eq!(Cluster::new().gap(Gap::Md).render(), r#"<ui-cluster gap="md"></ui-cluster>"#);
    }

    #[test]
    fn grid_min_col_and_dense() {
        assert_eq!(grid().render(), "<ui-grid></ui-grid>");
        assert_eq!(grid().dense().render(), "<ui-grid dense></ui-grid>");
        assert_eq!(
            grid().set_min_col(180).render(),
            r#"<ui-grid min-col="180"></ui-grid>"#,
        );
        // Explicit cols wins in the JS component; we mirror the wire format.
        assert_eq!(
            grid().cols(12).render(),
            r#"<ui-grid cols="12"></ui-grid>"#,
        );
    }

    #[test]
    fn sidebar_and_center_defaults_are_empty() {
        assert_eq!(sidebar().render(), "<ui-sidebar></ui-sidebar>");
        assert_eq!(center().render(),  "<ui-center></ui-center>");
    }

    #[test]
    fn sidebar_with_named_side_slot_and_main() {
        // The slot="side" attribute is set on the child, not the sidebar,
        // and there's no primitive for "raw HTML with attributes" — but
        // stack().add() etc. are enough for the common case. Verify the
        // basic composition renders as expected.
        let html = sidebar()
            .add(stack().add(badge("nav 1")).add(badge("nav 2")))
            .add(stack().add(badge("main")))
            .render();
        assert!(html.starts_with("<ui-sidebar>"));
        assert!(html.contains("<ui-stack>"));
        assert!(html.ends_with("</ui-sidebar>"));
    }
}
