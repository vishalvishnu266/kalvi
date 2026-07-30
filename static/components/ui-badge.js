import { BaseElement, html, css } from './base.js';

class UIBadge extends BaseElement {
  static properties = {
    variant: { type: String, reflect: true },
    size: { type: String, reflect: true },
    pill: { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: inline-flex;
      align-items: center;
      padding: 0 var(--space-2);
      height: 20px;
      font-size: 10px;
      font-weight: var(--fw-bold);
      text-transform: uppercase;
      letter-spacing: 0.05em;
      border-radius: var(--radius-sm);
      background: var(--color-surface-alt);
      color: var(--color-text-muted);
    }
    :host([pill]) { border-radius: var(--radius-pill); padding: 0 var(--space-3); }
    :host([variant="primary"]) { background: var(--color-primary); color: var(--color-primary-contrast); }
    :host([variant="success"]) { background: var(--color-success); color: white; }
    :host([variant="warning"]) { background: var(--color-warning); color: black; }
    :host([variant="danger"]) { background: var(--color-danger); color: white; }
    :host([variant="info"]) { background: var(--color-info); color: white; }
  `;

  render() {
    return html`<slot></slot>`;
  }
}

customElements.define('ui-badge', UIBadge);
