import { BaseElement, html, css } from './base.js';

class UISelect extends BaseElement {
  static properties = {
    label: { type: String },
    name: { type: String },
    value: { type: String },
    options: { type: Array },
    placeholder: { type: String },
  };

  constructor() {
    super();
    this.options = [];
  }

  static styles = css`
    :host { display: block; margin-bottom: var(--space-3); }
    label {
      display: block;
      font-size: var(--fs-xs);
      font-weight: var(--fw-semibold);
      color: var(--color-text-muted);
      margin-bottom: var(--space-1);
    }
    select {
      width: 100%;
      height: 40px;
      padding: 0 var(--space-3);
      background: var(--color-surface);
      border: 1px solid var(--color-border-strong);
      border-radius: var(--radius-md);
      color: var(--color-text);
      font-family: inherit;
      font-size: var(--fs-sm);
      cursor: pointer;
    }
  `;

  render() {
    return html`
      ${this.label ? html`<label>${this.label}</label>` : ''}
      <select 
        name="${this.name || ''}"
        .value="${this.value || ''}"
        @change="${(e) => this.value = e.target.value}"
      >
        ${this.placeholder ? html`<option value="">${this.placeholder}</option>` : ''}
        ${this.options.map(opt => html`
          <option value="${opt.value}" ?selected="${opt.selected || opt.value === this.value}">
            ${opt.label}
          </option>
        `)}
      </select>
    `;
  }
}

customElements.define('ui-select', UISelect);
