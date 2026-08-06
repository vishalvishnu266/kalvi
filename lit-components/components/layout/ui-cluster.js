import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-cluster
 *   gap="sm"                 → none | xs | sm (default) | md | lg | xl
 *   align="center"           → start | center (default) | end | stretch | baseline
 *   justify="start"          → start (default) | center | end | between
 *   nowrap>                  → boolean; disable wrapping
 *   <ui-badge>a</ui-badge>
 *   <ui-badge>b</ui-badge>
 * </ui-cluster>
 *
 * Wrapping inline row. Use for anything that's a sequence of same-height
 * things that should flow onto the next line when they run out of room:
 * chip / tag rows, meta-info bars, action-button groups that may wrap on
 * narrow viewports, breadcrumbs, filter pills, etc.
 *
 * Emits nothing — pure layout.
 */
class UICluster extends LitBaseElement {
  static properties = {
    gap:     { type: String,  reflect: true },
    align:   { type: String,  reflect: true },
    justify: { type: String,  reflect: true },
    nowrap:  { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: flex;
      flex-wrap: wrap;
      gap: var(--space-3);
      align-items: center;
      justify-content: flex-start;
      min-width: 0;
    }

    :host([nowrap]) { flex-wrap: nowrap; }

    :host([gap="none"]) { gap: 0; }
    :host([gap="xs"])   { gap: var(--space-2); }
    :host([gap="sm"])   { gap: var(--space-3); }
    :host([gap="md"])   { gap: var(--space-4); }
    :host([gap="lg"])   { gap: var(--space-6); }
    :host([gap="xl"])   { gap: var(--space-8); }

    :host([align="start"])    { align-items: flex-start; }
    :host([align="center"])   { align-items: center; }
    :host([align="end"])      { align-items: flex-end; }
    :host([align="stretch"])  { align-items: stretch; }
    :host([align="baseline"]) { align-items: baseline; }

    :host([justify="start"])   { justify-content: flex-start; }
    :host([justify="center"])  { justify-content: center; }
    :host([justify="end"])     { justify-content: flex-end; }
    :host([justify="between"]) { justify-content: space-between; }
  `;

  render() { return html`<slot></slot>`; }
}

customElements.define('ui-cluster', UICluster);
