import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Searchable, native-app-style select.
 *
 * <ui-select label="Grade" placeholder="Pick a grade" value="5-B">
 *   <option value="5-A">Grade 5-A</option>
 *   <option value="5-B">Grade 5-B</option>
 *   <option value="6-A" disabled>Grade 6-A (full)</option>
 * </ui-select>
 *
 * Attributes:
 *   label       – field label
 *   placeholder – shown when no value
 *   value       – current value (comma-separated for multiple)
 *   searchable  – show a search box (opt-in)
 *   clearable   – add an ✕ to clear
 *   multiple    – allow multiple selections
 *
 * Emits `ui-change` with { value, values, labels }.
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
    // When true, an "Add \"…\"" row appears in the search results if the
    // user's query doesn't exactly match any existing option. Selecting it
    // commits the query as a brand-new value. Implies `searchable`.
    'allow-new': { type: Boolean, reflect: true, attribute: 'allow-new' },
    _filter:     { state: true },
  };

  static styles = css`
    :host { display: inline-block; position: relative; box-sizing: border-box; width: 100%; z-index: auto; }
    :host([open]) { z-index: 3000; }
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
      padding: 2px 8px; border-radius: var(--radius-pill);
      font-size: var(--fs-xs); font-weight: var(--fw-medium);
    }

    /* ----- LEFT-SIDE DRAWER -----
       The popover behaves like a slide-in drawer anchored to the viewport's
       left edge — not floating near the trigger. Same on desktop and mobile. */
    .pop {
      position: fixed;
      top: 0;
      left: 0;
      bottom: 0;
      width: min(380px, 92vw);
      background: var(--color-surface);
      border-right: 1px solid var(--color-border);
      box-shadow: var(--shadow-lg);
      z-index: 3001;
      display: none;
      flex-direction: column;
      transform: translateX(-100%);
      transition: transform var(--dur-med) var(--ease);
      max-height: 100dvh;
      overflow: hidden;
    }
    :host([open]) .pop {
      display: flex;
      transform: translateX(0);
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
    .scrim {
      position: fixed; inset: 0;
      background: var(--color-scrim);
      z-index: 3000; display: none;
    }
    :host([open]) .scrim { display: block; }
    .search {
      display: flex; align-items: center; gap: 8px;
      padding: 10px 12px; border-bottom: 1px solid var(--color-border);
    }
    .search input {
      flex: 1; border: 0; outline: 0; background: transparent; font: inherit;
      font-size: var(--fs-sm); color: var(--color-text);
    }
    .list { overflow-y: auto; flex: 1; padding: 6px; }
    .opt {
      display: flex; align-items: center; gap: 8px;
      padding: 8px 10px; border-radius: 6px; cursor: pointer;
      font-size: var(--fs-sm); color: var(--color-text);
    }
    .opt:hover { background: var(--color-primary-soft); }
    .opt[aria-selected="true"] { color: var(--color-primary); font-weight: var(--fw-medium); }
    .opt .check { margin-left: auto; color: var(--color-primary); opacity: 0; }
    .opt[aria-selected="true"] .check { opacity: 1; }
    .opt[aria-disabled="true"] { color: var(--color-text-subtle); cursor: not-allowed; }
    .empty { padding: 24px; text-align: center; color: var(--color-text-subtle); font-size: var(--fs-sm); }
    /* Slightly larger targets when the drawer is narrow (mobile). */
    @media (max-width: 640px) {
      .search input { font-size: var(--fs-md); }
      .opt          { padding: 12px 12px; font-size: var(--fs-md); }
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
    this._items = [];   // {value, label, disabled}
    this._selected = new Set();
    this._boundOutside = (e) => { if (!this.contains(e.target)) this.#close(); };
    this._boundReposition = () => this.#positionPop();
  }

  connectedCallback() {
    super.connectedCallback();
    // Read <option> children (light DOM) as options
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
    document.removeEventListener('mousedown', this._boundOutside);
    window.removeEventListener('resize', this._boundReposition);
    window.removeEventListener('scroll', this._boundReposition, true);
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

  // ── Shared scroll-lock helpers ────────────────────────────────────────
  // Uses a data-attr counter on <html> so multiple open drawers cooperate.
  #lockScroll() {
    const el = document.documentElement;
    const n = (parseInt(el.dataset.uiDrawerLocks || '0', 10) || 0) + 1;
    el.dataset.uiDrawerLocks = String(n);
    if (n === 1) {
      el.dataset.uiPrevOverflow = el.style.overflow || '';
      el.style.overflow = 'hidden';
    }
  }
  #unlockScroll() {
    const el = document.documentElement;
    const n = Math.max(0, (parseInt(el.dataset.uiDrawerLocks || '0', 10) || 0) - 1);
    el.dataset.uiDrawerLocks = String(n);
    if (n === 0) {
      el.style.overflow = el.dataset.uiPrevOverflow || '';
      delete el.dataset.uiPrevOverflow;
    }
  }

  #openPop() {
    if (this.open) return;
    this.open = true;
    this.#lockScroll();
    this.updateComplete.then(() => {
      setTimeout(() => document.addEventListener('mousedown', this._boundOutside), 0);
      const s = this.renderRoot.querySelector('.search input');
      if (s) setTimeout(() => s.focus(), 20);
    });
  }

  #close() {
    if (!this.open) return;
    this.open = false;
    this.#unlockScroll();
    document.removeEventListener('mousedown', this._boundOutside);
  }

  #positionPop() {
    /* Drawer is CSS-anchored to the viewport's LEFT edge — no positioning
       math needed. Kept as a no-op so existing scroll/resize listeners
       don't break. */
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
    // Register the ad-hoc value so it survives closing/reopening and shows
    // up in #renderValue and in future searches.
    if (!this._items.find(i => i.value === v)) {
      this._items = [...this._items, { value: v, label: v, disabled: false }];
    }
    this.#onOptionClick({ value: v, label: v, disabled: false });
  }

  render() {
    const q = (this._filter || '').trim().toLowerCase();
    const items = this._items.filter(i => !q || i.label.toLowerCase().includes(q));
    // allow-new implies searchable; only show the "Add …" row when the
    // user has actually typed something AND no existing option matches
    // exactly.
    const allowNew = this['allow-new'];
    const showAddNew =
      allowNew && q.length > 0 &&
      !this._items.some(i => i.label.toLowerCase() === q);
    const showSearch = this.searchable || allowNew;
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
      <div class="scrim" @click=${() => this.#close()}></div>
      <div class="pop" role="listbox">
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
          ${items.length === 0 && !showAddNew
            ? html`<div class="empty">No matches</div>`
            : items.map(i => html`
                <div class="opt"
                     aria-selected=${this._selected.has(i.value)}
                     aria-disabled=${i.disabled}
                     @click=${() => this.#onOptionClick(i)}>
                  <span>${i.label}</span>
                  <ui-icon class="check" name="check" size="14"></ui-icon>
                </div>`)}
          ${showAddNew
            ? html`<div class="opt" @click=${() => this.#onAddNew()}>
                     <span>Add “<strong>${this._filter}</strong>”</span>
                     <ui-icon class="check" name="plus" size="14" style="opacity:1"></ui-icon>
                   </div>`
            : nothing}
        </div>
      </div>
    `;
  }
}
customElements.define('ui-select', UISelect);
