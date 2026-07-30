import { BaseElement, html, css } from './base.js';

class UIStat extends BaseElement {
  static properties = {
    label: { type: String },
    value: { type: String },
    delta: { type: String },
    deltaType: { type: String, attribute: 'delta-type' },
    icon: { type: String },
  };

  static styles = css`
    :host {
      display: block;
      padding: var(--space-4);
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
    }
    .label { font-size: var(--fs-xs); color: var(--color-text-muted); margin-bottom: var(--space-1); }
    .row { display: flex; align-items: baseline; gap: var(--space-2); }
    .value { font-size: var(--fs-xl); font-weight: var(--fw-bold); color: var(--color-text); }
    .delta { font-size: var(--fs-xs); font-weight: var(--fw-medium); }
    .delta.up { color: var(--color-success); }
    .delta.down { color: var(--color-danger); }
  `;

  render() {
    return html`
      <div class="label">${this.label}</div>
      <div class="row">
        <div class="value">${this.value}</div>
        ${this.delta ? html`
          <div class="delta ${this.deltaType || ''}">${this.delta}</div>
        ` : ''}
      </div>
    `;
  }
}

customElements.define('ui-stat', UIStat);
