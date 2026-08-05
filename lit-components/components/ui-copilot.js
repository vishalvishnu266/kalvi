// -----------------------------------------------------------------------------
// <ui-copilot> — the lean, IDE-style copilot pane.
//
// UX contract:
//   • Desktop (≥768px): occupies the shell's `copilot` slot on the RIGHT
//     side. Toggleable via `open` attribute or floating "Copilot" button.
//   • Mobile (<768px): slides up from the BOTTOM as a sheet, covering
//     ~75% of the viewport. Toggled via a fixed round FAB at the corner.
//
// Wire contract:
//   • Composer POSTs JSON `{utterance, context}` to /agent.
//   • Server responds with an SSE stream. Each `event: fragment` payload
//     is a `<ui-fragment target="…" action="…">…</ui-fragment>` envelope.
//   • We hand each envelope to `window.ui.applyAll(...)` so the exact
//     same code path applies agent-authored swaps and normal navigation.
//   • Chat bubbles are appended to the special `copilot-chat` island
//     which lives inside this component's light DOM.
//
// This is intentionally NOT a Lit element. Plain HTMLElement keeps the
// bundle small and side-steps the shadow-DOM slot boundary that would
// otherwise force us to re-implement <ui-fragment target="copilot-chat">
// lookup logic. Chat bubbles land in a light-DOM child.
// -----------------------------------------------------------------------------

class UiCopilot extends HTMLElement {
  connectedCallback() {
    if (this._mounted) return;
    this._mounted = true;

    // Endpoint is configurable so a subclass or app can point it at a
    // different backend (e.g. /agent/v2) without editing this file.
    this.endpoint = this.getAttribute('endpoint') || '/agent';

    // Build the internal DOM once. Everything is light DOM so the
    // fragment-applier can find [slot="copilot-chat"] with the same
    // selector strategy it uses for every other island.
    this.innerHTML = `
      <style>
        ui-copilot {
          --uc-bg:      var(--color-bg, #fff);
          --uc-fg:      var(--color-fg, #111);
          --uc-border:  var(--color-border, #e5e7eb);
          --uc-accent:  var(--color-primary, #4f46e5);
          --uc-radius:  14px;

          display: flex;
          flex-direction: column;
          height: 100%;
          background: var(--uc-bg);
          color: var(--uc-fg);
          font: inherit;
        }
        /* Desktop: always visible inside the shell's copilot slot. */
        /* Mobile: hidden internals when the sheet is closed so tabbing
           doesn't reach them through the fully-translated container. */
        @media (max-width: 768px) {
          ui-copilot:not([open]) [data-uc="header"],
          ui-copilot:not([open]) [data-uc="chat"],
          ui-copilot:not([open]) [data-uc="composer"] {
            visibility: hidden;
          }
        }
        /* The floating action button (mobile only). Lives OUTSIDE the
           copilot's flex column so its position isn't affected by the
           sheet's transform. Injected into <body> at connect time — see
           _installFab(). */
        .ui-copilot-fab {
          position: fixed;
          right: 16px; bottom: 16px;
          width: 52px; height: 52px;
          border-radius: 50%;
          background: var(--color-primary, #4f46e5);
          color: #fff;
          border: 0;
          font-size: 22px;
          cursor: pointer;
          box-shadow: 0 4px 14px rgba(0, 0, 0, .18);
          z-index: 95;
          display: none;
        }
        @media (max-width: 768px) {
          .ui-copilot-fab { display: grid; place-items: center; }
        }
        ui-copilot [data-uc="header"] {
          display: flex; align-items: center; justify-content: space-between;
          padding: 12px 14px;
          border-bottom: 1px solid var(--uc-border);
          font-weight: 600; font-size: 13px;
        }
        ui-copilot [data-uc="close"] {
          background: none; border: 0; font-size: 18px; cursor: pointer;
          color: inherit; opacity: .6; line-height: 1; padding: 4px;
        }
        ui-copilot [data-uc="close"]:hover { opacity: 1; }
        ui-copilot [data-uc="chat"] {
          flex: 1; overflow-y: auto;
          padding: 10px 14px;
          display: flex; flex-direction: column;
        }
        ui-copilot [data-uc="chat"]:empty::before {
          content: 'Ask me to navigate, add a user, toggle dark mode, …';
          opacity: .5; font-size: 12px; padding: 8px 0;
        }
        ui-copilot [data-uc="composer"] {
          display: flex; gap: 8px;
          padding: 10px 12px;
          border-top: 1px solid var(--uc-border);
        }
        ui-copilot [data-uc="input"] {
          flex: 1;
          padding: 8px 10px;
          font: inherit; font-size: 13px;
          border: 1px solid var(--uc-border);
          border-radius: 8px;
          background: var(--color-surface-2, #f5f5f7);
          color: inherit;
          outline: none;
        }
        ui-copilot [data-uc="input"]:focus {
          border-color: var(--uc-accent);
          background: var(--uc-bg);
        }
        ui-copilot [data-uc="send"] {
          padding: 8px 14px;
          font: inherit; font-size: 13px; font-weight: 500;
          border: 0; border-radius: 8px;
          background: var(--uc-accent); color: #fff;
          cursor: pointer;
        }
        ui-copilot [data-uc="send"]:disabled { opacity: .5; cursor: default; }
        ui-copilot [data-uc="you"] {
          align-self: flex-end;
          max-width: 85%;
          padding: 8px 12px; margin: 4px 0;
          background: var(--uc-accent); color: #fff;
          border-radius: 12px 12px 2px 12px;
          font-size: 13px; line-height: 1.4;
        }
      </style>
      <div data-uc="header">
        <span>🤖 Copilot</span>
        <button type="button" data-uc="close" aria-label="Close copilot">×</button>
      </div>
      <div slot="copilot-chat" data-uc="chat" aria-live="polite"></div>
      <form data-uc="composer" autocomplete="off">
        <input data-uc="input" type="text" name="utterance"
               placeholder="Try: go to dashboard, add user, dark mode …"
               aria-label="Message the copilot">
        <button data-uc="send" type="submit">Send</button>
      </form>
    `;

    this.$chat     = this.querySelector('[data-uc="chat"]');
    this.$input    = this.querySelector('[data-uc="input"]');
    this.$send     = this.querySelector('[data-uc="send"]');
    this.$composer = this.querySelector('[data-uc="composer"]');
    this.$close    = this.querySelector('[data-uc="close"]');

    this.$composer.addEventListener('submit', (e) => this._onSubmit(e));
    this.$close.addEventListener('click', () => this.close());

    // Install the mobile FAB once (idempotent). Lives on the document
    // body so it isn't affected by the sheet's transform.
    this._installFab();
  }

  /**
   * Add a floating "🤖" button to <body> on first connect. Clicking it
   * opens this copilot. If the button already exists (e.g. multiple
   * <ui-copilot> instances) the newest one wins the toggle target —
   * fine for our single-shell architecture.
   */
  _installFab() {
    let fab = document.querySelector('.ui-copilot-fab');
    if (!fab) {
      fab = document.createElement('button');
      fab.type = 'button';
      fab.className = 'ui-copilot-fab';
      fab.setAttribute('aria-label', 'Open copilot');
      fab.textContent = '🤖';
      document.body.appendChild(fab);
    }
    fab.onclick = () => this.toggle();
  }

  /** Open/close helpers — driven by the mobile FAB + close button. */
  open()   { this.setAttribute('open', ''); }
  close()  { this.removeAttribute('open'); }
  toggle() { this.hasAttribute('open') ? this.close() : this.open(); }

  async _onSubmit(e) {
    e.preventDefault();
    const utterance = this.$input.value.trim();
    if (!utterance) return;

    // Optimistic echo — user's message appears immediately, no round-trip.
    this._pushUserBubble(utterance);
    this.$input.value = '';
    this.$send.disabled = true;

    try {
      await this._streamTurn(utterance);
    } catch (err) {
      console.error('[copilot] turn failed:', err);
      this._pushSystemBubble(`Error: ${err.message || err}`);
    } finally {
      this.$send.disabled = false;
      this.$input.focus();
    }
  }

  /**
   * POST the utterance and stream the fragment envelopes back via the
   * Fetch Streams API. We could use EventSource, but it forces GET and
   * doesn't let us set headers cleanly.
   */
  async _streamTurn(utterance) {
    const res = await fetch(this.endpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'text/event-stream',
      },
      body: JSON.stringify({
        utterance,
        context: { url: location.href },
      }),
      credentials: 'same-origin',
    });
    if (!res.ok || !res.body) throw new Error(`HTTP ${res.status}`);

    const reader = res.body.getReader();
    const decoder = new TextDecoder();
    let buf = '';

    while (true) {
      const { value, done } = await reader.read();
      if (done) break;
      buf += decoder.decode(value, { stream: true });
      // SSE frames are separated by a blank line. Parse whichever
      // complete frames are in the buffer, keep the tail for next read.
      let idx;
      while ((idx = buf.indexOf('\n\n')) !== -1) {
        const raw = buf.slice(0, idx);
        buf = buf.slice(idx + 2);
        this._handleSseFrame(raw);
      }
    }
  }

  /** Parse a single SSE frame ("event: X\ndata: Y") and dispatch it. */
  _handleSseFrame(frame) {
    let event = 'message';
    let data  = '';
    for (const line of frame.split('\n')) {
      if (line.startsWith('event:')) event = line.slice(6).trim();
      else if (line.startsWith('data:')) {
        // Multi-line `data:` fields are joined with '\n'.
        data += (data ? '\n' : '') + line.slice(5).trim();
      }
    }
    if (event === 'done') return;
    if (event !== 'fragment' || !data) return;

    // Apply exactly like a normal navigation payload — reuses the
    // shell's `<ui-fragment>` upgrader.
    const tpl = document.createElement('template');
    tpl.innerHTML = data;
    if (window.ui && typeof window.ui.applyAll === 'function') {
      window.ui.applyAll(tpl.content);
    } else {
      // Fallback if shell.js hasn't booted for any reason: append raw.
      this.$chat.append(...tpl.content.childNodes);
    }
    this._scrollChat();
  }

  _pushUserBubble(text) {
    const el = document.createElement('div');
    el.dataset.uc = 'you';
    el.textContent = text;
    this.$chat.appendChild(el);
    this._scrollChat();
  }
  _pushSystemBubble(text) {
    const el = document.createElement('div');
    el.className = 'ui-copilot-bubble';
    el.style.cssText = 'padding:10px 12px;margin:4px 0;background:#fee2e2;color:#991b1b;border-radius:10px;font-size:13px;';
    el.textContent = text;
    this.$chat.appendChild(el);
    this._scrollChat();
  }
  _scrollChat() {
    // rAF so the DOM has painted before we measure scroll height.
    requestAnimationFrame(() => { this.$chat.scrollTop = this.$chat.scrollHeight; });
  }
}

if (!customElements.get('ui-copilot')) {
  customElements.define('ui-copilot', UiCopilot);
}

// ---------------------------------------------------------------------------
// Side-effect handlers for the copilot's non-DOM actions.
//
// The StubAgent emits `navigate` and `theme-toggle` side effects; the
// shell.js runtime dispatches them by `kind`. Registered here so they
// live with the copilot (not with ui-app-shell) — feels right because
// they only exist because the copilot exists.
// ---------------------------------------------------------------------------
if (window.ui && typeof window.ui.registerSideEffect === 'function') {
  window.ui.registerSideEffect('navigate', ({ url } = {}) => {
    if (url && typeof window.ui.navigate === 'function') window.ui.navigate(url);
  });
  window.ui.registerSideEffect('theme-toggle', () => {
    const root = document.documentElement;
    const next = root.dataset.theme === 'dark' ? 'light' : 'dark';
    root.dataset.theme = next;
    try { localStorage.setItem('erp.theme', next); } catch { /* ignore */ }
  });
}
