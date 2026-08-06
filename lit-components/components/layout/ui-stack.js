import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-stack
 *   gap="md"                 → none | xs | sm | md (default) | lg | xl
 *   align="stretch"          → start | center | end | stretch (default)
 *   distribute="start"       → start (default) | center | end | between | around | evenly
 *   inline                   → boolean; horizontal instead of vertical
 *   wrap>                    → boolean; allow wrapping (only meaningful with `inline`)
 *   <a></a>
 *   <b></b>
 * </ui-stack>
 *
 * Vertical rhythm primitive. Every direct child gets a consistent gap
 * between siblings. Use for page sections, form fields, sidebar nav items —
 * anywhere you'd otherwise reach for "margin-top on the next element".
 *
 * Toggle `inline` to get a horizontal stack (useful for icon+label rows
 * where you *don't* want the wrapping behaviour of <ui-cluster>).
 *
 * Pure layout — emits no events.
 */
class UIStack extends LitBaseElement {
  static properties = {
    gap:        { type: String,  reflect: true },
    align:      { type: String,  reflect: true },
    distribute: { type: String,  reflect: true },
    inline:     { type: Boolean, reflect: true },
    wrap:       { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: flex;
      flex-direction: column;
      gap: var(--space-4);
      align-items: stretch;
      justify-content: flex-start;
      min-width: 0;
    }

    :host([inline]) { flex-direction: row; align-items: center; }
    :host([wrap])   { flex-wrap: wrap; }

    /* Gap presets mapped to design tokens. */
    :host([gap="none"]) { gap: 0; }
    :host([gap="xs"])   { gap: var(--space-2); }
    :host([gap="sm"])   { gap: var(--space-3); }
    :host([gap="md"])   { gap: var(--space-4); }
    :host([gap="lg"])   { gap: var(--space-6); }
    :host([gap="xl"])   { gap: var(--space-8); }

    /* Cross-axis alignment. */
    :host([align="start"])   { align-items: flex-start; }
    :host([align="center"])  { align-items: center; }
    :host([align="end"])     { align-items: flex-end; }
    :host([align="stretch"]) { align-items: stretch; }

    /* Main-axis distribution. */
    :host([distribute="start"])   { justify-content: flex-start; }
    :host([distribute="center"])  { justify-content: center; }
    :host([distribute="end"])     { justify-content: flex-end; }
    :host([distribute="between"]) { justify-content: space-between; }
    :host([distribute="around"])  { justify-content: space-around; }
    :host([distribute="evenly"])  { justify-content: space-evenly; }
  `;

  render() { return html`<slot></slot>`; }
}

customElements.define('ui-stack', UIStack);
