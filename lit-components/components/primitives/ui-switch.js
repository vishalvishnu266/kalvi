import { LitBaseElement, html, css, nothing } from '../base.js';

/**
 * <ui-switch checked>Enable notifications</ui-switch>
 *
 * Emits `ui-change` with { checked, name, value }.
 */
class UISwitch extends LitBaseElement {
  static properties = {
    name:     { type: String,  reflect: true },
    value:    { type: String,  reflect: true },
    checked:  { type: Boolean, reflect: true },
    disabled: { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: inline-flex; align-items: center; gap: 10px;
      cursor: pointer; user-select: none;
      font-size: var(--fs-sm); color: var(--color-text);
    }
    :host([disabled]) { cursor: not-allowed; opacity: 0.55; }
    .track {
      width: 36px; height: 22px;
      background: var(--color-border-strong);
      border-radius: 999px;
      position: relative;
      transition: background var(--dur-fast) var(--ease),
                  box-shadow var(--dur-fast) var(--ease);
      flex: 0 0 auto;
    }
    :host(:focus-visible) .track {
      outline: none;
      box-shadow: 0 0 0 3px var(--color-primary-ring);
    }
    :host([checked]) .track { background: var(--color-primary); }
    .thumb {
      position: absolute; top: 2px; left: 2px;
      width: 18px; height: 18px;
      background: #fff;
      border-radius: 50%;
      box-shadow: 0 1px 3px rgba(0,0,0,0.25);
      transition: transform var(--dur-fast) var(--ease);
    }
    :host([checked]) .thumb { transform: translateX(14px); }
  `;

  constructor() {
    super();
    this.name = '';
    this.value = 'on';
    this.checked = false;
    this.disabled = false;
  }

  connectedCallback() {
    super.connectedCallback();
    if (!this.hasAttribute('tabindex')) this.setAttribute('tabindex', '0');
    this.setAttribute('role', 'switch');
    this.addEventListener('click', this.#toggle);
    this.addEventListener('keydown', this.#onKey);
  }

  updated() {
    this.setAttribute('aria-checked', String(this.checked));
    this.setAttribute('aria-disabled', String(this.disabled));
  }

  #toggle = () => {
    if (this.disabled) return;
    this.checked = !this.checked;
    this.emit('ui-change', { checked: this.checked, name: this.name, value: this.value });
  };
  #onKey = (e) => {
    if (e.key === ' ' || e.key === 'Enter') { e.preventDefault(); this.#toggle(); }
  };

  render() {
    return html`
      <span class="track"><span class="thumb"></span></span>
      <span class="label"><slot></slot></span>
    `;
  }
}
customElements.define('ui-switch', UISwitch);
