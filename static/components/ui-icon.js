import { BaseElement, html, css } from './base.js';

class UIIcon extends BaseElement {
  static properties = {
    name: { type: String, reflect: true },
    size: { type: String, reflect: true },
  };

  static styles = css`
    :host {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      line-height: 1;
      width: var(--icon-size, 1.25em);
      height: var(--icon-size, 1.25em);
    }
    i {
      font-family: "bootstrap-icons";
      font-style: normal;
      font-variant: normal;
      text-transform: none;
      -webkit-font-smoothing: antialiased;
    }
  `;

  render() {
    const size = this.size ? `${this.size}px` : '1.25em';
    return html`
      <i class="bi-${this.name}" style="font-size: ${size};"></i>
    `;
  }
}

customElements.define('ui-icon', UIIcon);
