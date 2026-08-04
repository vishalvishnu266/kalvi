// -----------------------------------------------------------------------------
// <ui-copilot-v2> — Conversational + command surface, non-technical-first.
//
// What this demonstrates in one component:
//   1. Always-on suggestion chips ABOVE the input (proactive discovery).
//   2. Live autocomplete BELOW the input (as user types), local-first.
//   3. Inline guided FORMS inside the chat when a tool needs arguments.
//   4. Result cards (text, tables, previews, confirmations) rendered inline.
//   5. Follow-up chips after every action (compounding workflow).
//   6. Voice input STUB (Web Speech API, graceful fallback).
//   7. Same tool registry serves English, taps, and symbols (`/`, `>`, `@`).
//
// All data is mocked via HTTP endpoints under `/agent/*` — see
// `src/web/agent.rs` for the server-side mock. No LLM. No DB.
// -----------------------------------------------------------------------------

import { LitBaseElement, html, css, nothing } from './base.js';

// ---- Fuzzy matcher (tiny, ~30 lines, MIT-flavoured) ------------------------
// Ranks a query against a haystack string.  Score composed of:
//  * exact substring match         → +100
//  * prefix match                  → +40
//  * subsequence match             → +20 (with tighter cluster => bonus)
// Returns 0 if no match at all so we can filter out irrelevant rows.
function fuzzyScore(query, hay) {
  if (!query) return 1;
  const q = query.toLowerCase();
  const h = hay.toLowerCase();
  if (h.includes(q)) return 100 + (h.startsWith(q) ? 40 : 0);
  let qi = 0, score = 0, streak = 0;
  for (let i = 0; i < h.length && qi < q.length; i++) {
    if (h[i] === q[qi]) { qi++; streak++; score += 2 + streak; }
    else streak = 0;
  }
  return qi === q.length ? score : 0;
}

class UICopilotV2 extends LitBaseElement {
  static properties = {
    open:         { type: Boolean, reflect: true },
    label:        { type: String },
    // internal state
    _suggestions: { state: true },   // proactive chips shown above input
    _matches:     { state: true },   // live autocomplete
    _query:       { state: true },
    _messages:    { state: true },   // array of message objects
    _pendingForm: { state: true },   // {tool, schema, values} when a form is up
    _listening:   { state: true },   // mic on/off
    _busy:        { state: true },
    _cursor:      { state: true },   // highlighted index in matches list
  };

  static styles = css`
    :host {
      position: fixed; inset: 0; pointer-events: none; z-index: 2500;
      font-family: var(--font-sans, system-ui, sans-serif);
      color: var(--color-text, #0f172a);
    }

    /* Floating trigger */
    .fab {
      position: absolute;
      right: max(16px, env(safe-area-inset-right));
      bottom: max(16px, env(safe-area-inset-bottom));
      width: 56px; height: 56px; border-radius: 50%;
      background: var(--color-primary, #0a84ff); color: #fff;
      border: 0; cursor: pointer; pointer-events: auto;
      box-shadow: var(--shadow-lg, 0 20px 40px rgba(0,0,0,.25));
      display: grid; place-items: center;
      transition: transform .15s;
    }
    .fab:hover { transform: translateY(-2px); }
    :host([open]) .fab { display: none; }

    /* Scrim */
    .scrim {
      position: absolute; inset: 0;
      background: rgba(15,23,42,.35);
      opacity: 0; pointer-events: none;
      transition: opacity .24s;
    }
    :host([open]) .scrim { opacity: 1; pointer-events: auto; }

    /* Panel */
    .panel {
      position: absolute;
      background: var(--color-surface, #fff);
      display: flex; flex-direction: column;
      pointer-events: auto;
      box-shadow: var(--shadow-lg, 0 20px 40px rgba(0,0,0,.25));
      transition: transform .24s;
    }
    @media (min-width: 720px) {
      .panel {
        top: 0; right: 0; bottom: 0;
        width: min(480px, 96vw);
        transform: translateX(100%);
        border-left: 1px solid var(--color-border, rgba(0,0,0,.08));
      }
      :host([open]) .panel { transform: translateX(0); }
    }
    @media (max-width: 719.98px) {
      .panel {
        left: 0; right: 0; bottom: 0;
        height: 88vh; transform: translateY(100%);
        border-radius: 16px 16px 0 0;
        padding-bottom: env(safe-area-inset-bottom);
      }
      :host([open]) .panel { transform: translateY(0); }
      .grabber {
        width: 40px; height: 4px; margin: 8px auto 0;
        background: rgba(0,0,0,.16); border-radius: 2px;
      }
    }

    /* Header */
    header {
      display: flex; align-items: center; gap: 8px;
      padding: 12px 16px;
      border-bottom: 1px solid var(--color-border, rgba(0,0,0,.08));
    }
    header .title {
      flex: 1; font-weight: 600; font-size: 15px;
      display: flex; align-items: center; gap: 8px;
    }
    header .title .dot {
      width: 8px; height: 8px; border-radius: 50%;
      background: #34c759;
      box-shadow: 0 0 0 3px rgba(52,199,89,.25);
    }
    header .iconbtn {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: #475569; width: 32px; height: 32px; border-radius: 8px;
      display: grid; place-items: center;
    }
    header .iconbtn:hover { background: rgba(0,0,0,.05); }

    /* Suggestion chips (proactive, above input) */
    .suggestions {
      padding: 10px 12px 6px;
      display: flex; gap: 6px; flex-wrap: wrap;
      border-bottom: 1px dashed rgba(0,0,0,.08);
      background: linear-gradient(to bottom, rgba(10,132,255,.04), transparent);
    }
    .suggestions .hint {
      width: 100%; font-size: 12px; color: #64748b; margin: 0 0 4px;
    }
    .chip {
      appearance: none; border: 1px solid rgba(10,132,255,.25);
      background: rgba(10,132,255,.08);
      color: #0a84ff; font-size: 13px;
      padding: 6px 10px; border-radius: 999px;
      cursor: pointer; font: inherit;
      transition: transform .1s;
    }
    .chip:hover { background: rgba(10,132,255,.16); }
    .chip:active { transform: scale(0.97); }

    /* Messages (transcript) */
    .messages {
      flex: 1; overflow: auto;
      padding: 10px 12px;
      display: flex; flex-direction: column; gap: 10px;
      -webkit-overflow-scrolling: touch;
    }
    .msg-user, .msg-bot { display: flex; }
    .msg-user { justify-content: flex-end; }
    .msg-user .bubble {
      background: var(--color-primary, #0a84ff); color: #fff;
      padding: 8px 12px; border-radius: 14px 14px 4px 14px;
      max-width: 88%;
    }
    .msg-bot .bubble {
      background: #f1f5f9; color: #0f172a;
      padding: 8px 12px; border-radius: 14px 14px 14px 4px;
      max-width: 88%;
      display: flex; align-items: center; gap: 6px;
    }
    .msg-bot.card .bubble {
      background: #fff;
      border: 1px solid #e2e8f0;
      padding: 12px;
      border-radius: 12px;
      width: 100%; max-width: 100%;
      display: block;
    }
    .followups {
      display: flex; flex-wrap: wrap; gap: 6px; margin-top: 8px;
    }
    .followups .chip { font-size: 12px; padding: 5px 9px; }

    /* Result card details */
    .kv { display: grid; grid-template-columns: 100px 1fr; gap: 4px 10px;
          font-size: 13px; margin: 4px 0; }
    .kv dt { color: #64748b; }
    .table {
      width: 100%; border-collapse: collapse; font-size: 13px;
      margin: 6px 0 4px;
    }
    .table th, .table td { padding: 5px 8px; text-align: left;
      border-bottom: 1px solid #e2e8f0; }
    .table th { color: #64748b; font-weight: 500; font-size: 12px; }
    .badge {
      display: inline-block; padding: 2px 7px; border-radius: 999px;
      font-size: 11px; font-weight: 500;
    }
    .badge.due  { background: #fee2e2; color: #991b1b; }
    .badge.ok   { background: #dcfce7; color: #166534; }
    .badge.info { background: #dbeafe; color: #1e40af; }

    /* Inline form (when a tool needs args) */
    .form-card {
      background: #fff; border: 1px solid #e2e8f0; border-radius: 12px;
      padding: 12px; display: flex; flex-direction: column; gap: 10px;
    }
    .form-card h4 { margin: 0 0 4px; font-size: 14px; }
    .field { display: flex; flex-direction: column; gap: 4px; }
    .field label { font-size: 12px; color: #475569; }
    .field select,
    .field input {
      appearance: none; -webkit-appearance: none;
      padding: 8px 10px; border-radius: 8px;
      border: 1px solid #cbd5e1; background: #fff;
      font: inherit; color: inherit; width: 100%;
      box-sizing: border-box;
    }
    .field select:focus,
    .field input:focus { outline: none; border-color: #0a84ff;
      box-shadow: 0 0 0 3px rgba(10,132,255,.2); }
    .form-actions { display: flex; gap: 6px; justify-content: flex-end; }
    .btn {
      appearance: none; border: 0; cursor: pointer; font: inherit;
      padding: 8px 14px; border-radius: 8px;
    }
    .btn.primary { background: #0a84ff; color: #fff; }
    .btn.ghost   { background: transparent; color: #475569; }
    .btn.danger  { background: #ef4444; color: #fff; }

    /* Confirm card (before mutations) */
    .confirm-card {
      background: #fff8e6; border: 1px solid #fde68a;
      border-radius: 12px; padding: 12px;
    }
    .confirm-card h4 { margin: 0 0 6px; font-size: 14px; color: #92400e; }
    .confirm-card .actions { display: flex; gap: 6px; justify-content: flex-end;
                              margin-top: 10px; }

    /* Autocomplete dropdown */
    .composer-wrap {
      position: relative;
      border-top: 1px solid rgba(0,0,0,.08);
      background: #fff;
    }
    .autocomplete {
      position: absolute; bottom: 100%; left: 0; right: 0;
      background: #fff; border-top: 1px solid #e2e8f0;
      box-shadow: 0 -8px 20px rgba(0,0,0,.06);
      max-height: 260px; overflow: auto;
      display: none;
    }
    .autocomplete.open { display: block; }
    .ac-item {
      display: flex; align-items: center; gap: 10px;
      padding: 8px 12px; cursor: pointer;
      border-bottom: 1px solid #f1f5f9;
    }
    .ac-item:last-child { border-bottom: 0; }
    .ac-item:hover,
    .ac-item.active { background: #eff6ff; }
    .ac-item .icon { font-size: 18px; width: 24px; text-align: center; }
    .ac-item .text { flex: 1; }
    .ac-item .text .title { font-size: 14px; }
    .ac-item .text .sub   { font-size: 12px; color: #64748b; margin-top: 1px; }
    .ac-item .shortcut {
      font-size: 11px; color: #94a3b8;
      background: #f1f5f9; padding: 2px 6px; border-radius: 4px;
      font-family: ui-monospace, monospace;
    }

    /* Composer */
    .composer {
      display: flex; align-items: end; gap: 6px;
      padding: 8px 10px;
    }
    .composer textarea {
      flex: 1; resize: none; min-height: 40px; max-height: 140px;
      padding: 10px 12px; border-radius: 12px;
      border: 1px solid #cbd5e1; background: #f8fafc;
      font: inherit; line-height: 1.35; outline: none;
    }
    .composer textarea:focus { border-color: #0a84ff; background: #fff;
      box-shadow: 0 0 0 3px rgba(10,132,255,.2); }
    .composer .iconbtn {
      appearance: none; border: 0; cursor: pointer;
      width: 40px; height: 40px; border-radius: 50%;
      display: grid; place-items: center;
      color: #fff;
    }
    .composer .mic  { background: #64748b; }
    .composer .mic.on  { background: #ef4444; animation: pulse 1.4s infinite; }
    .composer .send { background: #0a84ff; }
    @keyframes pulse {
      0%,100% { box-shadow: 0 0 0 0 rgba(239,68,68,.6); }
      70%     { box-shadow: 0 0 0 10px rgba(239,68,68,0); }
    }

    /* Empty-state helper text */
    .empty {
      padding: 24px 16px; text-align: center; color: #64748b;
    }
    .empty h3 { margin: 0 0 6px; color: #0f172a; font-size: 15px; }
    .empty p  { margin: 0; font-size: 13px; line-height: 1.5; }
  `;

  constructor() {
    super();
    this.open = false;
    this.label = 'ERP Copilot';
    this._suggestions = [];
    this._matches = [];
    this._query = '';
    this._messages = [];
    this._pendingForm = null;
    this._listening = false;
    this._busy = false;
    this._cursor = -1;
    this._agentBase = '/agent';
  }

  connectedCallback() {
    super.connectedCallback();
    this._kbd = (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'j') {
        e.preventDefault(); this._toggle();
      }
      if (e.key === 'Escape' && this.open) this._close();
    };
    document.addEventListener('keydown', this._kbd);
    // Prefetch proactive suggestions so the first open feels instant.
    queueMicrotask(() => this._loadSuggestions());
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener('keydown', this._kbd);
    try { this._recognizer?.abort(); } catch (_) {}
  }

  _open()   { this.open = true; setTimeout(() => this.$('textarea')?.focus(), 100); }
  _close()  { this.open = false; }
  _toggle() { this.open ? this._close() : this._open(); }

  // ── Data fetches ────────────────────────────────────────────────────
  async _loadSuggestions() {
    try {
      const r = await fetch(`${this._agentBase}/suggest?page=${encodeURIComponent(location.pathname)}`);
      if (r.ok) this._suggestions = await r.json();
    } catch (e) { console.warn('[copilot-v2] suggest failed', e); }
  }

  async _loadMatches(q) {
    this._query = q;
    if (!q.trim()) { this._matches = []; this._cursor = -1; return; }
    try {
      const r = await fetch(`${this._agentBase}/complete?q=${encodeURIComponent(q)}`);
      if (r.ok) {
        this._matches = await r.json();
        this._cursor = this._matches.length ? 0 : -1;
      }
    } catch (e) { console.warn('[copilot-v2] complete failed', e); }
  }

  // ── User actions ────────────────────────────────────────────────────
  _onInput(e) {
    const v = e.target.value;
    // Debounce would go here in real code; keep simple for demo.
    this._loadMatches(v);
  }

  _onKeydown(e) {
    if (!this._matches.length) {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        this._sendAsQuestion(e.target.value);
      }
      return;
    }
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      this._cursor = (this._cursor + 1) % this._matches.length;
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      this._cursor = (this._cursor - 1 + this._matches.length) % this._matches.length;
    } else if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      const pick = this._matches[this._cursor];
      if (pick) this._runAction(pick);
      else this._sendAsQuestion(e.target.value);
    }
  }

  _sendAsQuestion(text) {
    text = (text || '').trim();
    if (!text) return;
    this._pushUser(text);
    this.$('textarea').value = '';
    this._matches = [];
    // Ask the mock agent to answer freeform (fallback path).
    this._invoke('ask', { text });
  }

  // ── Suggestion / Autocomplete tap ───────────────────────────────────
  async _runAction(item) {
    // Clear input + matches; the user has committed.
    this.$('textarea').value = '';
    this._matches = [];
    this._pushUser(item.title);

    // Fetch tool schema. If it has required args we can't fill from context,
    // show an inline form. Otherwise invoke immediately with a confirm step
    // if the tool is mutating.
    const schema = await this._getSchema(item.tool);
    const prefill = item.prefill || {};
    const missing = (schema.required || []).filter(k => !(k in prefill));

    if (missing.length) {
      this._pendingForm = { tool: item.tool, title: item.title, schema, values: prefill };
    } else if (schema.mutating) {
      this._pushConfirm(item.tool, item.title, prefill, schema);
    } else {
      this._invoke(item.tool, prefill);
    }
  }

  async _getSchema(tool) {
    try {
      const r = await fetch(`${this._agentBase}/schema?tool=${encodeURIComponent(tool)}`);
      if (r.ok) return await r.json();
    } catch (e) { console.warn('[copilot-v2] schema failed', e); }
    return { required: [], fields: [], mutating: false };
  }

  // ── Inline form submission ──────────────────────────────────────────
  _onFormChange(name, value) {
    if (!this._pendingForm) return;
    this._pendingForm = {
      ...this._pendingForm,
      values: { ...this._pendingForm.values, [name]: value },
    };
  }

  _submitForm() {
    if (!this._pendingForm) return;
    const { tool, title, schema, values } = this._pendingForm;
    // Basic required-field check.
    const missing = (schema.required || []).filter(k => !values[k]);
    if (missing.length) {
      alert('Please fill: ' + missing.join(', '));
      return;
    }
    this._pendingForm = null;
    if (schema.mutating) this._pushConfirm(tool, title, values, schema);
    else this._invoke(tool, values);
  }
  _cancelForm() { this._pendingForm = null; }

  // ── Confirmation before mutation ────────────────────────────────────
  _pushConfirm(tool, title, args, schema) {
    const id = 'c-' + Math.random().toString(36).slice(2, 8);
    this._messages = [...this._messages, {
      id, kind: 'confirm', tool, title, args, schema,
    }];
    this._scrollBottom();
  }

  _approveConfirm(id) {
    const msg = this._messages.find(m => m.id === id);
    if (!msg) return;
    this._messages = this._messages.filter(m => m.id !== id);
    this._invoke(msg.tool, msg.args);
  }

  _rejectConfirm(id) {
    this._messages = this._messages.filter(m => m.id !== id);
    this._pushBot({ kind: 'text', text: 'Cancelled — nothing was changed.' });
  }

  // ── Tool invocation (the shared code path) ──────────────────────────
  async _invoke(tool, args) {
    this._busy = true;
    // Show a pending shimmer.
    const pid = 'p-' + Math.random().toString(36).slice(2, 6);
    this._messages = [...this._messages,
      { id: pid, kind: 'pending', text: 'Working…' }];
    try {
      const r = await fetch(`${this._agentBase}/invoke`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tool, args }),
      });
      const data = await r.json();
      // Replace pending with result.
      this._messages = this._messages
        .filter(m => m.id !== pid)
        .concat([{ id: 'r-' + pid, kind: data.kind || 'text', ...data }]);
    } catch (e) {
      this._messages = this._messages
        .filter(m => m.id !== pid)
        .concat([{ id: 'e-' + pid, kind: 'text',
                   text: 'Something went wrong: ' + e.message }]);
    } finally {
      this._busy = false;
      this._scrollBottom();
    }
  }

  // ── Transcript helpers ──────────────────────────────────────────────
  _pushUser(text) {
    this._messages = [...this._messages,
      { id: 'u-' + Math.random().toString(36).slice(2, 6), kind: 'user', text }];
    this._scrollBottom();
  }
  _pushBot(msg) {
    this._messages = [...this._messages, { id: 'b-' + Math.random().toString(36).slice(2,6), ...msg }];
    this._scrollBottom();
  }
  _scrollBottom() {
    requestAnimationFrame(() => {
      const el = this.$('.messages');
      if (el) el.scrollTop = el.scrollHeight;
    });
  }

  // ── Voice input (Web Speech API stub) ───────────────────────────────
  _toggleMic() {
    const SR = window.SpeechRecognition || window.webkitSpeechRecognition;
    if (!SR) {
      this._pushBot({ kind: 'text',
        text: '🎙️ Voice input is not supported in this browser yet. ' +
              'Try Chrome or Edge on desktop, or Safari on iOS.' });
      return;
    }
    if (this._listening) {
      try { this._recognizer?.stop(); } catch (_) {}
      this._listening = false;
      return;
    }
    const rec = new SR();
    rec.lang = navigator.language || 'en-US';
    rec.interimResults = true;
    rec.continuous = false;
    rec.onresult = (e) => {
      const ta = this.$('textarea');
      let final = '', interim = '';
      for (let i = e.resultIndex; i < e.results.length; i++) {
        const t = e.results[i][0].transcript;
        if (e.results[i].isFinal) final += t; else interim += t;
      }
      ta.value = (final || interim).trim();
      this._loadMatches(ta.value);
    };
    rec.onend = () => { this._listening = false; this._recognizer = null; };
    rec.onerror = () => { this._listening = false; };
    rec.start();
    this._recognizer = rec;
    this._listening = true;
  }

  // ── Renderers ───────────────────────────────────────────────────────
  render() {
    return html`
      <button class="fab" @click=${() => this._open()} aria-label="Open Copilot" title="Copilot (Ctrl+J)">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 2l2.09 4.26L18.5 7l-3.25 3.16.77 4.49L12 12.77 7.98 14.65l.77-4.49L5.5 7l4.41-.74L12 2z"/>
        </svg>
      </button>
      <div class="scrim" @click=${() => this._close()}></div>
      <aside class="panel" role="dialog" aria-modal="true" aria-label=${this.label}>
        <div class="grabber" @click=${() => this._close()}></div>
        <header>
          <div class="title"><span class="dot"></span>${this.label}</div>
          <button class="iconbtn" title="Clear" @click=${() => this._clear()}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3 6h18M8 6V4h8v2m-9 0v14a2 2 0 002 2h6a2 2 0 002-2V6"/>
            </svg>
          </button>
          <button class="iconbtn" title="Close" @click=${() => this._close()}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M18 6L6 18M6 6l12 12"/>
            </svg>
          </button>
        </header>

        ${this._renderSuggestions()}
        ${this._renderMessages()}
        ${this._renderComposer()}
      </aside>
    `;
  }

  _renderSuggestions() {
    if (!this._suggestions.length) return nothing;
    return html`
      <div class="suggestions">
        <div class="hint">Try one of these — or type in your own words below</div>
        ${this._suggestions.map(s => html`
          <button class="chip" @click=${() => this._runAction(s)}>
            ${s.icon ?? ''} ${s.title}
          </button>
        `)}
      </div>
    `;
  }

  _renderMessages() {
    if (!this._messages.length && !this._pendingForm) {
      return html`
        <div class="messages">
          <div class="empty">
            <h3>Ask, tap, or speak</h3>
            <p>Try “mark attendance for grade 5”, tap a suggestion above,
               or press the mic to speak.</p>
          </div>
        </div>
      `;
    }
    return html`
      <div class="messages">
        ${this._messages.map(m => this._renderMessage(m))}
        ${this._pendingForm ? this._renderForm() : nothing}
      </div>
    `;
  }

  _renderMessage(m) {
    if (m.kind === 'user') {
      return html`<div class="msg-user"><div class="bubble">${m.text}</div></div>`;
    }
    if (m.kind === 'pending') {
      return html`<div class="msg-bot"><div class="bubble">
        <span>⏳</span><span>${m.text}</span>
      </div></div>`;
    }
    if (m.kind === 'confirm') {
      return html`<div class="msg-bot card"><div class="bubble">
        <div class="confirm-card">
          <h4>⚠️ Confirm: ${m.title}</h4>
          <div class="kv">
            ${Object.entries(m.args).map(([k, v]) => html`
              <dt>${k}</dt><dd>${String(v)}</dd>
            `)}
          </div>
          <div class="actions">
            <button class="btn ghost" @click=${() => this._rejectConfirm(m.id)}>Cancel</button>
            <button class="btn primary" @click=${() => this._approveConfirm(m.id)}>Confirm &amp; do it</button>
          </div>
        </div>
      </div></div>`;
    }
    if (m.kind === 'table') {
      return html`<div class="msg-bot card"><div class="bubble">
        ${m.title ? html`<div style="font-weight:600;margin-bottom:6px">${m.title}</div>` : nothing}
        <table class="table">
          <thead><tr>${m.columns.map(c => html`<th>${c.label}</th>`)}</tr></thead>
          <tbody>
            ${m.rows.map(r => html`<tr>${m.columns.map(c => html`
              <td>${this._renderCell(r[c.key], c)}</td>
            `)}</tr>`)}
          </tbody>
        </table>
        ${this._renderFollowups(m.followups)}
      </div></div>`;
    }
    if (m.kind === 'stat') {
      return html`<div class="msg-bot card"><div class="bubble">
        <div style="display:flex; justify-content:space-between; gap:10px; flex-wrap:wrap">
          ${m.stats.map(s => html`
            <div style="min-width:120px">
              <div style="color:#64748b; font-size:12px">${s.label}</div>
              <div style="font-size:22px; font-weight:600">${s.value}</div>
              ${s.hint ? html`<div style="font-size:11px; color:#94a3b8">${s.hint}</div>` : nothing}
            </div>
          `)}
        </div>
        ${this._renderFollowups(m.followups)}
      </div></div>`;
    }
    // default: text
    return html`<div class="msg-bot"><div class="bubble">
      <div>${m.text}</div>
      ${this._renderFollowups(m.followups)}
    </div></div>`;
  }

  _renderCell(v, col) {
    if (col.badge && v) return html`<span class="badge ${col.badge}">${v}</span>`;
    return v ?? '';
  }

  _renderFollowups(followups) {
    if (!followups || !followups.length) return nothing;
    return html`
      <div class="followups">
        ${followups.map(f => html`
          <button class="chip" @click=${() => this._runAction(f)}>${f.title}</button>
        `)}
      </div>
    `;
  }

  _renderForm() {
    const f = this._pendingForm;
    return html`
      <div class="msg-bot card"><div class="bubble">
        <div class="form-card">
          <h4>${f.title} — a couple of details, please</h4>
          ${f.schema.fields.map(field => html`
            <div class="field">
              <label>${field.label}${field.required ? ' *' : ''}</label>
              ${field.enum
                ? html`<select @change=${e => this._onFormChange(field.name, e.target.value)}>
                    <option value="">— choose —</option>
                    ${field.enum.map(opt => html`
                      <option value=${opt.value}
                              ?selected=${f.values[field.name] === opt.value}>
                        ${opt.label}
                      </option>
                    `)}
                  </select>`
                : html`<input type=${field.type || 'text'}
                              placeholder=${field.placeholder || ''}
                              .value=${f.values[field.name] || ''}
                              @input=${e => this._onFormChange(field.name, e.target.value)} />`}
              ${field.help ? html`<div style="font-size:11px;color:#94a3b8">${field.help}</div>` : nothing}
            </div>
          `)}
          <div class="form-actions">
            <button class="btn ghost" @click=${() => this._cancelForm()}>Cancel</button>
            <button class="btn primary" @click=${() => this._submitForm()}>Continue</button>
          </div>
        </div>
      </div></div>
    `;
  }

  _renderComposer() {
    return html`
      <div class="composer-wrap">
        <div class="autocomplete ${this._matches.length ? 'open' : ''}">
          ${this._matches.map((m, i) => html`
            <div class="ac-item ${i === this._cursor ? 'active' : ''}"
                 @mouseenter=${() => this._cursor = i}
                 @click=${() => this._runAction(m)}>
              <div class="icon">${m.icon ?? '•'}</div>
              <div class="text">
                <div class="title">${m.title}</div>
                ${m.subtitle ? html`<div class="sub">${m.subtitle}</div>` : nothing}
              </div>
              ${m.shortcut ? html`<div class="shortcut">${m.shortcut}</div>` : nothing}
            </div>
          `)}
        </div>
        <div class="composer">
          <textarea rows="1"
                    placeholder="Ask, type, or tap the mic…"
                    @input=${e => this._onInput(e)}
                    @keydown=${e => this._onKeydown(e)}></textarea>
          <button class="iconbtn mic ${this._listening ? 'on' : ''}"
                  title=${this._listening ? 'Stop listening' : 'Speak'}
                  @click=${() => this._toggleMic()}>
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="9" y="2" width="6" height="12" rx="3"/>
              <path d="M5 10a7 7 0 0014 0"/>
              <line x1="12" y1="19" x2="12" y2="23"/>
            </svg>
          </button>
          <button class="iconbtn send"
                  title="Send" @click=${() => this._sendAsQuestion(this.$('textarea').value)}>
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z"/>
            </svg>
          </button>
        </div>
      </div>
    `;
  }

  _clear() {
    this._messages = [];
    this._pendingForm = null;
  }
}

customElements.define('ui-copilot-v2', UICopilotV2);
