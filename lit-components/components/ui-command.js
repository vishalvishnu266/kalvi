import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * ⌘K / Ctrl+K command palette (Slack / Linear / Notion style).
 *
 * <ui-command placeholder="Search or run a command…">
 *   <ui-command-item href="students.html" icon="users" group="Navigate">Students</ui-command-item>
 *   <ui-command-item href="fees.html"     icon="card"  group="Navigate">Fees</ui-command-item>
 *   <ui-command-item action="new-student" icon="plus"  group="Actions">Add student</ui-command-item>
 * </ui-command>
 *
 * Opens on ⌘K / Ctrl+K (or programmatically via .open()).
 * Emits `ui-command` with { action, href, label } when an item is chosen.
 */
class UICommand extends LitBaseElement {
  static properties = {
    placeholder: { type: String,  reflect: true },
    open:        { type: Boolean, reflect: true },
    _q:          { state: true },
    _idx:        { state: true },
  };

  static styles = css`
    :host { position: fixed; inset: 0; display: none; z-index: 5000; }
    :host([open]) { display: block; }
    .scrim {
      position: absolute; inset: 0;
      background: var(--color-scrim);
      backdrop-filter: blur(2px);
    }
    .panel {
      position: absolute; top: 12%; left: 50%; transform: translateX(-50%);
      width: min(620px, 92vw);
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-xl);
      box-shadow: var(--shadow-lg);
      overflow: hidden;
      display: flex; flex-direction: column;
      max-height: 76vh;
    }
    .search {
      display: flex; align-items: center; gap: 10px;
      padding: 14px 16px;
      border-bottom: 1px solid var(--color-border);
    }
    .search ui-icon { color: var(--color-text-muted); }
    .search input {
      flex: 1;
      border: 0; outline: 0; background: transparent;
      font: inherit; font-size: var(--fs-md);
      color: var(--color-text);
    }
    .search kbd {
      font: 11px var(--font-mono, ui-monospace);
      background: var(--color-surface-alt);
      border: 1px solid var(--color-border);
      border-radius: 4px; padding: 1px 5px;
      color: var(--color-text-muted);
    }
    .list { overflow-y: auto; padding: 6px; flex: 1; }
    .group {
      font-size: 10px; text-transform: uppercase; letter-spacing: 0.06em;
      color: var(--color-text-subtle); font-weight: var(--fw-semibold);
      padding: 10px 10px 4px;
    }
    .item {
      display: flex; align-items: center; gap: 10px;
      padding: 9px 10px; border-radius: 6px; cursor: pointer;
      color: var(--color-text); font-size: var(--fs-sm);
    }
    .item .icon {
      color: var(--color-text-muted);
      display: inline-flex;
    }
    .item[data-active], .item:hover {
      background: var(--color-primary-soft); color: var(--color-primary);
    }
    .item[data-active] .icon, .item:hover .icon { color: var(--color-primary); }
    .item .hint {
      margin-left: auto; font-size: 11px; color: var(--color-text-subtle);
      font-family: var(--font-mono, ui-monospace);
    }
    .empty { padding: 32px; text-align: center; color: var(--color-text-subtle); font-size: var(--fs-sm); }
  `;

  constructor() {
    super();
    this.placeholder = 'Search or run a command…';
    this.open = false;
    this._q = '';
    this._idx = 0;
    this._items = [];
    this._boundKey = (e) => this.#globalKey(e);
  }

  connectedCallback() {
    super.connectedCallback();
    document.addEventListener('keydown', this._boundKey);
    this._mo = new MutationObserver(() => this.#rebuild());
    this._mo.observe(this, { childList: true, subtree: true });
    queueMicrotask(() => this.#rebuild());
  }
  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener('keydown', this._boundKey);
    this._mo?.disconnect();
  }

  #rebuild() {
    this._items = [...this.querySelectorAll('ui-command-item')].map(el => ({
      label: (el.textContent || '').trim(),
      icon:  el.getAttribute('icon')  || '',
      href:  el.getAttribute('href')  || '',
      action:el.getAttribute('action')|| '',
      group: el.getAttribute('group') || '',
      el,
    }));
    // Hide light-DOM items; we render our own list.
    this._items.forEach(i => { i.el.style.display = 'none'; });
    this.requestUpdate();
  }

  #globalKey(e) {
    const mod = e.metaKey || e.ctrlKey;
    if (mod && e.key.toLowerCase() === 'k') {
      e.preventDefault();
      this.open ? this.close() : this.openPalette();
      return;
    }
    if (!this.open) return;
    if (e.key === 'Escape')      { e.preventDefault(); this.close(); }
    else if (e.key === 'ArrowDown') { e.preventDefault(); this.#moveIdx(1); }
    else if (e.key === 'ArrowUp')   { e.preventDefault(); this.#moveIdx(-1); }
    else if (e.key === 'Enter')     {
      e.preventDefault();
      const list = this.#filter();
      const item = list[this._idx];
      if (item) this.#choose(item);
    }
  }

  #moveIdx(delta) {
    const list = this.#filter();
    if (!list.length) return;
    this._idx = (this._idx + delta + list.length) % list.length;
  }

  #filter() {
    const q = this._q.trim().toLowerCase();
    return q ? this._items.filter(i => i.label.toLowerCase().includes(q)) : this._items;
  }

  #choose(item) {
    this.emit('ui-command', { action: item.action, href: item.href, label: item.label });
    if (item.href) window.location.href = item.href;
    this.close();
  }

  openPalette() {
    this.open = true;
    this._q = ''; this._idx = 0;
    this.updateComplete.then(() => this.$('input')?.focus());
  }
  close() { this.open = false; }

  render() {
    const list = this.#filter();
    // Group items by group label, preserving order.
    const groups = [];
    for (const i of list) {
      let g = groups.find(x => x.name === i.group);
      if (!g) { g = { name: i.group, items: [] }; groups.push(g); }
      g.items.push(i);
    }
    let cursor = 0;
    return html`
      <div class="scrim" @click=${() => this.close()}></div>
      <div class="panel" role="dialog">
        <div class="search">
          <ui-icon name="search" size="16"></ui-icon>
          <input placeholder=${this.placeholder}
                 .value=${this._q}
                 @input=${(e) => { this._q = e.target.value; this._idx = 0; }}>
          <kbd>Esc</kbd>
        </div>
        <div class="list">
          ${list.length === 0
            ? html`<div class="empty">No results for "${this._q}"</div>`
            : groups.map(g => html`
              ${g.name ? html`<div class="group">${g.name}</div>` : nothing}
              ${g.items.map(item => {
                const idx = cursor++;
                return html`<div class="item"
                     data-active=${idx === this._idx ? '' : nothing}
                     @mousemove=${() => this._idx = idx}
                     @click=${() => this.#choose(item)}>
                  ${item.icon ? html`<span class="icon"><ui-icon name=${item.icon} size="16"></ui-icon></span>` : nothing}
                  <span>${item.label}</span>
                  <span class="hint">${item.href ? '↵ go' : (item.action ? '↵ run' : '')}</span>
                </div>`;
              })}
            `)}
        </div>
      </div>
    `;
  }
}
customElements.define('ui-command', UICommand);

class UICommandItem extends HTMLElement {
  /* Data-only element — the parent <ui-command> reads its attrs & text. */
}
customElements.define('ui-command-item', UICommandItem);
