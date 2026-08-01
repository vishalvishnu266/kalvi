import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Searchable, native-app-style select using native top-layer dialog.
 * Options are automatically sorted so selected items float to the top in multi-select mode.
 *
 * <ui-select label="Grade" placeholder="Pick a grade" value="5-B" multiple>
 *   <option value="5-A">Grade 5-A</option>
 *   <option value="5-B">Grade 5-B</option>
 *   <option value="6-A" disabled>Grade 6-A (full)</option>
 * </ui-select>
 */
class UISelect extends LitBaseElement {
  static properties = {
    label:       { type: String, reflect: true },
    placeholder: { type: String, reflect: true },
    value:       { type: String, reflect: true },
    open:        { type: Boolean, reflect: true },
    searchable:  { type: Boolean, reflect: true },
    clearable:   { type: Boolean, reflect: true },
    multiple:    { type: Boolean, reflect: true },
    'allow-new': { type: Boolean, reflect: true, attribute: 'allow-new' },
    _filter:     { state: true },
  };

  static styles = css`
    :host { display: inline-block; position: relative; box-sizing: border-box; width: 100%; }
    .label {
      display: block; font-size: var(--fs-xs); color: var(--color-text-muted);
      margin-bottom: 6px; font-weight: var(--fw-medium);
    }
    .trigger {
      display: inline-flex; align-items: center; gap: 8px;
      width: 100%; box-sizing: border-box;
      background: var(--color-surface);
      border: 1px solid var(--color-border-strong);
      border-radius: var(--radius-md);
      height: 40px; padding: 0 12px;
      font: inherit; font-size: var(--fs-sm); color: var(--color-text);
      cursor: pointer; text-align: left;
      transition: border-color var(--dur-fast) var(--ease), box-shadow var(--dur-fast) var(--ease);
    }
    .trigger:hover { border-color: var(--color-text-subtle); }
    .trigger:focus { outline: none; box-shadow: 0 0 0 4px var(--color-primary-ring); border-color: var(--color-primary); }
    .placeholder { color: var(--color-text-subtle); }
    .val {
      flex: 1; min-width: 0; overflow: hidden;
      text-overflow: ellipsis; white-space: nowrap;
    }
    .caret { color: var(--color-text-subtle); flex: 0 0 auto; }
    .clear {
      background: none; border: 0; cursor: pointer; padding: 2px;
      color: var(--color-text-subtle); border-radius: 4px;
      display: grid; place-items: center;
    }
    .clear:hover { color: var(--color-text); background: color-mix(in srgb, var(--color-text) 8%, transparent); }

    .chips { display: flex; flex-wrap: nowrap; gap: 4px; overflow: hidden; }
    .chip {
      display: inline-flex; align-items: center; gap: 3px;
      background: var(--color-primary-soft); color: var(--color-primary);
      padding: 2px 8px; border-radius: var(--radius-pill, 9999px);
      font-size: var(--fs-xs); font-weight: var(--fw-medium);
    }

    /* ----- RESPONSIVE TOP-LAYER DIALOG ----- */
    .pop {
      position: fixed;
      inset: auto 0 0 0;
      width: 100vw;
      max-width: 100vw;
      max-height: 85dvh;
      margin: 0;
      padding: 0;
      border: none;
      border-top-left-radius: var(--radius-lg, 16px);
      border-top-right-radius: var(--radius-lg, 16px);
      box-shadow: var(--shadow-lg, 0 -4px 24px rgba(0,0,0,0.15));
      background: var(--color-surface);
      flex-direction: column;
      overflow: hidden;
      transform: translateY(100%);
      transition: transform var(--dur-med, 250ms) var(--ease, ease-out);
    }
    .pop[open] {
      display: flex;
      transform: translateY(0);
    }
    .pop::backdrop {
      background: var(--color-scrim, rgba(0, 0, 0, 0.4));
      backdrop-filter: blur(2px);
    }

    @media (min-width: 640px) {
      .pop {
        inset: 50% auto auto 50%;
        transform: translate(-50%, -40%);
        width: min(440px, 92vw);
        max-height: 80dvh;
        border-radius: var(--radius-lg, 12px);
        border: 1px solid var(--color-border);
        box-shadow: var(--shadow-xl, 0 12px 32px rgba(0,0,0,0.2));
        transition: transform var(--dur-med, 200ms) var(--ease, ease-out), opacity var(--dur-med, 200ms) ease-out;
        opacity: 0;
      }
      .pop[open] {
        transform: translate(-50%, -50%);
        opacity: 1;
      }
    }

    .drawer-head {
      display: flex; align-items: center; gap: 8px;
      padding: 14px 16px;
      border-bottom: 1px solid var(--color-border);
      background: var(--color-surface);
      flex: 0 0 auto;
    }
    .drawer-head .title {
      flex: 1; font-size: var(--fs-md); font-weight: var(--fw-semibold);
      color: var(--color-text);
    }
    .drawer-close {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-text-muted); width: 28px; height: 28px;
      border-radius: 6px; display: grid; place-items: center;
    }
    .drawer-close:hover {
      background: color-mix(in srgb, var(--color-text) 8%, transparent);
      color: var(--color-text);
    }
    .search {
      display: flex; align-items: center; gap: 8px;
      padding: 10px 12px; border-bottom: 1px solid var(--color-border);
      flex: 0 0 auto;
    }
    .search input {
      flex: 1; border: 0; outline: 0; background: transparent; font: inherit;
      font-size: var(--fs-sm); color: var(--color-text);
    }
    .list { overflow-y: auto; flex: 1; padding: 6px; min-height: 0; }
    .opt {
      display: flex; align-items: center; gap: 8px;
      padding: 10px 12px; border-radius: 6px; cursor: pointer;
      font-size: var(--fs-sm); color: var(--color-text);
    }
    .opt:hover { background: var(--color-primary-soft); }
    .opt[aria-selected="true"] { color: var(--color-primary); font-weight: var(--fw-medium); }
    .opt .check { margin-left: auto; color: var(--color-primary); opacity: 0; }
    .opt[aria-selected="true"] .check { opacity: 1; }
    .opt[aria-disabled="true"] { color: var(--color-text-subtle); cursor: not-allowed; }

    /* Divider separating selected and unselected items */
    .opt-divider {
      height: 1px;
      background: var(--color-border);
      margin: 6px 4px;
    }

    .empty { padding: 24px; text-align: center; color: var(--color-text-subtle); font-size: var(--fs-sm); }

    @media (max-width: 640px) {
      .search input { font-size: var(--fs-md); }
      .opt          { padding: 12px 14px; font-size: var(--fs-md); }
    }
  `;

  constructor() {
    super();
    this.label = '';
    this.placeholder = 'Select…';
    this.value = '';
    this.open = false;
    this.searchable = false;
    this.clearable = false;
    this.multiple = false;
    this['allow-new'] = false;
    this._filter = '';
    this._items = [];
    this._selected = new Set();
  }

  connectedCallback() {
    super.connectedCallback();
    this._items = [...this.querySelectorAll('option')].map(o => ({
      value: o.value ?? o.textContent.trim(),
      label: o.textContent.trim(),
      disabled: o.hasAttribute('disabled'),
    }));
    [...this.children].forEach(el => { if (el.tagName === 'OPTION') el.style.display = 'none'; });
    if (this.value) this.value.split(',').forEach(v => this._selected.add(v.trim()));
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    this.#unlockScroll();
  }

  updated(changedProperties) {
    super.updated(changedProperties);
    if (changedProperties.has('open')) {
      const dialog = this.renderRoot.querySelector('dialog.pop');
      if (dialog) {
        if (this.open && !dialog.open) {
          dialog.showModal();
          this.#lockScroll();
          const input = this.renderRoot.querySelector('.search input');
          if (input) setTimeout(() => input.focus(), 50);
        } else if (!this.open && dialog.open) {
          dialog.close();
          this.#unlockScroll();
        }
      }
    }
  }

  #lockScroll() {
    const el = document.documentElement;
    const body = document.body;
    const n = (parseInt(el.dataset.uiDrawerLocks || '0', 10) || 0) + 1;
    el.dataset.uiDrawerLocks = String(n);
    if (n === 1) {
      el.dataset.uiPrevOverflow = el.style.overflow || '';
      body.dataset.uiPrevOverflow = body.style.overflow || '';
      el.style.overflow = 'hidden';
      body.style.overflow = 'hidden';
      body.style.touchAction = 'none';
    }
  }

  #unlockScroll() {
    const el = document.documentElement;
    const body = document.body;
    const n = Math.max(0, (parseInt(el.dataset.uiDrawerLocks || '0', 10) || 0) - 1);
    el.dataset.uiDrawerLocks = String(n);
    if (n === 0) {
      el.style.overflow = el.dataset.uiPrevOverflow || '';
      body.style.overflow = body.dataset.uiPrevOverflow || '';
      body.style.touchAction = '';
      delete el.dataset.uiPrevOverflow;
      delete body.dataset.uiPrevOverflow;
    }
  }

  #onTriggerClick(e) {
    if (e.target.closest('.clear')) return;
    e.stopPropagation();
    this.open ? this.#close() : this.#openPop();
  }

  #onClear(e) {
    e.stopPropagation();
    this._selected.clear();
    this.#syncValue();
  }

  #onSearch(e) {
    this._filter = e.target.value;
  }

  #onOptionClick(item) {
    if (item.disabled) return;
    if (this.multiple) {
      this._selected.has(item.value) ? this._selected.delete(item.value) : this._selected.add(item.value);
    } else {
      this._selected.clear();
      this._selected.add(item.value);
      this.#close();
    }
    this.#syncValue();
  }

  #openPop() {
    this.open = true;
  }

  #close() {
    this.open = false;
  }

  #onDialogClick(e) {
    const rect = e.currentTarget.getBoundingClientRect();
    const isInDialog = (rect.top <= e.clientY && e.clientY <= rect.top + rect.height &&
        rect.left <= e.clientX && e.clientX <= rect.left + rect.width);
    if (!isInDialog) {
      this.#close();
    }
  }

  #syncValue() {
    const arr = [...this._selected];
    this.value = arr.join(',');
    const labels = arr.map(v => (this._items.find(i => i.value === v)?.label) || v);
    this.emit('ui-change', { value: this.value, values: arr, labels });
    this.requestUpdate();
  }

  #renderValue() {
    if (this._selected.size === 0) {
      return html`<span class="placeholder">${this.placeholder}</span>`;
    }
    if (this.multiple) {
      const labels = [...this._selected].map(v => (this._items.find(i => i.value === v)?.label) || v);
      return html`<span class="chips">${labels.map(l => html`<span class="chip">${l}</span>`)}</span>`;
    }
    const v = [...this._selected][0];
    return html`${(this._items.find(i => i.value === v)?.label) || v}`;
  }

  #onAddNew() {
    const v = (this._filter || '').trim();
    if (!v) return;
    if (!this._items.find(i => i.value === v)) {
      this._items = [...this._items, { value: v, label: v, disabled: false }];
    }
    this.#onOptionClick({ value: v, label: v, disabled: false });
  }

  #renderOption(i) {
    return html`
      <div class="opt"
           aria-selected=${this._selected.has(i.value)}
           aria-disabled=${i.disabled}
           @click=${() => this.#onOptionClick(i)}>
        <span>${i.label}</span>
        <ui-icon class="check" name="check" size="14"></ui-icon>
      </div>
    `;
  }

  render() {
    const q = (this._filter || '').trim().toLowerCase();
    const filteredItems = this._items.filter(i => !q || i.label.toLowerCase().includes(q));

    const allowNew = this['allow-new'];
    const showAddNew =
        allowNew && q.length > 0 &&
        !this._items.some(i => i.label.toLowerCase() === q);
    const showSearch = this.searchable || allowNew;

    // Partition items into selected vs unselected when in multi-select mode
    let selectedItems = [];
    let unselectedItems = filteredItems;

    if (this.multiple) {
      selectedItems = filteredItems.filter(i => this._selected.has(i.value));
      unselectedItems = filteredItems.filter(i => !this._selected.has(i.value));
    }

    return html`
      ${this.label ? html`<span class="label">${this.label}</span>` : nothing}
      <button class="trigger" type="button" @click=${(e) => this.#onTriggerClick(e)}>
        <span class="val">${this.#renderValue()}</span>
        ${this.clearable
        ? html`<button class="clear" title="Clear" @click=${(e) => this.#onClear(e)}>
              <ui-icon name="x" size="14"></ui-icon>
            </button>`
        : nothing}
        <ui-icon class="caret" name="chevronDown" size="14"></ui-icon>
      </button>

      <dialog class="pop" role="listbox" @click=${this.#onDialogClick} @cancel=${(e) => { e.preventDefault(); this.#close(); }}>
        <div class="drawer-head">
          <span class="title">${this.label || 'Select'}</span>
          <button class="drawer-close" title="Close" @click=${() => this.#close()}>
            <ui-icon name="x" size="16"></ui-icon>
          </button>
        </div>
        ${showSearch
        ? html`
            <div class="search">
              <ui-icon name="search" size="14"></ui-icon>
              <input placeholder=${allowNew ? 'Search or type new…' : 'Search…'}
                     .value=${this._filter}
                     @input=${(e) => this.#onSearch(e)}
                     @keydown=${(e) => { if (e.key === 'Enter' && showAddNew) { e.preventDefault(); this.#onAddNew(); } }}>
            </div>`
        : nothing}
        <div class="list">
          ${filteredItems.length === 0 && !showAddNew
        ? html`<div class="empty">No matches</div>`
        : html`
                ${selectedItems.map(i => this.#renderOption(i))}
                ${selectedItems.length > 0 && unselectedItems.length > 0 ? html`<div class="opt-divider"></div>` : nothing}
                ${unselectedItems.map(i => this.#renderOption(i))}
              `}
          ${showAddNew
        ? html`<div class="opt" @click=${() => this.#onAddNew()}>
                     <span>Add “<strong>${this._filter}</strong>”</span>
                     <ui-icon class="check" name="plus" size="14" style="opacity:1"></ui-icon>
                   </div>`
        : nothing}
        </div>
      </dialog>
    `;
  }
}
customElements.define('ui-select', UISelect);