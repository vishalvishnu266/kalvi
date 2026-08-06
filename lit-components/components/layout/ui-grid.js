import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-grid
 *   min-col="240"            → auto-fit mode: fills row with as many
 *                              (min: min-col px) tracks as fit. Wraps
 *                              naturally. Default 240.
 *   cols="12"                → fixed-cols mode: exactly N equal columns.
 *                              If both `cols` and `min-col` are set,
 *                              `cols` wins (explicit beats implicit).
 *   gap="md"                 → none | xs | sm | md (default) | lg | xl
 *   dense>                   → boolean; grid-auto-flow: dense
 *   <div>tile</div>
 *   <div>tile</div>
 * </ui-grid>
 *
 * Responsive auto-fit grid. The default (`min-col=240`) is the workhorse:
 * it handles card grids, tile menus, kanban lanes, etc. — one attribute,
 * fully responsive, no media queries.
 *
 * Switch to `cols="12"` (or any integer) when you need a strict fixed
 * grid — e.g., a Bootstrap-style 12-track layout where children set their
 * own `grid-column: span N`.
 *
 * Pure layout — emits no events.
 */
class UIGrid extends LitBaseElement {
  static properties = {
    'min-col': { type: Number,  reflect: true, attribute: 'min-col' },
    minCol:    { type: Number,  attribute: 'min-col', reflect: true },
    cols:      { type: Number,  reflect: true },
    gap:       { type: String,  reflect: true },
    dense:     { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: grid;
      gap: var(--space-4);
      min-width: 0;
    }

    :host([dense]) { grid-auto-flow: dense; }

    :host([gap="none"]) { gap: 0; }
    :host([gap="xs"])   { gap: var(--space-2); }
    :host([gap="sm"])   { gap: var(--space-3); }
    :host([gap="md"])   { gap: var(--space-4); }
    :host([gap="lg"])   { gap: var(--space-6); }
    :host([gap="xl"])   { gap: var(--space-8); }
  `;

  constructor() {
    super();
    this.minCol = 240;
    this.cols   = 0;   // 0 = unset; auto-fit wins
    this.gap    = 'md';
    this.dense  = false;
  }

  updated(changed) {
    if (changed.has('cols') || changed.has('minCol')) {
      // Fixed cols wins if explicitly set to > 0.
      if (Number(this.cols) > 0) {
        this.style.gridTemplateColumns = `repeat(${Math.floor(Number(this.cols))}, 1fr)`;
      } else {
        const min = Math.max(1, Number(this.minCol) || 240);
        // `auto-fit` collapses empty tracks so the last row doesn't have
        // ghost columns; `minmax(min, 1fr)` lets tracks grow to fill.
        this.style.gridTemplateColumns =
          `repeat(auto-fit, minmax(${min}px, 1fr))`;
      }
    }
  }

  render() { return html`<slot></slot>`; }
}

customElements.define('ui-grid', UIGrid);
