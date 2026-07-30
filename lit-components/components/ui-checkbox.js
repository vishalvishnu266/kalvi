import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-checkbox name="agree" checked>I agree to the terms</ui-checkbox>
 * <ui-checkbox indeterminate>Select all</ui-checkbox>
 *
 * DSL surface:
 *   - name          : form field name
 *   - value         : submitted value when checked (default "on")
 *   - checked       : boolean
 *   - indeterminate : boolean (visual state; e.g. "select-all" partial)
 *   - disabled      : boolean
 *
 * Emits `ui-change` with { checked, name, value }.
 */
class UICheckbox extends LitBaseElement {
  static properties = {
    name:          { type: String,  reflect: true },
    value:         { type: String,  reflect: true },
    checked:       { type: Boolean, reflect: true },
    indeterminate: { type: Boolean, reflect: true },
    disabled:      { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: inline-flex; align-items: center; gap: 8px;
      cursor: pointer; user-select: none;
      font-size: var(--fs-sm); color: var(--color-text);
    }
    :host([disabled]) { cursor: not-allowed; opacity: 0.55; }

    .box {
      width: 18px; height: 18px;
      border: 1.5px solid var(--color-border-strong);
      border-radius: 4px;
      background: var(--color-surface);
      display: grid; place-items: center;
      transition:
        background var(--dur-fast) var(--ease),
        border-color var(--dur-fast) var(--ease),
        box-shadow var(--dur-fast) var(--ease);
      color: var(--color-primary-contrast);
      flex: 0 0 auto;
    }
    :host(:hover:not([disabled])) .box { border-color: var(--color-primary); }
    :host(:focus-visible) .box {
      outline: none;
      box-shadow: 0 0 0 3px var(--color-primary-ring);
      border-color: var(--color-primary);
    }
    :host([checked]) .box,
    :host([indeterminate]) .box {
      background: var(--color-primary);
      border-color: var(--color-primary);
    }
    ui-icon { opacity: 0; transition: opacity var(--dur-fast) var(--ease); }
    :host([checked]) ui-icon.tick,
    :host([indeterminate]) ui-icon.dash { opacity: 1; }
    ui-icon.tick, ui-icon.dash { grid-area: 1 / 1; }
  `;

  constructor() {
    super();
    this.name = '';
    this.value = 'on';
    this.checked = false;
    this.indeterminate = false;
    this.disabled = false;
  }

  connectedCallback() {
    super.connectedCallback();
    if (!this.hasAttribute('tabindex')) this.setAttribute('tabindex', '0');
    this.setAttribute('role', 'checkbox');
    this.addEventListener('click', this.#toggle);
    this.addEventListener('keydown', this.#onKey);
  }

  updated() {
    this.setAttribute('aria-checked',
      this.indeterminate ? 'mixed' : String(this.checked));
    this.setAttribute('aria-disabled', String(this.disabled));
  }

  #toggle = (e) => {
    if (this.disabled) return;
    // Ignore clicks that originated from a text selection etc.
    this.checked = !this.checked;
    this.indeterminate = false;
    this.emit('ui-change', { checked: this.checked, name: this.name, value: this.value });
  };
  #onKey = (e) => {
    if (e.key === ' ' || e.key === 'Enter') { e.preventDefault(); this.#toggle(e); }
  };

  render() {
    return html`
      <span class="box">
        <ui-icon class="tick" name="check" size="12"></ui-icon>
        <ui-icon class="dash" name="minus" size="12"></ui-icon>
      </span>
      <span class="label"><slot></slot></span>
    `;
  }
}
customElements.define('ui-checkbox', UICheckbox);
