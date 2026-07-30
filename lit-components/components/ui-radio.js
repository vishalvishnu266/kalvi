import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-radio-group name="grade" value="5">
 *   <ui-radio value="4">Grade 4</ui-radio>
 *   <ui-radio value="5">Grade 5</ui-radio>
 *   <ui-radio value="6" disabled>Grade 6</ui-radio>
 * </ui-radio-group>
 *
 * The group holds the current `value`, listens to child clicks, and emits
 * `ui-change` with { value, name }.
 */
class UIRadio extends LitBaseElement {
  static properties = {
    value:    { type: String,  reflect: true },
    checked:  { type: Boolean, reflect: true },
    disabled: { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: inline-flex; align-items: center; gap: 8px;
      cursor: pointer; user-select: none;
      font-size: var(--fs-sm); color: var(--color-text);
    }
    :host([disabled]) { cursor: not-allowed; opacity: 0.55; }
    .ring {
      width: 18px; height: 18px; border-radius: 50%;
      border: 1.5px solid var(--color-border-strong);
      background: var(--color-surface);
      display: grid; place-items: center;
      transition:
        border-color var(--dur-fast) var(--ease),
        box-shadow var(--dur-fast) var(--ease);
      flex: 0 0 auto;
    }
    :host(:hover:not([disabled])) .ring { border-color: var(--color-primary); }
    :host(:focus-visible) .ring {
      outline: none;
      box-shadow: 0 0 0 3px var(--color-primary-ring);
      border-color: var(--color-primary);
    }
    .dot {
      width: 8px; height: 8px; border-radius: 50%;
      background: var(--color-primary);
      transform: scale(0);
      transition: transform var(--dur-fast) var(--ease);
    }
    :host([checked]) .ring { border-color: var(--color-primary); }
    :host([checked]) .dot  { transform: scale(1); }
  `;

  constructor() {
    super();
    this.value = '';
    this.checked = false;
    this.disabled = false;
  }

  connectedCallback() {
    super.connectedCallback();
    if (!this.hasAttribute('tabindex')) this.setAttribute('tabindex', '0');
    this.setAttribute('role', 'radio');
    this.addEventListener('click', this.#activate);
    this.addEventListener('keydown', this.#onKey);
  }

  updated() {
    this.setAttribute('aria-checked', String(this.checked));
    this.setAttribute('aria-disabled', String(this.disabled));
  }

  #activate = () => {
    if (this.disabled || this.checked) return;
    this.emit('ui-radio-activate', { value: this.value });
  };
  #onKey = (e) => {
    if (e.key === ' ' || e.key === 'Enter') { e.preventDefault(); this.#activate(); }
  };

  render() {
    return html`
      <span class="ring"><span class="dot"></span></span>
      <span class="label"><slot></slot></span>
    `;
  }
}
customElements.define('ui-radio', UIRadio);

/* ---------------- Group container ---------------- */
class UIRadioGroup extends LitBaseElement {
  static properties = {
    name:  { type: String, reflect: true },
    value: { type: String, reflect: true },
  };

  static styles = css`
    :host {
      display: flex; flex-direction: column; gap: 8px;
    }
    :host([orientation="horizontal"]) {
      flex-direction: row; align-items: center; flex-wrap: wrap; gap: 16px;
    }
  `;

  constructor() {
    super();
    this.name = '';
    this.value = '';
  }

  connectedCallback() {
    super.connectedCallback();
    this.setAttribute('role', 'radiogroup');
    this.addEventListener('ui-radio-activate', this.#onActivate);
    // Ensure initial sync
    queueMicrotask(() => this.#sync());
  }

  updated(changed) {
    if (changed.has('value')) this.#sync();
  }

  #onActivate = (e) => {
    e.stopPropagation();
    this.value = e.detail.value;
    this.emit('ui-change', { value: this.value, name: this.name });
    this.#sync();
  };

  #sync() {
    [...this.querySelectorAll('ui-radio')].forEach(r => {
      r.checked = r.getAttribute('value') === this.value;
    });
  }

  render() { return html`<slot></slot>`; }
}
customElements.define('ui-radio-group', UIRadioGroup);
