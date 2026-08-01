import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Single-trigger date-range picker with popover calendar + preset shortcuts.
 *
 * <ui-daterange label="Date range" from="2026-07-01" to="2026-07-28"></ui-daterange>
 *
 * Emits `ui-change` with { from, to } (YYYY-MM-DD strings).
 */
class UIDateRange extends LitBaseElement {
  static properties = {
    label: { type: String, reflect: true },
    from:  { type: String, reflect: true },
    to:    { type: String, reflect: true },
    open:  { type: Boolean, reflect: true },
    _viewYear:  { state: true },
    _viewMonth: { state: true },
    _pending:   { state: true },  // Date | null
    _activePreset: { state: true },
  };

  static styles = css`
    :host { display: inline-block; position: relative; box-sizing: border-box; z-index: auto; }
    :host([open]) { z-index: 3000; }
    .label { display: block; font-size: var(--fs-xs); color: var(--color-text-muted); margin-bottom: 6px; font-weight: var(--fw-medium); }
    .trigger {
      display: inline-flex; align-items: center; gap: 8px;
      background: var(--color-surface);
      border: 1px solid var(--color-border-strong);
      border-radius: var(--radius-md);
      height: 36px; padding: 0 12px;
      font: inherit; font-size: var(--fs-sm); color: var(--color-text);
      cursor: pointer; min-width: 240px;
      transition: border-color var(--dur-fast) var(--ease), box-shadow var(--dur-fast) var(--ease);
    }
    .trigger:hover { border-color: var(--color-text-subtle); }
    .trigger:focus { outline: none; box-shadow: 0 0 0 4px var(--color-primary-ring); border-color: var(--color-primary); }
    .trigger ui-icon { color: var(--color-text-muted); }
    .trigger .caret { margin-left: auto; color: var(--color-text-subtle); }

    /* ----- LEFT-SIDE DRAWER -----
       Wider so both calendars sit side-by-side without scroll on a typical
       laptop (1366 × 768 and up). */
    .pop {
      position: fixed;
      top: 0; left: 0; bottom: 0;
      width: min(760px, 96vw);
      background: var(--color-surface);
      border-right: 1px solid var(--color-border);
      box-shadow: var(--shadow-lg);
      z-index: 3001; display: none;
      flex-direction: column;
      transform: translateX(-100%);
      transition: transform var(--dur-med) var(--ease);
      overflow: hidden; max-height: 100dvh;
    }
    :host([open]) .pop { display: flex; transform: translateX(0); }
    /* Two-column body: presets on the left, calendars area on the right. */
    .drawer-body {
      display: grid;
      grid-template-columns: 150px minmax(0, 1fr);
      flex: 1;
      min-height: 0;
      overflow: hidden;
    }
    /* Below ~720px there isn't enough room for both months + presets;
       collapse presets to a chip strip on top. */
    @media (max-width: 720px) {
      .drawer-body { grid-template-columns: 1fr; }
    }
    .drawer-head {
      display: flex; align-items: center; gap: 8px;
      padding: 14px 16px;
      border-bottom: 1px solid var(--color-border);
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
    .cals-wrap {
      min-width: 0; display: flex; flex-direction: column;
      overflow: hidden;         /* container clips */
      background: var(--color-surface);
    }
    .scrim { position: fixed; inset: 0; background: var(--color-scrim); z-index: 3000; display: none; }
    :host([open]) .scrim { display: block; }

    /* Presets are a LEFT column of the drawer body (Google-Calendar style). */
    .presets {
      display: flex; flex-direction: column; gap: 2px;
      padding: 10px 8px;
      border-right: 1px solid var(--color-border);
      background: var(--color-surface-alt);
      overflow-y: auto;
      min-height: 0;
    }
    .presets button {
      appearance: none; border: 0; background: transparent;
      font: inherit; font-size: var(--fs-sm); color: var(--color-text);
      text-align: left; padding: 8px 10px; border-radius: 6px; cursor: pointer;
      white-space: nowrap;
    }
    .presets button:hover { background: color-mix(in srgb, var(--color-text) 6%, transparent); }
    .presets button[aria-pressed="true"] {
      background: var(--color-primary); color: var(--color-primary-contrast);
    }
    @media (max-width: 720px) {
      .presets {
        flex-direction: row; overflow-x: auto; overflow-y: hidden;
        border-right: 0; border-bottom: 1px solid var(--color-border);
        padding: 10px 12px;
      }
      .presets button { flex: 0 0 auto; border-radius: var(--radius-pill); border: 1px solid var(--color-border); background: var(--color-surface); font-size: var(--fs-xs); padding: 5px 10px; }
    }

    /* ONE nav row above BOTH calendars, then calendars side-by-side. */
    .cals-head {
      display: flex; align-items: center; gap: 4px;
      padding: 8px 14px 0;
      flex: 0 0 auto;
    }
    .cals-head .titles {
      flex: 1;
      display: grid; grid-template-columns: 1fr 1fr;
      text-align: center;
      font-size: var(--fs-sm); font-weight: var(--fw-semibold);
      color: var(--color-text);
    }
    .cals {
      padding: 6px 14px 12px;
      display: grid; grid-template-columns: 1fr 1fr; gap: 16px;
      overflow-y: auto;
      flex: 1;
      min-height: 0;
    }
    /* Stack calendars only when it's really tight. */
    @media (max-width: 540px) {
      .cals-head .titles { grid-template-columns: 1fr; }
      .cals              { grid-template-columns: 1fr; }
      .cals-head .titles > span:last-child { display: none; }
    }
    .cal { min-width: 0; }
    .nav-btn {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-text-muted); width: 26px; height: 26px;
      border-radius: 6px; display: grid; place-items: center;
    }
    .nav-btn:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); color: var(--color-text); }

    .dow, .grid { display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px; }
    .dow div { font-size: 10px; color: var(--color-text-subtle); text-align: center; padding: 4px 0; font-weight: var(--fw-medium); }
    .grid button {
      appearance: none; border: 0; background: transparent;
      font: inherit; font-size: var(--fs-xs); color: var(--color-text);
      height: 30px; width: 100%; cursor: pointer; border-radius: 6px;
      display: grid; place-items: center;
      font-variant-numeric: tabular-nums;
    }
    .grid button:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); }
    .grid button.muted { color: var(--color-text-subtle); cursor: default; opacity: 0.5; }
    .grid button.muted:hover { background: transparent; }
    .grid button[disabled] { pointer-events: none; }
    .grid button.today { box-shadow: inset 0 0 0 1px var(--color-primary); color: var(--color-primary); font-weight: var(--fw-semibold); }
    .grid button.in-range { background: var(--color-primary-soft); border-radius: 0; color: var(--color-text); }
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
      padding: 10px 14px; border-top: 1px solid var(--color-border);
      background: var(--color-surface); min-width: 0;
      flex: 0 0 auto;   /* footer stays visible; only .cals scrolls */
    }
    .foot .info {
      font-size: var(--fs-xs); color: var(--color-text-muted);
      font-variant-numeric: tabular-nums; min-width: 0;
      white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }
    .foot .actions { display: flex; gap: 8px; flex-shrink: 0; }

    /* Nothing else — the mobile bottom-sheet layout was replaced by the
       drawer above, so no extra media rules needed here. */
  `;

  constructor() {
    super();
    this.label = '';
    this.from = '';
    this.to = '';
    this.open = false;
    const today = new Date();
    this._viewYear = today.getFullYear();
    this._viewMonth = today.getMonth();
    this._pending = null;
    this._activePreset = '';
    this._boundOutside = (e) => { if (!this.contains(e.target)) this.#close(); };
    this._boundReposition = () => this.#positionPop();
  }

  connectedCallback() {
    super.connectedCallback();
    const f = this.#parse(this.from);
    if (f) { this._viewYear = f.getFullYear(); this._viewMonth = f.getMonth(); }
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener('mousedown', this._boundOutside);
    window.removeEventListener('resize', this._boundReposition);
    window.removeEventListener('scroll', this._boundReposition, true);
  }

  // ── Shared scroll-lock helpers ────────────────────────────────────────
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
    setTimeout(() => document.addEventListener('mousedown', this._boundOutside), 0);
  }
  #close() {
    if (!this.open) return;
    this.open = false;
    this.#unlockScroll();
    document.removeEventListener('mousedown', this._boundOutside);
  }
  // No-op — drawer is CSS-anchored to the left edge.
  #positionPop() {}

  #apply() {
    this.emit('ui-change', { from: this.from, to: this.to });
    this.#close();
  }
  #cancel() { this.#close(); }

  #applyPreset(key) {
    this._activePreset = key;
    const today = new Date();
    const set = (a, b) => { this.from = this.#fmtISO(a); this.to = this.#fmtISO(b); this._pending = null; };
    switch (key) {
      case 'today':      set(this.#startOfDay(today), this.#startOfDay(today)); break;
      case 'yesterday':  { const y = this.#addDays(today, -1); set(y, y); break; }
      case 'last7':      set(this.#addDays(today, -6), today); break;
      case 'last30':     set(this.#addDays(today, -29), today); break;
      case 'thisMonth':  set(this.#startOfMonth(today), this.#endOfMonth(today)); break;
      case 'lastMonth':  { const d = new Date(today.getFullYear(), today.getMonth()-1, 1); set(this.#startOfMonth(d), this.#endOfMonth(d)); break; }
      case 'thisYear':   set(new Date(today.getFullYear(),0,1), new Date(today.getFullYear(),11,31)); break;
      case 'custom':     /* keep current */ break;
    }
    const ref = this.#parse(this.from) || today;
    this._viewYear = ref.getFullYear();
    this._viewMonth = ref.getMonth();
  }

  #onNav(delta) {
    this._viewMonth += delta;
    if (this._viewMonth < 0)  { this._viewMonth = 11; this._viewYear--; }
    if (this._viewMonth > 11) { this._viewMonth = 0;  this._viewYear++; }
  }

  #pickCell(date) {
    if (!this._pending) {
      this.from = this.#fmtISO(date); this.to = this.#fmtISO(date); this._pending = date;
    } else {
      const a = this._pending, b = date;
      if (b < a) { this.from = this.#fmtISO(b); this.to = this.#fmtISO(a); }
      else       { this.from = this.#fmtISO(a); this.to = this.#fmtISO(b); }
      this._pending = null;
    }
    this._activePreset = '';
  }

  #renderMonth(refDate, isLeft) {
    const year  = refDate.getFullYear();
    const month = refDate.getMonth();
    const monthName = refDate.toLocaleString(undefined, { month: 'long', year: 'numeric' });
    const first = new Date(year, month, 1);
    const startDow = (first.getDay() + 6) % 7;
    const daysInMonth = new Date(year, month + 1, 0).getDate();
    const prevMonthDays = new Date(year, month, 0).getDate();

    const cells = [];
    for (let i = startDow - 1; i >= 0; i--)
      cells.push({ d: prevMonthDays - i, muted: true, date: new Date(year, month - 1, prevMonthDays - i) });
    for (let d = 1; d <= daysInMonth; d++)
      cells.push({ d, muted: false, date: new Date(year, month, d) });
    while (cells.length % 7 !== 0 || cells.length < 42) {
      const idx = cells.length - startDow - daysInMonth + 1;
      cells.push({ d: idx, muted: true, date: new Date(year, month + 1, idx) });
    }

    const today = this.#startOfDay(new Date());
    const from = this.#parse(this.from);
    const to = this.#parse(this.to);

    return html`
      <div class="cal">
        <div class="dow"><div>M</div><div>T</div><div>W</div><div>T</div><div>F</div><div>S</div><div>S</div></div>
        <div class="grid">
          ${cells.map(c => {
            const cls = [];
            if (c.muted) cls.push('muted');
            if (!c.muted) {
              if (this.#sameDay(c.date, today)) cls.push('today');
              if (from && to && c.date >= this.#startOfDay(from) && c.date <= this.#startOfDay(to)) cls.push('in-range');
              if (from && this.#sameDay(c.date, from)) cls.push('start');
              if (to   && this.#sameDay(c.date, to))   cls.push('end');
            }
            const disabled = c.muted;
            return html`<button class=${cls.join(' ')} ?disabled=${disabled}
              @click=${() => !disabled && this.#pickCell(c.date)}>${c.d}</button>`;
          })}
        </div>
      </div>
    `;
  }

  render() {
    const leftDate  = new Date(this._viewYear, this._viewMonth, 1);
    const rightDate = new Date(this._viewYear, this._viewMonth + 1, 1);
    const from = this.#parse(this.from);
    const to   = this.#parse(this.to);
    const info = (from && to)
      ? `${this.#fmtISO(from)}  →  ${this.#fmtISO(to)}  ·  ${this.#diffDays(from, to) + 1} day(s)`
      : 'Pick a start date';

    const presets = [
      ['today','Today'], ['yesterday','Yesterday'],
      ['last7','Last 7 days'], ['last30','Last 30 days'],
      ['thisMonth','This month'], ['lastMonth','Last month'],
      ['thisYear','This year'], ['custom','Custom…'],
    ];

    return html`
      ${this.label ? html`<span class="label">${this.label}</span>` : nothing}
      <button class="trigger" type="button" @click=${(e) => {
        e.stopPropagation();
        this.open ? this.#close() : this.#openPop();
      }}>
        <ui-icon name="calendar" size="16"></ui-icon>
        <span>${this.#fmtRange()}</span>
        <ui-icon class="caret" name="chevronDown" size="14"></ui-icon>
      </button>
      <div class="scrim" @click=${() => this.#close()}></div>
      <div class="pop" role="dialog">
        <div class="drawer-head">
          <span class="title">${this.label || 'Select range'}</span>
          <button class="drawer-close" title="Close" @click=${() => this.#close()}>
            <ui-icon name="x" size="16"></ui-icon>
          </button>
        </div>
        <div class="drawer-body">
          <div class="presets">
            ${presets.map(([k, l]) => html`
              <button aria-pressed=${this._activePreset === k}
                      @click=${() => this.#applyPreset(k)}>${l}</button>`)}
          </div>
          <div class="cals-wrap">
            <div class="cals-head">
              <button class="nav-btn" title="Previous month" @click=${() => this.#onNav(-1)}>
                <ui-icon name="chevronLeft" size="14"></ui-icon>
              </button>
              <div class="titles">
                <span>${leftDate.toLocaleString(undefined, { month: 'long', year: 'numeric' })}</span>
                <span>${rightDate.toLocaleString(undefined, { month: 'long', year: 'numeric' })}</span>
              </div>
              <button class="nav-btn" title="Next month" @click=${() => this.#onNav(1)}>
                <ui-icon name="chevronRight" size="14"></ui-icon>
              </button>
            </div>
            <div class="cals">
              ${this.#renderMonth(leftDate, true)}
              ${this.#renderMonth(rightDate, false)}
            </div>
            <div class="foot">
              <span class="info">${info}</span>
              <div class="actions">
                <ui-button size="sm" variant="secondary" @click=${() => this.#cancel()}>Cancel</ui-button>
                <ui-button size="sm" @click=${() => this.#apply()}>Apply</ui-button>
              </div>
            </div>
          </div>
        </div>
      </div>
    `;
  }

  // utils
  #parse(s) { if (!s) return null; const [y,m,d] = s.split('-').map(Number); return new Date(y, m-1, d); }
  #fmtISO(d) { if (!d) return ''; const p = n => String(n).padStart(2,'0'); return `${d.getFullYear()}-${p(d.getMonth()+1)}-${p(d.getDate())}`; }
  #fmtRange() {
    const f = this.#parse(this.from), t = this.#parse(this.to);
    if (!f || !t) return 'Select date range';
    const opts = { month: 'short', day: 'numeric', year: 'numeric' };
    const a = f.toLocaleDateString(undefined, opts);
    const b = t.toLocaleDateString(undefined, opts);
    return this.#sameDay(f, t) ? a : `${a}  →  ${b}`;
  }
  #sameDay(a, b) { return a && b && a.getFullYear()===b.getFullYear() && a.getMonth()===b.getMonth() && a.getDate()===b.getDate(); }
  #startOfDay(d)   { return new Date(d.getFullYear(), d.getMonth(), d.getDate()); }
  #startOfMonth(d) { return new Date(d.getFullYear(), d.getMonth(), 1); }
  #endOfMonth(d)   { return new Date(d.getFullYear(), d.getMonth() + 1, 0); }
  #addDays(d, n)   { return new Date(d.getFullYear(), d.getMonth(), d.getDate() + n); }
  #diffDays(a, b)  { return Math.round((this.#startOfDay(b) - this.#startOfDay(a)) / (1000 * 60 * 60 * 24)); }
}
customElements.define('ui-daterange', UIDateRange);
