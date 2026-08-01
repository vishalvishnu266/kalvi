import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Single-date picker with a responsive popover (Bottom-sheet on mobile, Centered Modal on desktop).
 *
 * <ui-datepicker label="Date of birth" value="2015-08-12"></ui-datepicker>
 *
 * Emits `ui-change` with { value } in ISO (YYYY-MM-DD).
 */
class UIDatepicker extends LitBaseElement {
  static properties = {
    label:       { type: String, reflect: true },
    value:       { type: String, reflect: true },
    placeholder: { type: String, reflect: true },
    open:        { type: Boolean, reflect: true },
    _viewYear:   { state: true },
    _viewMonth:  { state: true },
    _mode:       { state: true },   // "day" (default) | "year"
    _yearPage:   { state: true },   // top-left year of the 4x3 year grid
  };

  static styles = css`
    :host { display: inline-block; position: relative; box-sizing: border-box; }
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
    .trigger:focus { outline: none; box-shadow: 0 0 0 4px var(--color-primary-ring); border-color: var(--color-primary); }
    .trigger ui-icon { color: var(--color-text-muted); }
    .trigger .val { flex: 1; }
    .placeholder { color: var(--color-text-subtle); }

    /* ----- RESPONSIVE TOP-LAYER DIALOG -----
       Guaranteed no-overlap via native <dialog.showModal()>.
       Mobile: Bottom sheet sliding up.
       Desktop: Centered modal on screen. */
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

    /* Desktop Viewport Adaptation (Centered Modal sliding up slightly) */
    @media (min-width: 640px) {
      .pop {
        inset: 50% auto auto 50%;
        transform: translate(-50%, -40%);
        width: 360px;
        max-width: 90vw;
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
    .drawer-body { padding: 12px 16px; overflow-y: auto; flex: 1; }

    .head { display: flex; align-items: center; gap: 4px; margin-bottom: 8px; }
    .head .month {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      font: inherit; font-weight: var(--fw-semibold); font-size: var(--fs-sm);
      color: var(--color-text);
      padding: 4px 8px; border-radius: 6px;
      display: inline-flex; align-items: center; gap: 4px;
      flex: 1; justify-content: center;
    }
    .head .month:hover { background: color-mix(in srgb, var(--color-text) 6%, transparent); }
    .nav-btn {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-text-muted); width: 26px; height: 26px;
      border-radius: 6px; display: grid; place-items: center;
      flex: 0 0 auto;
    }
    .nav-btn:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); color: var(--color-text); }

    .years {
      display: grid; grid-template-columns: repeat(4, 1fr); gap: 4px;
      padding: 4px 0;
    }
    .years button {
      appearance: none; border: 0; background: transparent;
      font: inherit; font-size: var(--fs-xs); color: var(--color-text);
      height: 36px; cursor: pointer; border-radius: 6px;
      font-variant-numeric: tabular-nums;
    }
    .years button:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); }
    .years button.sel {
      background: var(--color-primary); color: var(--color-primary-contrast);
      font-weight: var(--fw-semibold);
    }

    .dow, .grid { display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px; }
    .dow div {
      font-size: 10px; color: var(--color-text-subtle); text-align: center;
      padding: 4px 0; font-weight: var(--fw-medium);
    }
    .grid button {
      appearance: none; border: 0; background: transparent;
      font: inherit; font-size: var(--fs-xs); color: var(--color-text);
      height: 36px; cursor: pointer; border-radius: 6px;
      display: grid; place-items: center;
      font-variant-numeric: tabular-nums;
    }
    .grid button:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); }
    .grid button.muted { color: var(--color-text-subtle); }
    .grid button.today {
      box-shadow: inset 0 0 0 1px var(--color-primary);
      color: var(--color-primary); font-weight: var(--fw-semibold);
    }
    .grid button.sel {
      background: var(--color-primary); color: var(--color-primary-contrast);
      font-weight: var(--fw-semibold);
    }

    .foot { margin-top: 10px; display: flex; justify-content: space-between; gap: 8px; }
    .foot button {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-primary); font: inherit;
      font-size: var(--fs-xs); font-weight: var(--fw-medium);
    }
  `;

  constructor() {
    super();
    this.label = '';
    this.value = '';
    this.placeholder = 'Select a date';
    this.open = false;
    const today = new Date();
    this._viewYear = today.getFullYear();
    this._viewMonth = today.getMonth();
    this._mode = 'day';
    this._yearPage = this._viewYear - (this._viewYear % 12);
  }

  connectedCallback() {
    super.connectedCallback();
    const v = this.#parse(this.value);
    if (v) { this._viewYear = v.getFullYear(); this._viewMonth = v.getMonth(); }
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

  #onNav(delta) {
    if (this._mode === 'year') {
      this._yearPage += delta * 12;
      return;
    }
    this._viewMonth += delta;
    if (this._viewMonth < 0)  { this._viewMonth = 11; this._viewYear--; }
    if (this._viewMonth > 11) { this._viewMonth = 0;  this._viewYear++; }
  }

  #openYearPicker() {
    this._mode = 'year';
    this._yearPage = this._viewYear - (this._viewYear % 12);
  }

  #pickYear(y) {
    this._viewYear = y;
    this._mode = 'day';
  }

  #pick(iso) {
    this.value = iso;
    this.emit('ui-change', { value: iso });
    this.#close();
  }

  #today() {
    this.value = this.#fmtISO(this.#startOfDay(new Date()));
    this.emit('ui-change', { value: this.value });
    this.#close();
  }

  #clear() {
    this.value = '';
    this.emit('ui-change', { value: '' });
    this.#close();
  }

  #buildCells() {
    const y = this._viewYear, m = this._viewMonth;
    const first = new Date(y, m, 1);
    const startDow = (first.getDay() + 6) % 7;
    const daysInMonth = new Date(y, m + 1, 0).getDate();
    const prev = new Date(y, m, 0).getDate();
    const cells = [];
    for (let i = startDow - 1; i >= 0; i--)
      cells.push({ d: prev - i, muted: true, date: new Date(y, m - 1, prev - i) });
    for (let d = 1; d <= daysInMonth; d++)
      cells.push({ d, muted: false, date: new Date(y, m, d) });
    while (cells.length < 42) {
      const idx = cells.length - startDow - daysInMonth + 1;
      cells.push({ d: idx, muted: true, date: new Date(y, m + 1, idx) });
    }
    return cells;
  }

  render() {
    const val = this.#parse(this.value);
    const today = this.#startOfDay(new Date());
    const monthName = new Date(this._viewYear, this._viewMonth, 1)
        .toLocaleString(undefined, { month: 'long', year: 'numeric' });
    const cells = this.#buildCells();

    return html`
      ${this.label ? html`<span class="label">${this.label}</span>` : nothing}
      <button class="trigger" type="button" @click=${(e) => {
        e.stopPropagation();
        this.open ? this.#close() : this.#openPop();
      }}>
        <ui-icon name="calendar" size="16"></ui-icon>
        <span class="val">
          ${val
              ? val.toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' })
              : html`<span class="placeholder">${this.placeholder}</span>`}
        </span>
        <ui-icon name="chevronDown" size="14"></ui-icon>
      </button>

      <dialog class="pop" @click=${this.#onDialogClick} @cancel=${(e) => { e.preventDefault(); this.#close(); }}>
        <div class="drawer-head">
          <span class="title">${this.label || 'Select date'}</span>
          <button class="drawer-close" title="Close" @click=${() => this.#close()}>
            <ui-icon name="x" size="16"></ui-icon>
          </button>
        </div>
        <div class="drawer-body">
          <div class="head">
            <button class="nav-btn" title=${this._mode === 'year' ? 'Previous 12 years' : 'Previous month'}
                    @click=${() => this.#onNav(-1)}>
              <ui-icon name="chevronLeft" size="14"></ui-icon>
            </button>
            ${this._mode === 'year'
                ? html`<span class="month" style="cursor:default">
                       ${this._yearPage} – ${this._yearPage + 11}
                     </span>`
                : html`<button class="month" title="Pick a year"
                               @click=${() => this.#openYearPicker()}>
                  ${monthName}
                  <ui-icon name="chevronDown" size="12"></ui-icon>
                </button>`}
            <button class="nav-btn" title=${this._mode === 'year' ? 'Next 12 years' : 'Next month'}
                    @click=${() => this.#onNav(1)}>
              <ui-icon name="chevronRight" size="14"></ui-icon>
            </button>
          </div>

          ${this._mode === 'year'
              ? html`
                <div class="years">
                  ${Array.from({length: 12}, (_, i) => this._yearPage + i).map(y => html`
                    <button class=${y === this._viewYear ? 'sel' : ''}
                            @click=${() => this.#pickYear(y)}>${y}</button>
                  `)}
                </div>`
              : html`
                <div class="dow"><div>M</div><div>T</div><div>W</div><div>T</div><div>F</div><div>S</div><div>S</div></div>
                <div class="grid">
                  ${cells.map(c => {
                    const cls = [];
                    if (c.muted) cls.push('muted');
                    if (this.#sameDay(c.date, today)) cls.push('today');
                    if (val && this.#sameDay(c.date, val)) cls.push('sel');
                    const iso = this.#fmtISO(c.date);
                    return html`<button class=${cls.join(' ')} @click=${() => this.#pick(iso)}>${c.d}</button>`;
                  })}
                </div>`}

          <div class="foot">
            <button @click=${() => this.#today()}>Today</button>
            <button @click=${() => this.#clear()}>Clear</button>
          </div>
        </div>
      </dialog>
    `;
  }

  #parse(s) { if (!s) return null; const [y,m,d] = s.split('-').map(Number); return new Date(y, m-1, d); }
  #fmtISO(d) { const p = n => String(n).padStart(2,'0'); return `${d.getFullYear()}-${p(d.getMonth()+1)}-${p(d.getDate())}`; }
  #sameDay(a,b) { return a && b && a.getFullYear()===b.getFullYear() && a.getMonth()===b.getMonth() && a.getDate()===b.getDate(); }
  #startOfDay(d) { return new Date(d.getFullYear(), d.getMonth(), d.getDate()); }
}
customElements.define('ui-datepicker', UIDatepicker);