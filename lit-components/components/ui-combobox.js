import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Free-typing autocomplete combobox — like ui-select, but the user can also
 * enter arbitrary text (useful for tag inputs, "add if not exists" flows).
 *
 * <ui-combobox label="Subject" name="subject" placeholder="Type or choose…"
 *              value="math">
 *   <option value="math">Mathematics</option>
 *   <option value="sci">Science</option>
 *   <option value="eng">English</option>
 * </ui-combobox>
 *
 * Attributes:
 *   - allow-new : boolean, permits values not in the options list
 *   - required, invalid : same semantics as ui-input
 *
 * Emits `ui-change` with { value, label, isNew }.
 */
class UICombobox extends LitBaseElement {
  static properties = {
    label:       { type: String,  reflect: true },
    name:        { type: String,  reflect: true },
    placeholder: { type: String,  reflect: true },
    value:       { type: String,  reflect: true },
    required:    { type: Boolean, reflect: true },
    invalid:     { type: Boolean, reflect: true },
    'allow-new': { type: Boolean, reflect: true, attribute: 'allow-new' },
    open:        { type: Boolean, reflect: true },
    _query:      { state: true },
  };

  static styles = css`
    :host { display: block; position: relative; box-sizing: border-box; }
    .label {
      display: block; font-size: var(--fs-xs); color: var(--color-text-muted);
      margin-bottom: 6px; font-weight: var(--fw-medium);
    }
    .wrap {
      display: flex; align-items: center;
      background: var(--color-surface);
      border: 1px solid var(--color-border-strong);
      border-radius: var(--radius-md); height: 40px; padding: 0 8px 0 12px;
      transition: border-color var(--dur-fast) var(--ease), box-shadow var(--dur-fast) var(--ease);
    }
    :host([invalid]) .wrap { border-color: var(--color-danger); }
    .wrap:focus-within {
      border-color: var(--color-primary);
      box-shadow: 0 0 0 3px var(--color-primary-soft);
    }
    input {
      flex: 1; border: 0; outline: 0; background: transparent;
      font: inherit; font-size: var(--fs-sm); color: var(--color-text);
      min-width: 0;
    }
    button.chev {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-text-subtle); width: 26px; height: 26px;
      border-radius: 6px; display: grid; place-items: center;
    }
    button.chev:hover { background: color-mix(in srgb, var(--color-text) 6%, transparent); color: var(--color-text); }

    .pop {
      position: absolute; top: calc(100% + 4px); left: 0; right: 0;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
      box-shadow: var(--shadow-lg);
      max-height: 260px; overflow: auto;
      z-index: 1000; display: none;
    }
    :host([open]) .pop { display: block; }
    .opt {
      padding: 8px 12px; cursor: pointer; font-size: var(--fs-sm);
      display: flex; align-items: center; gap: 6px;
      color: var(--color-text);
    }
    .opt:hover, .opt[data-active] { background: var(--color-primary-soft); color: var(--color-primary); }
    .opt .tag {
      margin-left: auto; font-size: 10px;
      color: var(--color-primary); background: var(--color-primary-soft);
      padding: 2px 6px; border-radius: 999px; font-weight: var(--fw-medium);
    }
    .empty { padding: 12px; text-align: center; font-size: var(--fs-sm); color: var(--color-text-subtle); }
  `;

  constructor() {
    super();
    this.label = '';
    this.name = '';
    this.placeholder = '';
    this.value = '';
    this.required = false;
    this.invalid = false;
    this['allow-new'] = false;
    this.open = false;
    this._query = '';
    this._items = [];
    this._boundOutside = (e) => { if (!this.contains(e.target)) this.#close(); };
  }

  connectedCallback() {
    super.connectedCallback();
    this._items = [...this.querySelectorAll('option')].map(o => ({
      value: o.value ?? o.textContent.trim(),
      label: o.textContent.trim(),
    }));
    [...this.children].forEach(c => { if (c.tagName === 'OPTION') c.style.display = 'none'; });
    if (this.value) this._query = this.#labelFor(this.value);
  }
  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener('mousedown', this._boundOutside);
  }

  #labelFor(v) { return (this._items.find(i => i.value === v)?.label) || v; }

  #openPop() {
    this.open = true;
    document.addEventListener('mousedown', this._boundOutside);
  }
  #close() {
    this.open = false;
    document.removeEventListener('mousedown', this._boundOutside);
  }

  #onInput = (e) => {
    this._query = e.target.value;
    this.#openPop();
    this.emit('ui-input', { value: this._query });
  };
  #onFocus = () => this.#openPop();

  #commit(item, isNew = false) {
    this.value = item.value;
    this._query = item.label;
    this.emit('ui-change', { value: item.value, label: item.label, isNew });
    this.#close();
  }
  #onKey = (e) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      const matches = this.#filtered();
      if (matches.length) this.#commit(matches[0]);
      else if (this['allow-new'] && this._query.trim()) {
        this.#commit({ value: this._query.trim(), label: this._query.trim() }, true);
      }
    } else if (e.key === 'Escape') {
      this.#close();
    }
  };

  #filtered() {
    const q = this._query.trim().toLowerCase();
    if (!q) return this._items;
    return this._items.filter(i => i.label.toLowerCase().includes(q));
  }

  render() {
    const list = this.#filtered();
    const exact = list.some(i => i.label.toLowerCase() === this._query.trim().toLowerCase());
    return html`
      ${this.label ? html`<span class="label">${this.label}${this.required ? ' *' : ''}</span>` : nothing}
      <div class="wrap">
        <input
          .value=${this._query}
          placeholder=${this.placeholder}
          @input=${this.#onInput}
          @focus=${this.#onFocus}
          @keydown=${this.#onKey}
        >
        <button class="chev" tabindex="-1" @click=${() => this.open ? this.#close() : this.#openPop()}>
          <ui-icon name="chevronDown" size="14"></ui-icon>
        </button>
      </div>
      <div class="pop">
        ${list.length
          ? list.map(i => html`
              <div class="opt" @mousedown=${(e) => { e.preventDefault(); this.#commit(i); }}>
                <span>${i.label}</span>
                ${i.value === this.value ? html`<ui-icon name="check" size="14"></ui-icon>` : nothing}
              </div>`)
          : html`<div class="empty">No matches</div>`}
        ${this['allow-new'] && this._query.trim() && !exact
          ? html`
            <div class="opt" @mousedown=${(e) => {
              e.preventDefault();
              this.#commit({ value: this._query.trim(), label: this._query.trim() }, true);
            }}>
              Add "<strong>${this._query}</strong>"
              <span class="tag">new</span>
            </div>`
          : nothing}
      </div>
    `;
  }
}
customElements.define('ui-combobox', UICombobox);
