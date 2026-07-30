import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-stat label="Present today" value="412" delta="+3.2%" trend="up" icon="check"></ui-stat>
 *
 * DSL surface:
 *   - label : string
 *   - value : string
 *   - delta : string (e.g. "+3.2%")
 *   - trend : "up" | "down" | "flat" (default "flat")
 *   - icon  : ui-icon name OR raw emoji/text
 */
class UIStat extends LitBaseElement {
  static properties = {
    label: { type: String, reflect: true },
    value: { type: String, reflect: true },
    delta: { type: String, reflect: true },
    trend: { type: String, reflect: true },
    icon:  { type: String, reflect: true },
  };

  static styles = css`
    :host { display: block; }
    .wrap {
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      padding: var(--space-4) var(--space-5);
      display: flex; flex-direction: column; gap: var(--space-2);
      min-height: 108px;
    }
    .top {
      display: flex; align-items: center; gap: var(--space-2);
      color: var(--color-text-muted); font-size: var(--fs-xs);
      text-transform: uppercase; letter-spacing: .04em;
      min-width: 0;
    }
    .top > span:last-child {
      white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
      min-width: 0; flex: 1;
    }
    .icon {
      flex: 0 0 auto;
      width: 28px; height: 28px;
      display: inline-flex; align-items: center; justify-content: center;
      border-radius: var(--radius-md);
      background: var(--color-primary-soft); color: var(--color-primary);
      font-size: 1rem; line-height: 0;
    }
    .value {
      font-size: var(--fs-2xl); font-weight: var(--fw-semibold);
      color: var(--color-text); letter-spacing: -0.02em;
      font-variant-numeric: tabular-nums;
    }
    .delta { font-size: var(--fs-xs); font-weight: var(--fw-medium); }
    .delta.up   { color: var(--color-success); }
    .delta.down { color: var(--color-danger); }
    .delta.flat { color: var(--color-text-muted); }
  `;

  constructor() {
    super();
    this.label = '';
    this.value = '';
    this.delta = '';
    this.trend = 'flat';
    this.icon = 'activity';
  }

  #renderIcon() {
    const isSvg = /^[a-zA-Z]+[a-zA-Z0-9]*$/.test(this.icon);
    return isSvg
      ? html`<span class="icon"><ui-icon name=${this.icon} size="16"></ui-icon></span>`
      : html`<span class="icon">${this.icon}</span>`;
  }

  render() {
    return html`
      <div class="wrap">
        <div class="top">
          ${this.#renderIcon()}
          <span>${this.label}</span>
        </div>
        <div class="value">${this.value}</div>
        ${this.delta
          ? html`<div class="delta ${this.trend}">${this.delta} vs last week</div>`
          : nothing}
      </div>`;
  }
}
customElements.define('ui-stat', UIStat);
