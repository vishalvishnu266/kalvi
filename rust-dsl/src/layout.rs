//! Layout primitives — minimal but compositional.
//!
//! Five plain wrappers around a `<div>` with sensible flex/grid CSS.
//! Combine them to build any responsive layout without touching CSS:
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! container()
//!   .add(row()
//!       .add(column().flex(2).add(main_content))
//!       .add(column().flex(1).add(sidebar_content)))
//!   .add(grid().cols_min("240px").children(stat_cards));
//! ```
//!
//! ## Why these five?
//!
//! * **`Container`** — centers content and caps width. Every page starts here.
//! * **`Row`** — horizontal flex. Wraps automatically → responsive on mobile.
//! * **`Column`** — vertical flex. Use `.flex(2)` to grow proportionally.
//! * **`Grid`** — CSS grid with `auto-fit, minmax(min, 1fr)` responsive columns.
//! * **`Spacer`** — flexible gap that pushes siblings apart in a flex row.
//!
//! All spacing uses the design tokens (`--space-*`) so a theme change re-skins
//! every layout at once.

use crate::core::{escape_html, wrap, Attr, Child, Component};

/// Size scale used for gaps and paddings.
/// Maps to `--space-{n}` design tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gap { None, Xs, Sm, Md, Lg, Xl, Xxl }
impl Gap {
    fn var(self) -> &'static str {
        match self {
            Gap::None => "0",
            Gap::Xs   => "var(--space-1)",
            Gap::Sm   => "var(--space-2)",
            Gap::Md   => "var(--space-4)",
            Gap::Lg   => "var(--space-5)",
            Gap::Xl   => "var(--space-6)",
            Gap::Xxl  => "var(--space-8)",
        }
    }
}

/// Alignment along the cross axis (align-items).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align { Start, Center, End, Stretch, Baseline }
impl Align {
    fn css(self) -> &'static str {
        match self {
            Align::Start    => "flex-start",
            Align::Center   => "center",
            Align::End      => "flex-end",
            Align::Stretch  => "stretch",
            Align::Baseline => "baseline",
        }
    }
}

/// Distribution along the main axis (justify-content).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Justify { Start, Center, End, Between, Around, Evenly }
impl Justify {
    fn css(self) -> &'static str {
        match self {
            Justify::Start   => "flex-start",
            Justify::Center  => "center",
            Justify::End     => "flex-end",
            Justify::Between => "space-between",
            Justify::Around  => "space-around",
            Justify::Evenly  => "space-evenly",
        }
    }
}

// ---------------------------------------------------------------------------
// Container
// ---------------------------------------------------------------------------

/// Centers content and caps its max width. Every page starts here.
pub struct Container {
    max_width: String,   // any CSS length; default 1200px
    padded: bool,
    children: Vec<Child>,
}

pub fn container() -> Container {
    Container { max_width: "1200px".into(), padded: true, children: Vec::new() }
}

impl Container {
    pub fn max_width(mut self, w: impl Into<String>) -> Self { self.max_width = w.into(); self }
    pub fn fluid(mut self)   -> Self { self.max_width = "100%".into(); self }
    pub fn no_padding(mut self) -> Self { self.padded = false; self }
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }
}

impl Component for Container {
    fn render(&self) -> String {
        let pad = if self.padded { "var(--space-5)" } else { "0" };
        let style = format!(
            "max-width:{};margin-left:auto;margin-right:auto;padding-left:{};padding-right:{};box-sizing:border-box;",
            escape_html(&self.max_width), pad, pad,
        );
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("div", &[Attr::kv("style", style)], &body)
    }
}

// ---------------------------------------------------------------------------
// Row
// ---------------------------------------------------------------------------

/// Horizontal flex layout. Wraps by default so it collapses to a stack on
/// narrow screens automatically.
pub struct Row {
    gap: Gap,
    align: Align,
    justify: Justify,
    wrap: bool,
    children: Vec<Child>,
}

pub fn row() -> Row {
    Row { gap: Gap::Md, align: Align::Stretch, justify: Justify::Start, wrap: true, children: Vec::new() }
}

impl Row {
    pub fn gap(mut self, g: Gap)         -> Self { self.gap = g; self }
    pub fn align(mut self, a: Align)     -> Self { self.align = a; self }
    pub fn justify(mut self, j: Justify) -> Self { self.justify = j; self }
    pub fn nowrap(mut self)              -> Self { self.wrap = false; self }
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }
}

impl Component for Row {
    fn render(&self) -> String {
        let style = format!(
            "display:flex;flex-direction:row;flex-wrap:{};gap:{};align-items:{};justify-content:{};min-width:0;",
            if self.wrap { "wrap" } else { "nowrap" },
            self.gap.var(),
            self.align.css(),
            self.justify.css(),
        );
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("div", &[Attr::kv("style", style)], &body)
    }
}

// ---------------------------------------------------------------------------
// Column
// ---------------------------------------------------------------------------

/// Vertical flex layout. Use `.flex(n)` inside a Row to grow proportionally.
pub struct Column {
    gap: Gap,
    align: Align,
    justify: Justify,
    flex: Option<u32>,
    min_width: Option<String>,
    children: Vec<Child>,
}

pub fn column() -> Column {
    Column {
        gap: Gap::Md, align: Align::Stretch, justify: Justify::Start,
        flex: None, min_width: None, children: Vec::new(),
    }
}

impl Column {
    pub fn gap(mut self, g: Gap)         -> Self { self.gap = g; self }
    pub fn align(mut self, a: Align)     -> Self { self.align = a; self }
    pub fn justify(mut self, j: Justify) -> Self { self.justify = j; self }
    /// Grow factor when this column sits inside a Row.
    pub fn flex(mut self, n: u32)        -> Self { self.flex = Some(n); self }
    /// Prevents crushing on very narrow screens. Default `0` (allow shrink).
    pub fn min_width(mut self, w: impl Into<String>) -> Self { self.min_width = Some(w.into()); self }
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }
}

impl Component for Column {
    fn render(&self) -> String {
        let flex = self.flex.map(|f| format!("flex:{} 1 0;", f)).unwrap_or_default();
        let minw = self.min_width.as_deref().map(|w|
            format!("min-width:{};", escape_html(w))
        ).unwrap_or_default();
        let style = format!(
            "display:flex;flex-direction:column;gap:{};align-items:{};justify-content:{};{}{}min-width:0;",
            self.gap.var(), self.align.css(), self.justify.css(), flex, minw,
        );
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("div", &[Attr::kv("style", style)], &body)
    }
}

// ---------------------------------------------------------------------------
// Grid
// ---------------------------------------------------------------------------

/// Auto-fit grid — a responsive way to say "as many columns of at least N
/// wide as fit, otherwise wrap". Perfect for KPI/stat cards.
///
/// * `.cols_min("220px")` → `repeat(auto-fit, minmax(220px, 1fr))`
/// * `.cols_count(3)`     → `repeat(3, 1fr)` (fixed columns; not fluid)
pub struct Grid {
    template: String,  // Full grid-template-columns value
    gap: Gap,
    children: Vec<Child>,
}

pub fn grid() -> Grid {
    Grid {
        template: "repeat(auto-fit, minmax(220px, 1fr))".into(),
        gap: Gap::Md,
        children: Vec::new(),
    }
}

impl Grid {
    pub fn cols_min(mut self, min_col_width: impl Into<String>) -> Self {
        self.template = format!("repeat(auto-fit, minmax({}, 1fr))", min_col_width.into());
        self
    }
    pub fn cols_count(mut self, n: u32) -> Self {
        self.template = format!("repeat({}, minmax(0, 1fr))", n);
        self
    }
    pub fn template(mut self, css: impl Into<String>) -> Self { self.template = css.into(); self }
    pub fn gap(mut self, g: Gap) -> Self { self.gap = g; self }
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }
}

impl Component for Grid {
    fn render(&self) -> String {
        let style = format!(
            "display:grid;grid-template-columns:{};gap:{};min-width:0;",
            escape_html(&self.template), self.gap.var(),
        );
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("div", &[Attr::kv("style", style)], &body)
    }
}

// ---------------------------------------------------------------------------
// Spacer
// ---------------------------------------------------------------------------

/// A flexible spacer — pushes siblings apart inside a `Row` or `Column`.
pub struct Spacer;
pub fn spacer() -> Spacer { Spacer }

impl Component for Spacer {
    fn render(&self) -> String {
        "<div style=\"flex:1 1 auto;\"></div>".into()
    }
}

// ---------------------------------------------------------------------------
// Section (titled block with optional subtitle + right-side actions)
// ---------------------------------------------------------------------------

/// A titled block — the canonical "H2 + subtitle + right-side actions,
/// then body content" pattern that every ERP page needs 5-10 times.
///
/// ```ignore
/// section()
///     .title("Fees")
///     .subtitle("August 2026")
///     .action(button().label("Export").icon("upload"))
///     .add(fees_table)
/// ```
///
/// Renders semantically as `<section>` so screen readers hear it as a
/// landmark region.
pub struct Section {
    title: Option<String>,
    subtitle: Option<String>,
    actions: Vec<Child>,
    children: Vec<Child>,
}

pub fn section() -> Section {
    Section { title: None, subtitle: None, actions: Vec::new(), children: Vec::new() }
}

impl Section {
    pub fn title(mut self, s: impl Into<String>)    -> Self { self.title    = Some(s.into()); self }
    pub fn subtitle(mut self, s: impl Into<String>) -> Self { self.subtitle = Some(s.into()); self }

    /// Add a control (button, tabs, badge, …) to the header's right side.
    pub fn action(mut self, c: impl Component + 'static) -> Self {
        self.actions.push(Box::new(c)); self
    }
    /// Add a body child.
    pub fn add(mut self, c: impl Component + 'static) -> Self {
        self.children.push(Box::new(c)); self
    }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }
}

impl Component for Section {
    fn render(&self) -> String {
        let mut out = String::from(
            r#"<section style="display:flex;flex-direction:column;gap:var(--space-3);min-width:0;">"#
        );

        let has_header = self.title.is_some() || self.subtitle.is_some() || !self.actions.is_empty();
        if has_header {
            out.push_str(
                r#"<header style="display:flex;align-items:baseline;gap:12px;flex-wrap:wrap;min-width:0;">"#
            );
            if let Some(ref t) = self.title {
                out.push_str(&format!(
                    r#"<h2 style="margin:0;font-size:var(--fs-lg);font-weight:var(--fw-semibold);letter-spacing:-0.01em;color:var(--color-text);">{}</h2>"#,
                    crate::core::escape_html(t)
                ));
            }
            if let Some(ref s) = self.subtitle {
                out.push_str(&format!(
                    r#"<span style="color:var(--color-text-muted);font-size:var(--fs-sm);">{}</span>"#,
                    crate::core::escape_html(s)
                ));
            }
            if !self.actions.is_empty() {
                out.push_str(
                    r#"<div style="margin-left:auto;display:flex;align-items:center;gap:var(--space-2);">"#
                );
                for a in &self.actions { out.push_str(&a.render()); }
                out.push_str("</div>");
            }
            out.push_str("</header>");
        }

        for c in &self.children { out.push_str(&c.render()); }
        out.push_str("</section>");
        out
    }
}
