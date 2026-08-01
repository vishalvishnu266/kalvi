import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-button variant="primary|secondary|ghost|danger" size="sm|md|lg" icon="plus" full>
 *   Save
 * </ui-button>
 *
 * DSL surface (all HTML attributes, DSL-friendly):
 *   - variant : "primary" (default) | "secondary" | "ghost" | "danger"
 *   - size    : "sm" | "md" (default) | "lg"
 *   - icon    : icon name from <ui-icon> OR raw emoji/text
 *   - full    : boolean, makes button width:100%
 *
 * Emits:
 *   - "ui-click" bubbling CustomEvent when clicked.
 */
class UIButton extends LitBaseElement {
  static properties = {
    variant: { type: String, reflect: true },
    size:    { type: String, reflect: true },
    icon:    { type: String, reflect: true },
    full:    { type: Boolean, reflect: true },
    disabled:{ type: Boolean, reflect: true },
    // "button" (default) | "submit" | "reset" — reflected so <ui-form>'s
    // click delegation (which looks for [type="submit"]) can find us.
    type:    { type: String, reflect: true },
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
    button:disabled { opacity: 0.55; cursor: not-allowed; transform: none; }

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
    :host([variant="ghost"]) button:hover {
      background: var(--color-primary-soft);
      color: var(--color-primary);
    }

    :host([variant="danger"]) button {
      background: var(--color-danger);
      color: #fff;
    }

    :host([size="sm"]) button { height: 32px; padding: 0 var(--space-3); font-size: var(--fs-xs); }
    :host([size="lg"]) button { height: 48px; padding: 0 var(--space-5); font-size: var(--fs-md); }

    .icon { font-size: 1.05em; line-height: 1; display: inline-flex; }
  `;

  constructor() {
    super();
    this.variant = 'primary';
    this.size = 'md';
    this.icon = '';
    this.full = false;
    this.disabled = false;
    this.type = 'button';
  }

  #onClick(e) {
    if (this.disabled) { e.preventDefault(); e.stopPropagation(); return; }
    this.emit('ui-click', { originalEvent: e });
  }

  #renderIcon() {
    if (!this.icon) return nothing;
    // Bare word => treat as ui-icon name; otherwise as raw glyph.
    const isSvg = /^[a-zA-Z]+[a-zA-Z0-9]*$/.test(this.icon);
    return isSvg
      ? html`<ui-icon name=${this.icon} size="16"></ui-icon>`
      : html`<span class="icon">${this.icon}</span>`;
  }

  render() {
    return html`
      <button part="btn" type=${this.type || 'button'}
              ?disabled=${this.disabled} @click=${this.#onClick}>
        ${this.#renderIcon()}<slot></slot>
      </button>
    `;
  }
}
customElements.define('ui-button', UIButton);
