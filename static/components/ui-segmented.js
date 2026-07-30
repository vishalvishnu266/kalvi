import { BaseElement, html, css } from './base.js';

class UISegmented extends BaseElement {
  static properties = {
    name: { type: String },
    value: { type: String, reflect: true },
    options: { type: Array },
  };

  constructor() {
    super();
    this.options = [];
  }

  static styles = css`
    :host {
      display: inline-flex;
      background: var(--color-surface-alt);
      padding: 2px;
      border-radius: var(--radius-md);
    }
    .option {
      padding: var(--space-1) var(--space-4);
      font-size: var(--fs-sm);
      font-weight: var(--fw-medium);
      color: var(--color-text-muted);
      cursor: pointer;
      border-radius: calc(var(--radius-md) - 2px);
      transition: all var(--dur-fast) var(--ease);
      display: flex;
      align-items: center;
      gap: var(--space-2);
    }
    .option.active {
      background: var(--color-surface);
      color: var(--color-text);
      box-shadow: var(--shadow-sm);
    }
  `;

  render() {
    return html`
      ${this.options.map(opt => html`
        <div 
          class="option ${this.value === opt.value ? 'active' : ''}"
          @click="${() => { this.value = opt.value; this.emit('change', opt); }}"
        >
          ${opt.icon ? html`<ui-icon name="${opt.icon}" size="14"></ui-icon>` : ''}
          ${opt.label}
        </div>
      `)}
    `;
  }
}

customElements.define('ui-segmented', UISegmented);
