import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-list-item title="Aarav K." subtitle="Grade 5 · Roll 12" trailing="Present" clickable>
 *   <ui-avatar slot="leading" name="Aarav K."></ui-avatar>
 * </ui-list-item>
 */
class UIListItem extends LitBaseElement {
  static properties = {
    title:    { type: String, reflect: true },
    subtitle: { type: String, reflect: true },
    trailing: { type: String, reflect: true },
    clickable:{ type: Boolean, reflect: true },
  };

  static styles = css`
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
    .title {
      font-size: var(--fs-sm); font-weight: var(--fw-medium);
      color: var(--color-text); white-space: nowrap;
      overflow: hidden; text-overflow: ellipsis;
    }
    .subtitle { font-size: var(--fs-xs); color: var(--color-text-muted); margin-top: 2px; }
    .trailing { color: var(--color-text-muted); font-size: var(--fs-xs); }
    .chev { color: var(--color-text-subtle); font-size: 1rem; }
  `;

  constructor() {
    super();
    this.title = '';
    this.subtitle = '';
    this.trailing = '';
    this.clickable = false;
  }

  #onClick(e) {
    if (this.clickable) this.emit('ui-click', { originalEvent: e });
  }

  render() {
    return html`
      <div class="row" @click=${this.#onClick}>
        <slot name="leading"></slot>
        <div class="texts">
          <div class="title">${this.title}</div>
          ${this.subtitle ? html`<div class="subtitle">${this.subtitle}</div>` : nothing}
        </div>
        <div class="trailing"><slot name="trailing">${this.trailing}</slot></div>
        ${this.clickable ? html`<span class="chev">›</span>` : nothing}
      </div>`;
  }
}
customElements.define('ui-list-item', UIListItem);
