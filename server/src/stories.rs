//! Storybook content — every story is a `Box<dyn Component>` built from
//! the `lit_ui` DSL. No raw HTML anywhere: even section titles/labels
//! are composed as primitives (button labels, badge text, etc.).
//!
//! Add a new story by pushing another `story(name, component)` entry
//! into the appropriate section in [`all_sections`]. That's it — the
//! main route picks it up automatically.

use lit_ui::core::Component;
use lit_ui::prelude::*;

/// A single story: a short name + a fully-configured primitive.
pub type Story = (&'static str, Box<dyn Component>);

/// A group of related stories rendered under a shared heading.
pub struct Section {
    pub title:   &'static str,
    pub stories: Vec<Story>,
}

/// The complete storybook. Sections render in this order, top to bottom.
pub fn all_sections() -> Vec<Section> {
    vec![
        Section { title: "Button",       stories: button_stories()       },
        Section { title: "Input",        stories: input_stories()        },
        Section { title: "Select",       stories: select_stories()       },
        Section { title: "Checkbox",     stories: checkbox_stories()     },
        Section { title: "Radio",        stories: radio_stories()        },
        Section { title: "Switch",       stories: switch_stories()       },
        Section { title: "Datepicker",   stories: datepicker_stories()   },
        Section { title: "Date range",   stories: daterange_stories()    },
        Section { title: "Icon",         stories: icon_stories()         },
        Section { title: "Badge",        stories: badge_stories()        },
        Section { title: "Avatar",       stories: avatar_stories()       },
        Section { title: "Tooltip",      stories: tooltip_stories()      },
        Section { title: "Skeleton",     stories: skeleton_stories()     },
        Section { title: "Slider",       stories: slider_stories()       },
        Section { title: "Progress",     stories: progress_stories()     },
        Section { title: "Color swatch", stories: color_swatch_stories() },
        Section { title: "Theme toggle", stories: theme_toggle_stories() },
        Section { title: "Layout",       stories: layout_stories()       },
    ]
}

// ── helpers ─────────────────────────────────────────────────────────────

/// Tiny helper so each story reads as one line: `story("name", widget)`.
fn story<C: Component + 'static>(name: &'static str, c: C) -> Story {
    (name, Box::new(c))
}

/// Wrap a set of primitives in a `ui-cluster` — the go-to "row of same-
/// height things" layout. Handy for "show all variants side-by-side".
fn row<I, C>(children: I) -> impl Component
where
    I: IntoIterator<Item = C>,
    C: Component + 'static,
{
    // The `ui-cluster` primitive already handles gap + wrap + alignment;
    // we just push children into it.
    let mut c = raw_cluster();
    for child in children { c = c.push_boxed(Box::new(child)); }
    c
}

/// A minimal `ui-cluster` wrapper built as its own tiny component so we
/// can put `Box<dyn Component>` children into it (the derived cluster's
/// `.add()` takes generics, which don't compose over trait objects).
struct RawCluster { children: Vec<Box<dyn Component>>, gap: &'static str }
fn raw_cluster() -> RawCluster { RawCluster { children: Vec::new(), gap: "sm" } }
impl RawCluster {
    fn push_boxed(mut self, c: Box<dyn Component>) -> Self { self.children.push(c); self }
    #[allow(dead_code)]
    fn gap(mut self, g: &'static str) -> Self { self.gap = g; self }
}
impl Component for RawCluster {
    fn render(&self) -> String {
        let mut body = String::new();
        for c in &self.children { body.push_str(&c.render()); }
        format!(r#"<ui-cluster gap="{}">{}</ui-cluster>"#, self.gap, body)
    }
}

// ── sections ────────────────────────────────────────────────────────────

fn button_stories() -> Vec<Story> {
    vec![
        story("default",  button().label("Save")),
        story("variants", row([
            button().label("Primary"),
            button().label("Secondary").variant(Variant::Secondary),
            button().label("Ghost").variant(Variant::Ghost),
            button().label("Danger").variant(Variant::Danger),
        ])),
        story("sizes", row([
            button().label("Small").size(Size::Sm),
            button().label("Medium").size(Size::Md),
            button().label("Large").size(Size::Lg),
        ])),
        story("with icon", row([
            button().label("Save").icon(Icons::CHECK),
            button().label("Delete").icon(Icons::DELETE).variant(Variant::Danger),
            button().label("Upload").icon(Icons::UPLOAD).variant(Variant::Secondary),
        ])),
        story("states", row([
            button().label("Normal"),
            button().label("Disabled").disabled(),
            button().label("Full width").full(),
        ])),
        story("submit / reset", row([
            button().label("Submit").submit(),
            button().label("Reset").reset().variant(Variant::Ghost),
        ])),
    ]
}

fn input_stories() -> Vec<Story> {
    vec![
        story("default",   input().label("Full name").placeholder("Aarav")),
        story("required",  input().label("Email").kind(InputType::Email).required()),
        story("password",  input().label("Password").kind(InputType::Password)),
        story("with hint", input().label("Handle").hint("3–20 chars, a–z 0–9 _")),
        story("with error", input().label("Handle").error("Required")),
        story("with icons", input().label("Search").icon_leading(Icons::SEARCH).icon_trailing(Icons::X)),
    ]
}

fn select_stories() -> Vec<Story> {
    vec![
        story("default", select()
            .label("Fruit")
            .option(SelectOption::new("a", "Apples"))
            .option(SelectOption::new("b", "Bananas"))
            .option(SelectOption::new("c", "Cherries"))),
        story("searchable + clearable", select()
            .label("Country").placeholder("Choose one").searchable().clearable()
            .option(SelectOption::new("in", "India"))
            .option(SelectOption::new("us", "United States"))
            .option(SelectOption::new("uk", "United Kingdom"))),
        story("multi + allow-new", select()
            .label("Tags").placeholder("Type to add").multiple().allow_new()),
        story("with error", select()
            .label("Priority").error("Please pick one")
            .option(SelectOption::new("hi", "High"))
            .option(SelectOption::new("lo", "Low"))),
    ]
}

fn checkbox_stories() -> Vec<Story> {
    vec![
        story("default",       checkbox("I agree")),
        story("checked",       checkbox("Enable telemetry").checked()),
        story("indeterminate", checkbox("Select all").indeterminate()),
        story("disabled",      checkbox("Locked").disabled().checked()),
        story("with error",    checkbox("Terms").error("Required to continue")),
    ]
}

fn radio_stories() -> Vec<Story> {
    vec![
        story("horizontal", radio_group("fruit").value("a").horizontal()
            .option(radio("a", "Apples"))
            .option(radio("b", "Bananas"))
            .option(radio("c", "Cherries"))),
        story("vertical", radio_group("plan").value("pro")
            .option(radio("free",   "Free tier"))
            .option(radio("pro",    "Pro"))
            .option(radio("teams",  "Teams"))
            .option(radio("enterp", "Enterprise").disabled())),
        story("with error", radio_group("x").error("Pick one")
            .option(radio("y", "Yes"))
            .option(radio("n", "No"))),
    ]
}

fn switch_stories() -> Vec<Story> {
    vec![
        story("default",  switch("Notifications")),
        story("checked",  switch("Marketing e-mails").checked()),
        story("disabled", switch("Locked").disabled().checked()),
    ]
}

fn datepicker_stories() -> Vec<Story> {
    vec![
        story("default",     datepicker().label("Date of birth")),
        story("prefilled",   datepicker().label("Joined").value("2024-01-15")),
        story("with error",  datepicker().label("Start").error("Required")),
    ]
}

fn daterange_stories() -> Vec<Story> {
    vec![
        story("default",   date_range().label("Report period")),
        story("filled",    date_range().label("Q1").from("2024-01-01").to("2024-03-31")),
    ]
}

fn icon_stories() -> Vec<Story> {
    vec![
        story("common actions", row([
            icon(Icons::CHECK), icon(Icons::X), icon(Icons::PLUS),
            icon(Icons::EDIT), icon(Icons::DELETE), icon(Icons::SEARCH),
            icon(Icons::FILTER), icon(Icons::SETTINGS), icon(Icons::REFRESH),
        ])),
        story("navigation", row([
            icon(Icons::HOME), icon(Icons::CHEVRON_LEFT), icon(Icons::CHEVRON_RIGHT),
            icon(Icons::ARROW_UP), icon(Icons::ARROW_DOWN),
        ])),
        story("sizes", row([
            icon(Icons::STAR).size(12),
            icon(Icons::STAR).size(18),
            icon(Icons::STAR).size(24),
            icon(Icons::STAR).size(36),
        ])),
    ]
}

fn badge_stories() -> Vec<Story> {
    vec![
        story("tones", row([
            badge("Neutral"),
            badge("Brand").tone(Tone::Brand),
            badge("Success").tone(Tone::Success),
            badge("Warning").tone(Tone::Warning),
            badge("Danger").tone(Tone::Danger),
            badge("Info").tone(Tone::Info),
        ])),
        story("with dot", row([
            badge("Live").tone(Tone::Success).dot(),
            badge("Draft").dot(),
            badge("Overdue").tone(Tone::Danger).dot(),
        ])),
    ]
}

fn avatar_stories() -> Vec<Story> {
    vec![
        story("sizes", row([
            avatar("Ada Lovelace").size(AvatarSize::Sm),
            avatar("Ada Lovelace").size(AvatarSize::Md),
            avatar("Ada Lovelace").size(AvatarSize::Lg),
            avatar("Ada Lovelace").size(AvatarSize::Xl),
        ])),
        story("initials from name", row([
            avatar("Grace Hopper"),
            avatar("Alan Turing"),
            avatar("Kernighan"),
        ])),
    ]
}

fn tooltip_stories() -> Vec<Story> {
    vec![
        story("placements", row([
            tooltip("Top").add(button().label("Top")),
            tooltip("Bottom").placement(Placement::Bottom).add(button().label("Bottom")),
            tooltip("Left").placement(Placement::Left).add(button().label("Left")),
            tooltip("Right").placement(Placement::Right).add(button().label("Right")),
        ])),
    ]
}

fn skeleton_stories() -> Vec<Story> {
    vec![
        story("line",    skeleton().width("240px")),
        story("rect",    skeleton().shape(SkeletonShape::Rect).width("240px").height("120px")),
        story("circle",  skeleton().shape(SkeletonShape::Circle).width("48px").height("48px")),
        story("multi-line", skeleton().lines(3).width("360px")),
    ]
}

fn slider_stories() -> Vec<Story> {
    vec![
        story("default",              slider().value(42).show_value()),
        story("custom bounds + step", slider().min(-20).max(20).step(2).value(0).show_value()),
        story("disabled",             slider().value(30).show_value().disabled()),
    ]
}

fn progress_stories() -> Vec<Story> {
    vec![
        story("tones", row([
            progress().value(25),
            progress().value(50).tone(ProgressTone::Brand),
            progress().value(75).tone(ProgressTone::Success),
            progress().value(60).tone(ProgressTone::Warning),
            progress().value(80).tone(ProgressTone::Danger),
        ])),
        story("with label", progress().value(65).tone(ProgressTone::Brand).label("Uploading photo")),
        story("indeterminate", progress().indeterminate().tone(ProgressTone::Brand)),
    ]
}

fn color_swatch_stories() -> Vec<Story> {
    vec![
        story("palette", row([
            color_swatch("#ef4444"), color_swatch("#f97316"), color_swatch("#eab308"),
            color_swatch("#22c55e"), color_swatch("#06b6d4"), color_swatch("#3b82f6"),
            color_swatch("#8b5cf6"), color_swatch("#ec4899"),
        ])),
        story("sizes", row([
            color_swatch("#4f46e5").size(SwatchSize::Sm),
            color_swatch("#4f46e5").size(SwatchSize::Md),
            color_swatch("#4f46e5").size(SwatchSize::Lg),
        ])),
        story("selectable + selected", row([
            color_swatch("#ef4444").selectable(),
            color_swatch("#22c55e").selectable().selected(),
            color_swatch("#3b82f6").selectable(),
        ])),
    ]
}

fn theme_toggle_stories() -> Vec<Story> {
    vec![
        story("default", theme_toggle()),
    ]
}

// ── Layout stories ──
// The layout primitives are web components but they don't have Rust
// builders (yet). We wrap them in small local structs so the storybook
// composes end-to-end via the DSL — no raw HTML string in the route.

fn layout_stories() -> Vec<Story> {
    vec![
        story("columns 1:2:1", Columns::ratios("1 2 1").with_cells(3)),
        story("columns 4 equal", Columns::cols(4).with_cells(4)),
        story("stack (vertical)", Stack::default().with_cells(3)),
        story("grid auto-fit min 180", Grid::auto(180).with_cells(6)),
        story("sidebar 200px", Sidebar::default().with_cells(3)),
    ]
}

// A generic filler cell used by the layout stories so the shape is
// obvious. Uses `<ui-badge>` under the hood so it's a real primitive.
fn cells(n: usize) -> Vec<Box<dyn Component>> {
    (1..=n)
        .map(|i| Box::new(badge(format!("{i}")).tone(Tone::Brand)) as Box<dyn Component>)
        .collect()
}

// Minimal DSL wrappers around the layout web components. Each is a
// standalone Component so it plugs into `story(...)` cleanly.
struct Columns { ratios: Option<String>, cols: Option<u32>, children: Vec<Box<dyn Component>> }
impl Columns {
    fn ratios(s: impl Into<String>) -> Self { Self { ratios: Some(s.into()), cols: None, children: Vec::new() } }
    fn cols(n: u32) -> Self { Self { ratios: None, cols: Some(n), children: Vec::new() } }
    fn with_cells(mut self, n: usize) -> Self { self.children = cells(n); self }
}
impl Component for Columns {
    fn render(&self) -> String {
        let attr = match (&self.ratios, self.cols) {
            (Some(r), _)   => format!(r#"ratios="{}""#, r),
            (None, Some(c)) => format!(r#"cols="{}""#, c),
            _ => String::new(),
        };
        let body: String = self.children.iter().map(|c| c.render()).collect();
        format!(r#"<ui-columns {attr} gap="sm">{body}</ui-columns>"#)
    }
}

struct Stack { gap: &'static str, children: Vec<Box<dyn Component>> }
impl Default for Stack { fn default() -> Self { Self { gap: "sm", children: Vec::new() } } }
impl Stack { fn with_cells(mut self, n: usize) -> Self { self.children = cells(n); self } }
impl Component for Stack {
    fn render(&self) -> String {
        let body: String = self.children.iter().map(|c| c.render()).collect();
        format!(r#"<ui-stack gap="{}">{}</ui-stack>"#, self.gap, body)
    }
}

struct Grid { min_col: u32, children: Vec<Box<dyn Component>> }
impl Grid {
    fn auto(min: u32) -> Self { Self { min_col: min, children: Vec::new() } }
    fn with_cells(mut self, n: usize) -> Self { self.children = cells(n); self }
}
impl Component for Grid {
    fn render(&self) -> String {
        let body: String = self.children.iter().map(|c| c.render()).collect();
        format!(r#"<ui-grid min-col="{}" gap="sm">{}</ui-grid>"#, self.min_col, body)
    }
}

struct Sidebar { width: &'static str, side_children: Vec<Box<dyn Component>>, main_children: Vec<Box<dyn Component>> }
impl Default for Sidebar { fn default() -> Self { Self { width: "200px", side_children: Vec::new(), main_children: Vec::new() } } }
impl Sidebar {
    fn with_cells(mut self, n: usize) -> Self {
        // Half the cells go into the sidebar, half into main.
        let mid = n / 2 + 1;
        self.side_children = cells(mid);
        self.main_children = cells(n - mid + 2);
        self
    }
}
impl Component for Sidebar {
    fn render(&self) -> String {
        // Wrap each side/main child stack in a `<ui-stack>` so the demo
        // reads clearly, then slot into `<ui-sidebar>`.
        let side: String = self.side_children.iter().map(|c| c.render()).collect();
        let main: String = self.main_children.iter().map(|c| c.render()).collect();
        format!(
            r#"<ui-sidebar width="{w}" collapse-at="0"><ui-stack slot="side" gap="xs">{side}</ui-stack><ui-stack gap="xs">{main}</ui-stack></ui-sidebar>"#,
            w = self.width, side = side, main = main,
        )
    }
}
