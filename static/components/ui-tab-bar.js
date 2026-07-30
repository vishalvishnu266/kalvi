import { BaseElement, html, css } from './base.js';

class UITabBar extends BaseElement {
  static properties = {
    active: { type: String, reflect: true },
    tabs: { type: Array },
  };

  constructor() {
    super();
    this.tabs = [];
  }

  static styles = css`
    :host { display: block; border-bottom: 1px solid var(--color-border); }
    .tabs { display: flex; gap: var(--space-6); }
    .tab {
      padding: var(--space-3) 0;
      font-size: var(--fs-sm);
      font-weight: var(--fw-medium);
      color: var(--color-text-muted);
      cursor: pointer;
      position: relative;
      transition: color var(--dur-fast) var(--ease);
      display: flex;
      align-items: center;
      gap: var(--space-2);
    }
    .tab:hover { color: var(--color-text); }
    .tab.active { color: var(--color-primary); }
    .tab.active::after {
      content: '';
      position: absolute;
      bottom: -1px;
      left: 0;
      right: 0;
      height: 2px;
      background: var(--color-primary);
    }
  `;

  render() {
    return html`
      <div class="tabs">
        ${this.tabs.map(tab => html`
          <div 
            class="tab ${this.active === tab.value ? 'active' : ''}"
            @click="${() => { this.active = tab.value; this.emit('change', tab); }}"
          >
            ${tab.icon ? html`<ui-icon name="${tab.icon}" size="16"></ui-icon>` : ''}
            ${tab.label}
          </div>
        `)}
      </div>
    `;
  }
}

customElements.define('ui-tab-bar', UITabBar);
