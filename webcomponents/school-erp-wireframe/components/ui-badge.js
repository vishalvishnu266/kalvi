import { BaseElement, attr } from './base.js';

/** <ui-badge tone="success|warning|danger|info|neutral">Paid</ui-badge> */
class UIBadge extends BaseElement {
  static styles = `
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
    .dot { width:6px; height:6px; border-radius:50%; background: currentColor; }
  `;
  render() {
    const dot = this.hasAttribute('dot');
    return `<span>${dot ? '<span class="dot"></span>' : ''}<slot></slot></span>`;
  }
}
customElements.define('ui-badge', UIBadge);
