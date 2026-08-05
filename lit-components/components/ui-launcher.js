// -----------------------------------------------------------------------------
// <ui-launcher> — OS-style command palette / launcher.
//
// UX contract (kept small so beginners get it in 5 seconds):
//   • Cmd+K / Ctrl+K anywhere    → open
//   • click the topbar's ⊞ button → open
//   • type                        → server searches Apps + Commands
//   • ↑ / ↓                       → move highlight
//   • Enter                       → run the highlighted row
//                                    - nav  → navigate the shell to the row's route
//                                    - cmd  → POST /agent { utterance: id } to execute
//                                    - ask  → POST /agent { utterance: raw text } for a chat turn
//   • Esc / click backdrop         → close
//
// Backend does all the ranking / grouping. Client only:
//   1. debounces input
//   2. shows the fragment the server returns
//   3. maps arrow keys / Enter to the selected row's data attributes
//
// Plain HTMLElement (no Lit). Keeps first-paint cost trivial.
// -----------------------------------------------------------------------------

const OPEN_KEY = 'k'; // Cmd+K / Ctrl+K
const DEBOUNCE_MS = 90;
const ENDPOINT = '/launcher/search';

class UiLauncher extends HTMLElement {
  connectedCallback() {
    if (this._mounted) return;
    this._mounted = true;

    this.innerHTML = `
      <style>
        ui-launcher {
          position: fixed; inset: 0;
          z-index: 200;
          display: none;
          align-items: flex-start; justify-content: center;
          padding-top: 12dvh;
          background: rgba(15, 15, 22, .35);
          backdrop-filter: blur(2px);
          font: inherit;
        }
        ui-launcher[open] { display: flex; }
        ui-launcher [data-ul="panel"] {
          width: min(560px, 92vw);
          max-height: 70dvh;
          display: flex; flex-direction: column;
          background: var(--color-bg, #fff);
          color: var(--color-fg, #111);
          border: 1px solid var(--color-border, #e5e7eb);
          border-radius: 12px;
          box-shadow: 0 20px 60px rgba(0, 0, 0, .25);
          overflow: hidden;
        }
        ui-launcher [data-ul="input-wrap"] {
          display: flex; align-items: center; gap: 10px;
          padding: 12px 14px;
          border-bottom: 1px solid var(--color-border, #e5e7eb);
        }
        ui-launcher [data-ul="input"] {
          flex: 1;
          font: inherit; font-size: 15px;
          border: 0; outline: none;
          background: transparent; color: inherit;
        }
        ui-launcher [data-ul="input"]::placeholder { opacity: .5; }
        ui-launcher [data-ul="kbd"] {
          font-size: 11px; opacity: .5;
          padding: 2px 6px; border: 1px solid var(--color-border, #e5e7eb);
          border-radius: 4px;
        }
        ui-launcher [data-ul="results"] {
          overflow-y: auto;
          padding: 6px 0;
        }
        ui-launcher .ui-launcher-section {
          font-size: 10px; font-weight: 600;
          letter-spacing: .08em; opacity: .5;
          padding: 10px 14px 4px;
        }
        ui-launcher .ui-launcher-divider {
          height: 1px; margin: 6px 12px;
          background: var(--color-border, #e5e7eb);
        }
        ui-launcher .ui-launcher-row {
          display: flex; align-items: center; gap: 10px;
          width: 100%;
          padding: 8px 14px;
          background: transparent; color: inherit;
          border: 0; text-align: left;
          font: inherit; font-size: 13px;
          cursor: pointer;
        }
        ui-launcher .ui-launcher-row:hover,
        ui-launcher .ui-launcher-row[aria-selected="true"] {
          background: var(--color-surface-2, #f5f5f7);
        }
        ui-launcher .ui-launcher-label { flex: 1; }
        ui-launcher .ui-launcher-hint  { font-size: 11px; opacity: .5; }
        ui-launcher [data-ul="footer"] {
          padding: 8px 14px;
          border-top: 1px solid var(--color-border, #e5e7eb);
          font-size: 11px; opacity: .55;
          display: flex; justify-content: space-between;
        }
        ui-launcher [data-ul="footer"] kbd {
          padding: 1px 5px; border: 1px solid var(--color-border, #e5e7eb);
          border-radius: 3px; font: inherit; font-size: 10px;
        }
      </style>
      <div data-ul="panel" role="dialog" aria-label="Launcher">
        <div data-ul="input-wrap">
          <ui-icon name="search" size="18"></ui-icon>
          <input data-ul="input" type="text" autocomplete="off" spellcheck="false"
                 placeholder="Search apps, commands, or ask…"
                 aria-label="Launcher search">
          <span data-ul="kbd">Esc</span>
        </div>
        <div data-ul="results" role="listbox"></div>
        <div data-ul="footer">
          <span><kbd>↑</kbd> <kbd>↓</kbd> to navigate</span>
          <span><kbd>Enter</kbd> to run · <kbd>Esc</kbd> to close</span>
        </div>
      </div>
    `;

    this.$panel    = this.querySelector('[data-ul="panel"]');
    this.$input    = this.querySelector('[data-ul="input"]');
    this.$results  = this.querySelector('[data-ul="results"]');

    // Backdrop click (anywhere outside the panel) closes.
    this.addEventListener('click', (e) => {
      if (!this.$panel.contains(e.target)) this.close();
    });

    this.$input.addEventListener('input',   () => this._scheduleSearch());
    this.$input.addEventListener('keydown', (e) => this._onKeydown(e));
    this.$results.addEventListener('click', (e) => {
      const row = e.target.closest('.ui-launcher-row');
      if (row) this._activate(row);
    });

    // Register global shortcut once.
    UiLauncher._installGlobalShortcut(this);
  }

  /** Open + focus + prime results. */
  async open() {
    this.setAttribute('open', '');
    this.$input.value = '';
    await this._search('');
    this.$input.focus();
  }
  close() { this.removeAttribute('open'); }
  toggle() { this.hasAttribute('open') ? this.close() : this.open(); }

  _scheduleSearch() {
    clearTimeout(this._searchTimer);
    this._searchTimer = setTimeout(() => this._search(this.$input.value), DEBOUNCE_MS);
  }

  async _search(q) {
    // The server returns pre-rendered HTML — just swap it in.
    try {
      const url = ENDPOINT + '?q=' + encodeURIComponent(q);
      const res = await fetch(url, { credentials: 'same-origin' });
      const html = await res.text();
      this.$results.innerHTML = html;
      // Auto-highlight the first row.
      this._selectIndex(0);
    } catch (err) {
      console.error('[launcher] search failed:', err);
    }
  }

  _rows() { return [...this.$results.querySelectorAll('.ui-launcher-row')]; }

  _selectIndex(i) {
    const rows = this._rows();
    if (!rows.length) return;
    const clamped = ((i % rows.length) + rows.length) % rows.length;
    rows.forEach((r, idx) => r.setAttribute('aria-selected', idx === clamped ? 'true' : 'false'));
    rows[clamped].scrollIntoView({ block: 'nearest' });
    this._selected = clamped;
  }

  _onKeydown(e) {
    switch (e.key) {
      case 'ArrowDown': e.preventDefault(); this._selectIndex((this._selected ?? -1) + 1); break;
      case 'ArrowUp':   e.preventDefault(); this._selectIndex((this._selected ??  1) - 1); break;
      case 'Enter': {
        e.preventDefault();
        const rows = this._rows();
        const row = rows[this._selected ?? 0];
        if (row) this._activate(row);
        break;
      }
      case 'Escape': e.preventDefault(); this.close(); break;
    }
  }

  /**
   * Dispatch a row. Kind determines behaviour — a small `switch` here
   * keeps the client logic centralised and easy to grep.
   */
  _activate(row) {
    const kind    = row.dataset.kind;
    const payload = row.dataset.payload || '';
    this.close();
    switch (kind) {
      case 'nav':
        if (window.ui && window.ui.navigate) window.ui.navigate(payload);
        else window.location.assign(payload);
        break;
      case 'cmd':
        // Ask the copilot to run the command by id. The server's
        // StubAgent could route by id; for now we just send the id as
        // the utterance and let the registry fuzzy-match handle it.
        // (Wiring the direct-id path is a small server change we can
        // do when the stub agent is refactored to consume the registry.)
        this._postAgent(payload);
        break;
      case 'ask':
        this._postAgent(payload || this.$input.value);
        break;
    }
  }

  /**
   * Send the utterance to the copilot AND open the copilot pane so
   * the user sees the streamed response. If there's no copilot on
   * this page (some app might disable it), we silently POST anyway.
   */
  async _postAgent(utterance) {
    const cop = document.querySelector('ui-copilot');
    if (cop && typeof cop.open === 'function') cop.open();
    // Ask the copilot component to run the turn if it's around — it
    // owns the streaming + rendering.
    if (cop && typeof cop._streamTurn === 'function') {
      cop._pushUserBubble(utterance);
      try { await cop._streamTurn(utterance); } catch (e) { console.error(e); }
    } else {
      // Fallback: fire-and-forget POST if no copilot pane exists yet.
      fetch('/agent', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'Accept': 'text/event-stream' },
        credentials: 'same-origin',
        body: JSON.stringify({ utterance, context: { url: location.href } }),
      });
    }
  }

  // ---------------------------------------------------------------------
  // Global keyboard shortcut — installed once, on the first instance.
  // Cmd+K on Mac, Ctrl+K elsewhere. Works even when focus is inside
  // form fields (that's the whole point of a launcher).
  // ---------------------------------------------------------------------
  static _installGlobalShortcut(instance) {
    if (UiLauncher._shortcutInstalled) return;
    UiLauncher._shortcutInstalled = true;

    document.addEventListener('keydown', (e) => {
      const mod = e.metaKey || e.ctrlKey;
      if (mod && e.key.toLowerCase() === OPEN_KEY) {
        e.preventDefault();
        instance.toggle();
      }
    });

    // Also expose an imperative API on window.ui.
    window.ui = Object.assign(window.ui || {}, {
      openLauncher: () => instance.open(),
    });
  }
}

if (!customElements.get('ui-launcher')) {
  customElements.define('ui-launcher', UiLauncher);
}
