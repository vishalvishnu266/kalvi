import { BaseElement, attr } from './base.js';

/**
 * Single-trigger date-range picker with popover calendar + preset shortcuts.
 *
 * <ui-daterange label="Date range" from="2026-07-01" to="2026-07-28"></ui-daterange>
 *
 * Emits `change` with { from, to } (YYYY-MM-DD strings) when the range changes.
 */
class UIDateRange extends BaseElement {
  static styles = `
    :host { display: inline-block; position: relative; box-sizing: border-box; z-index: auto; }
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
      height: 36px; padding: 0 12px;
      font: inherit; font-size: var(--fs-sm); color: var(--color-text);
      cursor: pointer;
      transition: border-color var(--dur-fast) var(--ease), box-shadow var(--dur-fast) var(--ease);
      min-width: 240px;
    }
    .trigger:hover { border-color: var(--color-text-subtle); }
    .trigger:focus { outline: none; box-shadow: var(--shadow-focus); border-color: var(--color-primary); }
    .trigger ui-icon { color: var(--color-text-muted); }
    .trigger .caret { margin-left: auto; color: var(--color-text-subtle); }

    /* Popover — default anchor to the LEFT edge of the trigger, but we
       flip to the right (via [data-align="right"]) at runtime when there
       isn't enough room to the right of the trigger. */
    .pop {
      position: absolute; top: calc(100% + 6px); left: 0;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      box-shadow: var(--shadow-lg);
      display: none;
      z-index: 1000;              /* above sticky topbar / bottom nav */
      overflow: hidden;
      width: 620px;
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
    @media (min-width: 721px) { .scrim { display: none !important; } }
    :host([data-align="right"]) .pop { left: auto; right: 0; }
    /* IMPORTANT: min-width: 0 on both tracks so the right column can
       shrink and its footer never overflows the popover. */
    :host([open]) .pop {
      display: grid;
      grid-template-columns: 160px minmax(0, 1fr);
    }
    .cals-wrap { min-width: 0; display: flex; flex-direction: column; }

    .presets {
      display: flex; flex-direction: column; gap: 2px;
      padding: 10px; border-right: 1px solid var(--color-border);
      background: var(--color-surface-alt);
    }
    .presets button {
      appearance: none; border: 0; background: transparent;
      font: inherit; font-size: var(--fs-sm); color: var(--color-text);
      text-align: left; padding: 8px 10px; border-radius: 6px;
      cursor: pointer;
    }
    .presets button:hover { background: color-mix(in srgb, var(--color-text) 6%, transparent); }
    .presets button[aria-pressed="true"] { background: var(--color-primary); color: var(--color-primary-contrast); }

    .cals { padding: 12px; display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
    .cal { min-width: 220px; }
    .cal-head {
      display: flex; align-items: center; justify-content: space-between;
      margin-bottom: 8px;
    }
    .cal-head .month {
      font-size: var(--fs-sm); font-weight: var(--fw-semibold);
    }
    .nav-btn {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-text-muted); width: 26px; height: 26px;
      border-radius: 6px; display: grid; place-items: center;
    }
    .nav-btn:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); color: var(--color-text); }

    .dow, .grid {
      display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px;
    }
    .dow div {
      font-size: 10px; color: var(--color-text-subtle); text-align: center;
      padding: 4px 0; font-weight: var(--fw-medium);
    }
    .grid button {
      appearance: none; border: 0; background: transparent;
      font: inherit; font-size: var(--fs-xs); color: var(--color-text);
      height: 30px; width: 100%; cursor: pointer;
      border-radius: 6px;
      display: grid; place-items: center;
      font-variant-numeric: tabular-nums;
    }
    .grid button:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); }
    .grid button.muted { color: var(--color-text-subtle); cursor: default; opacity: 0.5; }
    .grid button.muted:hover { background: transparent; }
    .grid button[disabled] { pointer-events: none; }
    .grid button.today { box-shadow: inset 0 0 0 1px var(--color-primary); color: var(--color-primary); font-weight: var(--fw-semibold); }
    .grid button.in-range {
      background: var(--color-primary-soft); border-radius: 0;
      color: var(--color-text);
    }
    .grid button.start, .grid button.end {
      background: var(--color-primary); color: var(--color-primary-contrast);
      border-radius: 6px; font-weight: var(--fw-semibold);
    }
    .grid button.start { border-top-right-radius: 0; border-bottom-right-radius: 0; }
    .grid button.end   { border-top-left-radius:  0; border-bottom-left-radius:  0; }
    .grid button.start.end { border-radius: 6px; }

    .foot {
      display: flex; align-items: center; flex-wrap: wrap;
      justify-content: space-between; gap: 10px;
      padding: 10px 12px; border-top: 1px solid var(--color-border);
      background: var(--color-surface);
      min-width: 0;
    }
    .foot .info {
      font-size: var(--fs-xs); color: var(--color-text-muted);
      font-variant-numeric: tabular-nums;
      min-width: 0;
      white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }
    .foot .actions { display: flex; gap: 8px; flex-shrink: 0; }

    /* Manual entry row */
    .manual {
      display: flex; align-items: center; gap: 8px;
      padding: 0 12px 10px;
    }
    .manual .field {
      display: flex; flex-direction: column; gap: 3px; flex: 1; min-width: 0;
    }
    .manual .field label {
      font-size: 10px; color: var(--color-text-subtle); font-weight: var(--fw-medium);
      letter-spacing: 0.04em; text-transform: uppercase;
    }
    .manual input {
      width: 100%; min-width: 0; box-sizing: border-box;
      background: var(--color-surface);
      border: 1px solid var(--color-border-strong);
      border-radius: 6px;
      padding: 6px 8px;
      font: inherit; font-size: var(--fs-xs); color: var(--color-text);
      font-variant-numeric: tabular-nums;
    }
    .manual input:focus { outline: none; border-color: var(--color-primary); box-shadow: var(--shadow-focus); }
    .manual input.invalid { border-color: var(--color-danger); }
    .manual .arrow { color: var(--color-text-subtle); margin-top: 14px; flex: 0 0 auto; }

    /* Mobile: stack calendars, popover is a bottom sheet above the nav bar */
    @media (max-width: 720px) {
      .pop {
        position: fixed;
        left: 8px; right: 8px; top: auto;
        bottom: calc(var(--bottomnav-h, 62px) + 16px + env(safe-area-inset-bottom, 0));
        width: auto; min-width: 0; max-width: none;
        max-height: min(80dvh, 600px);
        overflow-y: auto;
        grid-template-columns: 1fr;
        border-radius: var(--radius-xl);
        animation: dr-slide-up var(--dur-med) var(--ease);
      }
      @keyframes dr-slide-up { from { transform: translateY(24px); opacity: 0; } to { transform: none; opacity: 1; } }
      /* Neutralize desktop right-align rule on mobile */
      :host([data-align="right"]) .pop { left: 8px; right: 8px; }
      :host([open]) .pop { grid-template-columns: 1fr; }
      .presets { flex-direction: row; overflow-x: auto; border-right: 0; border-bottom: 1px solid var(--color-border); }
      .presets button { flex: 0 0 auto; white-space: nowrap; }
      .cals { grid-template-columns: 1fr; }
      .grid button { height: 36px; }
      .manual { flex-direction: column; align-items: stretch; }
      .manual .arrow { margin: 0; align-self: center; }
    }
  `;

  static get observedAttributes() { return ['from', 'to']; }

  constructor() {
    super();
    this._from = null;    // Date
    this._to = null;      // Date
    this._pending = null; // clicked start while waiting for end
    // Reference month shown in the LEFT calendar
    this._viewYear = null;
    this._viewMonth = null; // 0-11
    this._boundOutside = (e) => {
      if (!this.contains(e.target)) this._close();
    };
  }

  connectedCallback() {
    // parse initial attrs
    this._from = this._parse(attr(this, 'from')) || this._startOfMonth(new Date());
    this._to   = this._parse(attr(this, 'to'))   || new Date();
    const start = this._from || new Date();
    this._viewYear  = start.getFullYear();
    this._viewMonth = start.getMonth();
    super.connectedCallback();
  }

  render() {
    const label = attr(this, 'label');
    return `
      ${label ? `<span class="label">${label}</span>` : ''}
      <button class="trigger" type="button" data-role="trigger">
        <ui-icon name="calendar" size="16"></ui-icon>
        <span data-role="text">${this._fmtRange()}</span>
        <ui-icon class="caret" name="chevronDown" size="14"></ui-icon>
      </button>
      <div class="scrim" data-role="scrim"></div>
      <div class="pop" role="dialog">
        <div class="presets">
          <button data-preset="today">Today</button>
          <button data-preset="yesterday">Yesterday</button>
          <button data-preset="last7">Last 7 days</button>
          <button data-preset="last30">Last 30 days</button>
          <button data-preset="thisMonth">This month</button>
          <button data-preset="lastMonth">Last month</button>
          <button data-preset="thisYear">This year</button>
          <button data-preset="custom">Custom…</button>
        </div>
        <div class="cals-wrap">
          <div class="cals">
            <div class="cal" data-cal="left"></div>
            <div class="cal" data-cal="right"></div>
          </div>
          <div class="manual">
            <div class="field">
              <label>From</label>
              <input data-role="manual-from" placeholder="YYYY-MM-DD" inputmode="numeric" autocomplete="off">
            </div>
            <span class="arrow">→</span>
            <div class="field">
              <label>To</label>
              <input data-role="manual-to" placeholder="YYYY-MM-DD" inputmode="numeric" autocomplete="off">
            </div>
          </div>
          <div class="foot">
            <span class="info" data-role="foot-info"></span>
            <div class="actions">
              <ui-button size="sm" variant="secondary" data-action="cancel">Cancel</ui-button>
              <ui-button size="sm" data-action="apply">Apply</ui-button>
            </div>
          </div>
        </div>
      </div>
    `;
  }

  afterRender() {
    this.$('[data-role="trigger"]').addEventListener('click', (e) => {
      e.stopPropagation();
      this.hasAttribute('open') ? this._close() : this._open();
    });
    this.$$('.presets button').forEach(b => b.addEventListener('click', () => this._applyPreset(b.dataset.preset)));
    this.shadowRoot.addEventListener('click', (e) => {
      const act = e.target.closest('[data-action]');
      if (!act) return;
      if (act.dataset.action === 'cancel') this._close();
      if (act.dataset.action === 'apply')  this._apply();
    });

    // ---- Manual entry inputs ----
    const fromInp = this.$('[data-role="manual-from"]');
    const toInp   = this.$('[data-role="manual-to"]');
    const commit = (which, input) => {
      const raw = input.value.trim();
      if (!raw) return;
      const parsed = this._parseFlexible(raw);
      if (!parsed || isNaN(parsed)) { input.classList.add('invalid'); return; }
      input.classList.remove('invalid');
      // Normalize input display back to YYYY-MM-DD
      input.value = this._fmtISO(parsed);
      if (which === 'from') this._from = parsed;
      else                  this._to   = parsed;
      // If user reversed the range, auto-swap so from <= to
      if (this._from && this._to && this._from > this._to) {
        [this._from, this._to] = [this._to, this._from];
      }
      // Jump left calendar to the FROM month (fall back to TO)
      const ref = this._from || this._to;
      if (ref) { this._viewYear = ref.getFullYear(); this._viewMonth = ref.getMonth(); }
      this._pending = null;
      this._renderCalendars();
    };
    fromInp.addEventListener('change', () => commit('from', fromInp));
    fromInp.addEventListener('keydown', e => { if (e.key === 'Enter') commit('from', fromInp); });
    toInp.addEventListener('change',   () => commit('to',   toInp));
    toInp.addEventListener('keydown',  e => { if (e.key === 'Enter') commit('to',   toInp); });
    // Live validation (visual only) as the user types
    [fromInp, toInp].forEach(inp => inp.addEventListener('input', () => {
      const v = inp.value.trim();
      inp.classList.toggle('invalid', !!v && !this._parseFlexible(v));
    }));

    this._renderCalendars();
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
    const isMobile = window.matchMedia('(max-width: 720px)').matches;
    if (isMobile) { pop.style.cssText = ''; return; }
    // Predictable: below-left of trigger, clamped inside the viewport.
    const r = trigger.getBoundingClientRect();
    const POP_W = Math.min(620, window.innerWidth - 16);
    const POP_H = 460, GAP = 6;
    let top  = r.bottom + GAP;
    let left = r.left;
    if (left + POP_W > window.innerWidth  - 8) left = window.innerWidth  - POP_W - 8;
    if (top  + POP_H > window.innerHeight - 8) top  = window.innerHeight - POP_H - 8;
    pop.style.position = 'fixed';
    pop.style.top   = `${Math.max(8, top)}px`;
    pop.style.left  = `${Math.max(8, left)}px`;
    pop.style.right = 'auto';
    pop.style.bottom = 'auto';
    pop.style.width = `${POP_W}px`;
  }

  _apply() {
    this.setAttribute('from', this._fmtISO(this._from));
    this.setAttribute('to',   this._fmtISO(this._to));
    this.$('[data-role="text"]').textContent = this._fmtRange();
    this.emit('change', { from: this._fmtISO(this._from), to: this._fmtISO(this._to) });
    this._close();
  }

  _applyPreset(key) {
    const today = new Date();
    const set = (a, b) => { this._from = a; this._to = b; this._pending = null; };
    switch (key) {
      case 'today':      set(this._startOfDay(today), this._startOfDay(today)); break;
      case 'yesterday':  { const y = this._addDays(today, -1); set(y, y); break; }
      case 'last7':      set(this._addDays(today, -6), today); break;
      case 'last30':     set(this._addDays(today, -29), today); break;
      case 'thisMonth':  set(this._startOfMonth(today), this._endOfMonth(today)); break;
      case 'lastMonth':  { const d = new Date(today.getFullYear(), today.getMonth()-1, 1); set(this._startOfMonth(d), this._endOfMonth(d)); break; }
      case 'thisYear':   set(new Date(today.getFullYear(),0,1), new Date(today.getFullYear(),11,31)); break;
      case 'custom':     /* just keep current */ break;
    }
    this._viewYear = (this._from || today).getFullYear();
    this._viewMonth = (this._from || today).getMonth();
    this._renderCalendars();
    this.$$('.presets button').forEach(b => b.setAttribute('aria-pressed', b.dataset.preset === key));
  }

  _renderCalendars() {
    const leftDate  = new Date(this._viewYear, this._viewMonth, 1);
    const rightDate = new Date(this._viewYear, this._viewMonth + 1, 1);
    this._renderMonth(this.$('[data-cal="left"]'),  leftDate,  true);
    this._renderMonth(this.$('[data-cal="right"]'), rightDate, false);
    this.$('[data-role="foot-info"]').textContent =
      this._from && this._to
        ? `${this._fmtISO(this._from)}  →  ${this._fmtISO(this._to)}  ·  ${this._diffDays(this._from, this._to) + 1} day(s)`
        : 'Pick a start date';
    // Keep manual inputs in sync (but don't clobber focus)
    const fromInp = this.$('[data-role="manual-from"]');
    const toInp   = this.$('[data-role="manual-to"]');
    if (fromInp && document.activeElement !== fromInp) fromInp.value = this._from ? this._fmtISO(this._from) : '';
    if (toInp   && document.activeElement !== toInp)   toInp.value   = this._to   ? this._fmtISO(this._to)   : '';
  }

  // Flexible parse: accepts YYYY-MM-DD, YYYY/MM/DD, DD-MM-YYYY, DD/MM/YYYY, MM/DD/YYYY
  // and any string the browser's Date() can parse (e.g. "Aug 12 2026").
  _parseFlexible(str) {
    if (!str) return null;
    const s = str.trim();
    // YYYY-MM-DD or YYYY/MM/DD
    let m = s.match(/^(\d{4})[-/](\d{1,2})[-/](\d{1,2})$/);
    if (m) return this._safeDate(+m[1], +m[2] - 1, +m[3]);
    // DD-MM-YYYY or DD/MM/YYYY  (assumes day-first because that's the label order)
    m = s.match(/^(\d{1,2})[-/](\d{1,2})[-/](\d{4})$/);
    if (m) {
      const d = +m[1], mo = +m[2], y = +m[3];
      // If day > 12 it's definitely day-first; otherwise still treat as day-first.
      return this._safeDate(y, mo - 1, d);
    }
    // Fallback to Date parser (handles "Aug 12 2026", ISO with time, etc.)
    const d = new Date(s);
    return isNaN(d) ? null : this._startOfDay(d);
  }
  _safeDate(y, mo, d) {
    if (mo < 0 || mo > 11 || d < 1 || d > 31) return null;
    const dt = new Date(y, mo, d);
    // Reject invalid rollover like Feb 30 → Mar 2
    if (dt.getFullYear() !== y || dt.getMonth() !== mo || dt.getDate() !== d) return null;
    return dt;
  }

  _renderMonth(host, refDate, isLeft) {
    if (!host) return;
    const year  = refDate.getFullYear();
    const month = refDate.getMonth();
    const monthName = refDate.toLocaleString(undefined, { month: 'long', year: 'numeric' });
    const first = new Date(year, month, 1);
    const startDow = (first.getDay() + 6) % 7; // week starts Monday
    const daysInMonth = new Date(year, month + 1, 0).getDate();
    const prevMonthDays = new Date(year, month, 0).getDate();

    const cells = [];
    // Leading (previous month)
    for (let i = startDow - 1; i >= 0; i--) {
      cells.push({ d: prevMonthDays - i, muted: true, date: new Date(year, month - 1, prevMonthDays - i) });
    }
    // Current month
    for (let d = 1; d <= daysInMonth; d++) {
      cells.push({ d, muted: false, date: new Date(year, month, d) });
    }
    // Trailing (next month) – fill to full weeks (42 cells)
    while (cells.length % 7 !== 0 || cells.length < 42) {
      const idx = cells.length - startDow - daysInMonth + 1;
      cells.push({ d: idx, muted: true, date: new Date(year, month + 1, idx) });
    }

    const today = this._startOfDay(new Date());
    const grid = cells.map(c => {
      const cls = [];
      if (c.muted) cls.push('muted');
      // IMPORTANT: only apply today / selection / range classes to the
      // current month's own cells. Otherwise the trailing muted days of the
      // left calendar (which are actually the right calendar's current
      // month) would appear "selected twice" — once on each grid.
      if (!c.muted) {
        if (this._sameDay(c.date, today)) cls.push('today');
        if (this._from && this._to && c.date >= this._startOfDay(this._from) && c.date <= this._startOfDay(this._to))
          cls.push('in-range');
        if (this._from && this._sameDay(c.date, this._from)) cls.push('start');
        if (this._to   && this._sameDay(c.date, this._to))   cls.push('end');
      }
      // Disable muted cells entirely — clicking them was confusing.
      const disabled = c.muted ? 'disabled tabindex="-1"' : '';
      return `<button class="${cls.join(' ')}" data-date="${this._fmtISO(c.date)}" ${disabled}>${c.d}</button>`;
    }).join('');

    host.innerHTML = `
      <div class="cal-head">
        ${isLeft ? '<button class="nav-btn" data-nav="-1"><ui-icon name="chevronRight" size="14" style="transform: rotate(180deg)"></ui-icon></button>' : '<span></span>'}
        <div class="month">${monthName}</div>
        ${!isLeft ? '<button class="nav-btn" data-nav="+1"><ui-icon name="chevronRight" size="14"></ui-icon></button>' : '<span></span>'}
      </div>
      <div class="dow"><div>M</div><div>T</div><div>W</div><div>T</div><div>F</div><div>S</div><div>S</div></div>
      <div class="grid">${grid}</div>
    `;

    host.querySelectorAll('[data-nav]').forEach(b => b.addEventListener('click', () => {
      this._viewMonth += parseInt(b.dataset.nav, 10);
      if (this._viewMonth < 0) { this._viewMonth = 11; this._viewYear--; }
      if (this._viewMonth > 11) { this._viewMonth = 0;  this._viewYear++; }
      this._renderCalendars();
    }));
    host.querySelectorAll('button[data-date]:not([disabled])').forEach(btn => btn.addEventListener('click', () => {
      const clicked = this._parse(btn.dataset.date);
      // Two-click range selection
      if (!this._pending) {
        this._from = clicked; this._to = clicked; this._pending = clicked;
      } else {
        if (clicked < this._pending) { this._from = clicked; this._to = this._pending; }
        else                          { this._from = this._pending; this._to = clicked; }
        this._pending = null;
      }
      this._renderCalendars();
    }));
  }

  // ---------- utils ----------
  _parse(str) {
    if (!str) return null;
    const [y, m, d] = str.split('-').map(Number);
    return new Date(y, m - 1, d);
  }
  _fmtISO(d) {
    if (!d) return '';
    const pad = n => String(n).padStart(2, '0');
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
  }
  _fmtRange() {
    if (!this._from || !this._to) return 'Select date range';
    const opts = { month: 'short', day: 'numeric', year: 'numeric' };
    const a = this._from.toLocaleDateString(undefined, opts);
    const b = this._to.toLocaleDateString(undefined, opts);
    return this._sameDay(this._from, this._to) ? a : `${a}  →  ${b}`;
  }
  _sameDay(a, b) { return a && b && a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate(); }
  _startOfDay(d)   { return new Date(d.getFullYear(), d.getMonth(), d.getDate()); }
  _startOfMonth(d) { return new Date(d.getFullYear(), d.getMonth(), 1); }
  _endOfMonth(d)   { return new Date(d.getFullYear(), d.getMonth() + 1, 0); }
  _addDays(d, n)   { return new Date(d.getFullYear(), d.getMonth(), d.getDate() + n); }
  _diffDays(a, b)  { return Math.round((this._startOfDay(b) - this._startOfDay(a)) / (1000 * 60 * 60 * 24)); }
}
customElements.define('ui-daterange', UIDateRange);
