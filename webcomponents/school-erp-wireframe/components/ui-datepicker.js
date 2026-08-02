import { BaseElement, attr } from './base.js';

/**
 * Single-date picker with a calendar popover.
 *
 * <ui-datepicker label="Date of birth" value="2015-08-12"></ui-datepicker>
 *
 * Emits `change` with { value } in ISO (YYYY-MM-DD).
 */
class UIDatepicker extends BaseElement {
  static styles = `
    :host { display: inline-block; position: relative; box-sizing: border-box; z-index: auto; }
    /* When open, host becomes a very high stacking context so the (fixed)
       popover paints above sticky topbar / bottom nav even inside modals. */
    :host([open]) { z-index: 3000; }
    .label {
      display: block; font-size: var(--fs-xs); color: var(--color-text-muted);
      margin-bottom: 6px; font-weight: var(--fw-medium);
    }
    .trigger {
      display: inline-flex; align-items: center; gap: 8px;
      background: var(--color-surface);
      border: 1px solid var(--color-border-strong);
      border-radius: var(--radius-md);
      height: 40px; padding: 0 12px;
      font: inherit; font-size: var(--fs-sm); color: var(--color-text);
      cursor: pointer; min-width: 180px;
      transition: border-color var(--dur-fast) var(--ease), box-shadow var(--dur-fast) var(--ease);
    }
    .trigger:hover { border-color: var(--color-text-subtle); }
    .trigger:focus { outline: none; box-shadow: var(--shadow-focus); border-color: var(--color-primary); }
    .trigger ui-icon { color: var(--color-text-muted); }
    .trigger .val { flex: 1; }
    .trigger .placeholder { color: var(--color-text-subtle); }
    .trigger .caret { color: var(--color-text-subtle); }

    .pop {
      position: absolute; top: calc(100% + 6px); left: 0;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      box-shadow: var(--shadow-lg);
      /* Sits above sticky topbar (20), bottom nav (30) and shell chrome.
         Modal (2000) is still above so pickers inside modals overlay too. */
      z-index: 1000;
      display: none;
      padding: 12px;
      width: 280px;
      max-width: calc(100vw - 24px);
    }
    /* Scrim behind the mobile bottom sheet */
    .scrim {
      position: fixed; inset: 0;
      background: var(--color-scrim);
      z-index: 999; display: none;
      animation: sc-in var(--dur-med) var(--ease);
    }
    :host([open]) .scrim { display: block; }
    @keyframes sc-in { from { opacity: 0; } to { opacity: 1; } }
    @media (min-width: 641px) { .scrim { display: none !important; } }
    :host([data-align="right"]) .pop { left: auto; right: 0; }
    /* On mobile, the popover is fixed to the viewport bottom, so
       the desktop right-align rule must NOT apply. */
    @media (max-width: 640px) {
      :host([data-align="right"]) .pop { left: 8px; right: 8px; }
    }
    :host([open]) .pop { display: block; }

    .head { display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px; }
    .head .month { font-weight: var(--fw-semibold); font-size: var(--fs-sm); }
    .nav-btn {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-text-muted); width: 26px; height: 26px;
      border-radius: 6px; display: grid; place-items: center;
    }
    .nav-btn:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); color: var(--color-text); }

    .dow, .grid { display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px; }
    .dow div {
      font-size: 10px; color: var(--color-text-subtle); text-align: center;
      padding: 4px 0; font-weight: var(--fw-medium);
    }
    .grid button {
      appearance: none; border: 0; background: transparent;
      font: inherit; font-size: var(--fs-xs); color: var(--color-text);
      height: 32px; cursor: pointer;
      border-radius: 6px;
      display: grid; place-items: center;
      font-variant-numeric: tabular-nums;
    }
    .grid button:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); }
    .grid button.muted { color: var(--color-text-subtle); }
    .grid button.today { box-shadow: inset 0 0 0 1px var(--color-primary); color: var(--color-primary); font-weight: var(--fw-semibold); }
    .grid button.sel { background: var(--color-primary); color: var(--color-primary-contrast); font-weight: var(--fw-semibold); }

    .foot { margin-top: 10px; display: flex; justify-content: space-between; gap: 8px; }
    .foot .today-link {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-primary); font: inherit; font-size: var(--fs-xs); font-weight: var(--fw-medium);
    }

    /* Mobile: fixed bottom-sheet, sits above the bottom nav */
    @media (max-width: 640px) {
      .pop {
        position: fixed;
        left: 8px; right: 8px; top: auto;
        bottom: calc(var(--bottomnav-h, 62px) + 16px + env(safe-area-inset-bottom, 0));
        width: auto; max-width: none;
        padding: 16px;
        border-radius: var(--radius-xl);
        animation: dp-slide-up var(--dur-med) var(--ease);
      }
      @keyframes dp-slide-up { from { transform: translateY(24px); opacity: 0; } to { transform: none; opacity: 1; } }
      .grid button { height: 40px; font-size: var(--fs-sm); }
      .dow div { font-size: 11px; padding: 6px 0; }
    }
  `;

  static get observedAttributes() { return ['value']; }

  constructor() {
    super();
    this._val = null;
    this._viewYear = null;
    this._viewMonth = null;
    this._boundOutside = (e) => { if (!this.contains(e.target)) this._close(); };
  }

  connectedCallback() {
    this._val = this._parse(attr(this, 'value'));
    const ref = this._val || new Date();
    this._viewYear = ref.getFullYear();
    this._viewMonth = ref.getMonth();
    super.connectedCallback();
  }

  render() {
    const label = attr(this, 'label');
    return `
      ${label ? `<span class="label">${label}</span>` : ''}
      <button class="trigger" type="button" data-role="trigger">
        <ui-icon name="calendar" size="16"></ui-icon>
        <span class="val" data-role="text">${this._fmtDisplay()}</span>
        <ui-icon class="caret" name="chevronDown" size="14"></ui-icon>
      </button>
      <div class="scrim" data-role="scrim"></div>
      <div class="pop" role="dialog">
        <div class="head">
          <button class="nav-btn" data-nav="-1"><ui-icon name="chevronRight" size="14" style="transform: rotate(180deg)"></ui-icon></button>
          <span class="month" data-role="month"></span>
          <button class="nav-btn" data-nav="+1"><ui-icon name="chevronRight" size="14"></ui-icon></button>
        </div>
        <div class="dow"><div>M</div><div>T</div><div>W</div><div>T</div><div>F</div><div>S</div><div>S</div></div>
        <div class="grid" data-role="grid"></div>
        <div class="foot">
          <button class="today-link" data-action="today">Today</button>
          <button class="today-link" data-action="clear" style="color:var(--color-text-muted)">Clear</button>
        </div>
      </div>
    `;
  }

  afterRender() {
    this.$('[data-role="trigger"]').addEventListener('click', (e) => {
      e.stopPropagation();
      this.hasAttribute('open') ? this._close() : this._open();
    });
    this.$$('.nav-btn').forEach(b => b.addEventListener('click', () => {
      this._viewMonth += parseInt(b.dataset.nav, 10);
      if (this._viewMonth < 0) { this._viewMonth = 11; this._viewYear--; }
      if (this._viewMonth > 11) { this._viewMonth = 0;  this._viewYear++; }
      this._renderGrid();
    }));
    this.shadowRoot.addEventListener('click', (e) => {
      const b = e.target.closest('[data-action]');
      if (!b) return;
      if (b.dataset.action === 'today') { this._val = this._startOfDay(new Date()); this._sync(); }
      if (b.dataset.action === 'clear') { this._val = null; this._sync(); }
    });
    this._renderGrid();
  }

  _open() {
    this.setAttribute('open', '');
    this._positionPop();
    document.addEventListener('mousedown', this._boundOutside);
    window.addEventListener('resize',  this._boundReposition = () => this._positionPop());
    window.addEventListener('scroll',  this._boundReposition, true);
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
    // Predictable anchor: always below the trigger, aligned to its LEFT edge.
    // The only adjustment we make is clamping inside the viewport so it
    // never renders off-screen.
    const r = trigger.getBoundingClientRect();
    const POP_W = 280, POP_H = 340, GAP = 6;
    let top  = r.bottom + GAP;
    let left = r.left;
    // Clamp right/bottom edges
    if (left + POP_W > window.innerWidth  - 8) left = window.innerWidth  - POP_W - 8;
    if (top  + POP_H > window.innerHeight - 8) top  = window.innerHeight - POP_H - 8;
    pop.style.position = 'fixed';
    pop.style.top  = `${Math.max(8, top)}px`;
    pop.style.left = `${Math.max(8, left)}px`;
    pop.style.right = 'auto';
    pop.style.bottom = 'auto';
  }

  _renderGrid() {
    const y = this._viewYear, m = this._viewMonth;
    this.$('[data-role="month"]').textContent =
      new Date(y, m, 1).toLocaleString(undefined, { month: 'long', year: 'numeric' });
    const first = new Date(y, m, 1);
    const startDow = (first.getDay() + 6) % 7;
    const daysInMonth = new Date(y, m + 1, 0).getDate();
    const prev = new Date(y, m, 0).getDate();
    const cells = [];
    for (let i = startDow - 1; i >= 0; i--) cells.push({ d: prev - i, muted: true, date: new Date(y, m - 1, prev - i) });
    for (let d = 1; d <= daysInMonth; d++)  cells.push({ d, muted: false, date: new Date(y, m, d) });
    while (cells.length < 42) {
      const idx = cells.length - startDow - daysInMonth + 1;
      cells.push({ d: idx, muted: true, date: new Date(y, m + 1, idx) });
    }
    const today = this._startOfDay(new Date());
    this.$('[data-role="grid"]').innerHTML = cells.map(c => {
      const cls = [];
      if (c.muted) cls.push('muted');
      if (this._sameDay(c.date, today)) cls.push('today');
      if (this._val && this._sameDay(c.date, this._val)) cls.push('sel');
      return `<button class="${cls.join(' ')}" data-date="${this._fmtISO(c.date)}">${c.d}</button>`;
    }).join('');
    this.$$('.grid button').forEach(btn => btn.addEventListener('click', () => {
      this._val = this._parse(btn.dataset.date);
      this._sync();
      this._close();
    }));
  }

  _sync() {
    this.setAttribute('value', this._val ? this._fmtISO(this._val) : '');
    this.$('[data-role="text"]').innerHTML = this._fmtDisplay();
    this.emit('change', { value: this._val ? this._fmtISO(this._val) : '' });
    this._renderGrid();
  }

  _fmtDisplay() {
    if (!this._val) return `<span class="placeholder">${attr(this, 'placeholder', 'Select a date')}</span>`;
    return this._val.toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' });
  }
  _parse(s) { if (!s) return null; const [y,m,d] = s.split('-').map(Number); return new Date(y, m-1, d); }
  _fmtISO(d) { const p = n => String(n).padStart(2,'0'); return `${d.getFullYear()}-${p(d.getMonth()+1)}-${p(d.getDate())}`; }
  _sameDay(a, b) { return a && b && a.getFullYear()===b.getFullYear() && a.getMonth()===b.getMonth() && a.getDate()===b.getDate(); }
  _startOfDay(d) { return new Date(d.getFullYear(), d.getMonth(), d.getDate()); }

  get value() { return this._val ? this._fmtISO(this._val) : ''; }
}
customElements.define('ui-datepicker', UIDatepicker);
