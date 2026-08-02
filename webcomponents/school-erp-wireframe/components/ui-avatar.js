import { BaseElement, attr } from './base.js';

/** <ui-avatar name="Aarav K." src="..." size="md"></ui-avatar> */
class UIAvatar extends BaseElement {
  static styles = `
    :host { display: inline-block; }
    .a {
      width: var(--sz, 40px); height: var(--sz, 40px);
      border-radius: 50%;
      background: var(--color-primary-soft);
      color: var(--color-primary);
      display: grid; place-items: center;
      font-weight: var(--fw-semibold); font-size: calc(var(--sz, 40px) * .38);
      overflow: hidden;
      border: 2px solid var(--color-surface);
    }
    img { width:100%; height:100%; object-fit:cover; }
    :host([size="sm"]) .a { --sz: 28px; }
    :host([size="lg"]) .a { --sz: 56px; }
    :host([size="xl"]) .a { --sz: 80px; }
  `;
  render() {
    const name = attr(this, 'name', '?');
    const src = attr(this, 'src');
    const initials = name.split(/\s+/).map(n => n[0]).slice(0,2).join('').toUpperCase();
    return `<div class="a">${src ? `<img src="${src}" alt="${name}">` : initials}</div>`;
  }
}
customElements.define('ui-avatar', UIAvatar);
