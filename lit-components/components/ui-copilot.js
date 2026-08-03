// -----------------------------------------------------------------------------
// <ui-copilot> — persistent, mobile-friendly agentic chat window.
//
// Design notes
// ------------
// * The component is a single custom element that mounts a floating FAB and,
//   on click, opens a right-hand drawer on desktop / bottom-sheet on mobile.
// * State is kept INSIDE the element so Hotwire Turbo page swaps do not reset
//   the conversation. Mount the element ONCE, OUTSIDE the `<turbo-frame id="page">`
//   swap target.
// * Content of chat messages is server-rendered HTML delivered via SSE — the
//   component just appends fragments. Small control events are JSON.
// * A tiny client-side "tools" registry lets the server drive UI-only actions
//   (navigate, focus, scroll_to, highlight, fill_form, open_modal, toast).
//
// Public attributes
//   session-url  POST → { session_id }
//   stream-url   POST { session_id, prompt } → text/event-stream
//   history-url  GET  /:id → text/html (already-rendered messages)
//
// SSE events understood
//   message_start  data: <html fragment>          (append a new msg bubble)
//   token          data: {"id":"m-42","text":"…"} (append text to bubble)
//   tool_call      data: <html fragment>          (append tool-card)
//   tool_result    data: <html fragment>          (append result / patch DOM)
//   step           data: {"n":2,"of":4,"label":"Fetching fees"}
//   ui_action      data: {"action":"navigate","href":"/x"}
//   message_end    data: {"id":"m-42"}            (finalise: enable input)
//   error          data: {"message":"…"}
//
// Mock-friendly: the server can emit any subset of these — the component
// tolerates missing IDs by targeting the last assistant bubble.
// -----------------------------------------------------------------------------

import { LitBaseElement, html, css, nothing } from './base.js';

// ---- Client-side "UI tools" the agent can invoke ---------------------------
// Keep tiny and predictable. Every tool takes a plain object of args.
const TOOLS = {
  navigate:  ({ href }) => {
    if (!href) return;
    if (window.Turbo?.visit) window.Turbo.visit(href);
    else window.location.href = href;
  },
  focus:     ({ selector }) => document.querySelector(selector)?.focus(),
  scroll_to: ({ selector }) => document.querySelector(selector)?.scrollIntoView({ behavior: 'smooth', block: 'center' }),
  highlight: ({ selector, ms = 1500 }) => {
    const el = document.querySelector(selector);
    if (!el) return;
    el.classList.add('ai-highlight');
    setTimeout(() => el.classList.remove('ai-highlight'), ms);
  },
  fill_form: ({ selector, values = {} }) => {
    const form = document.querySelector(selector);
    if (!form) return;
    for (const [name, value] of Object.entries(values)) {
      const field = form.querySelector(`[name="${name}"]`);
      if (!field) continue;
      // Web components (<ui-input>) reflect `value` attribute.
      if ('value' in field) field.value = value;
      field.setAttribute?.('value', String(value));
      field.dispatchEvent(new Event('input',  { bubbles: true }));
      field.dispatchEvent(new Event('change', { bubbles: true }));
    }
  },
  open_modal: ({ selector }) => document.querySelector(selector)?.openModal?.(),
  toast:      ({ message, tone = 'info' }) => {
    const host = document.querySelector('ui-toast-host');
    host?.push?.({ message, tone });
  },
  set_theme:  ({ theme }) => { if (theme) document.documentElement.dataset.theme = theme; },
};

class UICopilot extends LitBaseElement {
  static properties = {
    open:         { type: Boolean, reflect: true },
    sessionUrl:   { type: String, attribute: 'session-url' },
    streamUrl:    { type: String, attribute: 'stream-url' },
    historyUrl:   { type: String, attribute: 'history-url' },
    label:        { type: String },
    // Internal state:
    busy:         { type: Boolean, state: true },
    step:         { type: Object,  state: true },
    unread:       { type: Number,  state: true },
  };

  static styles = css`
    :host {
      /* Live outside document flow so it works on any page. */
      position: fixed;
      inset: 0;
      pointer-events: none; /* only children with pointer-events:auto react */
      z-index: 2500;
      font-family: var(--font-sans, system-ui, sans-serif);
      color: var(--color-text, #0f172a);
    }

    /* ── Floating action button ─────────────────────────────────────────── */
    .fab {
      position: absolute;
      right: max(16px, env(safe-area-inset-right));
      bottom: max(16px, env(safe-area-inset-bottom));
      width: 56px; height: 56px;
      border-radius: 50%;
      border: 0;
      background: var(--color-primary, #0a84ff);
      color: var(--color-primary-contrast, #fff);
      box-shadow: var(--shadow-lg, 0 20px 40px rgba(0,0,0,.25));
      cursor: pointer;
      pointer-events: auto;
      display: grid;
      place-items: center;
      transition: transform var(--dur-fast, 120ms) var(--ease, ease);
    }
    .fab:hover  { transform: translateY(-2px); }
    .fab:active { transform: translateY(0); }
    .fab .badge {
      position: absolute; top: -4px; right: -4px;
      min-width: 20px; height: 20px; padding: 0 6px;
      border-radius: 999px;
      background: var(--color-danger, #ff3b30);
      color: #fff;
      font-size: 11px; font-weight: 600;
      display: grid; place-items: center;
    }
    :host([open]) .fab { display: none; }

    /* ── Scrim ──────────────────────────────────────────────────────────── */
    .scrim {
      position: absolute; inset: 0;
      background: var(--color-scrim, rgba(15,23,42,.35));
      opacity: 0;
      pointer-events: none;
      transition: opacity var(--dur-med, 240ms) var(--ease, ease);
    }
    :host([open]) .scrim {
      opacity: 1;
      pointer-events: auto;
    }

    /* ── Panel: desktop right-drawer, mobile bottom-sheet ───────────────── */
    .panel {
      position: absolute;
      background: var(--color-surface, #fff);
      box-shadow: var(--shadow-lg, 0 20px 40px rgba(0,0,0,.25));
      display: flex;
      flex-direction: column;
      pointer-events: auto;
      transition: transform var(--dur-med, 240ms) var(--ease, ease);
    }

    /* Desktop (>= 720px): right-hand drawer */
    @media (min-width: 720px) {
      .panel {
        top: 0; right: 0; bottom: 0;
        width: min(440px, 96vw);
        transform: translateX(100%);
        border-left: 1px solid var(--color-border, rgba(0,0,0,.08));
      }
      :host([open]) .panel { transform: translateX(0); }
    }

    /* Mobile (< 720px): bottom sheet, ~85vh tall, with drag handle */
    @media (max-width: 719.98px) {
      .panel {
        left: 0; right: 0; bottom: 0;
        height: 85vh;
        max-height: 85vh;
        transform: translateY(100%);
        border-top: 1px solid var(--color-border, rgba(0,0,0,.08));
        border-radius: var(--radius-xl, 16px) var(--radius-xl, 16px) 0 0;
        padding-bottom: env(safe-area-inset-bottom);
      }
      :host([open]) .panel { transform: translateY(0); }
      .grabber {
        width: 40px; height: 4px;
        background: var(--color-border-strong, rgba(0,0,0,.16));
        border-radius: 2px;
        margin: 8px auto 0;
      }
    }

    /* ── Header ─────────────────────────────────────────────────────────── */
    header {
      display: flex; align-items: center; gap: 8px;
      padding: 12px 16px;
      border-bottom: 1px solid var(--color-border, rgba(0,0,0,.08));
      flex: 0 0 auto;
    }
    header .title {
      flex: 1;
      font-size: var(--fs-md, 15px);
      font-weight: var(--fw-semibold, 600);
      display: flex; align-items: center; gap: 8px;
    }
    header .title .dot {
      width: 8px; height: 8px; border-radius: 50%;
      background: var(--color-success, #34c759);
      box-shadow: 0 0 0 3px color-mix(in srgb, var(--color-success, #34c759) 25%, transparent);
    }
    header button {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-text-muted, #475569);
      width: 32px; height: 32px; border-radius: 8px;
      display: grid; place-items: center;
    }
    header button:hover {
      background: var(--color-surface-hover, rgba(0,0,0,.05));
      color: var(--color-text, #0f172a);
    }

    /* ── Suggestions (empty state) ──────────────────────────────────────── */
    .suggestions {
      display: grid; gap: 8px;
      padding: 12px 16px;
      grid-template-columns: 1fr 1fr;
    }
    .suggestions button {
      appearance: none;
      text-align: left;
      padding: 10px 12px;
      border-radius: 12px;
      border: 1px solid var(--color-border, rgba(0,0,0,.08));
      background: var(--color-surface-2, #f8faff);
      color: var(--color-text, #0f172a);
      font: inherit;
      cursor: pointer;
      line-height: 1.3;
    }
    .suggestions button:hover {
      border-color: var(--color-primary, #0a84ff);
      background: var(--color-primary-soft, rgba(10,132,255,.14));
    }
    .suggestions .hint {
      grid-column: 1 / -1;
      color: var(--color-text-muted, #475569);
      font-size: var(--fs-sm, 13px);
      padding: 0 2px;
    }

    /* ── Messages list ──────────────────────────────────────────────────── */
    .messages {
      flex: 1;
      overflow: auto;
      padding: 12px 16px 8px;
      display: flex; flex-direction: column; gap: 10px;
      scroll-behavior: smooth;
      -webkit-overflow-scrolling: touch;
    }
    .msg { display: flex; }
    .msg .bubble {
      max-width: 88%;
      padding: 10px 12px;
      border-radius: 14px;
      white-space: pre-wrap;
      word-wrap: break-word;
      line-height: 1.35;
      font-size: var(--fs-md, 15px);
    }
    .msg-user            { justify-content: flex-end; }
    .msg-user   .bubble  {
      background: var(--color-primary, #0a84ff);
      color: var(--color-primary-contrast, #fff);
      border-bottom-right-radius: 4px;
    }
    .msg-assistant       { justify-content: flex-start; }
    .msg-assistant .bubble {
      background: var(--color-surface-2, #f2f4fb);
      color: var(--color-text, #0f172a);
      border-bottom-left-radius: 4px;
    }
    .tool-card {
      align-self: flex-start;
      display: inline-flex; align-items: center; gap: 8px;
      padding: 8px 10px;
      border-radius: 10px;
      background: var(--color-info-soft, rgba(90,200,250,.14));
      color: var(--color-info-strong, #036);
      font-size: var(--fs-sm, 13px);
      border: 1px dashed var(--color-info, #5ac8fa);
    }
    .tool-card code {
      font-family: 'JetBrains Mono', ui-monospace, monospace;
      font-size: 12px;
      background: rgba(0,0,0,.05);
      padding: 1px 6px; border-radius: 4px;
    }
    .thinking {
      display: inline-flex; align-items: center; gap: 4px;
      color: var(--color-text-muted, #475569);
    }
    .thinking span {
      width: 6px; height: 6px; border-radius: 50%;
      background: currentColor;
      animation: bp 1s infinite ease-in-out;
    }
    .thinking span:nth-child(2) { animation-delay: .15s; }
    .thinking span:nth-child(3) { animation-delay: .30s; }
    @keyframes bp { 0%,80%,100% { opacity: .2; transform: translateY(0); }
                    40%          { opacity: 1;  transform: translateY(-2px); } }

    /* ── Step / progress bar ────────────────────────────────────────────── */
    .step-bar {
      padding: 6px 16px 4px;
      font-size: var(--fs-sm, 13px);
      color: var(--color-text-muted, #475569);
      display: flex; align-items: center; gap: 10px;
      border-top: 1px dashed var(--color-border, rgba(0,0,0,.08));
      background: var(--color-surface-alt, #f2f4fb);
    }
    .step-bar .track {
      flex: 1; height: 4px; border-radius: 2px;
      background: var(--color-border, rgba(0,0,0,.08));
      overflow: hidden;
    }
    .step-bar .fill {
      height: 100%; background: var(--color-primary, #0a84ff);
      transition: width var(--dur-med, 240ms) var(--ease, ease);
    }

    /* ── Composer ───────────────────────────────────────────────────────── */
    .composer {
      display: flex; align-items: end; gap: 8px;
      padding: 10px 12px;
      border-top: 1px solid var(--color-border, rgba(0,0,0,.08));
      background: var(--color-surface, #fff);
    }
    .composer textarea {
      flex: 1;
      resize: none;
      min-height: 40px;
      max-height: 140px;
      padding: 10px 12px;
      border-radius: 12px;
      border: 1px solid var(--color-border, rgba(0,0,0,.08));
      background: var(--color-surface-2, #f8faff);
      color: var(--color-text, #0f172a);
      font: inherit;
      line-height: 1.35;
      outline: none;
    }
    .composer textarea:focus {
      border-color: var(--color-primary, #0a84ff);
      box-shadow: 0 0 0 3px var(--color-primary-ring, rgba(10,132,255,.28));
      background: var(--color-surface, #fff);
    }
    .composer .send {
      appearance: none; border: 0; cursor: pointer;
      width: 40px; height: 40px; border-radius: 50%;
      background: var(--color-primary, #0a84ff);
      color: var(--color-primary-contrast, #fff);
      display: grid; place-items: center;
      transition: opacity .15s;
    }
    .composer .send[disabled] { opacity: .5; cursor: default; }
  `;

  constructor() {
    super();
    this.open        = false;
    this.sessionUrl  = '/copilot/session';
    this.streamUrl   = '/copilot/message';
    this.historyUrl  = '/copilot/history';
    this.label       = 'ERP Copilot';
    this.busy        = false;
    this.step        = null;
    this.unread      = 0;
    this._sessionId  = null;
    this._abort      = null;
    this._lastAssistantId = null;
  }

  connectedCallback() {
    super.connectedCallback();
    this._esc = (e) => { if (e.key === 'Escape' && this.open) this._close(); };
    document.addEventListener('keydown', this._esc);

    // Global keybind: Ctrl/Cmd + J opens the copilot from anywhere.
    this._kbd = (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'j') {
        e.preventDefault();
        this._toggle();
      }
    };
    document.addEventListener('keydown', this._kbd);
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener('keydown', this._esc);
    document.removeEventListener('keydown', this._kbd);
    this._abort?.abort();
  }

  // ── Public API ────────────────────────────────────────────────────────
  openPanel()  { this._open(); }
  closePanel() { this._close(); }

  // ── Internals ─────────────────────────────────────────────────────────
  async _ensureSession() {
    if (this._sessionId) return;
    try {
      const r = await fetch(this.sessionUrl, { method: 'POST' });
      if (!r.ok) throw new Error('session http ' + r.status);
      const j = await r.json();
      this._sessionId = j.session_id;
      // Hydrate any previous history for this session.
      if (this.historyUrl) {
        const h = await fetch(`${this.historyUrl}/${this._sessionId}`);
        if (h.ok) this._messagesEl().innerHTML = await h.text();
      }
    } catch (err) {
      console.warn('[ui-copilot] session init failed; running detached', err);
      this._sessionId = 'local-' + Math.random().toString(36).slice(2);
    }
  }

  _messagesEl() { return this.renderRoot.querySelector('.messages'); }
  _composerEl() { return this.renderRoot.querySelector('textarea'); }

  _open()  { this.open = true; this.unread = 0; setTimeout(() => this._composerEl()?.focus(), 100); }
  _close() { this.open = false; this._abort?.abort(); }
  _toggle(){ this.open ? this._close() : this._open(); }

  async _send(explicit) {
    const ta = this._composerEl();
    const prompt = (explicit ?? ta?.value ?? '').trim();
    if (!prompt || this.busy) return;
    if (ta && !explicit) ta.value = '';
    await this._ensureSession();
    this.busy = true;
    this.step = null;

    const msgs = this._messagesEl();
    // Optimistically render the user's message.
    msgs.insertAdjacentHTML('beforeend',
      `<div class="msg msg-user"><div class="bubble"></div></div>`);
    msgs.lastElementChild.querySelector('.bubble').textContent = prompt;
    // "Thinking…" placeholder that gets replaced on message_start.
    msgs.insertAdjacentHTML('beforeend',
      `<div class="msg msg-assistant" data-pending>
         <div class="bubble"><span class="thinking"><span></span><span></span><span></span></span></div>
       </div>`);
    this._scrollBottom();

    try {
      this._abort = new AbortController();
      const resp = await fetch(this.streamUrl, {
        method: 'POST',
        signal: this._abort.signal,
        headers: {
          'Content-Type': 'application/json',
          'Accept':       'text/event-stream',
        },
        body: JSON.stringify({ session_id: this._sessionId, prompt }),
      });
      if (!resp.ok || !resp.body) throw new Error('stream http ' + resp.status);
      await this._readSse(resp.body);
    } catch (err) {
      if (err.name !== 'AbortError') {
        console.error('[ui-copilot] stream failed', err);
        this._pushError(err.message || 'Failed to reach copilot');
      }
    } finally {
      this.busy = false;
      this.step = null;
      // Clean up any leftover pending placeholder.
      msgs.querySelectorAll('[data-pending]').forEach(el => el.remove());
      // If unopened, flash unread badge.
      if (!this.open) this.unread++;
    }
  }

  async _readSse(body) {
    const reader = body.getReader();
    const dec = new TextDecoder();
    let buf = '';
    while (true) {
      const { value, done } = await reader.read();
      if (done) break;
      buf += dec.decode(value, { stream: true });
      let sep;
      while ((sep = buf.indexOf('\n\n')) >= 0) {
        this._handleEvent(buf.slice(0, sep));
        buf = buf.slice(sep + 2);
      }
    }
  }

  _handleEvent(chunk) {
    let event = 'message';
    let dataLines = [];
    for (const line of chunk.split('\n')) {
      if (line.startsWith(':')) continue; // comment / keep-alive
      if (line.startsWith('event:')) event = line.slice(6).trim();
      else if (line.startsWith('data:')) dataLines.push(line.slice(5).replace(/^ /, ''));
    }
    const data = dataLines.join('\n');
    const msgs = this._messagesEl();
    if (!msgs) return;

    switch (event) {
      case 'message_start': {
        // Replace pending placeholder with the real bubble HTML.
        const pend = msgs.querySelector('[data-pending]');
        if (pend) pend.outerHTML = data;
        else msgs.insertAdjacentHTML('beforeend', data);
        // Track last assistant bubble id for token appends.
        const last = msgs.querySelector('.msg-assistant:last-of-type');
        this._lastAssistantId = last?.id || null;
        this._scrollBottom();
        break;
      }
      case 'tool_call':
      case 'tool_result': {
        // Remove pending on first content event too.
        msgs.querySelectorAll('[data-pending]').forEach(el => el.remove());
        msgs.insertAdjacentHTML('beforeend', data);
        this._scrollBottom();
        break;
      }
      case 'token': {
        let id, text;
        try { ({ id, text } = JSON.parse(data)); }
        catch { text = data; }
        const target = id
          ? msgs.querySelector(`#${CSS.escape(id)} .bubble`)
          : msgs.querySelector('.msg-assistant:last-of-type .bubble');
        if (!target) return;
        // First token — clear the "thinking" placeholder if present.
        if (target.querySelector('.thinking')) target.textContent = '';
        target.append(text ?? '');
        this._scrollBottom();
        break;
      }
      case 'step': {
        try { this.step = JSON.parse(data); } catch { /* ignore */ }
        break;
      }
      case 'ui_action': {
        try {
          const { action, ...args } = JSON.parse(data);
          TOOLS[action]?.(args);
        } catch (e) { console.warn('[ui-copilot] bad ui_action', e); }
        break;
      }
      case 'message_end': {
        // No-op for now; hook for analytics / logging.
        break;
      }
      case 'error': {
        let msg = data;
        try { msg = JSON.parse(data).message ?? data; } catch {}
        this._pushError(msg);
        break;
      }
      default: /* ignore unknown events */
    }
  }

  _pushError(message) {
    this._messagesEl().insertAdjacentHTML('beforeend',
      `<div class="msg msg-assistant"><div class="bubble" style="background:var(--color-danger-soft);color:var(--color-danger-strong)">⚠️ ${escapeHtml(message)}</div></div>`);
    this._scrollBottom();
  }

  _scrollBottom() {
    const msgs = this._messagesEl();
    if (msgs) msgs.scrollTop = msgs.scrollHeight;
  }

  _onKeydown(e) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      this._send();
    }
  }

  render() {
    const suggestions = [
      'Show me pending admissions',
      'Add a new student',
      'Mark attendance for Grade 5',
      'Collect fees for Aarav',
    ];
    const pct = this.step ? Math.round((this.step.n / this.step.of) * 100) : 0;

    return html`
      <button class="fab" @click=${() => this._toggle()} title="Ask ERP Copilot (Ctrl+J)"
              aria-label="Open Copilot">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 2l2.09 4.26L18.5 7l-3.25 3.16.77 4.49L12 12.77 7.98 14.65l.77-4.49L5.5 7l4.41-.74L12 2z"/>
        </svg>
        ${this.unread > 0 ? html`<span class="badge">${this.unread}</span>` : nothing}
      </button>

      <div class="scrim" @click=${() => this._close()}></div>

      <aside class="panel" role="dialog" aria-modal="true" aria-label=${this.label}>
        <div class="grabber" @click=${() => this._close()}></div>
        <header>
          <div class="title"><span class="dot"></span>${this.label}</div>
          <button title="Clear conversation" @click=${() => this._reset()}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3 6h18M8 6V4h8v2m-9 0v14a2 2 0 002 2h6a2 2 0 002-2V6"/>
            </svg>
          </button>
          <button title="Close" @click=${() => this._close()}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M18 6L6 18M6 6l12 12"/>
            </svg>
          </button>
        </header>

        <div class="messages" @click=${(e) => this._onMessagesClick(e)}></div>

        ${this.step ? html`
          <div class="step-bar">
            <span>Step ${this.step.n}/${this.step.of}: ${this.step.label ?? ''}</span>
            <div class="track"><div class="fill" style="width:${pct}%"></div></div>
          </div>` : nothing}

        <div class="suggestions" ?hidden=${this._hasMessages()}>
          <div class="hint">Try one of these to get started:</div>
          ${suggestions.map(s => html`
            <button @click=${() => this._send(s)}>${s}</button>
          `)}
        </div>

        <div class="composer">
          <textarea rows="1"
                    placeholder="Ask, navigate, or automate…  (Enter to send, Shift+Enter for newline)"
                    @keydown=${(e) => this._onKeydown(e)}
                    @input=${(e) => this._autosize(e.target)}></textarea>
          <button class="send" ?disabled=${this.busy} @click=${() => this._send()}
                  title="Send (Enter)" aria-label="Send">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z"/>
            </svg>
          </button>
        </div>
      </aside>
    `;
  }

  _hasMessages() {
    return (this._messagesEl()?.children.length ?? 0) > 0;
  }

  _reset() {
    this._sessionId = null;
    this._lastAssistantId = null;
    this.step = null;
    if (this._messagesEl()) this._messagesEl().innerHTML = '';
    this.requestUpdate();
  }

  _autosize(ta) {
    ta.style.height = 'auto';
    ta.style.height = Math.min(ta.scrollHeight, 140) + 'px';
  }

  // Delegate clicks inside server-rendered bubbles (e.g. suggestion chips
  // the agent renders, "Approve" buttons on confirmation cards, etc.).
  _onMessagesClick(e) {
    const el = e.target.closest('[data-copilot-suggest]');
    if (el) {
      e.preventDefault();
      this._send(el.getAttribute('data-copilot-suggest'));
      return;
    }
    const nav = e.target.closest('[data-copilot-nav]');
    if (nav) {
      e.preventDefault();
      TOOLS.navigate({ href: nav.getAttribute('data-copilot-nav') });
    }
  }
}

function escapeHtml(s) {
  return String(s)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

customElements.define('ui-copilot', UICopilot);

// Small global for the highlight ui-action to have a visible style even
// when the host page hasn't added one. Injected once per document.
if (typeof document !== 'undefined' && !document.getElementById('ai-highlight-style')) {
  const s = document.createElement('style');
  s.id = 'ai-highlight-style';
  s.textContent = `
    .ai-highlight {
      outline: 2px solid var(--color-primary, #0a84ff) !important;
      outline-offset: 2px;
      transition: outline-color .3s ease;
      animation: ai-pulse 1.2s ease-in-out;
    }
    @keyframes ai-pulse {
      0%,100% { box-shadow: 0 0 0 0 var(--color-primary-ring, rgba(10,132,255,.28)); }
      50%     { box-shadow: 0 0 0 8px var(--color-primary-ring, rgba(10,132,255,.28)); }
    }
  `;
  document.head.appendChild(s);
}
