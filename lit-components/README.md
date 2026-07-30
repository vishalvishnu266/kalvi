# Lit Components — DSL-friendly UI kit

This folder is the **LitElement port** of `webcomponents/school-erp-wireframe/components/`.
It preserves the existing tag names (`ui-button`, `ui-card`, `ui-input`, …) and design
tokens, but drops the hand-rolled `BaseElement` in favor of [Lit](https://lit.dev),
which gives us reactive properties, efficient template diffing, and better DX.

The syntax was deliberately kept **HTML-attribute-only** so a future
Vaadin/Yew-style **Rust DSL** can compose components purely as HTML strings.

## Run

Any static file server works — **no build step, no CDN, no import map**.
Lit is vendored locally at `components/vendor/lit-all.min.js` (~15 KB).

```bash
# from repo root
python -m http.server 8080
# or:  npx serve

# then open one of:
#   http://localhost:8080/lit-components/demos/index.html   ← component browser
#   http://localhost:8080/lit-components/index.html         ← combined playground
```

### Regenerating the vendored Lit bundle

Only needed when bumping the Lit version.

```bash
cd lit-components
npm install                 # first time only
npm run vendor-lit          # regenerates components/vendor/lit-all.min.js
```

The bundler script lives at `scripts/vendor-lit.mjs`. Add more Lit exports
there if you start using things like `repeat`, `styleMap`, `until`, etc.

## File layout

```
lit-components/
├── assets/
│   ├── tokens.css        # design tokens (colors, spacing, radii, motion)
│   └── global.css        # anti-FOUC + host-page utilities
├── components/
│   ├── base.js           # LitBaseElement + Lit re-exports (html, css, nothing)
│   ├── index.js          # single entry-point that registers everything
│   ├── ui-icon.js
│   ├── ui-button.js
│   ├── ui-badge.js
│   ├── ui-input.js
│   ├── ui-card.js
│   ├── ui-stat.js
│   ├── ui-avatar.js
│   ├── ui-list-item.js
│   ├── ui-table.js
│   ├── ui-tab-bar.js
│   ├── ui-segmented.js
│   ├── ui-modal.js
│   ├── ui-select.js
│   ├── ui-datepicker.js
│   ├── ui-daterange.js
│   ├── ui-toast.js       # <ui-toast>, <ui-toast-host>, window.toast()
│   └── app-shell.js      # macOS-style shell (menu bar + dock + mobile tabs)
├── demos/
│   ├── index.html        # hub linking to every component demo
│   ├── _shell.js         # shared nav header
│   ├── _page.css         # shared demo page styling
│   ├── icons.html
│   ├── buttons.html
│   ├── badges.html
│   ├── inputs.html
│   ├── cards.html
│   ├── stats.html
│   ├── avatars.html
│   ├── list-items.html
│   ├── tables.html
│   ├── tab-bars.html
│   ├── segmented.html
│   ├── modals.html
│   ├── selects.html
│   ├── datepicker.html
│   ├── daterange.html
│   ├── toasts.html
│   └── app-shell.html
├── index.html            # combined playground / smoke-test page
└── README.md
```

Open <http://localhost:8080/lit-components/demos/index.html> to browse every component,
or <http://localhost:8080/lit-components/index.html> for a single combined page.

## DSL-friendly conventions (why this design)

The goal is that a future Rust `view!` macro can generate code like:

```rust
view! {
    ui_card(title = "Attendance", padded) {
        ui_button(variant = "primary", icon = "plus") { "Add student" }
        ui_badge(tone = "success", dot) { "Present" }
    }
}
```

…and have that macro simply emit HTML. To make that trivial, every component here
follows these rules:

| Rule | Why |
| --- | --- |
| **All config = HTML attributes** (no JS-only object properties). | Rust can emit them as strings. |
| **Enums are string attributes** — e.g. `variant="primary"`. | One-to-one mapping to a Rust `enum` → `&str`. |
| **Booleans are HTML boolean attributes** — e.g. `<ui-input required>`. | Rust bool → attribute-present. |
| **Children are always slotted** (default slot + named slots). | Rust child nodes go straight into the template. |
| **Custom events are `bubbles: true, composed: true`** with kebab-case names (`ui-click`, `ui-input`, `ui-change`). | A Rust/Hotwire adapter can listen once at document level. |
| **All reflected props use `reflect: true`** so `:host([variant="…"])` CSS keeps working. | The DSL doesn't need to know about internal state; CSS just sees the attribute. |
| **Design tokens live in `:root`, inherited into every shadow root.** | Rust never has to inject styles per-component. |

## Writing a new component

```js
// components/ui-thing.js
import { LitBaseElement, html, css, nothing } from './base.js';

class UIThing extends LitBaseElement {
  static properties = {
    label:    { type: String, reflect: true },   // <ui-thing label="…">
    variant:  { type: String, reflect: true },   // enum via string attribute
    padded:   { type: Boolean, reflect: true },  // boolean attribute
  };

  static styles = css`
    :host { display: block; }
    :host([variant="primary"]) .box { border-color: var(--color-primary); }
    :host([padded]) .box { padding: var(--space-5); }
    .box {
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
      color: var(--color-text);
    }
  `;

  constructor() {
    super();
    this.label = '';
    this.variant = '';
    this.padded = false;
  }

  #onClick(e) {
    this.emit('ui-click', { originalEvent: e });   // bubbles + composed
  }

  render() {
    return html`
      <div class="box" @click=${this.#onClick}>
        ${this.label ? html`<strong>${this.label}</strong>` : nothing}
        <slot></slot>
      </div>
    `;
  }
}
customElements.define('ui-thing', UIThing);
```

Then add `import './ui-thing.js';` to `components/index.js`.
Use it from HTML — or, in the future, from Rust:

```html
<ui-thing label="Hello" variant="primary" padded>World</ui-thing>
```

## Theming

Because every component uses `var(--color-primary)` (and friends), you can re-skin
the whole app by changing tokens on `:root`:

```js
document.documentElement.style.setProperty('--color-primary', '#059669');
document.documentElement.dataset.theme = 'dark';
```

The playground (`index.html`) has a color picker and a dark-mode toggle to demo this.

## Component parity

Every component from `webcomponents/school-erp-wireframe/components/` has been
ported to Lit:

| Component        | Status |
| ---------------- | :----: |
| `ui-icon`        | ✅     |
| `ui-button`      | ✅     |
| `ui-badge`       | ✅     |
| `ui-input`       | ✅     |
| `ui-card`        | ✅     |
| `ui-stat`        | ✅     |
| `ui-avatar`      | ✅     |
| `ui-list-item`   | ✅     |
| `ui-table`       | ✅     |
| `ui-tab-bar`     | ✅     |
| `ui-segmented`   | ✅     |
| `ui-modal`       | ✅     |
| `ui-select`      | ✅     |
| `ui-datepicker`  | ✅     |
| `ui-daterange`   | ✅     |
| `ui-toast`       | ✅ (+ `ui-toast-host`, `window.toast()`) |
| `app-shell`      | ✅     |

## Roadmap

- A tiny Rust DSL (Askama filter or macro) that emits these tags with typed
  attributes so the compiler catches typos.
- Vendor Lit locally (drop the CDN) for offline / firewalled environments.
- Storybook-style per-state controls for each demo page.
