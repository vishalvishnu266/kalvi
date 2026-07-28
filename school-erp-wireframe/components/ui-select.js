import { BaseElement, attr } from './base.js';

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
 *   label       – field label (optional)
 *   placeholder – shown when no value
 *   value       – current value
 *   searchable  – (default true) filter box in the popover
 *   clearable   – add an ✕ to clear the value
 *   multiple    – allow multiple selections (returns comma-separated value)
 *
 * Emits `change` with { value, values, labels }.
 */
class UISelect extends BaseElement {
  static styles = `
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
      cursor: pointer;
      transition: border-color var(--dur-fast) var(--ease), box-shadow var(--dur-fast) var(--ease);
      text-align: left;
    }
    .trigger:hover { border-color: var(--color-text-subtle); }
    .trigger:focus { outline: none; box-shadow: var(--shadow-focus); border-color: var(--color-primary); }
    .trigger .placeholder { color: var(--color-text-subtle); }
    .trigger .value { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
    .trigger .caret { color: var(--color-text-subtle); flex: 0 0 auto; }
    .trigger .clear {
      background: none; border: 0; cursor: pointer; padding: 2px;
      color: var(--color-text-subtle); border-radius: 4px; display: grid; place-items: center;
    }
    .trigger .clear:hover { color: var(--color-text); background: color-mix(in srgb, var(--color-text) 8%, transparent); }

    /* chips for multiple */
    .chips { display: flex; flex-wrap: nowrap; gap: 4px; overflow: hidden; }
    .chip {
      display: inline-flex; align-items: center; gap: 3px;
      background: var(--color-primary-soft); color: var(--color-primary);
      padding: 2px 8px; border-radius: var(--radius-pill);
      font-size: var(--fs-xs); font-weight: var(--fw-medium);
    }

    .pop {
      position: absolute; top: calc(100% + 6px); left: 0; right: 0;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      box-shadow: var(--shadow-lg);
      z-index: 1000;              /* above sticky topbar / bottom nav */
      display: none;
      max-height: 320px;
      overflow: hidden;
      max-width: calc(100vw - 24px);
    }
    .scrim {
      position: fixed; inset: 0;
      background: var(--color-scrim);
      z-index: 999; display: none;
      animation: sc-in var(--dur-med) var(--ease);
    }
    :host([open]) .scrim { display: block; }
    @keyframes sc-in { from { opacity: 0; } to { opacity: 1; } }
    @media (min-width: 641px) { .scrim { display: none !important; } }
    :host([open]) .pop { display: flex; flex-direction: column; }
    .search {
      display: flex; align-items: center; gap: 8px;
      padding: 10px 12px; border-bottom: 1px solid var(--color-border);
    }
    .search ui-icon { color: var(--color-text-muted); }
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
    .opt:hover, .opt.hi { background: var(--color-primary-soft); }
    .opt[aria-selected="true"] { color: var(--color-primary); font-weight: var(--fw-medium); }
    .opt .check { margin-left: auto; color: var(--color-primary); opacity: 0; }
    .opt[aria-selected="true"] .check { opacity: 1; }
    .opt[aria-disabled="true"] { color: var(--color-text-subtle); cursor: not-allowed; }
    .empty { padding: 24px; text-align: center; color: var(--color-text-subtle); font-size: var(--fs-sm); }

    /* Mobile: bottom-sheet style, sits above the bottom nav bar */
    @media (max-width: 640px) {
      .pop {
        position: fixed;
        left: 8px; right: 8px; top: auto;
        bottom: calc(var(--bottomnav-h, 62px) + 16px + env(safe-area-inset-bottom, 0));
        max-height: min(70dvh, 480px);
        max-width: none;
        border-radius: var(--radius-xl);
        animation: sel-slide-up var(--dur-med) var(--ease);
      }
      @keyframes sel-slide-up { from { transform: translateY(24px); opacity: 0; } to { transform: none; opacity: 1; } }
      .search { padding: 12px 14px; }
      .search input { font-size: var(--fs-md); }
      .opt { padding: 12px 12px; font-size: var(--fs-md); }
    }
  `;

  static get observedAttributes() { return ['value', 'label', 'placeholder']; }

  constructor() {
    super();
    this._items = [];   // {value, label, disabled}
    this._selected = new Set();
    this._boundOutside = (e) => { if (!this.contains(e.target)) this._close(); };
  }

  connectedCallback() {
    // Read <option> children (light DOM) as options
    this._items = [...this.querySelectorAll('option')].map(o => ({
      value: o.value ?? o.textContent.trim(),
      label: o.textContent.trim(),
      disabled: o.hasAttribute('disabled'),
    }));
    // hide the <option>s (they were only used as declarative data)
    [...this.children].forEach(el => { if (el.tagName === 'OPTION') el.style.display = 'none'; });

    const initial = attr(this, 'value');
    if (initial) initial.split(',').forEach(v => this._selected.add(v.trim()));
    super.connectedCallback();
  }

  render() {
    const label = attr(this, 'label');
    return `
      ${label ? `<span class="label">${label}</span>` : ''}
      <button class="trigger" type="button" data-role="trigger">
        <span data-role="value" class="value"></span>
        ${this.hasAttribute('clearable') ? `<button class="clear" data-role="clear" title="Clear"><ui-icon name="x" size="14"></ui-icon></button>` : ''}
        <ui-icon class="caret" name="chevronDown" size="14"></ui-icon>
      </button>
      <div class="scrim" data-role="scrim"></div>
      <div class="pop" role="listbox">
        ${this.hasAttribute('searchable') || !this.hasAttribute('no-search') ? `
        <div class="search">
          <ui-icon name="search" size="14"></ui-icon>
          <input placeholder="Search…" data-role="search">
        </div>` : ''}
        <div class="list" data-role="list"></div>
      </div>
    `;
  }

  afterRender() {
    this.$('[data-role="trigger"]').addEventListener('click', (e) => {
      if (e.target.closest('[data-role="clear"]')) return; // handled below
      e.stopPropagation();
      this.hasAttribute('open') ? this._close() : this._open();
    });
    const clear = this.$('[data-role="clear"]');
    if (clear) clear.addEventListener('click', (e) => {
      e.stopPropagation();
      this._selected.clear();
      this._syncValue();
      this._renderTrigger();
      this._renderList();
    });
    const search = this.$('[data-role="search"]');
    if (search) search.addEventListener('input', () => this._renderList(search.value));
    this._renderTrigger();
    this._renderList();
  }

  _open() {
    this.setAttribute('open', '');
    this._positionPop();
    document.addEventListener('mousedown', this._boundOutside);
    window.addEventListener('resize',  this._boundReposition = () => this._positionPop());
    window.addEventListener('scroll',  this._boundReposition, true);
    const s = this.$('[data-role="search"]');
    if (s) setTimeout(() => s.focus(), 20);
  }
  _close() {
    this.removeAttribute('open');
    document.removeEventListener('mousedown', this._boundOutside);
    if (this._boundReposition) {
      window.removeEventListener('resize', this._boundReposition);
      window.removeEventListener('scroll', this._boundReposition, true);
    }
  }
  _positionPop() {
    const trigger = this.$('[data-role="trigger"]');
    const pop     = this.$('.pop');
    if (!trigger || !pop) return;
    const isMobile = window.matchMedia('(max-width: 640px)').matches;
    if (isMobile) { pop.style.cssText = ''; return; }
    // Always below-left of the trigger, matching its width.
    const r = trigger.getBoundingClientRect();
    const POP_H_MAX = 320, GAP = 6;
    const width = Math.max(220, Math.min(r.width, window.innerWidth - 16));
    let top  = r.bottom + GAP;
    let left = r.left;
    if (left + width      > window.innerWidth  - 8) left = window.innerWidth  - width      - 8;
    if (top  + POP_H_MAX  > window.innerHeight - 8) top  = window.innerHeight - POP_H_MAX  - 8;
    pop.style.position = 'fixed';
    pop.style.top   = `${Math.max(8, top)}px`;
    pop.style.left  = `${Math.max(8, left)}px`;
    pop.style.right = 'auto';
    pop.style.bottom = 'auto';
    pop.style.width = `${width}px`;
  }

  _renderTrigger() {
    const box = this.$('[data-role="value"]');
    const ph  = attr(this, 'placeholder', 'Select…');
    if (this._selected.size === 0) {
      box.innerHTML = `<span class="placeholder">${ph}</span>`;
      return;
    }
    if (this.hasAttribute('multiple')) {
      const labels = [...this._selected].map(v => (this._items.find(i => i.value === v)?.label) || v);
      box.innerHTML = `<span class="chips">${labels.map(l => `<span class="chip">${l}</span>`).join('')}</span>`;
    } else {
      const v = [...this._selected][0];
      box.textContent = (this._items.find(i => i.value === v)?.label) || v;
    }
  }

  _renderList(filter = '') {
    const list = this.$('[data-role="list"]');
    const q = filter.trim().toLowerCase();
    const items = this._items.filter(i => !q || i.label.toLowerCase().includes(q));
    if (!items.length) { list.innerHTML = `<div class="empty">No matches</div>`; return; }
    list.innerHTML = items.map(i => `
      <div class="opt"
           data-value="${i.value}"
           aria-selected="${this._selected.has(i.value)}"
           aria-disabled="${i.disabled}">
        <span>${i.label}</span>
        <ui-icon class="check" name="check" size="14"></ui-icon>
      </div>
    `).join('');
    list.querySelectorAll('.opt').forEach(o => o.addEventListener('click', () => {
      if (o.getAttribute('aria-disabled') === 'true') return;
      const v = o.dataset.value;
      if (this.hasAttribute('multiple')) {
        this._selected.has(v) ? this._selected.delete(v) : this._selected.add(v);
      } else {
        this._selected.clear(); this._selected.add(v); this._close();
      }
      this._syncValue();
      this._renderTrigger();
      this._renderList(this.$('[data-role="search"]')?.value || '');
    }));
  }

  _syncValue() {
    const arr = [...this._selected];
    this.setAttribute('value', arr.join(','));
    const labels = arr.map(v => (this._items.find(i => i.value === v)?.label) || v);
    this.emit('change', { value: arr.join(','), values: arr, labels });
  }

  get value() { return [...this._selected].join(','); }
}
customElements.define('ui-select', UISelect);
