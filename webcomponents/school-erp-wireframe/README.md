# Sunrise K-12 · Web Components UI Wireframe

A sample UI kit + wireframe pages designed to be **rendered from Rust/Axum + Hotwire**,
but with all UI complexity moved to native **Web Components**. Server-side Rust
templates only emit intent-revealing tags — real HTML/CSS lives in isolated components.

## Why this pattern

Rendering deeply-nested HTML from Rust is painful (poor IDE support, hard to reuse,
weak type safety). By exposing every UI concept as a **custom element**, Rust code
becomes:

```html
<ui-card title="Attendance">
  <ui-list-item title="Aarav K." subtitle="Roll 12">
    <ui-badge slot="trailing" tone="success">Present</ui-badge>
  </ui-list-item>
</ui-card>
```

Benefits:
- **Intent-first tags** — Rust templates read like a domain model.
- **Independent UI dev** — a UI engineer owns `/components` without touching Rust.
- **IDE support** — plain HTML + JS, works in every editor / LSP.
- **Style isolation** — Shadow DOM prevents CSS leakage.
- **Themable** — one CSS variable file drives light/dark + brand color.
- **Hotwire-friendly** — Turbo swaps still work; components auto-upgrade.

## Run

Any static file server. Examples:

```bash
# python
python -m http.server 8080

# node
npx serve .

# rust (once you wire it up)
# Just serve /assets and /components as static files from Axum.
```

Then open <http://localhost:8080/index.html>.

## File layout

```
school-erp-wireframe/
├── assets/
│   ├── tokens.css      # design tokens (colors, spacing, radii, motion)
│   └── global.css      # resets + tiny utility helpers
├── components/
│   ├── base.js         # BaseElement class (Shadow DOM + shared tokens)
│   ├── index.js        # single import for the whole kit
│   ├── app-shell.js    # topbar + sidebar (desktop) + bottom nav (mobile)
│   ├── ui-button.js
│   ├── ui-card.js
│   ├── ui-stat.js
│   ├── ui-badge.js
│   ├── ui-avatar.js
│   ├── ui-input.js
│   ├── ui-list-item.js
│   ├── ui-table.js
│   ├── ui-tab-bar.js
│   └── ui-modal.js
├── index.html          # Dashboard
├── students.html
├── attendance.html
├── grades.html
├── fees.html
└── settings.html
```

## Theming

All colors, spacing, radii, and motion live in `assets/tokens.css` as CSS variables.
Because components use **shared adoptable stylesheets**, changing a variable
re-skins the entire app instantly:

```js
document.documentElement.style.setProperty('--color-primary', '#059669');
document.documentElement.dataset.theme = 'dark';
```

The top-bar exposes both controls; selections are persisted in `localStorage`
under `erp.theme` and `erp.primary`.

## Native-app feel on mobile

- Bottom tab bar with `env(safe-area-inset-bottom)` for iOS notch/gesture-bar.
- Sticky, blurred top-bar.
- Tables collapse into stacked list cards under 860px.
- Modals slide up from the bottom (like an iOS sheet) below 640px.
- `overscroll-behavior-y: contain` disables Safari elastic bounce.

## Creating a custom component

1. Add `components/ui-thing.js`:

```js
import { BaseElement, attr } from './base.js';

class UIThing extends BaseElement {
  static styles = `
    :host { display: block; }
    .box {
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
      padding: var(--space-4);
      color: var(--color-text);
    }
  `;

  render() {
    const label = attr(this, 'label', 'Untitled');
    return `<div class="box"><strong>${label}</strong><slot></slot></div>`;
  }
}

customElements.define('ui-thing', UIThing);
```

2. Register it in `components/index.js`:

```js
import './ui-thing.js';
```

3. Use it anywhere — from a Rust template, static HTML, or a Hotwire stream:

```html
<ui-thing label="Hello">World</ui-thing>
```

Because it extends `BaseElement`, it automatically:
- Uses Shadow DOM.
- Inherits every design token (works in light & dark, follows primary color).
- Has `.$()`, `.$$()`, `.emit()` helpers.

## Using from Rust / Axum / Hotwire

In your Askama / Maud / minijinja template:

```html
{# base.html #}
<!doctype html>
<html>
  <head>
    <link rel="stylesheet" href="/assets/tokens.css">
    <link rel="stylesheet" href="/assets/global.css">
    <script type="module" src="/components/index.js"></script>
  </head>
  <body>
    <app-shell page-title="{{ title }}" active="{{ active }}">
      {% block content %}{% endblock %}
    </app-shell>
  </body>
</html>
```

```html
{# students.html #}
{% extends "base.html" %}
{% block content %}
  <ui-card title="Students">
    {% for s in students %}
      <ui-list-item title="{{ s.name }}" subtitle="Grade {{ s.grade }}">
        <ui-avatar slot="leading" name="{{ s.name }}"></ui-avatar>
        <ui-badge slot="trailing" tone="{{ s.fee_tone }}">{{ s.fee_status }}</ui-badge>
      </ui-list-item>
    {% endfor %}
  </ui-card>
{% endblock %}
```

Hotwire Turbo drives navigation & partial updates; components are re-upgraded
by the browser automatically whenever new HTML enters the DOM.

## Included pages

| Page          | File              | Notes                                                  |
| ------------- | ----------------- | ------------------------------------------------------ |
| Dashboard     | `index.html`      | KPI stats, admissions list, schedule, announcements    |
| Students      | `students.html`   | Table (desktop) + list (mobile), tabs, add modal       |
| Attendance    | `attendance.html` | Per-class marking with P/A/L quick actions             |
| Grades        | `grades.html`     | Score list with grade badges, export/publish actions   |
| Fees          | `fees.html`       | Invoice list with status, invoice-create modal         |
| Settings      | `settings.html`   | Profile, appearance (theme + color), notifications     |

## Roadmap ideas

- `ui-datepicker`, `ui-select` (searchable), `ui-toast`, `ui-drawer`.
- `ui-form` with declarative validation → Hotwire submit adapter.
- Component `data-controller` attributes to bind directly to Stimulus controllers.
