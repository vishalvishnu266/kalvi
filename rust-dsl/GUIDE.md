# lit-ui — Quick Guide

Everything you need to build a real ERP page in five minutes.

- [1. The 30-second mental model](#1-the-30-second-mental-model)
- [2. Your first page](#2-your-first-page)
- [3. Using components](#3-using-components)
- [4. Layout: rows, columns, grids, sections](#4-layout-rows-columns-grids-sections)
- [5. Making it responsive](#5-making-it-responsive)
- [6. Modifying / extending the layout system](#6-modifying--extending-the-layout-system)
- [7. Rendering the page — file or Axum handler](#7-rendering-the-page--file-or-axum-handler)
- [8. Cheat sheet](#8-cheat-sheet)

---

## 1. The 30-second mental model

There are three layers, and only three:

| Layer | What lives there | Where |
| --- | --- | --- |
| **HTML tags** | The `<ui-*>` Lit web components | `lit-components/components/*.js` |
| **Layout classes** | `.lu-row`, `.lu-col`, `.lu-grid`, `.lu-gap-md`, `.lu-stack-m`, … | `lit-components/assets/layout.css` |
| **Rust builders** | Typed functions like `button()`, `card()`, `row()`, `stat()` | `rust-dsl/src/**` |

**The Rust builders emit HTML + class names**. That's it. No inline `style=""`, no CSS-in-JS, no macros. Everything responsive is CSS, driven by class names that Rust picks via typed enums.

---

## 2. Your first page

```rust
use lit_ui::prelude::*;

fn main() {
    let html = page()
        .title("Hello")
        .add(container()
            .add(card()
                .title("Welcome")
                .padded()
                .add(button().label("Save").variant(Variant::Primary))))
        .render();          // → String

    println!("{html}");
}
```

Save that as an example under `rust-dsl/examples/hello.rs`, register it in `rust-dsl/Cargo.toml`, run `cargo run -p lit-ui --example hello`. Or feed the string into an Axum handler:

```rust
use axum::response::Html;

async fn hello_page() -> Html<String> {
    Html(hello::build().render())
}
```

---

## 3. Using components

Every component follows the same three-line rhythm:

```rust
component_name()                 // 1. constructor (free function, lowercase)
    .attribute(...)              // 2. chainable setters
    .add(child)                  // 3. optional children
```

### Common patterns

**Buttons:**
```rust
button().label("Save")                                       // primary by default
button().label("Cancel").variant(Variant::Secondary)
button().label("Delete").variant(Variant::Danger).icon("x")
button().label("Add").icon("plus").size(Size::Sm)
button().label("Disabled").disabled()
button().label("Full width").full()
```

**Inputs:**
```rust
input().label("Full name").name("fullName").required()
input().label("Email").name("email").kind(InputType::Email)
input().label("Notes").kind(InputType::Textarea).hint("Optional")
```

**Cards:**
```rust
card()
    .title("Attendance")
    .subtitle("This week")
    .padded()
    .action(button().label("See all").variant(Variant::Ghost))   // top-right action
    .add(some_content)
    .add(other_content)
```

**Badges + tones:**
```rust
badge("Present").tone(Tone::Success).dot()
badge("Late").tone(Tone::Warning).dot()
badge("Absent").tone(Tone::Danger).dot()
```

**Data table (typed):**
```rust
let mut t = data_table("students-table")
    .searchable().selectable().per_page(10)
    .col("name",   "Name",   ColOpts::text().sortable())
    .col("grade",  "Grade",  ColOpts::text().sortable().center())
    .col("status", "Status",
        ColOpts::render(
            "(v)=>`<ui-badge tone=\"${v==='present'?'success':'danger'}\">${v}</ui-badge>`"
        ).sortable());

for s in &students {
    t = t.row(vec![
        ("name", s.name.to_string()),
        ("grade", s.grade.to_string()),
        ("status", s.status.to_string()),
    ]);
}
```

**Form:**
```rust
form().action("/students").method("post")
    .add(input().label("Full name").name("fullName").required())
    .add(select().label("Grade").name("grade").required()
        .option(SelectOption::new("5", "Grade 5"))
        .option(SelectOption::new("6", "Grade 6")))
    .add(checkbox("I agree").name("agree").required())
    .add(switch("Email me").name("notify").checked())
    .action_btn(button().label("Reset").variant(Variant::Secondary))
    .action_btn(button().label("Save").variant(Variant::Primary))
```

**Full inventory:** `button`, `input`, `select`, `combobox`, `checkbox`, `radio` + `radio_group`, `switch`, `datepicker`, `date_range`, `file_upload`, `inline_edit`, `form`, `card`, `badge`, `icon`, `avatar`, `avatar_group`, `stat`, `list_item`, `table`, `data_table`, `pagination`, `breadcrumb`, `tab_bar`, `segmented`, `tooltip`, `drawer`, `modal`, `dropdown_menu`, `toast` + `toast_host`, `command` + `command_item`, `progress`, `empty_state`, `skeleton`, `stepper`, `timeline` + `timeline_item`, `kanban` + `kanban_column` + `kanban_card`.

---

## 4. Layout: rows, columns, grids, sections

Only five primitives. Everything else composes from them.

### `container()` — centered page frame

```rust
container()                                    // default: max-width 1200px
container().size(ContainerSize::Md)            // 960px  (sm=640, md=960, lg=1200, xl=1400)
container().fluid()                            // 100%
container().no_padding()                       // remove side padding
```

### `row()` — horizontal flex, wraps by default

```rust
row()
    .gap(Gap::Md)                              // Gap::{None,Xs,Sm,Md,Lg,Xl,Xxl}
    .align(Align::Center)                      // Start / Center / End / Stretch / Baseline
    .justify(Justify::Between)                 // Start / Center / End / Between / Around / Evenly
    .add(first)
    .add(spacer())                             // pushes remaining children to the right
    .add(last)
```

### `column()` — vertical flex

```rust
column()
    .gap(Gap::Sm)
    .flex(2)                                   // proportional grow when inside a Row (1..=5)
    .min_w(MinW::W300)                         // W200 / W260 / W280 / W300 / W320 / W400
    .add(...)
```

### `grid()` — responsive card walls

```rust
// Fluid — as many columns of at least the min width as fit
grid().cols_min(MinCol::W220).gap(Gap::Md)
    .add(kpi_card_1)
    .add(kpi_card_2)
    .add(kpi_card_3)
    .add(kpi_card_4)

// Fixed
grid().cols(Cols::Three)                       // exactly 3 equal columns
```

### `section()` — semantic titled block (H2 + subtitle + right-aligned actions)

```rust
section()
    .title("Fees")
    .subtitle("August 2026")
    .action(button().label("Export").icon("upload"))
    .action(button().label("New invoice").icon("plus"))
    .add(fees_table)
```

Renders as `<section><header><h2>…</h2>…</header>…children…</section>`.

---

## 5. Making it responsive

**Everything is responsive by default** — grids auto-fit, rows wrap. But `.wrap` alone isn't enough when you want the *layout* to change (e.g. side-by-side becomes stacked). That's what breakpoint helpers are for.

### `.mobile_stack()` — the workhorse

Any `Row` or `Grid` can be told "at ≤ 640 px, become a single column":

```rust
row().gap(Gap::Lg).align(Align::Start).mobile_stack()
    .add(column().flex(2).min_w(MinW::W320).add(main_content))
    .add(column().flex(1).min_w(MinW::W260).add(side_panel))
```

- **Desktop:** `main` (2/3) | `side` (1/3), respecting `min_w`.
- **Mobile:** `main` full-width, `side` underneath, full-width, same `Gap::Lg` between them.

### The three helpers you'll actually use

| Helper | Meaning | Available on |
| --- | --- | --- |
| `.mobile_stack()`  | Collapse to a Column at ≤ 640 px | `Row`, `Grid` |
| `.tablet_stack()`  | Collapse to a Column at ≤ 900 px | `Row`, `Grid` |
| `.hide_at(Breakpoint::Mobile)` | Hide entirely at that breakpoint | `Row`, `Column` |

That's the whole responsive vocabulary. No `@media`, no calc, no random px values in Rust.

### The two breakpoints

```rust
Breakpoint::Mobile   // ≤ 640 px  (phones)
Breakpoint::Tablet   // ≤ 900 px  (tablets + narrow laptops)
```

### Real-world recipes

**A two-column app-shell body (main + sidebar):**
```rust
row().gap(Gap::Lg).align(Align::Start).mobile_stack()
    .add(column().flex(3).min_w(MinW::W300).add(main))
    .add(column().flex(1).min_w(MinW::W280).add(sidebar))
```

**A three-KPI dashboard row that stays readable on all sizes:**
```rust
grid().cols_min(MinCol::W240).gap(Gap::Md)
    .add(stat_card_1)
    .add(stat_card_2)
    .add(stat_card_3)
// → 3 cols on desktop, 2 on tablet, 1 on mobile — automatic
```

**A header that stacks on phones (breadcrumb above buttons):**
```rust
row().align(Align::Center).gap(Gap::Md).mobile_stack()
    .add(breadcrumb().item(...))
    .add(spacer())
    .add(button().label("Export"))
    .add(button().label("Add"))
```

**Hide a side rail on mobile (show only the main content):**
```rust
row().gap(Gap::Lg)
    .add(column().flex(3).add(main))
    .add(column().flex(1).hide_at(Breakpoint::Mobile).add(context_rail))
```

---

## 6. Modifying / extending the layout system

Every layout primitive resolves to a **class name** that lives in one file:
`lit-components/assets/layout.css`

**Two rules:**
1. Never write inline `style="…"` inside a component builder for layout.
2. If you need a new class, add it to `layout.css`, then add a Rust enum variant that maps to it.

### Example — adding a new gap size `Xxxl` (48 px)

**1. Add the CSS class:**
```css
/* lit-components/assets/layout.css */
.lu-gap-xxxl { gap: 48px; }
```

**2. Add the enum variant:**
```rust
// rust-dsl/src/layout.rs
pub enum Gap { None, Xs, Sm, Md, Lg, Xl, Xxl, Xxxl }

impl Gap {
    fn class(self) -> &'static str {
        match self {
            /* … existing arms … */
            Gap::Xxxl => "lu-gap-xxxl",
        }
    }
}
```

**3. Use it:**
```rust
row().gap(Gap::Xxxl).add(...)
```

The pattern is identical for a new min-width, new breakpoint, new grid template, new alignment, whatever. **Class first, enum second, use it in Rust.**

### Example — adding a new responsive helper `.desktop_only()`

**1. CSS:**
```css
.lu-desktop-only { display: none; }
@media (min-width: 901px) { .lu-desktop-only { display: initial; } }
```

**2. Rust — add a builder method to `Row`, `Column`, `Grid`:**
```rust
impl Row {
    pub fn desktop_only(mut self) -> Self { self.extra_class = Some("lu-desktop-only"); self }
}
```
(You'd also add `extra_class: Option<&'static str>` as a field, and include it in the `class_attr(...)` call inside `render()`.)

### Never do this in a component

```rust
// ❌ BAD — leaks CSS into Rust
wrap("div", &[Attr::kv("style", "display:flex;gap:16px")], &body)

// ✅ GOOD — use the class layer
wrap("div", &[class_attr(["lu-row", "lu-gap-md"])], &body)
```

If a component needs a *visual* style (colour, radius, shadow), that goes into the Lit component's shadow-DOM CSS in `lit-components/components/ui-*.js`, not into Rust.

---

## 7. Rendering the page — file or Axum handler

### File (dev iteration)

Every example writes its HTML file directly to `lit-components/dsl-*.html`:

```bash
cargo run -p lit-ui --example students
cargo run -p lit-ui --example dashboard
cargo run -p lit-ui --example fees
cargo run -p lit-ui --example attendance
```

Then open `http://localhost:3000/lit-components/dsl-students.html`.

### Axum handler (production)

```rust
// src/web/dsl.rs
use axum::response::Html;
use lit_ui::core::Component;
use lit_ui::pages::students;

pub async fn students_page() -> Html<String> {
    let rows = students::mock_students();       // swap for real DB call
    Html(students::build(&rows).render())
}
```

Wire it in `src/http/routes.rs`:
```rust
.route("/dsl/students", get(wd::students_page))
```

Serve `lit-components/` off disk (already set up):
```rust
.route("/lit-components/{*path}", get(wa::serve_lit_components))
```

That's it — visit `/dsl/students` and you get a live page rendered from Rust with typed data.

---

## 8. Cheat sheet

```rust
use lit_ui::prelude::*;

// Layout
container()                             // .size(ContainerSize::Lg | Md | Sm | Xl | Fluid)
row()      .gap(Gap::Md).align(Align::Center).justify(Justify::Between).mobile_stack()
column()   .flex(2).min_w(MinW::W300).hide_at(Breakpoint::Mobile)
grid()     .cols_min(MinCol::W220).mobile_stack()
grid()     .cols(Cols::Three)
section()  .title("").subtitle("").action(...).add(...)
spacer()

// Enums
Gap::{None, Xs, Sm, Md, Lg, Xl, Xxl}
Align::{Start, Center, End, Stretch, Baseline}
Justify::{Start, Center, End, Between, Around, Evenly}
Breakpoint::{Mobile, Tablet}          // 640px, 900px
ContainerSize::{Sm, Md, Lg, Xl, Fluid}
MinW::{W200, W260, W280, W300, W320, W400}
MinCol::{W200, W220, W240, W280, W320}
Cols::{Two, Three, Four, Six}

// Text & data
badge("Present").tone(Tone::Success).dot()
icon("check").size(20)
avatar("Aarav K.").size(AvatarSize::Sm)
avatar_group().max(4).add(...).add(...)
stat("Present", "412").icon("check").trend(Trend::Up).delta("+3.2%")
list_item("Aarav K.").subtitle("Roll 12").leading(...).trailing(...)
breadcrumb().item(Crumb::link("Home", "/")).item(Crumb::current("Students"))

// Forms
input().label("Name").name("name").required()
input().kind(InputType::Email | Password | Number | Textarea)
select().label("Grade").required().option(SelectOption::new("5", "Grade 5"))
checkbox("Agree").required()
switch("Notify").checked()
radio_group("gender").value("F").option(radio("M", "Male")).option(radio("F", "Female"))
datepicker().value("2026-08-14")
date_range().from("2026-08-01").to("2026-08-31")
combobox().option(ComboOption::new("k", "K")).allow_new()
file_upload().accept(".csv").multiple().max_size(5_242_880)

// Buttons
button().label("Save").variant(Variant::Primary | Secondary | Ghost | Danger).size(Size::Md).icon("check").disabled().full()

// Containers / overlays
card().title("").subtitle("").padded().action(...).add(...)
drawer().placement(DrawerPlacement::Right).size(DrawerSize::Md).title("").add(...).footer(...)
modal().title("").add(...).footer(...)
dropdown_menu().align(MenuAlign::End).trigger(button()).item(MenuItem::link("...", "/x")).divider().item(MenuItem::action("...").danger())
tooltip("Hover me").placement(Placement::Top).add(button())
tab_bar().tab(Tab::new("Overview", "#/o").active()).tab(Tab::new("Attendance", "#/a"))
segmented().value("week").segment(Segment::new("day","Day")).segment(Segment::new("week","Week"))

// Feedback / workflow
progress(65).label("Fees").tone(ProgTone::Success).show_value()
progress(0).indeterminate()
empty_state("Nothing here").icon("users").action(button().label("Add"))
skeleton().shape(SkeletonShape::Circle).width("40")
stepper().current(1).clickable().step("A").step("B").step("C")
timeline().item(timeline_item().icon("check").tone(TimelineTone::Success).time("10:24").add(...))
kanban().column(kanban_column("To do").add(kanban_card().text("Task")))
command().item(command_item("Add student").icon("plus").group("Actions").action("new-student"))
toast_host()

// Table
data_table("id").searchable().selectable().per_page(10)
    .col("name", "Name", ColOpts::text().sortable())
    .col("status", "Status", ColOpts::render("(v)=>`<ui-badge>${v}</ui-badge>`"))
    .row(vec![("name", "Aarav".into()), ("status", "present".into())])

pagination(3, 125).per_page(10)

// Page shell
page().title("Students").add(container().add(...)).render()   // → String
```

---

**Quick rule of thumb:**
> "If it changes with screen size, add `.mobile_stack()` or `.hide_at(...)`.
> If it needs a new layout behaviour, add a class in `layout.css` and an enum variant.
> Never write inline `style=""` for layout inside Rust."
