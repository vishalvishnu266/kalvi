import { BaseElement, attr } from './base.js';

/**
 * <ui-stat label="Present today" value="412" delta="+3.2%" trend="up" icon="🟢"></ui-stat>
 */
class UIStat extends BaseElement {
  static styles = `
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
    .icon ui-icon { display: inline-flex; }
    .value { font-size: var(--fs-2xl); font-weight: var(--fw-semibold); color: var(--color-text); letter-spacing: -0.02em; font-variant-numeric: tabular-nums; }
    .delta { font-size: var(--fs-xs); font-weight: var(--fw-medium); }
    .delta.up   { color: var(--color-success); }
    .delta.down { color: var(--color-danger); }
    .delta.flat { color: var(--color-text-muted); }
  `;
  render() {
    const label = attr(this, 'label');
    const value = attr(this, 'value');
    const delta = attr(this, 'delta');
    const trend = attr(this, 'trend', 'flat');
    const icon = attr(this, 'icon', 'activity');
    const isSvg = /^[a-zA-Z]+[a-zA-Z0-9]*$/.test(icon);
    const iconHtml = isSvg
      ? `<span class="icon"><ui-icon name="${icon}" size="16"></ui-icon></span>`
      : `<span class="icon">${icon}</span>`;
    return `
      <div class="wrap">
        <div class="top">
          ${iconHtml}
          <span>${label}</span>
        </div>
        <div class="value">${value}</div>
        ${delta ? `<div class="delta ${trend}">${delta} vs last week</div>` : ''}
      </div>`;
  }
}
customElements.define('ui-stat', UIStat);
