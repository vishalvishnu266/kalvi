// -----------------------------------------------------------------------------
// LitBaseElement — the shared base class for every Lit-based UI component.
//
// Design goals (kept in mind for a future Rust / Vaadin-like DSL):
//   1. All configuration comes in via **HTML attributes** (strings / booleans),
//      never via JS-only object props. This means the DSL can compose components
//      purely as HTML strings and the browser will pick them up correctly.
//   2. Enums are string attributes (variant="primary"), booleans are boolean
//      attributes (<ui-input required>), children are always slotted.
//   3. Events bubble & compose so a Rust Hotwire adapter can listen at any
//      ancestor. Custom events use kebab-case names ("ui-change", "ui-click").
//   4. Design tokens are inherited from `:root` — components do NOT re-import
//      tokens.css inside their shadow root, which avoids FOUC.
// -----------------------------------------------------------------------------

// Lit is vendored locally at `./vendor/lit-all.min.js` (bundled with esbuild
// from the npm `lit` package). Regenerate it with:
//
//     node scripts/vendor-lit.mjs
//
// No CDN dependency, works fully offline / behind corporate firewalls.
import {
  LitElement,
  html,
  css,
  nothing,
} from './vendor/lit-all.min.js';

export { LitElement, html, css, nothing };

export class LitBaseElement extends LitElement {
  /**
   * Emit a bubbling, composed CustomEvent so listeners on any ancestor
   * (or the root host page) can react — including a future Rust/Hotwire
   * event adapter that listens at document level.
   */
  emit(name, detail = {}) {
    this.dispatchEvent(
      new CustomEvent(name, { detail, bubbles: true, composed: true })
    );
  }

  /**
   * Small convenience: query inside this element's shadow root.
   * Prefer `@query` decorators in real components; this is just a helper.
   */
  $(sel)  { return this.renderRoot.querySelector(sel); }
  $$(sel) { return [...this.renderRoot.querySelectorAll(sel)]; }
}

/**
 * Helper — reflect an *enum* attribute onto :host so component CSS can
 * style with :host([variant="primary"]) selectors, exactly matching the
 * ergonomics of the vanilla components we're porting from. This keeps
 * the CSS syntax portable and DSL-friendly.
 */
export const enumProp = (defaultValue = '') => ({
  type: String,
  reflect: true,
  attribute: true,
});

/** Helper — boolean attribute reflected to :host, e.g. <ui-input required>. */
export const boolProp = () => ({
  type: Boolean,
  reflect: true,
  attribute: true,
});
