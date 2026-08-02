import { BaseElement, attr } from './base.js';

/**
 * <ui-list-item title="Aarav K." subtitle="Grade 5 · Roll 12" trailing="Present" clickable>
 *   <ui-avatar slot="leading" name="Aarav K."></ui-avatar>
 * </ui-list-item>
 */
class UIListItem extends BaseElement {
  static styles = `
    :host { display: block; }
    .row {
      display: flex; align-items: center; gap: var(--space-3);
      padding: var(--space-3) var(--space-4);
      border-bottom: 1px solid var(--color-border);
      background: transparent;
      transition: background var(--dur-fast) var(--ease);
    }
    :host([clickable]) .row { cursor: pointer; }
    :host([clickable]) .row:hover { background: var(--color-surface-alt); }
    :host(:last-of-type) .row { border-bottom: 0; }
    :host(:last-child)   .row { border-bottom: 0; }
    .texts { min-width: 0; flex: 1; }
    .title { font-size: var(--fs-sm); font-weight: var(--fw-medium); color: var(--color-text); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
    .subtitle { font-size: var(--fs-xs); color: var(--color-text-muted); margin-top: 2px; }
    .trailing { color: var(--color-text-muted); font-size: var(--fs-xs); }
    .chev { color: var(--color-text-subtle); font-size: 1rem; }
  `;
  render() {
    const title = attr(this, 'title');
    const subtitle = attr(this, 'subtitle');
    const trailing = attr(this, 'trailing');
    const chev = this.hasAttribute('clickable');
    return `
      <div class="row">
        <slot name="leading"></slot>
        <div class="texts">
          <div class="title">${title}</div>
          ${subtitle ? `<div class="subtitle">${subtitle}</div>` : ''}
        </div>
        <div class="trailing"><slot name="trailing">${trailing}</slot></div>
        ${chev ? '<span class="chev">›</span>' : ''}
      </div>`;
  }
}
customElements.define('ui-list-item', UIListItem);
