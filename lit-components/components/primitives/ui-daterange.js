import { LitBaseElement, html, css, nothing } from '../base.js';

/**
 * Single-trigger date-range picker with popover calendar + preset shortcuts + custom inputs.
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
    _viewYear:     { state: true },
    _viewMonth:    { state: true },
    _pending:      { state: true },  // Date | null
    _activePreset: { state: true },
    _customFrom:   { state: true },
    _customTo:     { state: true },
    _customError:  { state: true },
  };

  static styles = css`
    :host { display: inline-block; position: relative; box-sizing: border-box; }
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

    /* ----- RESPONSIVE TOP-LAYER DIALOG ----- */
    .pop {
      position: fixed;
      inset: auto 0 0 0;
      width: 100vw;
      max-width: 100vw;
      max-height: 90dvh;
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

    @media (min-width: 720px) {
      .pop {
        inset: 50% auto auto 50%;
        transform: translate(-50%, -40%);
        width: 760px;
        max-width: 92vw;
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

    .drawer-body {
      display: grid;
      grid-template-columns: 160px minmax(0, 1fr);
      flex: 1;
      min-height: 0;
      overflow: hidden;
    }
    @media (max-width: 720px) {
      .drawer-body { grid-template-columns: 1fr; grid-template-rows: auto 1fr; }
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
      overflow: hidden;
      background: var(--color-surface);
    }

    /* Presets column */
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
      .presets button { flex: 0 0 auto; border-radius: var(--radius-pill, 9999px); border: 1px solid var(--color-border); background: var(--color-surface); font-size: var(--fs-xs); padding: 5px 10px; }
    }

    /* Custom Input Controls */
    .custom-inputs {
      display: flex; gap: 8px; align-items: center; padding: 8px 14px;
      border-bottom: 1px solid var(--color-border); background: var(--color-surface-alt);
    }
    .custom-group { display: flex; flex-direction: column; gap: 2px; flex: 1; }
    .custom-group label { font-size: 10px; color: var(--color-text-muted); font-weight: var(--fw-medium); }
    .custom-group input {
      height: 30px; padding: 0 8px; font-size: var(--fs-xs); font-family: inherit;
      border: 1px solid var(--color-border-strong); border-radius: var(--radius-md);
      background: var(--color-surface); color: var(--color-text);
    }
    .custom-group input:focus { outline: none; border-color: var(--color-primary); }
    .custom-error { font-size: 11px; color: var(--color-danger, #e53935); margin-left: 4px; }

    /* Navigation & Month Grids */
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
      height: 32px; width: 100%; cursor: pointer; border-radius: 6px;
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
      flex: 0 0 auto;
    }
    .foot .info {
      font-size: var(--fs-xs); color: var(--color-text-muted);
      font-variant-numeric: tabular-nums; min-width: 0;
      white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }
    .foot .actions { display: flex; gap: 8px; flex-shrink: 0; }
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
    this._customFrom = '';
    this._customTo = '';
    this._customError = '';
  }

  connectedCallback() {
    super.connectedCallback();
    const f = this.#parseStrict(this.from);
    if (f) { this._viewYear = f.getFullYear(); this._viewMonth = f.getMonth(); }
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
          this._customFrom = this.from;
          this._customTo = this.to;
          this._customError = '';
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

  #apply() {
    if (this._activePreset === 'custom') {
      const err = this.#validateCustomRange();
      if (err) return;
    }
    this.emit('ui-change', { from: this.from, to: this.to });
    this.#close();
  }

  #cancel() {
    this.#close();
  }

  #applyPreset(key) {
    this._activePreset = key;
    this._customError = '';
    const today = new Date();
    const set = (a, b) => {
      this.from = this.#fmtISO(a);
      this.to = this.#fmtISO(b);
      this._pending = null;
      this._customFrom = this.from;
      this._customTo = this.to;
    };

    switch (key) {
      case 'today':      set(this.#startOfDay(today), this.#startOfDay(today)); break;
      case 'yesterday':  { const y = this.#addDays(today, -1); set(y, y); break; }
      case 'last7':      set(this.#addDays(today, -6), today); break;
      case 'last30':     set(this.#addDays(today, -29), today); break;
      case 'thisMonth':  set(this.#startOfMonth(today), this.#endOfMonth(today)); break;
      case 'lastMonth':  { const d = new Date(today.getFullYear(), today.getMonth()-1, 1); set(this.#startOfMonth(d), this.#endOfMonth(d)); break; }
      case 'thisYear':   set(new Date(today.getFullYear(),0,1), new Date(today.getFullYear(),11,31)); break;
      case 'custom':
        this._customFrom = this.from;
        this._customTo = this.to;
        this.#validateCustomRange();
        break;
    }

    const ref = this.#parseStrict(this.from) || today;
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
    this._customFrom = this.from;
    this._customTo = this.to;
    this._customError = '';
    this._activePreset = '';
  }

  #onCustomInput(type, val) {
    if (type === 'from') this._customFrom = val;
    if (type === 'to')   this._customTo = val;

    this._activePreset = 'custom';
    const err = this.#validateCustomRange();
    if (!err) {
      this.from = this._customFrom;
      this.to = this._customTo;
      const ref = this.#parseStrict(this.from);
      if (ref) {
        this._viewYear = ref.getFullYear();
        this._viewMonth = ref.getMonth();
      }
    }
  }

  #validateCustomRange() {
    const d1 = this.#parseStrict(this._customFrom);
    const d2 = this.#parseStrict(this._customTo);

    if (!d1 || !d2) {
      this._customError = 'Enter valid dates (YYYY-MM-DD)';
      return this._customError;
    }

    if (d1 > d2) {
      this._customError = '"From" date cannot be after "To" date';
      return this._customError;
    }

    this._customError = '';
    return '';
  }

  #renderMonth(refDate) {
    const year  = refDate.getFullYear();
    const month = refDate.getMonth();
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
    const from = this.#parseStrict(this.from);
    const to = this.#parseStrict(this.to);

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
    const from = this.#parseStrict(this.from);
    const to   = this.#parseStrict(this.to);
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

      <dialog class="pop" @click=${this.#onDialogClick} @cancel=${(e) => { e.preventDefault(); this.#close(); }}>
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
            ${this._activePreset === 'custom'
        ? html`
                <div class="custom-inputs">
                  <div class="custom-group">
                    <label>From</label>
                    <input type="text" 
                           placeholder="YYYY-MM-DD" 
                           .value=${this._customFrom}
                           @input=${(e) => this.#onCustomInput('from', e.target.value)} />
                  </div>
                  <div class="custom-group">
                    <label>To</label>
                    <input type="text" 
                           placeholder="YYYY-MM-DD" 
                           .value=${this._customTo}
                           @input=${(e) => this.#onCustomInput('to', e.target.value)} />
                  </div>
                </div>
                ${this._customError ? html`<div class="custom-error">${this._customError}</div>` : nothing}
              `
        : nothing}
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
              ${this.#renderMonth(leftDate)}
              ${this.#renderMonth(rightDate)}
            </div>
            <div class="foot">
              <span class="info">${info}</span>
              <div class="actions">
                <ui-button size="sm" variant="secondary" @click=${() => this.#cancel()}>Cancel</ui-button>
                <ui-button size="sm" ?disabled=${!!this._customError} @click=${() => this.#apply()}>Apply</ui-button>
              </div>
            </div>
          </div>
        </div>
      </dialog>
    `;
  }

  // Strict Date Parser: Ensures YYYY-MM-DD format and valid calendar dates (e.g. rejects 2025-02-29)
  #parseStrict(s) {
    if (!s || typeof s !== 'string') return null;
    const match = s.trim().match(/^(\d{4})-(\d{2})-(\d{2})$/);
    if (!match) return null;

    const y = Number(match[1]);
    const m = Number(match[2]);
    const d = Number(match[3]);

    if (m < 1 || m > 12 || d < 1 || d > 31) return null;

    const date = new Date(y, m - 1, d);
    if (date.getFullYear() !== y || date.getMonth() !== m - 1 || date.getDate() !== d) {
      return null;
    }
    return date;
  }

  #fmtISO(d) { if (!d) return ''; const p = n => String(n).padStart(2,'0'); return `${d.getFullYear()}-${p(d.getMonth()+1)}-${p(d.getDate())}`; }
  #fmtRange() {
    const f = this.#parseStrict(this.from), t = this.#parseStrict(this.to);
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