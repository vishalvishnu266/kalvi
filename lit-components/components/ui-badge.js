import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-badge tone="success|warning|danger|info|brand|neutral" dot>Paid</ui-badge>
 *
 * DSL surface:
 *   - tone : "neutral" (default) | "success" | "warning" | "danger" | "info" | "brand"
 *   - dot  : boolean, render a leading dot in current color
 */
class UIBadge extends LitBaseElement {
  static properties = {
    tone: { type: String, reflect: true },
    dot:  { type: Boolean, reflect: true },
  };

  static styles = css`
    :host { display: inline-block; vertical-align: middle; line-height: 1; }
    span {
      display: inline-flex; align-items: center; gap: 6px;
      padding: 3px 10px;
      border-radius: var(--radius-pill);
      font-size: var(--fs-xs);
      font-weight: var(--fw-medium);
      line-height: 1.2;
      background: var(--color-surface-alt);
      color: var(--color-text-muted);
      border: 1px solid var(--color-border);
      white-space: nowrap;
    }
    :host([tone="success"]) span { background: color-mix(in srgb, var(--color-success) 15%, transparent); color: var(--color-success); border-color: transparent; }
    :host([tone="warning"]) span { background: color-mix(in srgb, var(--color-warning) 15%, transparent); color: var(--color-warning); border-color: transparent; }
    :host([tone="danger"])  span { background: color-mix(in srgb, var(--color-danger)  15%, transparent); color: var(--color-danger);  border-color: transparent; }
    :host([tone="info"])    span { background: color-mix(in srgb, var(--color-info)    15%, transparent); color: var(--color-info);    border-color: transparent; }
    :host([tone="brand"])   span { background: var(--color-primary-soft); color: var(--color-primary); border-color: transparent; }
    /* Fixed circular dot — flex-basis 0 keeps flex from stretching it into an oval. */
    .dot {
      flex: 0 0 auto;
      display: inline-block;
      width: 7px;
      height: 7px;
      min-width: 7px;
      min-height: 7px;
      border-radius: 50%;
      background: currentColor;
      align-self: center;
    }
  `;

  constructor() {
    super();
    this.tone = 'neutral';
    this.dot = false;
  }

  render() {
    return html`
      <span>
        ${this.dot ? html`<span class="dot"></span>` : nothing}
        <slot></slot>
      </span>`;
  }
}
customElements.define('ui-badge', UIBadge);
