import { BaseElement, html, css } from './base.js';

class UICard extends BaseElement {
  static styles = css`
    :host {
      display: block;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      box-shadow: var(--shadow-sm);
      overflow: hidden;
    }
    .header {
      padding: var(--space-4) var(--space-5);
      border-bottom: 1px solid var(--color-border);
      background: var(--color-surface-alt);
    }
    .body {
      padding: var(--space-5);
    }
    .footer {
      padding: var(--space-3) var(--space-5);
      border-top: 1px solid var(--color-border);
      background: var(--color-surface-alt);
    }
  `;

  render() {
    return html`
      <div class="header" part="header"><slot name="header"></slot></div>
      <div class="body" part="body"><slot></slot></div>
      <div class="footer" part="footer"><slot name="footer"></slot></div>
    `;
  }
}

customElements.define('ui-card', UICard);
