import { BaseElement, html, css } from './base.js';

class UIDaterange extends BaseElement {
  static properties = {
    label: { type: String },
    startValue: { type: String, attribute: 'start-value' },
    endValue: { type: String, attribute: 'end-value' },
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
    .inputs {
      display: flex;
      align-items: center;
      gap: var(--space-2);
    }
    input {
      flex: 1;
      height: 40px;
      padding: 0 var(--space-3);
      background: var(--color-surface);
      border: 1px solid var(--color-border-strong);
      border-radius: var(--radius-md);
      color: var(--color-text);
      font-family: inherit;
    }
    span { color: var(--color-text-subtle); }
  `;

  render() {
    return html`
      ${this.label ? html`<label>${this.label}</label>` : ''}
      <div class="inputs">
        <input type="date" .value="${this.startValue || ''}" />
        <span>to</span>
        <input type="date" .value="${this.endValue || ''}" />
      </div>
    `;
  }
}

customElements.define('ui-daterange', UIDaterange);
