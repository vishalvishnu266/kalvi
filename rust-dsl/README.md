# lit-ui — a macro-free Rust DSL over the Lit component kit

A tiny Vaadin-style Rust library that lets you compose UIs with typed builders
and get back an HTML string that mounts our Lit web-components:

```rust
use lit_ui::prelude::*;

let html = card()
    .title("Add student")
    .padded()
    .add(input().label("Full name").name("fullName").required())
    .add(input().label("Email").name("email").kind(InputType::Email))
    .add(button().label("Save").variant(Variant::Primary).icon("check"))
    .render();
```

Produces (formatted for readability):

```html
<ui-card title="Add student" padded>
  <ui-input type="text" label="Full name" name="fullName" required></ui-input>
  <ui-input type="email" label="Email" name="email"></ui-input>
  <ui-button variant="primary" size="md" icon="check">Save</ui-button>
</ui-card>
```

## Design principles

* **No macros.** Everything is normal Rust code. Great IDE support, no
  proc-macro compile hit, readable error messages.
* **No dependencies.** The crate is just data structures + a small HTML escaper.
* **String rendering.** `Component::render()` returns an owned `String`. Plug
  it into Axum handlers, SSE streams, whatever writes bytes.
* **Vaadin-style chaining.** Free function to construct, method chain for
  attributes, `.add(child)` / `.children(iter)` for children.
* **Web-component parity.** Each Rust builder is a *typed view* over the exact
  same HTML attributes the Lit components already accept, so you can freely
  switch between "hand-written HTML" and "typed Rust" per page.

## Included

**Layout primitives** (no external CSS needed):

| Rust builder | What it does |
| --- | --- |
| `container()` | Centered wrapper with `max-width` (default 1200px). `.fluid()` to fill. |
| `row()`       | Horizontal flex with responsive wrap. `.gap()` / `.align()` / `.justify()` / `.nowrap()`. |
| `column()`    | Vertical flex. `.flex(n)` grows inside a Row. `.min_width("300px")`. |
| `grid()`      | Auto-fit grid. `.cols_min("220px")` for responsive KPI/card grids. |
| `spacer()`    | Flexible gap that pushes flex siblings apart. |
| `section()`   | Semantic `<section>` with title/subtitle + right-side actions header. |

**Components** (typed builders → `<ui-*>` Lit elements):

| Rust builder     | Web component |
| ---              | --- |
| `page()`         | Full HTML shell that loads the Lit kit |
| `button()`       | `<ui-button>` |
| `input()`        | `<ui-input>` |
| `card()`         | `<ui-card>` |
| `badge()`        | `<ui-badge>` |
| `icon()`         | `<ui-icon>` |
| `avatar()`       | `<ui-avatar>` |
| `stat()`         | `<ui-stat>` |
| `list_item()`    | `<ui-list-item>` |
| `select()`       | `<ui-select>` |
| `checkbox()`     | `<ui-checkbox>` |
| `radio()` + `radio_group()` | `<ui-radio>` + `<ui-radio-group>` |
| `switch()`       | `<ui-switch>` |
| `form()`         | `<ui-form>` |
| `table()`        | `<ui-table>` (hand-rolled `<table>`) |
| `data_table()`   | `<ui-data-table>` (typed columns + rows via inline `<script>`) |
| `pagination()`   | `<ui-pagination>` |
| `breadcrumb()` + `Crumb::link/current` | `<ui-breadcrumb>` |
| `avatar_group()`  | `<ui-avatar-group>` |
| `tooltip(text)`   | `<ui-tooltip>` |
| `tab_bar()` + `Tab::new(...).active()` | `<ui-tab-bar>` |
| `segmented()` + `Segment::new(...)`    | `<ui-segmented>` |
| `empty_state(title)` | `<ui-empty-state>` |
| `skeleton()`      | `<ui-skeleton>` |
| `progress(v)`     | `<ui-progress>` (linear + circular) |
| `drawer()`        | `<ui-drawer>` |
| `dropdown_menu()` + `MenuItem::link/action`/`.divider()` | `<ui-dropdown-menu>` |
| `modal()`         | `<ui-modal>` |
| `stepper()`       | `<ui-stepper>` |
| `timeline()` + `timeline_item()` | `<ui-timeline>` + `<ui-timeline-item>` |
| `kanban()` + `kanban_column()` + `kanban_card()` | full kanban board |
| `file_upload()`   | `<ui-file-upload>` |
| `datepicker()`    | `<ui-datepicker>` |
| `date_range()`    | `<ui-daterange>` |
| `inline_edit(value)` | `<ui-inline-edit>` |
| `toast(title)` + `toast_host()` | `<ui-toast>` + `<ui-toast-host>` |
| `command()` + `command_item(label)` | `<ui-command>` + `<ui-command-item>` |

Every `<ui-*>` element on the JS side now has a typed Rust builder. New
components are 30–60 lines each — follow the pattern in
`src/components/badge.rs`.

## Responsive by default

The DSL emits the same HTML you'd write by hand, so **every responsive rule
lives in the Lit component CSS** and applies automatically:

* `app-shell` collapses its menu bar into a bottom tab bar under 860 px.
* `data-table` scrolls horizontally on narrow screens.
* `drawer`, `select`, `datepicker`, `daterange` become bottom sheets on mobile.
* `grid().cols_min("220px")` fluidly reflows KPI cards down to 1 column.
* `row()` wraps by default (opt-out with `.nowrap()`) so multi-column
  layouts fold into stacks on narrow devices.

For a page-level example, open the pre-generated Students page:

* <http://localhost:3000/lit-components/dsl-students.html>

Resize the browser — the KPI grid, the table, and the two-column body all
reflow with no per-page code.

## Adding a new component

The pattern is 30–60 lines of Rust. Copy `src/components/badge.rs` and:

1. Create a struct with fields for each attribute.
2. Add a free-function constructor (`pub fn foo() -> Foo`).
3. Add method-chaining setters (`.label`, `.variant`, …).
4. Implement `Component`'s `render()` — build a `Vec<Attr>`, call `wrap(...)`.
5. Register it in `src/components/mod.rs` and export from `prelude` in `src/lib.rs`.

## Try the demos

Two pages are pre-rendered so you can preview without Cargo:

```bash
# from the repo root
npx serve           # or:  python -m http.server 3000
```

Then open:

* <http://localhost:3000/lit-components/dsl-demo.html> — tiny form + button demo
* <http://localhost:3000/lit-components/dsl-students.html> — realistic ERP
  "Students" page with KPIs, sortable/filterable/paged table, side form,
  breadcrumbs, and a responsive two-column layout.

Both pages come from real Rust files. Each example **writes the HTML file
directly** (no shell redirection needed) into `../lit-components/`:

```bash
cargo run -p lit-ui --example demo
# wrote 1523 bytes → …/lit-components/dsl-demo.html
# open  → http://localhost:3000/lit-components/dsl-demo.html

cargo run -p lit-ui --example students
# wrote 3812 bytes → …/lit-components/dsl-students.html
# open  → http://localhost:3000/lit-components/dsl-students.html
```

The output path is resolved from `CARGO_MANIFEST_DIR`, so you can invoke
`cargo run` from anywhere in the workspace.

## Children API — `.add()` vs `.children()`

Every container exposes both:

```rust
// One at a time — reads great in a chain
card().title("Actions")
      .add(button().label("Save"))
      .add(button().label("Cancel").variant(Variant::Secondary))

// Many at once — cleaner for lists you build in a loop
card().title("Buttons")
      .children(vec![
          button().label("A"),
          button().label("B"),
          button().label("C"),
      ])
```

Both accept **any `Component + 'static`** — mix builders, `Node::text("…")`
for plain text, and `Node::raw("<b>hi</b>")` for pre-escaped HTML.

## Layout

```
rust-dsl/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs                  # Prelude re-exports
│   ├── core.rs                 # Component trait + escaping + tag helpers
│   ├── layout.rs               # container, row, column, grid, spacer + Gap/Align/Justify
│   └── components/
│       ├── mod.rs
│       ├── page.rs             # HTML shell (fluid; use container() inside)
│       ├── button.rs
│       ├── input.rs
│       ├── card.rs
│       ├── badge.rs
│       ├── icon.rs
│       ├── avatar.rs
│       ├── stat.rs
│       ├── list_item.rs
│       ├── select.rs
│       ├── checkbox.rs
│       ├── radio.rs            # radio + radio_group
│       ├── switch.rs
│       ├── form.rs
│       ├── table.rs            # hand-rolled table wrapped in <ui-table>
│       ├── data_table.rs       # typed columns + rows via inline <script>
│       ├── pagination.rs
│       └── breadcrumb.rs
└── examples/
    ├── demo.rs                 # Tiny form + button demo
    └── students.rs             # Full ERP "Students" page
```

## Integration hint (Axum)

```rust
use axum::response::Html;
use lit_ui::prelude::*;

async fn students() -> Html<String> {
    Html(page()
        .title("Students")
        .add(card().title("Students").padded()
            .add(input().label("Search")))
        .render())
}
```

## Next steps

* Port the remaining ~35 components (badge, list-item, table, form, drawer,
  data-table, kanban, timeline, …). Each is ~60 LOC.
* Add typed event names as `pub const` constants (e.g. `UI_CLICK: &str = "ui-click"`)
  so listeners can reference them by symbol instead of string.
* Optional: publish to crates.io once the surface stabilises.
