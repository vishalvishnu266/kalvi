import { BaseElement, html, css } from './base.js';

class UIToast extends BaseElement {
  static properties = {
    variant: { type: String, reflect: true },
    duration: { type: Number },
    open: { type: Boolean, reflect: true },
  };

  constructor() {
    super();
    this.duration = 3000;
    this.open = false;
  }

  static styles = css`
    :host {
      display: flex;
      position: fixed;
      bottom: var(--space-6);
      right: var(--space-6);
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      padding: var(--space-3) var(--space-4);
      border-radius: var(--radius-md);
      box-shadow: var(--shadow-lg);
      transform: translateY(100px);
      opacity: 0;
      transition: all 0.3s var(--ease);
      z-index: 2000;
      align-items: center;
      gap: var(--space-3);
    }
    :host([open]) { transform: translateY(0); opacity: 1; }
    :host([variant="success"]) { border-left: 4px solid var(--color-success); }
    :host([variant="danger"]) { border-left: 4px solid var(--color-danger); }
  `;

  show() {
    this.open = true;
    setTimeout(() => this.open = false, this.duration);
  }

  render() {
    return html`<slot></slot>`;
  }
}

customElements.define('ui-toast', UIToast);
