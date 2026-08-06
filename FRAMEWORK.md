# lit-ui — a typed primitives kit

A small, focused UI library made of two halves:

1. **Web components** — thin custom elements written in Lit that render
   themselves in the browser. All primitive (buttons, inputs, icons…)
   plus layout primitives (columns, stack, grid, sidebar, center,
   cluster).
2. **A Rust DSL** that generates HTML strings targeting those web
   components, using `#[derive(UiComponent)]` so component definitions
   read as *intent* (fields + attributes) with zero setter/render
   boilerplate.

Compose primitives + layout primitives to build any page. There is no
router, no chrome, no fragment protocol, no server integration in this
repo — those live wherever you want to put them.

---

## Repo layout

```
rust-dsl/                             # typed builders for every <ui-*> primitive
├── src/components/                   # 13 primitives — one file each, all derive-driven
│   ├── button.rs / input.rs / select.rs / checkbox.rs / radio.rs
│   ├── switch.rs / datepicker.rs / daterange.rs
│   ├── icon.rs / badge.rs / avatar.rs / tooltip.rs / skeleton.rs
│   └── mod.rs
└── src/core.rs                        # tiny render/attr helpers used by the derive

rust-dsl-macros/                      # #[derive(UiComponent)] + #[derive(AttrEnum)]
                                      # collapses setter/Default/render boilerplate to zero

lit-components/
├── components/
│   ├── base.js                       # LitBaseElement + shared prop helpers
│   ├── core.js                       # imports every primitive + layout (single bundle)
│   ├── index.js                      # single-line entry: `import './core.js'`
│   ├── primitives/                   # 13 web components — one per primitive
│   └── layout/                       # ui-columns / -stack / -cluster / -grid / -sidebar / -center
├── demos/                            # per-primitive playground pages + layout demo
├── assets/                           # tokens.css / global.css / layout.css
└── vendor/                           # lit-all.min.js (vendored, no CDN)

server/                               # tiny Axum static-file server for lit-components/
```

Nothing else. If it isn't a primitive, a layout primitive, the derive
macro that generates them, or the demo static-file server, it isn't in
this repo.

---

## The two rules the library enforces

### 1. All configuration flows through HTML attributes
Every web component takes only string/boolean HTML attributes — never
JS-only object props. This is why the Rust DSL can compose components as
plain HTML strings and the browser picks them up correctly.

### 2. The Rust side generates that HTML from typed structs
```rust
use lit_ui::prelude::*;

let html = button()
    .label("Save")
    .variant(Variant::Primary)
    .icon(Icons::CHECK)
    .render();
// → <ui-button variant="primary" size="md" type="button" icon="check">Save</ui-button>
```
`Button` is a `#[derive(UiComponent)]` struct: fields declare which HTML
attribute they map to, and the derive generates the setter methods,
`Default`, and `impl Component` (i.e. `render()`). The whole struct is
~15 lines — no hand-written boilerplate.

---

## Add a new primitive

1. **Web component** — create `lit-components/components/primitives/ui-thing.js`,
   extend `LitBaseElement`, declare attribute-typed properties, add it
   to `core.js`.
2. **Rust builder** — add `rust-dsl/src/components/thing.rs`:
   ```rust
   use lit_ui_macros::UiComponent;

   #[derive(UiComponent)]
   #[ui(tag = "ui-thing")]
   pub struct Thing {
       #[ui(attr = "label")]      pub label: Option<String>,
       #[ui(flag = "disabled")]   pub disabled: bool,
   }
   ```
3. **Re-export** it in `rust-dsl/src/components/mod.rs` and `prelude`.
4. **Snapshot test** — one `#[test]` per notable state, `assert_eq!` the
   exact HTML output.

That's the whole contract.

---

## The derive macro at a glance

Struct-level:
- `#[ui(tag = "ui-thing")]` — required, sets the HTML tag
- `#[ui(no_ctor)]` — skip the free `pub fn thing()` (use when you have a custom constructor)
- `#[ui(no_component)]` — skip `impl Component` (use when you hand-write render)

Per-field:
- `#[ui(attr = "wire-name")]` — render as key/value HTML attribute
- `#[ui(enum_attr = "wire-name")]` — same, but calls `.as_str()` on the value (pairs with `#[derive(AttrEnum)]`)
- `#[ui(flag = "wire-name")]` — bool field → HTML boolean attribute
- `#[ui(slot)]` — string field → escaped default-slot text
- `#[ui(children)]` — `Vec<Child>` → rendered children + auto `.add()` / `.children()`
- `#[ui(default = "...")]` — expression used in Default and constructor
- `#[ui(skip)]` — internal field, not rendered, no setter
- `#[ui(skip_if = "...")]` — skip rendering when the expression is true
- `#[ui(no_setter)]` — render the field but skip generating a setter (used when a custom setter mutates multiple fields, e.g. `.error()` which sets both `error` and `invalid`)

`Option<T>` fields are automatically omitted when `None`.

---

## Non-goals

- **No client-side routing / chrome / fragment protocol.** The prior
  version of this repo had one; it was intentionally removed to keep the
  library focused on primitives.
- **No dependency on any Rust web framework.** The DSL only produces
  `String`s. Wire it into whatever server framework you like.
- **No CSS-in-JS or Tailwind coupling.** Components read design tokens
  from `:root` (see `lit-components/assets/tokens.css`).
