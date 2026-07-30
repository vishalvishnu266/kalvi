import { BaseElement, html, css } from './base.js';

class UIAvatar extends BaseElement {
  static properties = {
    src: { type: String },
    initials: { type: String },
    size: { type: String, reflect: true },
    shape: { type: String, reflect: true },
  };

  static styles = css`
    :host {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      width: 32px;
      height: 32px;
      background: var(--color-surface-alt);
      border-radius: 50%;
      overflow: hidden;
      font-size: var(--fs-xs);
      font-weight: var(--fw-semibold);
      color: var(--color-text-muted);
    }
    :host([size="lg"]) { width: 48px; height: 48px; font-size: var(--fs-md); }
    :host([size="xl"]) { width: 64px; height: 64px; font-size: var(--fs-xl); }
    :host([shape="square"]) { border-radius: var(--radius-md); }
    img { width: 100%; height: 100%; object-fit: cover; }
  `;

  render() {
    if (this.src) {
      return html`<img src="${this.src}" alt="avatar" />`;
    }
    return html`<span>${this.initials || ''}</span>`;
  }
}

customElements.define('ui-avatar', UIAvatar);
