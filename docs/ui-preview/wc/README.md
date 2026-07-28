# UI Preview — Shoelace edition

## Why Shoelace?

After two hand-rolled attempts (plain custom elements, then Lit + light DOM),
we hit fundamental issues:

- `<slot>` only works inside Shadow DOM. Light-DOM projection was fragile
  and the UI kept breaking whenever Lit's render diff moved children.
- Attribute-driven Tailwind classes required custom safelists and a
  rebuild every time we added a variant.

**Shoelace** solves all of this and more, out of the box:

- ~60 production-grade Web Components (`<sl-button>`, `<sl-input>`,
  `<sl-card>`, `<sl-alert>`, `<sl-dialog>`, `<sl-drawer>`,
  `<sl-tab-group>`, `<sl-select>`, `<sl-tooltip>`, `<sl-icon>`, …).
- Uses Shadow DOM correctly — slot projection just works.
- First-class dark mode (`<html class="sl-theme-dark">`).
- No build step. Load from a CDN, use immediately.
- Framework-agnostic. Used by GitHub, Microsoft, Shopify.

## Files

```
docs/ui-preview/wc/
├── index.html          ← launcher / links to every screen
├── dashboard.html      ← full app shell + Shoelace primitives
├── components.html     ← every Shoelace element on one showcase page
└── assets/
    ├── app-shell.css   ← app-specific chrome (sidebar, topbar, tiles) —
    │                     the parts Shoelace doesn't opinionate on.
    │                     Plain CSS, uses Shoelace design tokens.
    └── boot.js         ← theme resolver + [data-theme-toggle] wiring
```

## API cheat sheet

Every element is documented at [shoelace.style](https://shoelace.style). A
few examples we use here:

```html
<sl-button variant="primary" size="large">
  <sl-icon slot="prefix" name="check2"></sl-icon> Save
</sl-button>

<sl-input label="Password" type="password" password-toggle>
  <sl-icon slot="prefix" name="lock"></sl-icon>
</sl-input>

<sl-alert variant="success" open closable>
  <sl-icon slot="icon" name="check2-circle"></sl-icon>
  Tenant created.
</sl-alert>

<sl-badge variant="success" pill>Active</sl-badge>

<sl-dialog id="dlg" label="Confirm">…</sl-dialog>
<script>document.getElementById('dlg').show();</script>
```

## Trade-offs vs the earlier attempts

| | Semantic classes | Custom Lit (light DOM) | **Shoelace** |
|---|---|---|---|
| Readability            | High | Highest | High (with `<sl-*>` prefix) |
| Works out of the box   | ✅   | ⚠️ fragile | ✅ |
| Design system included | ❌ (we defined it) | ❌ | ✅ full token system |
| Component count        | ~40 CSS classes | ~15 custom elements | ~60 web components |
| Bundle cost            | 0 JS | ~5 KB + Lit (~15 KB) | Autoloaded per-tag (~2–5 KB each, cached) |
| SSR-friendly           | ✅ | ⚠️ | ⚠️ (upgrade on client) |
| Slot projection        | N/A | Broken in light DOM | ✅ native (Shadow DOM) |

## How to preview

Open `index.html` directly in your browser (double-click). Everything loads
from the jsdelivr CDN — first load fetches Shoelace + icons (~200 KB),
subsequent loads hit the cache.

## Theming

Shoelace exposes ~50 CSS custom properties. Our `app-shell.css` maps a few
brand tokens to them:

```css
:root {
  --brand:        var(--sl-color-indigo-600);
  --brand-strong: var(--sl-color-indigo-700);
}
```

To rebrand the entire app, change those two lines.

Dark mode is toggled via `<html class="sl-theme-dark">` — `boot.js` handles
this before first paint and wires `[data-theme-toggle]` buttons.

## Going to production

If you decide to ship Shoelace to the real Rust app:

1. `npm i @shoelace-style/shoelace` and copy the CDN URLs to local imports.
2. Only import the components you actually use (tree-shaking):
   ```js
   import '@shoelace-style/shoelace/dist/components/button/button.js';
   import '@shoelace-style/shoelace/dist/components/input/input.js';
   ```
3. Configure the icon library base path so `<sl-icon>` finds icons offline.
4. Add `@shoelace-style/shoelace/dist/themes/light.css` (and `dark.css`) to
   your CSS pipeline.

See <https://shoelace.style/getting-started/installation> for the full
guide.

<!-- ============================================================
     Historical notes — kept for context. The rest of this file
     documents the earlier Lit-based attempt for comparison.
     Skip if you're just using Shoelace.
     ============================================================ -->

# (Historical) Lit edition

A **class-less** version of the UI preview. Every visual element is a custom
HTML tag; attributes drive variants. No `class=""` attributes appear in the
templates.

Built on top of [Lit 3](https://lit.dev/) for reactive properties, declarative
templates, and efficient re-renders on attribute change. Loaded as a native
**ES module from esm.sh** — no build step, no bundler.

## How it works

- `assets/components.js` is an ES module. It imports Lit from
  `https://esm.sh/lit@3.2.0` and registers ~15 custom elements
  (`<app-btn>`, `<app-card>`, `<app-field>`, `<app-alert>`,
  `<app-badge>`, `<app-brand>`, `<app-avatar>`, `<app-nav-link>`,
  `<app-tab>`, `<app-tile>`, `<app-kbd>`, plus layout tags
  `<app-shell>`, `<app-sidebar>`, `<app-topbar>`, `<app-content>`,
  `<app-bottom-nav>`, …).
- Each element extends a tiny `LightElement` base that overrides
  `createRenderRoot()` to return `this`, so rendering lands in **light
  DOM**. The global `app.build.css` (Tailwind) styles it automatically
  via the same semantic classes (`.btn`, `.card`, `.input`,
  `.nav-link`, `.menu-tile`, …) defined in `styles/app.css`. There is
  **zero style duplication**.
- Lucide icons are hydrated in one debounced `requestAnimationFrame`
  after each element's `updated()` lifecycle.
- HTML files load the module with `<script type="module"
  src="assets/components.js"></script>`.

## Why Lit?

The previous hand-rolled version worked, but Lit gives us for ~5 KB:

- **Reactive properties.** Change `element.active = true` from the
  console (or from data-binding later) and it re-renders automatically.
- **Declarative templates.** No more `document.createElement()` /
  `appendChild()` chains — just tagged template literals with
  attributes and slots.
- **Efficient updates.** Lit's `lit-html` diffs the template and only
  touches the DOM nodes that actually changed on an attribute change.
- **Idiomatic slots.** `<slot>` works in light DOM too — children are
  projected the same way they would be in Shadow DOM.
- **Standard tooling later.** If you eventually want a build step,
  TypeScript, decorators, or reactive controllers, Lit is the on-ramp.

## API cheat sheet

```html
<app-btn>Primary</app-btn>
<app-btn variant="secondary" size="lg" icon="check" trailing-icon="arrow-right">
  Save
</app-btn>
<app-btn href="/tenants" block>Go</app-btn>

<app-icon-btn icon="bell" title="Notifications"/>

<app-card pad="body">
  <h2>Header</h2>
  <p>Any content.</p>
</app-card>

<app-field label="Email" icon="user" type="email" name="email"
           value="jane@school.io" required autocomplete="email"/>
<app-field label="Tenant ID" mono placeholder="acme"
           help="Letters, digits, <code>-</code>, <code>_</code>."/>

<app-alert type="error">Something went wrong.</app-alert>
<app-alert type="success">Saved.</app-alert>

<app-badge tone="ok" icon="check-circle-2">Active</app-badge>
<app-kbd>⌘K</app-kbd>

<app-brand size="lg" icon="school"/>
<app-avatar size="md">J</app-avatar>

<app-nav-link href="/students" icon="users" active>Students</app-nav-link>
<app-tab      href="/home"     icon="home"  active>Home</app-tab>

<app-tile href="/students" icon="users"
          gradient="from-blue-500 to-indigo-600"
          label="Students" description="Enrolment · profiles"/>
```

## Files

```
docs/ui-preview/wc/
├── index.html                ← launcher for this variant
├── dashboard.html            ← full showcase screen (class-less)
├── components.html           ← every custom element in isolation
├── assets/
│   ├── app.build.css         ← copy of ../../static/app.build.css
│   ├── boot.js               ← shared theme + icon boot
│   └── components.js         ← the component library
└── README.md
```

## Trade-offs vs the semantic-class version

| | Semantic classes | Web Components |
|---|---|---|
| Readability | High (short class names) | Highest (no attribute noise) |
| Server-render friendly | ✅ works with any templating engine | ⚠️ elements upgrade client-side (progressive enhancement — SSR still ships the raw markup) |
| Debugging | Standard DOM | Standard DOM (light DOM) + one tiny JS layer |
| A11y | Whatever you write | Same, but you must remember to keep semantics inside the custom elements (`<button>`, `<label>`, `<input>` still emitted) |
| Bundle cost | 0 JS | ~5 KB `components.js` + ~15 KB Lit (cached across pages) |
| Reactive updates | N/A | Change an attribute → element re-renders automatically |

## Keeping in sync

If you change `styles/app.css`, rebuild + copy:

```powershell
npm run build:css
Copy-Item static/app.build.css docs/ui-preview/wc/assets/app.build.css -Force
```

If you add a new custom element, edit `assets/components.js` and refresh.

## Known caveats

- **No SSR of upgraded output.** The rendered HTML lives in the DOM after
  JS runs. If a bot or curl fetches the page, it sees the pre-upgraded
  `<app-btn>` tags. For public marketing pages, prefer the semantic-class
  version.
- **Form auto-fill.** Because inputs are created by JS, some password
  managers may miss them on first paint. Given this is a preview, that's
  acceptable — if you ship WC to production, add `autocomplete` and test
  1Password/Bitwarden explicitly.
- **Attribute-driven Tailwind classes** (like `gradient="from-blue-500
  to-indigo-600"` on `<app-tile>`) still need to be in Tailwind's
  safelist — already handled in `tailwind.config.js`.
- **First page-load fetches Lit from esm.sh** (~30 KB gzipped). The
  browser caches it aggressively; subsequent loads and other pages hit
  cache. If you want fully offline previews, switch to a bundled build
  step (see "Going to production" below).

## Going to production

For a shippable version, do these three things:

1. Add `lit` to `package.json` (`npm i lit`).
2. Replace the esm.sh import with a bare specifier
   (`import { LitElement, html, nothing } from 'lit';`) and add an
   esbuild step (`esbuild components.js --bundle --format=esm
   --outfile=components.bundle.js --minify`).
3. Load `components.bundle.js` from `templates/base.html` instead of the
   CDN version.

You'll gain: offline dev, no CDN dependency, tree-shaking, ~20 KB minified.
You'll lose: the "just open the HTML file" DX we have here.
