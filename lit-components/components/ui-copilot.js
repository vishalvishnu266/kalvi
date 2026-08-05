// -----------------------------------------------------------------------------
// <ui-copilot> — floating AI chat pane.
//
// Placement:
//   • Desktop (≥769px): floating popover, docked to the bottom-right
//     corner. 420 × 640, rounded, soft shadow.
//   • Mobile  (≤768px): bottom sheet, 85dvh, rounded top corners.
//
// Both modes share the exact same DOM; the CSS below flips between the
// two. Hidden by default; the AI icon in <ui-primary-bar> toggles it.
//
// Wire contract:
//   • POST /agent  with { utterance, context } as JSON
//   • Response is an SSE stream of `event: fragment` frames whose data
//     is a <ui-fragment target=… action=…>…</ui-fragment> envelope.
//   • Each fragment is applied via window.ui.applyAll(), so agent-driven
//     changes flow through the same pipeline as normal navigation.
//
// UX:
//   • Every "turn" (user question + agent reply) is rendered as a card
//     to feel less like IRC and more like Perplexity / ChatGPT.
//   • Enter to send, Shift+Enter for newline.
//   • Composer autofocuses when opened.
//   • Esc closes.
// -----------------------------------------------------------------------------

class UiCopilot extends HTMLElement {
  connectedCallback() {
    if (this._mounted) return;
    this._mounted = true;

    this.endpoint = this.getAttribute('endpoint') || '/agent';

    // Install document-level styles once — shared across all instances.
    UiCopilot._installStyles();

    this.innerHTML = `
      <div class="ui-cop-shell" role="dialog" aria-label="AI copilot">
        <div class="ui-cop-header">
          <div class="ui-cop-title">
            <span class="ui-cop-title-glyph">✨</span>
            <span>AI</span>
          </div>
          <button type="button" class="ui-cop-close" aria-label="Close">×</button>
        </div>

        <div class="ui-cop-turns" data-uc="chat" aria-live="polite"></div>

        <form class="ui-cop-composer" data-uc="composer" autocomplete="off">
          <textarea data-uc="input"
                    placeholder="Ask anything, or try: go to dashboard, dark mode…"
                    rows="1"
                    aria-label="Message the copilot"></textarea>
          <button type="submit" class="ui-cop-send" data-uc="send" aria-label="Send">
            <ui-icon name="arrow-right" size="16"></ui-icon>
          </button>
        </form>
      </div>
    `;

    this.$chat     = this.querySelector('[data-uc="chat"]');
    this.$input    = this.querySelector('[data-uc="input"]');
    this.$send     = this.querySelector('[data-uc="send"]');
    this.$composer = this.querySelector('[data-uc="composer"]');
    this.$close    = this.querySelector('.ui-cop-close');

    this.$composer.addEventListener('submit',  (e) => this._onSubmit(e));
    this.$close.addEventListener('click',      () => this.close());
    this.$input.addEventListener('keydown',    (e) => this._onKeydown(e));
    this.$input.addEventListener('input',      () => this._autogrow());

    // Backdrop click on the host closes on mobile (matches sheet UX).
    this.addEventListener('click', (e) => {
      if (e.target === this) this.close();
    });

    // Esc anywhere in the copilot closes.
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && this.hasAttribute('open')) this.close();
    });
  }

  open() {
    this.setAttribute('open', '');
    // rAF so the input exists in the layout tree when we focus.
    requestAnimationFrame(() => this.$input?.focus());
  }
  close()   { this.removeAttribute('open'); }
  toggle()  { this.hasAttribute('open') ? this.close() : this.open(); }

  _onKeydown(e) {
    // Enter → send (Shift+Enter → newline).
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      this.$composer.requestSubmit();
    }
  }

  _autogrow() {
    // Textarea grows with content, capped at 6 lines.
    const ta = this.$input;
    ta.style.height = 'auto';
    ta.style.height = Math.min(ta.scrollHeight, 6 * 22) + 'px';
  }

  async _onSubmit(e) {
    e.preventDefault();
    const utterance = this.$input.value.trim();
    if (!utterance) return;

    const turn = this._beginTurn(utterance);
    this.$input.value = '';
    this._autogrow();
    this.$send.disabled = true;

    try {
      await this._streamTurn(utterance, turn);
    } catch (err) {
      console.error('[copilot] turn failed:', err);
      turn.appendError(err.message || String(err));
    } finally {
      this.$send.disabled = false;
      this.$input.focus();
    }
  }

  /**
   * Start a new turn card. Returns an object with helpers to append
   * agent bubbles / tool cards / errors as SSE frames arrive.
   */
  _beginTurn(utterance) {
    const card = document.createElement('div');
    card.className = 'ui-cop-turn';
    card.innerHTML = `
      <div class="ui-cop-you">
        <span class="ui-cop-avatar ui-cop-avatar--you">You</span>
        <div class="ui-cop-you-text"></div>
      </div>
      <div class="ui-cop-answer" data-cop-answer>
        <span class="ui-cop-avatar ui-cop-avatar--ai">✨</span>
        <div class="ui-cop-answer-body">
          <span class="ui-cop-thinking">
            <span class="ui-cop-dot"></span><span class="ui-cop-dot"></span><span class="ui-cop-dot"></span>
          </span>
        </div>
      </div>
    `;
    card.querySelector('.ui-cop-you-text').textContent = utterance;
    this.$chat.appendChild(card);
    this._scroll();

    const body = card.querySelector('.ui-cop-answer-body');
    const thinking = card.querySelector('.ui-cop-thinking');
    let cleared = false;

    // Return a small controller the caller uses to fill the answer.
    return {
      // Called when a real fragment lands — remove the thinking dots.
      ensureCleared: () => {
        if (!cleared) { thinking?.remove(); cleared = true; }
      },
      appendHTML: (html) => {
        body.insertAdjacentHTML('beforeend', html);
        this._scroll();
      },
      appendError: (msg) => {
        thinking?.remove();
        const el = document.createElement('div');
        el.className = 'ui-cop-error';
        el.textContent = msg;
        body.appendChild(el);
        this._scroll();
      },
    };
  }

  async _streamTurn(utterance, turn) {
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
      let idx;
      while ((idx = buf.indexOf('\n\n')) !== -1) {
        const raw = buf.slice(0, idx);
        buf = buf.slice(idx + 2);
        this._handleSseFrame(raw, turn);
      }
    }
  }

  _handleSseFrame(frame, turn) {
    let event = 'message';
    let data  = '';
    for (const line of frame.split('\n')) {
      if (line.startsWith('event:'))     event = line.slice(6).trim();
      else if (line.startsWith('data:')) data  += (data ? '\n' : '') + line.slice(5).trim();
    }
    if (event === 'done') return;
    if (event !== 'fragment' || !data) return;

    // Extract the <ui-fragment target=…>...</ui-fragment> envelope.
    const tpl = document.createElement('template');
    tpl.innerHTML = data;
    const frag = tpl.content.querySelector('ui-fragment');
    if (!frag) return;

    const target = frag.getAttribute('target') || 'main';

    // Route:
    //   copilot-chat → append inside THIS turn's answer body as an
    //                  agent bubble (not directly into the chat area)
    //   anything else → hand off to window.ui.applyAll (main swap,
    //                  side effect, etc.).
    if (target === 'copilot-chat') {
      turn.ensureCleared();
      turn.appendHTML(frag.innerHTML);
    } else {
      if (window.ui?.applyAll) window.ui.applyAll(tpl.content);
    }
  }

  _scroll() {
    requestAnimationFrame(() => { this.$chat.scrollTop = this.$chat.scrollHeight; });
  }

  // ---- One shared stylesheet at document level. ---------------------------
  static _installStyles() {
    if (document.querySelector('style[data-ui-copilot]')) return;
    const style = document.createElement('style');
    style.dataset.uiCopilot = '';
    style.textContent = `
      ui-copilot {
        --uc-radius:  16px;
        --uc-shadow:  0 20px 60px rgba(0, 0, 0, .22);
        position: fixed;
        z-index: 120;
        display: none;
        font: inherit;
      }
      ui-copilot[open] { display: block; }

      .ui-cop-shell {
        display: flex; flex-direction: column;
        background: var(--color-bg, #fff);
        color: var(--color-fg, #111);
        border: 1px solid var(--color-border, #e5e7eb);
        border-radius: var(--uc-radius);
        box-shadow: var(--uc-shadow);
        overflow: hidden;
      }
      .ui-cop-header {
        display: flex; align-items: center; justify-content: space-between;
        padding: 12px 14px;
        border-bottom: 1px solid var(--color-border, #e5e7eb);
      }
      .ui-cop-title {
        display: flex; align-items: center; gap: 8px;
        font-weight: 600; font-size: 14px;
      }
      .ui-cop-title-glyph {
        display: grid; place-items: center;
        width: 22px; height: 22px; border-radius: 6px;
        font-size: 13px;
        background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%);
        color: #fff;
      }
      .ui-cop-close {
        background: none; border: 0; font-size: 20px; line-height: 1;
        padding: 4px 8px; border-radius: 6px;
        color: inherit; opacity: .55; cursor: pointer;
      }
      .ui-cop-close:hover { opacity: 1; background: var(--color-surface-2, #f5f5f7); }

      .ui-cop-turns {
        flex: 1; overflow-y: auto;
        padding: 14px;
        display: flex; flex-direction: column; gap: 14px;
        background: var(--color-surface, #fafafa);
      }
      .ui-cop-turns:empty::before {
        content: 'Ask me anything…';
        display: block; text-align: center;
        opacity: .5; font-size: 13px; padding: 24px 0;
      }

      /* ─── Turn card ─── */
      .ui-cop-turn {
        display: flex; flex-direction: column; gap: 8px;
      }
      .ui-cop-you, .ui-cop-answer {
        display: flex; gap: 10px; align-items: flex-start;
        background: var(--color-bg, #fff);
        border: 1px solid var(--color-border, #e5e7eb);
        border-radius: 12px;
        padding: 10px 12px;
        font-size: 13px; line-height: 1.5;
      }
      .ui-cop-avatar {
        flex: 0 0 22px;
        display: grid; place-items: center;
        width: 22px; height: 22px; border-radius: 6px;
        font-size: 11px; font-weight: 600;
        color: #fff;
      }
      .ui-cop-avatar--you { background: linear-gradient(135deg, #f59e0b, #ef4444); }
      .ui-cop-avatar--ai  { background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%); }
      .ui-cop-you-text    { flex: 1; }
      .ui-cop-answer-body { flex: 1; }
      .ui-cop-answer-body > * + * { margin-top: 6px; }

      .ui-cop-error {
        background: #fef2f2; color: #991b1b;
        padding: 8px 10px; border-radius: 8px;
      }

      /* ─── Thinking dots ─── */
      .ui-cop-thinking {
        display: inline-flex; gap: 4px; align-items: center;
      }
      .ui-cop-dot {
        width: 6px; height: 6px; border-radius: 50%;
        background: currentColor; opacity: .3;
        animation: ui-cop-pulse 1.2s ease-in-out infinite;
      }
      .ui-cop-dot:nth-child(2) { animation-delay: .15s; }
      .ui-cop-dot:nth-child(3) { animation-delay: .30s; }
      @keyframes ui-cop-pulse {
        0%, 60%, 100% { opacity: .25; transform: scale(1); }
        30%           { opacity: 1;   transform: scale(1.25); }
      }

      /* ─── Composer ─── */
      .ui-cop-composer {
        display: flex; align-items: flex-end; gap: 8px;
        padding: 10px 12px;
        border-top: 1px solid var(--color-border, #e5e7eb);
        background: var(--color-bg, #fff);
      }
      .ui-cop-composer textarea {
        flex: 1;
        padding: 8px 10px;
        font: inherit; font-size: 13px; line-height: 1.4;
        border: 1px solid var(--color-border, #e5e7eb);
        border-radius: 10px;
        background: var(--color-surface-2, #f5f5f7);
        color: inherit;
        outline: none; resize: none;
        max-height: 140px;
      }
      .ui-cop-composer textarea:focus {
        background: var(--color-bg, #fff);
        border-color: #7c3aed;
        box-shadow: 0 0 0 3px rgba(124, 58, 237, .15);
      }
      .ui-cop-send {
        display: grid; place-items: center;
        width: 34px; height: 34px;
        padding: 0;
        border: 0; border-radius: 10px;
        color: #fff; cursor: pointer;
        background: linear-gradient(135deg, #7c3aed 0%, #4f46e5 100%);
      }
      .ui-cop-send:disabled { opacity: .5; cursor: default; }

      /* ─── Desktop: floating popover, bottom-right ─── */
      @media (min-width: 769px) {
        ui-copilot {
          right: 20px; bottom: 20px;
          width: min(420px, 92vw);
          height: min(640px, 78dvh);
        }
        ui-copilot .ui-cop-shell { height: 100%; }
      }

      /* ─── Mobile: bottom sheet, full width ─── */
      @media (max-width: 768px) {
        ui-copilot {
          left: 0; right: 0; bottom: 0;
        }
        ui-copilot .ui-cop-shell {
          height: 85dvh;
          border-radius: 18px 18px 0 0;
          border-bottom: 0;
        }
        /* Small drag-handle affordance. */
        ui-copilot .ui-cop-shell::before {
          content: '';
          display: block; margin: 8px auto 0;
          width: 40px; height: 4px; border-radius: 2px;
          background: var(--color-border, #e5e7eb);
        }
      }
    `;
    document.head.appendChild(style);
  }
}

if (!customElements.get('ui-copilot')) {
  customElements.define('ui-copilot', UiCopilot);
}

// ---------------------------------------------------------------------------
// Side-effect handlers registered for the copilot's non-DOM actions.
// Same as before — the primary-bar's AI toggle just controls open/close.
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
