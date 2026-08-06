import { LitBaseElement, html, css, nothing } from '../base.js';

/**
 * <ui-avatar name="Aarav K." src="…" size="sm|md|lg|xl"></ui-avatar>
 */
class UIAvatar extends LitBaseElement {
  static properties = {
    name: { type: String, reflect: true },
    src:  { type: String, reflect: true },
    size: { type: String, reflect: true },
  };

  static styles = css`
    :host { display: inline-block; }
    .a {
      width: var(--sz, 40px); height: var(--sz, 40px);
      border-radius: 50%;
      background: var(--color-primary-soft);
      color: var(--color-primary);
      display: grid; place-items: center;
      font-weight: var(--fw-semibold);
      font-size: calc(var(--sz, 40px) * .38);
      overflow: hidden;
      border: 2px solid var(--color-surface);
    }
    img { width:100%; height:100%; object-fit:cover; }
    :host([size="sm"]) .a { --sz: 28px; }
    :host([size="lg"]) .a { --sz: 56px; }
    :host([size="xl"]) .a { --sz: 80px; }
  `;

  constructor() {
    super();
    this.name = '?';
    this.src  = '';
    this.size = 'md';
  }

  render() {
    const initials = (this.name || '?')
      .split(/\s+/).map(n => n[0]).slice(0, 2).join('').toUpperCase();
    return html`
      <div class="a">
        ${this.src
          ? html`<img src=${this.src} alt=${this.name}>`
          : initials}
      </div>`;
  }
}
customElements.define('ui-avatar', UIAvatar);
