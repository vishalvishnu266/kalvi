import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Sortable / filterable data table driven by a JS `.rows` array + `.columns`.
 *
 * <ui-data-table id="t" searchable per-page="10"></ui-data-table>
 *
 * const t = document.getElementById('t');
 * t.columns = [
 *   { key: 'name',   label: 'Name',   sortable: true },
 *   { key: 'grade',  label: 'Grade',  sortable: true, align: 'center' },
 *   { key: 'status', label: 'Status', render: (v) =>
 *     `<ui-badge tone="${v==='present'?'success':'danger'}">${v}</ui-badge>` },
 * ];
 * t.rows = [{ name:'Aarav', grade:5, status:'present' }, …];
 *
 * DSL surface (HTML attributes):
 *   - searchable  : boolean, shows a filter box
 *   - per-page    : number of rows per page (default 10, set 0 to disable)
 *   - selectable  : boolean, adds checkbox column + emits `ui-select-rows`
 *
 * Also emits: `ui-sort` { key, dir }, `ui-page` { page }.
 */
class UIDataTable extends LitBaseElement {
  static properties = {
    searchable:   { type: Boolean, reflect: true },
    selectable:   { type: Boolean, reflect: true },
    'per-page':   { type: Number,  reflect: true, attribute: 'per-page' },
    _q:           { state: true },
    _sortKey:     { state: true },
    _sortDir:     { state: true },
    _page:        { state: true },
    _selected:    { state: true },
    _tick:        { state: true },     // used to force re-render on rows/columns setter
  };

  static styles = css`
    :host {
      display: block;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      overflow: hidden;
    }
    .head {
      display: flex; align-items: center; gap: 12px;
      padding: 10px 14px;
      border-bottom: 1px solid var(--color-border);
      background: var(--color-surface);
    }
    .head .search {
      display: flex; align-items: center; gap: 6px;
      background: var(--color-surface-alt);
      border-radius: var(--radius-md);
      padding: 4px 10px; flex: 1; max-width: 320px;
    }
    .head input {
      flex: 1;
      border: 0; outline: 0; background: transparent;
      font: inherit; font-size: var(--fs-sm); color: var(--color-text);
    }
    .head .count {
      margin-left: auto;
      font-size: var(--fs-xs);
      color: var(--color-text-muted);
    }
    .scroll { overflow-x: auto; }
    table {
      width: 100%; border-collapse: separate; border-spacing: 0;
      font-size: var(--fs-sm);
      min-width: 480px;
    }
    thead th {
      background: var(--color-surface-alt);
      text-align: left; padding: 10px 12px;
      font-weight: var(--fw-semibold); color: var(--color-text-muted);
      font-size: var(--fs-xs); text-transform: uppercase; letter-spacing: .04em;
      border-bottom: 1px solid var(--color-border);
      user-select: none; white-space: nowrap;
    }
    thead th.sort { cursor: pointer; }
    thead th.sort:hover { color: var(--color-text); }
    thead th .arrow {
      display: inline-flex; margin-left: 4px; color: var(--color-text-subtle);
      transition: transform var(--dur-fast) var(--ease), color var(--dur-fast) var(--ease);
    }
    thead th[aria-sort="ascending"]  .arrow { color: var(--color-primary); transform: rotate(180deg); }
    thead th[aria-sort="descending"] .arrow { color: var(--color-primary); }
    tbody td {
      padding: 10px 12px;
      border-bottom: 1px solid var(--color-border);
      color: var(--color-text);
      vertical-align: middle;
    }
    tbody tr:last-child td { border-bottom: 0; }
    tbody tr:hover { background: var(--color-surface-alt); }
    .align-center { text-align: center; }
    .align-right  { text-align: right;  font-variant-numeric: tabular-nums; }

    .empty { padding: 32px 16px; text-align: center; color: var(--color-text-subtle); font-size: var(--fs-sm); }
    .foot {
      padding: 10px 14px;
      border-top: 1px solid var(--color-border);
      display: flex; align-items: center; justify-content: flex-end;
    }
    .foot:empty { display: none; }

    .cbx {
      appearance: none; width: 16px; height: 16px;
      border: 1.5px solid var(--color-border-strong); border-radius: 3px;
      background: var(--color-surface); cursor: pointer; margin: 0;
      display: inline-grid; place-items: center;
    }
    .cbx:checked, .cbx[data-mixed] {
      background: var(--color-primary); border-color: var(--color-primary);
    }
    .cbx:checked::after {
      content: '✓'; font-size: 11px; color: var(--color-primary-contrast); line-height: 1;
    }
    .cbx[data-mixed]::after {
      content: '–'; font-size: 12px; color: var(--color-primary-contrast); line-height: 1;
    }
  `;

  constructor() {
    super();
    this.searchable = false;
    this.selectable = false;
    this['per-page'] = 10;
    this._q = '';
    this._sortKey = '';
    this._sortDir = 'asc';
    this._page = 1;
    this._selected = new Set();
    this._columns = [];
    this._rows = [];
    this._tick = 0;
  }

  set columns(v) { this._columns = Array.isArray(v) ? v : []; this._tick++; }
  get columns() { return this._columns; }
  set rows(v)    { this._rows    = Array.isArray(v) ? v : []; this._page = 1; this._selected.clear(); this._tick++; }
  get rows()    { return this._rows; }

  #filtered() {
    const q = this._q.trim().toLowerCase();
    let out = q
      ? this._rows.filter(r =>
          this._columns.some(c => String(r[c.key] ?? '').toLowerCase().includes(q)))
      : [...this._rows];
    if (this._sortKey) {
      const dir = this._sortDir === 'desc' ? -1 : 1;
      out.sort((a, b) => {
        const av = a[this._sortKey], bv = b[this._sortKey];
        if (av == null) return  1;
        if (bv == null) return -1;
        if (typeof av === 'number' && typeof bv === 'number') return (av - bv) * dir;
        return String(av).localeCompare(String(bv)) * dir;
      });
    }
    return out;
  }

  #paged(rows) {
    const per = this['per-page'] || 0;
    if (!per) return rows;
    const start = (this._page - 1) * per;
    return rows.slice(start, start + per);
  }

  #sortBy(col) {
    if (!col.sortable) return;
    if (this._sortKey === col.key) {
      this._sortDir = this._sortDir === 'asc' ? 'desc' : 'asc';
    } else {
      this._sortKey = col.key; this._sortDir = 'asc';
    }
    this.emit('ui-sort', { key: this._sortKey, dir: this._sortDir });
  }

  #onPage(e) {
    this._page = e.detail.page;
    this.emit('ui-page', { page: this._page });
  }

  #toggleAll(rows, checked) {
    if (checked) rows.forEach((_, i) => this._selected.add(i));
    else rows.forEach((_, i) => this._selected.delete(i));
    this._selected = new Set(this._selected);
    this.emit('ui-select-rows', { rows: [...this._selected].map(i => rows[i]) });
  }
  #toggleOne(idx, rows) {
    this._selected.has(idx) ? this._selected.delete(idx) : this._selected.add(idx);
    this._selected = new Set(this._selected);
    this.emit('ui-select-rows', { rows: [...this._selected].map(i => rows[i]) });
  }

  render() {
    const filtered = this.#filtered();
    const paged = this.#paged(filtered);
    const cols = this._columns;

    return html`
      <div class="head">
        ${this.searchable
          ? html`<label class="search">
              <ui-icon name="search" size="14"></ui-icon>
              <input placeholder="Filter…" .value=${this._q}
                     @input=${(e) => { this._q = e.target.value; this._page = 1; }}>
            </label>`
          : nothing}
        <span class="count">${filtered.length} row${filtered.length === 1 ? '' : 's'}</span>
      </div>
      <div class="scroll">
        <table>
          <thead>
            <tr>
              ${this.selectable
                ? html`<th style="width: 32px;">
                    <input type="checkbox" class="cbx"
                      .checked=${paged.length > 0 && paged.every((_, i) =>
                        this._selected.has((this._page - 1) * (this['per-page'] || paged.length) + i))}
                      @change=${(e) => this.#toggleAll(this._rows, e.target.checked)}>
                  </th>`
                : nothing}
              ${cols.map(c => html`
                <th class=${['sort', c.align ? 'align-' + c.align : ''].filter(Boolean).join(' ')}
                    aria-sort=${this._sortKey === c.key ? (this._sortDir === 'asc' ? 'ascending' : 'descending') : nothing}
                    @click=${() => this.#sortBy(c)}>
                  ${c.label}
                  ${c.sortable
                    ? html`<span class="arrow"><ui-icon name="chevronDown" size="12"></ui-icon></span>`
                    : nothing}
                </th>`)}
            </tr>
          </thead>
          <tbody>
            ${paged.length === 0
              ? html`<tr><td class="empty" colspan=${cols.length + (this.selectable ? 1 : 0)}>No matching rows</td></tr>`
              : paged.map((r, i) => {
                  const rowIdx = this._rows.indexOf(r);
                  return html`<tr>
                    ${this.selectable
                      ? html`<td>
                          <input type="checkbox" class="cbx"
                            .checked=${this._selected.has(rowIdx)}
                            @change=${() => this.#toggleOne(rowIdx, this._rows)}>
                        </td>`
                      : nothing}
                    ${cols.map(c => {
                      const raw = r[c.key];
                      const align = c.align ? 'align-' + c.align : '';
                      if (typeof c.render === 'function') {
                        const el = document.createElement('div');
                        el.innerHTML = c.render(raw, r);
                        return html`<td class=${align}>${el}</td>`;
                      }
                      return html`<td class=${align}>${raw ?? ''}</td>`;
                    })}
                  </tr>`;
                })}
          </tbody>
        </table>
      </div>
      <div class="foot">
        ${(this['per-page'] || 0) > 0 && filtered.length > this['per-page']
          ? html`<ui-pagination
                    page=${this._page}
                    total=${filtered.length}
                    per-page=${this['per-page']}
                    @ui-change=${(e) => this.#onPage(e)}></ui-pagination>`
          : nothing}
      </div>
    `;
  }
}
customElements.define('ui-data-table', UIDataTable);
