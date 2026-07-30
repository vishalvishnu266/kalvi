import { BaseElement, html, css } from './base.js';

class UIDatepicker extends BaseElement {
  static properties = {
    label: { type: String },
    value: { type: String },
    name: { type: String },
  };

  static styles = css`
    :host { display: block; margin-bottom: var(--space-3); }
    label {
      display: block;
      font-size: var(--fs-xs);
      font-weight: var(--fw-semibold);
      color: var(--color-text-muted);
      margin-bottom: var(--space-1);
    }
    input {
      width: 100%;
      height: 40px;
      padding: 0 var(--space-3);
      background: var(--color-surface);
      border: 1px solid var(--color-border-strong);
      border-radius: var(--radius-md);
      color: var(--color-text);
      font-family: inherit;
    }
  `;

  render() {
    return html`
      ${this.label ? html`<label>${this.label}</label>` : ''}
      <input type="date" name="${this.name || ''}" .value="${this.value || ''}" />
    `;
  }
}

customElements.define('ui-datepicker', UIDatepicker);
