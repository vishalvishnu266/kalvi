import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-columns
 *   cols="3"              → integer, number of equal columns (default 2)
 *   ratios="1 2 1"        → space-separated ints; when set, overrides `cols`
 *   gap="md"              → none | xs | sm | md (default) | lg | xl
 *   align="stretch"       → start | center | end | stretch (default stretch)
 *   collapse-at="768">    → px breakpoint below which columns stack.
 *                           Set `collapse-at="0"` to disable collapsing.
 *   <div>...</div>
 *   <div>...</div>
 *   <div>...</div>
 * </ui-columns>
 *
 * Generic column layout primitive. Attribute-only API (DSL-friendly).
 * Mobile-first collapse: below `collapse-at`px viewport width the columns
 * become a single stacked column via a `[stacked]` reflected attribute.
 *
 * Ergonomic presets (no extra tags needed):
 *   <ui-columns cols="2">                 →  two equal columns
 *   <ui-columns cols="4">                 →  four equal columns
 *   <ui-columns ratios="1 2">             →  2 columns, 1:2 ratio
 *   <ui-columns ratios="1 2 1">           →  3 columns, 1:2:1 ratio
 *   <ui-columns ratios="2 3 4 3">         →  4 columns, 2:3:4:3 ratio
 *
 * Emits nothing — pure layout.
 */
class UIColumns extends LitBaseElement {
  static properties = {
    cols:         { type: Number,  reflect: true },
    ratios:       { type: String,  reflect: true },
    gap:          { type: String,  reflect: true },
    align:        { type: String,  reflect: true },
    'collapse-at':{ type: Number,  reflect: true, attribute: 'collapse-at' },
    collapseAt:   { type: Number,  attribute: 'collapse-at', reflect: true },
    stacked:      { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: grid;
      width: 100%;
      /* Defaults; overridden inline via updated() to keep dynamic ratios
         out of the static stylesheet. */
      grid-template-columns: repeat(2, 1fr);
      gap: var(--space-4);
      align-items: stretch;
    }

    /* Gap presets — mapped to design tokens declared in tokens.css. */
    :host([gap="none"]) { gap: 0; }
    :host([gap="xs"])   { gap: var(--space-2); }
    :host([gap="sm"])   { gap: var(--space-3); }
    :host([gap="md"])   { gap: var(--space-4); }
    :host([gap="lg"])   { gap: var(--space-6); }
    :host([gap="xl"])   { gap: var(--space-8); }

    /* Alignment presets. */
    :host([align="start"])   { align-items: start; }
    :host([align="center"])  { align-items: center; }
    :host([align="end"])     { align-items: end; }
    :host([align="stretch"]) { align-items: stretch; }

    /* Mobile-first collapse — single stacked column below the breakpoint.
       Toggled via a reflected [stacked] attribute, driven by matchMedia in
       JS so we can honour a runtime `collapse-at` value (CSS can't read
       attribute values as media-query widths). */
    :host([stacked]) {
      grid-template-columns: 1fr !important;
    }
  `;

  constructor() {
    super();
    this.cols       = 2;
    this.ratios     = '';
    this.gap        = 'md';
    this.align      = 'stretch';
    this.collapseAt = 768;
    this.stacked    = false;
    this._mql       = null;
    this._onMqlChange = (e) => { this.stacked = e.matches; };
  }

  connectedCallback() {
    super.connectedCallback();
    this._bindBreakpoint();
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    this._unbindBreakpoint();
  }

  updated(changed) {
    // Re-bind the media query listener when the breakpoint changes.
    if (changed.has('collapseAt')) {
      this._unbindBreakpoint();
      this._bindBreakpoint();
    }
    // Apply grid-template-columns based on ratios / cols. Done imperatively
    // (rather than in `styles`) so it stays reactive to attribute changes.
    if (changed.has('ratios') || changed.has('cols')) {
      this.style.gridTemplateColumns = this._computeTemplate();
    }
  }

  _computeTemplate() {
    const ratios = String(this.ratios || '').trim();
    if (ratios) {
      const parts = ratios.split(/\s+/).filter(Boolean);
      if (parts.length) return parts.map((p) => `${Number(p) || 1}fr`).join(' ');
    }
    const n = Math.max(1, Number(this.cols) || 1);
    return `repeat(${n}, 1fr)`;
  }

  _bindBreakpoint() {
    const bp = Number(this.collapseAt);
    if (!bp || bp <= 0 || typeof window === 'undefined' || !window.matchMedia) {
      this.stacked = false;
      return;
    }
    // "max-width: (bp - 0.02)px" avoids the classic off-by-one where a
    // viewport exactly at `bp` matches both min- and max-width queries.
    this._mql = window.matchMedia(`(max-width: ${bp - 0.02}px)`);
    this.stacked = this._mql.matches;
    // `addEventListener` is preferred over the deprecated `addListener`,
    // but keep a fallback for old Safari.
    if (this._mql.addEventListener) {
      this._mql.addEventListener('change', this._onMqlChange);
    } else if (this._mql.addListener) {
      this._mql.addListener(this._onMqlChange);
    }
  }

  _unbindBreakpoint() {
    if (!this._mql) return;
    if (this._mql.removeEventListener) {
      this._mql.removeEventListener('change', this._onMqlChange);
    } else if (this._mql.removeListener) {
      this._mql.removeListener(this._onMqlChange);
    }
    this._mql = null;
  }

  render() {
    return html`<slot></slot>`;
  }
}

customElements.define('ui-columns', UIColumns);
