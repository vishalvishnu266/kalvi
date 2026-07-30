import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-pagination page="3" total="125" per-page="10"></ui-pagination>
 *
 * DSL surface:
 *   - page       : current page (1-based)
 *   - total      : total number of items
 *   - per-page   : items per page (default 10)
 *   - siblings   : how many page numbers to show either side of current (default 1)
 *
 * Emits `ui-change` with { page, from, to }.
 */
class UIPagination extends LitBaseElement {
  static properties = {
    page:      { type: Number, reflect: true },
    total:     { type: Number, reflect: true },
    'per-page':{ type: Number, reflect: true, attribute: 'per-page' },
    siblings:  { type: Number, reflect: true },
  };

  static styles = css`
    :host {
      display: flex; align-items: center; gap: 4px;
      font-size: var(--fs-sm); color: var(--color-text);
      flex-wrap: wrap;
    }
    .info {
      color: var(--color-text-muted); font-size: var(--fs-xs);
      margin-right: auto;
    }
    button {
      appearance: none; border: 1px solid var(--color-border);
      background: var(--color-surface); color: var(--color-text);
      font: inherit; font-size: var(--fs-sm);
      min-width: 34px; height: 34px; padding: 0 8px;
      border-radius: var(--radius-md); cursor: pointer;
      display: inline-flex; align-items: center; justify-content: center;
      transition: background var(--dur-fast) var(--ease),
                  border-color var(--dur-fast) var(--ease),
                  color var(--dur-fast) var(--ease);
    }
    button:hover:not(:disabled):not([aria-current]) {
      background: color-mix(in srgb, var(--color-text) 6%, transparent);
      border-color: var(--color-text-subtle);
    }
    button:disabled { opacity: 0.4; cursor: not-allowed; }
    button[aria-current="page"] {
      background: var(--color-primary);
      border-color: var(--color-primary);
      color: var(--color-primary-contrast);
      font-weight: var(--fw-semibold);
    }
    .dots { padding: 0 4px; color: var(--color-text-subtle); user-select: none; }
  `;

  constructor() {
    super();
    this.page = 1;
    this.total = 0;
    this['per-page'] = 10;
    this.siblings = 1;
  }

  #pageCount() {
    return Math.max(1, Math.ceil((this.total || 0) / (this['per-page'] || 10)));
  }

  #go(p) {
    const max = this.#pageCount();
    p = Math.max(1, Math.min(max, p));
    if (p === this.page) return;
    this.page = p;
    const from = (p - 1) * this['per-page'] + 1;
    const to   = Math.min(this.total, p * this['per-page']);
    this.emit('ui-change', { page: p, from, to });
  }

  #buildRange() {
    const total = this.#pageCount();
    const cur = this.page;
    const sib = this.siblings;
    const first = 1, last = total;
    const start = Math.max(cur - sib, first + 1);
    const end   = Math.min(cur + sib, last - 1);
    const out = [first];
    if (start > first + 1) out.push('…');
    for (let i = start; i <= end; i++) out.push(i);
    if (end < last - 1) out.push('…');
    if (last !== first) out.push(last);
    return out;
  }

  render() {
    const max = this.#pageCount();
    const from = (this.page - 1) * this['per-page'] + 1;
    const to   = Math.min(this.total, this.page * this['per-page']);
    return html`
      <span class="info">
        ${this.total > 0 ? html`Showing <strong>${from}–${to}</strong> of ${this.total}` : 'No items'}
      </span>
      <button ?disabled=${this.page === 1} title="Previous"
              @click=${() => this.#go(this.page - 1)}>
        <ui-icon name="chevronLeft" size="14"></ui-icon>
      </button>
      ${this.#buildRange().map(p => p === '…'
        ? html`<span class="dots">…</span>`
        : html`
          <button aria-current=${p === this.page ? 'page' : nothing}
                  @click=${() => this.#go(p)}>${p}</button>`)}
      <button ?disabled=${this.page === max} title="Next"
              @click=${() => this.#go(this.page + 1)}>
        <ui-icon name="chevronRight" size="14"></ui-icon>
      </button>
    `;
  }
}
customElements.define('ui-pagination', UIPagination);
