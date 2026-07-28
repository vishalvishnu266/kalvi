import { BaseElement } from './base.js';

/**
 * macOS-style segmented control.
 *
 * <ui-segmented value="week">
 *   <button value="day">Day</button>
 *   <button value="week">Week</button>
 *   <button value="month">Month</button>
 * </ui-segmented>
 *
 * Fires `change` with { value }.
 */
class UISegmented extends BaseElement {
  static styles = `
    :host { display: inline-block; }
    .bar {
      display: inline-flex; padding: 3px;
      background: var(--color-surface-alt);
      border-radius: 10px;
      gap: 2px;
    }
    ::slotted(button) {
      appearance: none; border: 0; background: transparent;
      font: inherit; font-size: var(--fs-xs); font-weight: var(--fw-medium);
      padding: 6px 12px; border-radius: 7px; cursor: pointer;
      color: var(--color-text-muted);
      transition: background var(--dur-fast) var(--ease), color var(--dur-fast) var(--ease);
    }
    ::slotted(button[aria-pressed="true"]) {
      background: var(--color-surface);
      color: var(--color-text);
      box-shadow: var(--shadow-xs), 0 0 0 1px var(--color-border);
    }
  `;
  render() { return `<div class="bar"><slot></slot></div>`; }
  afterRender() {
    this.sync();
    this.addEventListener('click', (e) => {
      const btn = e.target.closest('button');
      if (!btn) return;
      this.setAttribute('value', btn.getAttribute('value'));
      this.sync();
      this.emit('change', { value: btn.getAttribute('value') });
    });
  }
  sync() {
    const val = this.getAttribute('value');
    [...this.querySelectorAll('button')].forEach(b =>
      b.setAttribute('aria-pressed', b.getAttribute('value') === val));
  }
}
customElements.define('ui-segmented', UISegmented);
