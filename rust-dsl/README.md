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
  it into Axum handlers, Hotwire streams, whatever writes bytes.
* **Vaadin-style chaining.** Free function to construct, method chain for
  attributes, `.add(child)` / `.children(iter)` for children.
* **Web-component parity.** Each Rust builder is a *typed view* over the exact
  same HTML attributes the Lit components already accept, so you can freely
  switch between "hand-written HTML" and "typed Rust" per page.

## Included in this initial cut

Only a representative slice — enough to prove the pattern and build a real
page:

| Rust builder | Web component |
| --- | --- |
| `button()` | `<ui-button>` |
| `input()`  | `<ui-input>` |
| `card()`   | `<ui-card>` |
| `page()`   | Full HTML shell that loads the Lit kit |

Adding more components is 30–60 lines of Rust each. Follow the pattern in
`src/components/button.rs`:

1. Create a struct with fields for each attribute.
2. Add a free-function constructor (`pub fn foo() -> Foo`).
3. Add method-chaining setters (`.label`, `.variant`, …).
4. Implement `Component`'s `render()` — build a `Vec<Attr>`, call `wrap(...)`.

## Try the demo

Because you don't need Cargo just to view the output, we've pre-rendered the
example page:

* Start any static server from the repo root:
  ```bash
  npx serve            # or python -m http.server 3000
  ```
* Open: <http://localhost:3000/lit-components/dsl-demo.html>

The same page can be produced with:

```bash
cargo run -p lit-ui --example demo > lit-components/dsl-demo.html
```

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
│   ├── lib.rs
│   ├── core.rs                 # Component trait + escaping + tag helpers
│   └── components/
│       ├── mod.rs
│       ├── button.rs
│       ├── input.rs
│       ├── card.rs
│       └── page.rs
└── examples/
    └── demo.rs                 # Prints an HTML page to stdout
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
