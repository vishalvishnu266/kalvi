import { BaseElement, html, css } from './base.js';

class UIButton extends BaseElement {
  static properties = {
    variant: { type: String, reflect: true },
    size: { type: String, reflect: true },
    icon: { type: String, reflect: true },
    full: { type: Boolean, reflect: true },
  };

  static styles = css`
    :host { display: inline-block; }
    :host([full]) { display: block; }
    :host([full]) button { width: 100%; }

    button {
      display: inline-flex; align-items: center; justify-content: center;
      gap: var(--space-2);
      font-family: inherit;
      font-weight: var(--fw-medium);
      cursor: pointer;
      border: 1px solid transparent;
      border-radius: var(--radius-md);
      padding: 0 var(--space-4);
      height: 40px;
      transition: background var(--dur-fast) var(--ease),
                  transform var(--dur-fast) var(--ease),
                  border-color var(--dur-fast) var(--ease);
      background: var(--color-primary);
      color: var(--color-primary-contrast);
      font-size: var(--fs-sm);
    }
    button:hover  { background: var(--color-primary-hover); }
    button:active { transform: translateY(1px); }

    :host([variant="secondary"]) button {
      background: var(--color-surface);
      color: var(--color-text);
      border-color: var(--color-border-strong);
    }
    :host([variant="secondary"]) button:hover { background: var(--color-surface-alt); }

    :host([variant="ghost"]) button {
      background: transparent;
      color: var(--color-text);
    }
    :host([variant="ghost"]) button:hover { background: var(--color-primary-soft); color: var(--color-primary); }

    :host([variant="danger"]) button {
      background: var(--color-danger);
      color: #fff;
    }

    :host([size="sm"]) button { height: 32px; padding: 0 var(--space-3); font-size: var(--fs-xs); }
    :host([size="lg"]) button { height: 48px; padding: 0 var(--space-5); font-size: var(--fs-md); }

    .icon { font-size: 1.05em; line-height: 1; display: inline-flex; }
  `;

  render() {
    const isSvg = this.icon && /^[a-zA-Z]+[a-zA-Z0-9]*$/.test(this.icon);
    const iconHtml = this.icon
      ? (isSvg ? html`<ui-icon name="${this.icon}" size="16"></ui-icon>` : html`<span class="icon">${this.icon}</span>`)
      : '';

    return html`
      <button part="btn">
        ${iconHtml}
        <slot></slot>
      </button>
    `;
  }
}

customElements.define('ui-button', UIButton);
