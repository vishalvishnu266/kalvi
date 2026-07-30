import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * macOS-style segmented control.
 *
 * <ui-segmented value="week">
 *   <button value="day">Day</button>
 *   <button value="week">Week</button>
 *   <button value="month">Month</button>
 * </ui-segmented>
 *
 * Emits "ui-change" with { value }.
 */
class UISegmented extends LitBaseElement {
  static properties = {
    value: { type: String, reflect: true },
  };

  static styles = css`
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
      transition: background var(--dur-fast) var(--ease),
                  color var(--dur-fast) var(--ease);
    }
    ::slotted(button[aria-pressed="true"]) {
      background: var(--color-surface);
      color: var(--color-text);
      box-shadow: var(--shadow-xs), 0 0 0 1px var(--color-border);
    }
  `;

  constructor() {
    super();
    this.value = '';
  }

  firstUpdated() {
    this.#sync();
    this.addEventListener('click', this.#onClick);
  }

  updated(changed) {
    if (changed.has('value')) this.#sync();
  }

  #onClick = (e) => {
    const btn = e.target.closest('button');
    if (!btn || !this.contains(btn)) return;
    const v = btn.getAttribute('value');
    if (v == null || v === this.value) return;
    this.value = v;
    this.emit('ui-change', { value: v });
  };

  #sync() {
    [...this.querySelectorAll('button')].forEach(b =>
      b.setAttribute('aria-pressed', b.getAttribute('value') === this.value));
  }

  render() {
    return html`<div class="bar"><slot></slot></div>`;
  }
}
customElements.define('ui-segmented', UISegmented);
