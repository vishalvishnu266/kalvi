//! Layout primitives — **class-based**, standardised, responsive.
//!
//! ## Rules the DSL enforces
//!
//! * **Zero inline layout CSS.** Rust builders emit `class="lu-…"` names
//!   only. All layout rules live in `lit-components/assets/layout.css`.
//! * **Typed enums** for gap, alignment, and justification map 1:1 to
//!   class-name suffixes so IDEs (and the Rust compiler) catch typos.
//! * **Responsive is opt-in via typed helpers.** Call `.mobile_stack()`
//!   on a `Row` or `Grid` to have it collapse to a Column on mobile
//!   (≤ 640 px). Similar `.tablet_stack()` for ≤ 900 px. No CSS to write.
//!
//! ## Available primitives
//!
//! * [`container()`] — centred wrapper with `max-width`.
//! * [`row()`]       — horizontal flex, wraps by default.
//! * [`column()`]    — vertical flex.
//! * [`grid()`]      — CSS grid with auto-fit responsive columns.
//! * [`spacer()`]    — pushes flex siblings apart.
//! * [`section()`]   — semantic `<section>` with title / subtitle / actions.

use crate::core::{escape_html, wrap, Attr, Child, Component};

// ---------------------------------------------------------------------------
// Enums (typed knobs)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gap { None, Xs, Sm, Md, Lg, Xl, Xxl }
impl Gap {
    fn class(self) -> &'static str {
        match self {
            Gap::None => "lu-gap-none",
            Gap::Xs   => "lu-gap-xs",
            Gap::Sm   => "lu-gap-sm",
            Gap::Md   => "lu-gap-md",
            Gap::Lg   => "lu-gap-lg",
            Gap::Xl   => "lu-gap-xl",
            Gap::Xxl  => "lu-gap-xxl",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align { Start, Center, End, Stretch, Baseline }
impl Align {
    fn class(self) -> &'static str {
        match self {
            Align::Start    => "lu-align-start",
            Align::Center   => "lu-align-center",
            Align::End      => "lu-align-end",
            Align::Stretch  => "lu-align-stretch",
            Align::Baseline => "lu-align-baseline",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Justify { Start, Center, End, Between, Around, Evenly }
impl Justify {
    fn class(self) -> &'static str {
        match self {
            Justify::Start   => "lu-justify-start",
            Justify::Center  => "lu-justify-center",
            Justify::End     => "lu-justify-end",
            Justify::Between => "lu-justify-between",
            Justify::Around  => "lu-justify-around",
            Justify::Evenly  => "lu-justify-evenly",
        }
    }
}

/// The breakpoint at which a Row/Grid stacks into a Column.
///
/// * `Mobile` → ≤ 640 px
/// * `Tablet` → ≤ 900 px
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Breakpoint { Mobile, Tablet }
impl Breakpoint {
    fn stack_class(self) -> &'static str {
        match self { Breakpoint::Mobile => "lu-stack-m", Breakpoint::Tablet => "lu-stack-t" }
    }
}

// ---------------------------------------------------------------------------
// Class-list helper — small string builder for `class="..."`
// ---------------------------------------------------------------------------

/// Compose a single `class="…"` attribute from an iterator of `&str` parts.
/// Empty parts are skipped so callers can freely include conditional classes.
fn class_attr<'a, I: IntoIterator<Item = &'a str>>(parts: I) -> Attr {
    let mut s = String::new();
    for p in parts.into_iter().filter(|p| !p.is_empty()) {
        if !s.is_empty() { s.push(' '); }
        s.push_str(p);
    }
    Attr::kv("class", s)
}

// ---------------------------------------------------------------------------
// Container
// ---------------------------------------------------------------------------

/// Size preset for [`Container`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerSize { Sm, Md, Lg, Xl, Fluid }
impl ContainerSize {
    fn class(self) -> &'static str {
        match self {
            ContainerSize::Sm    => "lu-container-sm",
            ContainerSize::Md    => "lu-container-md",
            ContainerSize::Lg    => "lu-container-lg",
            ContainerSize::Xl    => "lu-container-xl",
            ContainerSize::Fluid => "lu-container-fluid",
        }
    }
}

pub struct Container {
    size: ContainerSize,
    padded: bool,
    children: Vec<Child>,
}

pub fn container() -> Container {
    Container { size: ContainerSize::Lg, padded: true, children: Vec::new() }
}

impl Container {
    pub fn size(mut self, s: ContainerSize) -> Self { self.size = s; self }
    pub fn fluid(mut self)      -> Self { self.size = ContainerSize::Fluid; self }
    pub fn no_padding(mut self) -> Self { self.padded = false; self }
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }

    /// **Deprecated shortcut kept for backwards compat.**
    /// Prefer `.size(ContainerSize::Lg)` etc.
    pub fn max_width(mut self, w: impl Into<String>) -> Self {
        let w = w.into();
        self.size = match w.as_str() {
            "640px"  | "sm" => ContainerSize::Sm,
            "960px"  | "md" => ContainerSize::Md,
            "1280px" | "xl" => ContainerSize::Xl,
            "100%"   | "fluid" => ContainerSize::Fluid,
            _ => ContainerSize::Lg,
        };
        self
    }
}

impl Component for Container {
    fn render(&self) -> String {
        let nopad = if !self.padded { "lu-container-nopad" } else { "" };
        let attrs = [class_attr(["lu-container", self.size.class(), nopad])];
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("div", &attrs, &body)
    }
}

// ---------------------------------------------------------------------------
// Row
// ---------------------------------------------------------------------------

pub struct Row {
    gap: Gap,
    align: Align,
    justify: Justify,
    wrap: bool,
    stack_at: Option<Breakpoint>,
    hide_at: Option<Breakpoint>,
    children: Vec<Child>,
}

pub fn row() -> Row {
    Row { gap: Gap::Md, align: Align::Stretch, justify: Justify::Start,
          wrap: true, stack_at: None, hide_at: None, children: Vec::new() }
}

impl Row {
    pub fn gap(mut self, g: Gap)         -> Self { self.gap = g; self }
    pub fn align(mut self, a: Align)     -> Self { self.align = a; self }
    pub fn justify(mut self, j: Justify) -> Self { self.justify = j; self }
    pub fn nowrap(mut self)              -> Self { self.wrap = false; self }

    /// Collapse into a Column at `breakpoint` (or narrower).
    /// Children go full-width, `min_width`s neutralised.
    pub fn stack_at(mut self, b: Breakpoint) -> Self { self.stack_at = Some(b); self }
    /// Convenience: `stack_at(Breakpoint::Mobile)`.
    pub fn mobile_stack(self) -> Self { self.stack_at(Breakpoint::Mobile) }
    /// Convenience: `stack_at(Breakpoint::Tablet)`.
    pub fn tablet_stack(self) -> Self { self.stack_at(Breakpoint::Tablet) }
    /// Hide this Row at `breakpoint` (or narrower).
    pub fn hide_at(mut self, b: Breakpoint) -> Self { self.hide_at = Some(b); self }

    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }
}

impl Component for Row {
    fn render(&self) -> String {
        let stack = self.stack_at.map(|b| b.stack_class()).unwrap_or("");
        let hide  = self.hide_at.map(|b| match b { Breakpoint::Mobile=>"lu-hide-m", Breakpoint::Tablet=>"lu-hide-t" }).unwrap_or("");
        let nowrap = if !self.wrap { "lu-nowrap" } else { "" };
        let attrs = [class_attr([
            "lu-row", self.gap.class(), self.align.class(), self.justify.class(),
            nowrap, stack, hide,
        ])];
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("div", &attrs, &body)
    }
}

// ---------------------------------------------------------------------------
// Column
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinW { W200, W260, W280, W300, W320, W400 }
impl MinW {
    fn class(self) -> &'static str {
        match self {
            MinW::W200 => "lu-min-w-200",
            MinW::W260 => "lu-min-w-260",
            MinW::W280 => "lu-min-w-280",
            MinW::W300 => "lu-min-w-300",
            MinW::W320 => "lu-min-w-320",
            MinW::W400 => "lu-min-w-400",
        }
    }
}

pub struct Column {
    gap: Gap,
    align: Align,
    justify: Justify,
    flex: Option<u8>,             // 1..=5
    min_w: Option<MinW>,
    hide_at: Option<Breakpoint>,
    children: Vec<Child>,
}

pub fn column() -> Column {
    Column { gap: Gap::Md, align: Align::Stretch, justify: Justify::Start,
             flex: None, min_w: None, hide_at: None, children: Vec::new() }
}

impl Column {
    pub fn gap(mut self, g: Gap)         -> Self { self.gap = g; self }
    pub fn align(mut self, a: Align)     -> Self { self.align = a; self }
    pub fn justify(mut self, j: Justify) -> Self { self.justify = j; self }

    /// Grow factor when this column sits inside a Row (1..=5).
    /// Values outside the range are clamped.
    pub fn flex(mut self, n: u8) -> Self {
        self.flex = Some(n.clamp(1, 5)); self
    }

    /// Prevent this column being crushed on wider screens (typed choices).
    pub fn min_w(mut self, w: MinW) -> Self { self.min_w = Some(w); self }
    /// Back-compat alias — accepts a string like "300px". Prefer [`min_w`].
    pub fn min_width(mut self, w: impl Into<String>) -> Self {
        let w = w.into();
        self.min_w = Some(match w.as_str() {
            "200px" => MinW::W200,
            "260px" => MinW::W260,
            "280px" => MinW::W280,
            "300px" => MinW::W300,
            "320px" => MinW::W320,
            "400px" => MinW::W400,
            _       => MinW::W300, // sensible default
        });
        self
    }
    pub fn hide_at(mut self, b: Breakpoint) -> Self { self.hide_at = Some(b); self }

    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }
}

impl Component for Column {
    fn render(&self) -> String {
        let flex_class = self.flex.map(|n| match n {
            1 => "lu-flex-1", 2 => "lu-flex-2", 3 => "lu-flex-3",
            4 => "lu-flex-4", _ => "lu-flex-5",
        }).unwrap_or("");
        let min_w = self.min_w.map(|m| m.class()).unwrap_or("");
        let hide  = self.hide_at.map(|b| match b { Breakpoint::Mobile=>"lu-hide-m", Breakpoint::Tablet=>"lu-hide-t" }).unwrap_or("");
        let attrs = [class_attr([
            "lu-col", self.gap.class(), self.align.class(), self.justify.class(),
            flex_class, min_w, hide,
        ])];
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("div", &attrs, &body)
    }
}

// ---------------------------------------------------------------------------
// Grid
// ---------------------------------------------------------------------------

/// Fixed column count for [`Grid`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cols { Two, Three, Four, Six }
impl Cols {
    fn class(self) -> &'static str {
        match self { Cols::Two=>"lu-grid-cols-2", Cols::Three=>"lu-grid-cols-3", Cols::Four=>"lu-grid-cols-4", Cols::Six=>"lu-grid-cols-6" }
    }
}

/// Minimum column width for an auto-fit [`Grid`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinCol { W200, W220, W240, W280, W320 }
impl MinCol {
    fn class(self) -> &'static str {
        match self { MinCol::W200=>"lu-grid-fit-200", MinCol::W220=>"lu-grid-fit-220",
                     MinCol::W240=>"lu-grid-fit-240", MinCol::W280=>"lu-grid-fit-280", MinCol::W320=>"lu-grid-fit-320" }
    }
}

pub struct Grid {
    template_class: &'static str,
    gap: Gap,
    stack_at: Option<Breakpoint>,
    children: Vec<Child>,
}

pub fn grid() -> Grid {
    // Default matches the old `repeat(auto-fit, minmax(220px, 1fr))` behaviour.
    Grid { template_class: MinCol::W220.class(), gap: Gap::Md, stack_at: None, children: Vec::new() }
}

impl Grid {
    /// Auto-fit responsive columns — pick the smallest column width you want.
    pub fn cols_min(mut self, m: MinCol) -> Self { self.template_class = m.class(); self }
    /// Fixed number of equal columns (does NOT reflow — use `cols_min` for that).
    pub fn cols(mut self, c: Cols) -> Self { self.template_class = c.class(); self }
    /// Back-compat alias — accepts a "220px" style string. Prefer [`cols_min`].
    pub fn cols_min_str(mut self, s: impl Into<String>) -> Self {
        let s = s.into();
        self.template_class = match s.as_str() {
            "200px" => MinCol::W200.class(),
            "220px" => MinCol::W220.class(),
            "240px" => MinCol::W240.class(),
            "280px" => MinCol::W280.class(),
            "320px" => MinCol::W320.class(),
            _       => MinCol::W220.class(),
        };
        self
    }

    pub fn gap(mut self, g: Gap) -> Self { self.gap = g; self }

    /// Collapse the grid to a single column at this breakpoint (or narrower).
    pub fn stack_at(mut self, b: Breakpoint) -> Self { self.stack_at = Some(b); self }
    pub fn mobile_stack(self) -> Self { self.stack_at(Breakpoint::Mobile) }
    pub fn tablet_stack(self) -> Self { self.stack_at(Breakpoint::Tablet) }

    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }
}

impl Component for Grid {
    fn render(&self) -> String {
        let stack = self.stack_at.map(|b| b.stack_class()).unwrap_or("");
        let attrs = [class_attr(["lu-grid", self.template_class, self.gap.class(), stack])];
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("div", &attrs, &body)
    }
}

// ---------------------------------------------------------------------------
// Spacer
// ---------------------------------------------------------------------------

pub struct Spacer;
pub fn spacer() -> Spacer { Spacer }
impl Component for Spacer {
    fn render(&self) -> String { r#"<div class="lu-spacer"></div>"#.into() }
}

// ---------------------------------------------------------------------------
// Divider — thin visual separator (horizontal by default, vertical inside a Row)
// ---------------------------------------------------------------------------

/// Orientation of a [`Divider`]. Horizontal is the default and works in any
/// block context; Vertical is meant to sit between siblings inside a `Row`
/// (or any flex container).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerAxis { Horizontal, Vertical }

pub struct Divider { axis: DividerAxis }

/// A 1-px separator line. `divider()` is horizontal; call `.vertical()` to
/// switch. Styled by the `lu-divider*` rules in `layout.css` — no inline CSS.
pub fn divider() -> Divider { Divider { axis: DividerAxis::Horizontal } }

impl Divider {
    pub fn vertical(mut self)   -> Self { self.axis = DividerAxis::Vertical;   self }
    pub fn horizontal(mut self) -> Self { self.axis = DividerAxis::Horizontal; self }
}

impl Component for Divider {
    fn render(&self) -> String {
        let cls = match self.axis {
            DividerAxis::Horizontal => "lu-divider",
            DividerAxis::Vertical   => "lu-divider lu-divider-vertical",
        };
        // <hr> for horizontal (semantic + accessible); <span> for vertical
        // so it can live inline in a flex row without breaking layout.
        match self.axis {
            DividerAxis::Horizontal => format!(r#"<hr class="{cls}">"#),
            DividerAxis::Vertical   => format!(r#"<span class="{cls}" role="separator" aria-orientation="vertical"></span>"#),
        }
    }
}

// ---------------------------------------------------------------------------
// Action row — the ONE preset every page uses for button bars.
// ---------------------------------------------------------------------------

/// Convenience preset for a **horizontal action bar** — a `row()` pre-configured
/// with:
///
/// * `Gap::Md`        — consistent spacing between actions,
/// * `Align::Center`  — icon-buttons and plain buttons share the same baseline,
/// * default wrap enabled — bars wrap gracefully on narrow viewports.
///
/// Use this everywhere you have a group of buttons (dialogs, form footers,
/// toolbars, card footers). It's just `row().gap(Md).align(Center)` — nothing
/// magical — but centralising it means every action bar in the app looks the
/// same and you never have to remember the tokens.
///
/// ```ignore
/// row_actions()
///     .add(button().label("Save").variant(Variant::Primary).icon(Icons::CHECK))
///     .add(button().label("Cancel").variant(Variant::Secondary))
///     .add(button().label("Delete").variant(Variant::Danger).icon(Icons::DELETE));
/// ```
///
/// Need the buttons pushed to the right? Chain `.justify(Justify::End)`.
/// Need Save on the right and Cancel on the left? Insert a `spacer()`
/// between them.
pub fn row_actions() -> Row {
    row().gap(Gap::Md).align(Align::Center)
}

// ---------------------------------------------------------------------------
// Section
// ---------------------------------------------------------------------------

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
    pub fn action(mut self, c: impl Component + 'static) -> Self { self.actions.push(Box::new(c)); self }
    pub fn add(mut self, c: impl Component + 'static)    -> Self { self.children.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.children.push(Box::new(c)); } self
    }
}

impl Component for Section {
    fn render(&self) -> String {
        let has_header = self.title.is_some() || self.subtitle.is_some() || !self.actions.is_empty();
        let mut out = String::from(r#"<section class="lu-section">"#);

        if has_header {
            out.push_str(r#"<header class="lu-section-header">"#);
            if let Some(ref t) = self.title {
                out.push_str(&format!("<h2>{}</h2>", escape_html(t)));
            }
            if let Some(ref s) = self.subtitle {
                out.push_str(&format!(r#"<span class="lu-section-subtitle">{}</span>"#, escape_html(s)));
            }
            if !self.actions.is_empty() {
                out.push_str(r#"<div class="lu-section-actions">"#);
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
