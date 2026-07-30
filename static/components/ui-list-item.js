import { BaseElement, html, css } from './base.js';

class UIListItem extends BaseElement {
  static properties = {
    label: { type: String },
    sublabel: { type: String },
    icon: { type: String },
    href: { type: String },
    active: { type: Boolean, reflect: true },
    static: { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: flex;
      align-items: center;
      gap: var(--space-4);
      padding: var(--space-3) var(--space-4);
      border-radius: var(--radius-md);
      transition: background var(--dur-fast) var(--ease);
      cursor: pointer;
      color: var(--color-text);
      text-decoration: none;
    }
    :host(:hover:not([static])) { background: var(--color-surface-alt); }
    :host([active]) { background: var(--color-primary-soft); color: var(--color-primary); }
    :host([static]) { cursor: default; }

    .icon-box {
      width: 32px;
      height: 32px;
      display: flex;
      align-items: center;
      justify-content: center;
      background: var(--color-surface-alt);
      border-radius: var(--radius-sm);
      color: var(--color-text-muted);
    }
    :host([active]) .icon-box { background: var(--color-primary); color: white; }

    .content { flex: 1; min-width: 0; }
    .label { font-size: var(--fs-sm); font-weight: var(--fw-medium); }
    .sublabel { font-size: var(--fs-xs); color: var(--color-text-subtle); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  `;

  render() {
    const content = html`
      ${this.icon ? html`
        <div class="icon-box">
          <ui-icon name="${this.icon}" size="18"></ui-icon>
        </div>
      ` : ''}
      <div class="content">
        <div class="label">${this.label}</div>
        ${this.sublabel ? html`<div class="sublabel">${this.sublabel}</div>` : ''}
      </div>
      <slot name="suffix"></slot>
    `;

    if (this.href && !this.static) {
      return html`<a href="${this.href}" style="display:contents; color:inherit;">${content}</a>`;
    }
    return content;
  }
}

customElements.define('ui-list-item', UIListItem);
