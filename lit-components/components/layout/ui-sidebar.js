import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-sidebar
 *   side="start"             → start (default) | end — which side the sidebar sits on
 *   width="240px"            → intrinsic width of the sidebar track. Any CSS length.
 *   content-min="50%"        → main content minimum width before wrapping (default 50%)
 *   gap="md"                 → none | xs | sm | md (default) | lg | xl
 *   collapse-at="768">       → viewport px below which we stack to a single column
 *
 *   <nav slot="side">sidebar</nav>
 *   <main>main content</main>
 * </ui-sidebar>
 *
 * The classic sidebar + fluid main pattern:
 * - Above `collapse-at`: sidebar at `width` on the specified side, main
 *   takes the rest.
 * - Below `collapse-at`: sidebar and main stack vertically (sidebar first
 *   for `side="start"`, main first for `side="end"`).
 *
 * Use for the outer app body, split panels, docs (nav + article), etc.
 * Pure layout — emits no events.
 */
class UISidebar extends LitBaseElement {
  static properties = {
    side:          { type: String,  reflect: true },
    width:         { type: String,  reflect: true },
    'content-min': { type: String,  reflect: true, attribute: 'content-min' },
    contentMin:    { type: String,  attribute: 'content-min', reflect: true },
    gap:           { type: String,  reflect: true },
    'collapse-at': { type: Number,  reflect: true, attribute: 'collapse-at' },
    collapseAt:    { type: Number,  attribute: 'collapse-at', reflect: true },
    stacked:       { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: grid;
      gap: var(--space-4);
      min-width: 0;
    }

    :host([gap="none"]) { gap: 0; }
    :host([gap="xs"])   { gap: var(--space-2); }
    :host([gap="sm"])   { gap: var(--space-3); }
    :host([gap="md"])   { gap: var(--space-4); }
    :host([gap="lg"])   { gap: var(--space-6); }
    :host([gap="xl"])   { gap: var(--space-8); }

    /* Stacked (mobile) — single column, natural DOM order but slot="side"
       goes first for side="start", last for side="end". */
    :host([stacked]) {
      grid-template-columns: 1fr !important;
      grid-template-areas: "side" "main" !important;
    }
    :host([stacked][side="end"]) {
      grid-template-areas: "main" "side" !important;
    }
  `;

  constructor() {
    super();
    this.side       = 'start';
    this.width      = '240px';
    this.contentMin = '50%';
    this.gap        = 'md';
    this.collapseAt = 768;
    this.stacked    = false;
    this._mql = null;
    this._onMqlChange = (e) => { this.stacked = e.matches; };
  }

  connectedCallback() {
    super.connectedCallback();
    this._bind();
  }
  disconnectedCallback() {
    super.disconnectedCallback();
    this._unbind();
  }

  updated(changed) {
    if (changed.has('collapseAt')) { this._unbind(); this._bind(); }
    if (changed.has('side') || changed.has('width') || changed.has('contentMin')) {
      this._applyTracks();
    }
  }

  _applyTracks() {
    const w   = String(this.width || '240px');
    const min = String(this.contentMin || '50%');
    // The `minmax(min, 1fr)` trick prevents the main column from shrinking
    // below `min`, which would otherwise squish the sidebar's content.
    if (this.side === 'end') {
      this.style.gridTemplateColumns = `minmax(${min}, 1fr) ${w}`;
      this.style.gridTemplateAreas = `"main side"`;
    } else {
      this.style.gridTemplateColumns = `${w} minmax(${min}, 1fr)`;
      this.style.gridTemplateAreas = `"side main"`;
    }
  }

  _bind() {
    const bp = Number(this.collapseAt);
    if (!bp || bp <= 0 || typeof window === 'undefined' || !window.matchMedia) {
      this.stacked = false; return;
    }
    this._mql = window.matchMedia(`(max-width: ${bp - 0.02}px)`);
    this.stacked = this._mql.matches;
    if (this._mql.addEventListener) this._mql.addEventListener('change', this._onMqlChange);
    else if (this._mql.addListener)  this._mql.addListener(this._onMqlChange);
  }
  _unbind() {
    if (!this._mql) return;
    if (this._mql.removeEventListener) this._mql.removeEventListener('change', this._onMqlChange);
    else if (this._mql.removeListener)  this._mql.removeListener(this._onMqlChange);
    this._mql = null;
  }

  render() {
    // Two named slots; children can use `slot="side"` for the sidebar,
    // anything else goes to the main slot. Assign `grid-area` per slot.
    return html`
      <style>
        ::slotted([slot="side"]) { grid-area: side; min-width: 0; }
        ::slotted(:not([slot])), ::slotted([slot="main"]) { grid-area: main; min-width: 0; }
      </style>
      <slot name="side"></slot>
      <slot></slot>
    `;
  }
}

customElements.define('ui-sidebar', UISidebar);
