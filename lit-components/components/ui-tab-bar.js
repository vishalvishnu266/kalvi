import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-tab-bar>
 *   <a href="#" data-active>Overview</a>
 *   <a href="#">Attendance</a>
 * </ui-tab-bar>
 */
class UITabBar extends LitBaseElement {
  static styles = css`
    :host { display: block; }
    .bar {
      display: flex; gap: var(--space-2);
      border-bottom: 1px solid var(--color-border);
      overflow-x: auto;
      scrollbar-width: none;
    }
    .bar::-webkit-scrollbar { display: none; }
    ::slotted(a) {
      padding: var(--space-3) var(--space-4);
      font-size: var(--fs-sm);
      color: var(--color-text-muted) !important;
      text-decoration: none;
      border-bottom: 2px solid transparent;
      white-space: nowrap;
      font-weight: var(--fw-medium);
    }
    ::slotted(a[data-active]) {
      color: var(--color-primary) !important;
      border-bottom-color: var(--color-primary);
    }
  `;

  render() {
    return html`<div class="bar"><slot></slot></div>`;
  }
}
customElements.define('ui-tab-bar', UITabBar);
