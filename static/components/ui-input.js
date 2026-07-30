import { BaseElement, html, css } from './base.js';

class UIInput extends BaseElement {
  static properties = {
    label: { type: String },
    value: { type: String },
    placeholder: { type: String },
    type: { type: String },
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
      font-size: var(--fs-sm);
      transition: border-color var(--dur-fast) var(--ease),
                  box-shadow var(--dur-fast) var(--ease);
    }
    input:focus {
      outline: none;
      border-color: var(--color-primary);
      box-shadow: 0 0 0 3px var(--color-primary-ring);
    }
  `;

  render() {
    return html`
      ${this.label ? html`<label>${this.label}</label>` : ''}
      <input 
        type="${this.type || 'text'}" 
        name="${this.name || ''}"
        .value="${this.value || ''}"
        placeholder="${this.placeholder || ''}"
        @input="${(e) => this.value = e.target.value}"
      />
    `;
  }
}

customElements.define('ui-input', UIInput);
