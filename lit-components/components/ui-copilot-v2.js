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
    /// Base URL for the agent endpoints (`/agent` default). Change to a
    /// per-tenant prefix like `/web/acme/agent` for multi-tenant setups.
    agentBase:    { type: String, attribute: 'agent-base' },
    // internal state
    _suggestions: { state: true },   // proactive chips shown above input
    _matches:     { state: true },   // live autocomplete
    _query:       { state: true },
    _messages:    { state: true },   // array of message objects
    _pendingForm: { state: true },   // {tool, schema, values} when a form is up
    _listening:   { state: true },   // mic on/off
    _busy:        { state: true },
    _cursor:      { state: true },   // highlighted index in matches list
    // @-mention typeahead — one popover shared by composer + form fields.
    _mention:     { state: true },   // {host, q, items} or null
    _mentionCursor: { state: true },
    // Streaming state during an SSE turn: {tool, step:{n,of,label}, bubbleId, aborter}
    _stream:      { state: true },
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

    /* @-mention chip inside the composer's textarea (rendered via a fake
       overlay for the demo; a full implementation would use a contenteditable
       or a decorated textarea). For now we keep mentions in a "chip strip"
       above the composer so it's mobile-friendly and requires no
       contenteditable hackery. */
    .mention-strip {
      display: flex; flex-wrap: wrap; gap: 4px;
      padding: 6px 12px 0; min-height: 0;
    }
    .mention-strip:empty { display: none; }
    .mention-chip {
      display: inline-flex; align-items: center; gap: 4px;
      padding: 3px 8px; border-radius: 999px;
      background: #eff6ff; color: #1e40af;
      border: 1px solid #bfdbfe;
      font-size: 12px;
    }
    .mention-chip button {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: #1e40af; font-size: 14px; line-height: 1; padding: 0 0 0 4px;
    }

    /* @-mention popover — appears above the input, floats over autocomplete */
    .mention-popover {
      position: absolute;
      bottom: 100%; left: 8px; right: 8px;
      background: #fff; border: 1px solid #e2e8f0;
      border-radius: 12px;
      box-shadow: 0 -12px 32px rgba(0,0,0,.10);
      max-height: 260px; overflow: auto;
      z-index: 10;
    }
    .mention-item {
      display: flex; align-items: center; gap: 10px;
      padding: 8px 12px; cursor: pointer;
      border-bottom: 1px solid #f8fafc;
    }
    .mention-item:last-child { border-bottom: 0; }
    .mention-item:hover,
    .mention-item.active { background: #eff6ff; }
    .mention-item .avatar {
      width: 28px; height: 28px; border-radius: 50%;
      background: #f1f5f9; display: grid; place-items: center;
      font-size: 14px;
    }
    .mention-item .info { flex: 1; min-width: 0; }
    .mention-item .info .name { font-size: 13px; font-weight: 500; }
    .mention-item .info .sub  { font-size: 11px; color: #64748b;
                                white-space: nowrap; overflow: hidden;
                                text-overflow: ellipsis; }
    .mention-item .type {
      font-size: 10px; text-transform: uppercase; letter-spacing: .04em;
      background: #f1f5f9; padding: 2px 6px; border-radius: 4px;
      color: #64748b;
    }

    /* Streaming step bar shown between messages + composer */
    .step-bar {
      display: flex; align-items: center; gap: 10px;
      padding: 8px 12px;
      font-size: 12px; color: #475569;
      background: linear-gradient(to right,
        rgba(10,132,255,.06), rgba(124,58,237,.04));
      border-top: 1px dashed #e2e8f0;
    }
    .step-bar .track {
      flex: 1; height: 4px; border-radius: 2px;
      background: #e2e8f0; overflow: hidden;
    }
    .step-bar .fill {
      height: 100%;
      background: linear-gradient(to right, #0a84ff, #7c3aed);
      transition: width .35s ease;
    }
    .step-bar .cancel {
      appearance: none; border: 0; background: #ef4444; color: #fff;
      padding: 4px 10px; border-radius: 999px; font-size: 11px;
      cursor: pointer;
    }

    /* Server-rendered tool cards from SSE — style matches the .card look */
    .tool-card {
      display: inline-flex; align-items: center; gap: 8px;
      padding: 8px 10px; border-radius: 10px;
      background: rgba(90,200,250,.14); color: #036;
      font-size: 13px;
      border: 1px dashed #5ac8fa; margin-top: 6px;
    }
    .tool-card code {
      font-family: ui-monospace, monospace; font-size: 11px;
      background: rgba(0,0,0,.05); padding: 1px 6px; border-radius: 4px;
    }
    .thinking {
      display: inline-flex; align-items: center; gap: 3px;
    }
    .thinking span {
      width: 5px; height: 5px; border-radius: 50%;
      background: currentColor;
      animation: bp 1s infinite ease-in-out;
    }
    .thinking span:nth-child(2) { animation-delay: .15s; }
    .thinking span:nth-child(3) { animation-delay: .30s; }
    @keyframes bp {
      0%,80%,100% { opacity: .2; transform: translateY(0); }
      40%          { opacity: 1;  transform: translateY(-2px); }
    }

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
    this._mention = null;
    this._mentionCursor = -1;
    this._composerMentions = []; // {id, label} chips above composer
    this._stream = null;
    this.agentBase = '/agent'; // reflected from `agent-base` attribute
    // Legacy alias — some internal methods still reference this._agentBase.
    // We keep them in sync via a getter below to avoid an intrusive refactor.
    // Recency / favorites tracking, persisted to localStorage.
    this._recent = [];       // most recent actions (max 6)
    this._favorites = [];    // pinned by user (max 6)
    // Client-side UI tools invoked by "core.*" tool responses. Ported from
    // v1 so the assistant can drive the app (navigate, theme, toast).
    this._uiTools = {
      navigate:  ({ href })  => href && (window.Turbo?.visit?.(href) ?? (location.href = href)),
      set_theme: ({ theme }) => { if (theme) document.documentElement.dataset.theme = theme; },
      toast:     ({ message, tone = 'info' }) => {
        const host = document.querySelector('ui-toast-host');
        host?.push?.({ message, tone });
      },
      highlight: ({ selector, ms = 1500 }) => {
        const el = document.querySelector(selector); if (!el) return;
        el.classList.add('ai-highlight');
        setTimeout(() => el.classList.remove('ai-highlight'), ms);
      },
      focus: ({ selector }) => document.querySelector(selector)?.focus(),
    };
  }

  connectedCallback() {
    super.connectedCallback();
    this._kbd = (e) => {
      // Ctrl/Cmd+J — global toggle, works on every page (ported from v1).
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'j') {
        e.preventDefault(); this._toggle();
      }
      // Esc: cancel stream if running, else close panel.
      if (e.key === 'Escape' && this.open) {
        if (this._stream) { this._cancelStream(); e.preventDefault(); return; }
        this._close();
      }
    };
    document.addEventListener('keydown', this._kbd);
    // Restore any persisted transcript + recent/favorites first, then
    // prefetch proactive suggestions so the first open feels instant.
    this._restoreState();
    queueMicrotask(() => this._loadSuggestions());
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener('keydown', this._kbd);
    try { this._recognizer?.abort(); } catch (_) {}
    try { this._stream?.aborter?.abort(); } catch (_) {}
  }

  // Legacy internal alias so the many `this._agentBase` reads in this file
  // continue to work while the public property is `agentBase`.
  get _agentBase() { return this.agentBase || '/agent'; }

  // ── Persistence (localStorage) ──────────────────────────────────────
  // Keyed by agent endpoint base so multi-tenant setups don't collide.
  get _storageKey() { return `ui-copilot:v2:${this._agentBase}`; }

  _saveState() {
    try {
      // Only persist "settled" messages — skip in-flight streams to avoid
      // dead references, and drop the pendingForm (transient UI).
      const messages = this._messages.filter(m => m.kind !== 'pending');
      const payload = {
        messages, recent: this._recent, favorites: this._favorites,
        savedAt: Date.now(),
      };
      localStorage.setItem(this._storageKey, JSON.stringify(payload));
    } catch (_) { /* private-mode, quota, etc. */ }
  }

  _restoreState() {
    try {
      const raw = localStorage.getItem(this._storageKey);
      if (!raw) return;
      const p = JSON.parse(raw);
      if (Array.isArray(p.messages))  this._messages  = p.messages;
      if (Array.isArray(p.recent))    this._recent    = p.recent;
      if (Array.isArray(p.favorites)) this._favorites = p.favorites;
    } catch (_) {}
  }

  _clearState() {
    try { localStorage.removeItem(this._storageKey); } catch (_) {}
  }

  // Bump an action into recents (dedup, cap at 6).
  _rememberAction(item) {
    if (!item?.tool) return;
    const dedup = this._recent.filter(r => r.tool !== item.tool || JSON.stringify(r.prefill) !== JSON.stringify(item.prefill));
    this._recent = [{ title: item.title, icon: item.icon, tool: item.tool, prefill: item.prefill || {} }, ...dedup].slice(0, 6);
    this._saveState();
  }

  _toggleFavorite(item) {
    const key = (i) => `${i.tool}|${JSON.stringify(i.prefill || {})}`;
    const k = key(item);
    const has = this._favorites.some(f => key(f) === k);
    this._favorites = has
      ? this._favorites.filter(f => key(f) !== k)
      : [{ title: item.title, icon: item.icon, tool: item.tool, prefill: item.prefill || {} }, ...this._favorites].slice(0, 6);
    this._saveState();
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

  // ── @-mention detection & typeahead ─────────────────────────────────
  // Called with the current textarea value and caret position. If the
  // caret is inside an "@word" token, we open the mention popover;
  // otherwise we close it and fall through to normal autocomplete.
  _detectMention(text, caret) {
    const before = text.slice(0, caret);
    const m = before.match(/(?:^|\s)@(\w*)$/);
    if (!m) { this._mention = null; return false; }
    this._openMention('composer', m[1]);
    return true;
  }

  async _openMention(host, q) {
    try {
      const url = `${this._agentBase}/entities?type=student&q=${encodeURIComponent(q)}`;
      const r = await fetch(url);
      if (!r.ok) return;
      const items = await r.json();
      this._mention = { host, q, items };
      this._mentionCursor = items.length ? 0 : -1;
    } catch (e) { console.warn('[copilot-v2] mentions', e); }
  }

  _pickMention(item) {
    if (!this._mention) return;
    if (this._mention.host === 'composer') {
      // Add a chip above the composer + strip the "@word" from the textarea.
      this._composerMentions = [...this._composerMentions, item];
      const ta = this.$('textarea');
      if (ta) {
        ta.value = ta.value.replace(/(^|\s)@\w*$/, '$1').trimEnd() + ' ';
        ta.focus();
      }
    } else if (this._mention.host.startsWith('form:')) {
      const field = this._mention.host.slice(5);
      this._onFormChange(field, item.id, item.label);
    }
    this._mention = null;
  }

  _removeComposerMention(id) {
    this._composerMentions = this._composerMentions.filter(m => m.id !== id);
  }

  // ── User actions ────────────────────────────────────────────────────
  _onInput(e) {
    const v = e.target.value;
    const caret = e.target.selectionStart ?? v.length;
    // Debounce would go here in real code; keep simple for demo.
    if (this._detectMention(v, caret)) {
      this._matches = []; // hide normal autocomplete while mentioning
    } else {
      this._loadMatches(v);
    }
  }

  _onKeydown(e) {
    // Mention popover takes priority when open.
    if (this._mention && this._mention.items.length) {
      const n = this._mention.items.length;
      if (e.key === 'ArrowDown') { e.preventDefault();
        this._mentionCursor = (this._mentionCursor + 1) % n; return; }
      if (e.key === 'ArrowUp')   { e.preventDefault();
        this._mentionCursor = (this._mentionCursor - 1 + n) % n; return; }
      if (e.key === 'Enter' || e.key === 'Tab') {
        e.preventDefault();
        const pick = this._mention.items[this._mentionCursor];
        if (pick) this._pickMention(pick);
        return;
      }
      if (e.key === 'Escape') { e.preventDefault(); this._mention = null; return; }
    }

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
    // If the user typed a message with @mentions, render both text + chips
    // in the transcript, then decide what to do based on whether mentions
    // resolved to actionable entities.
    const mentions = this._composerMentions;
    if (!text && !mentions.length) return;

    const echo = text + (mentions.length
      ? ' ' + mentions.map(m => `@${m.label}`).join(' ')
      : '');
    this._pushUser(echo);

    this.$('textarea').value = '';
    this._matches = [];
    this._composerMentions = [];

    // Simple demo heuristic: if the user mentioned a single student and
    // the text hints at reminding/fees, dispatch the direct tool. Otherwise
    // fall through to the free-form "ask" path (which is the LLM in prod).
    if (mentions.length === 1 &&
        /remind|fee|due|pay/.test(text.toLowerCase())) {
      this._invoke('student.remind_fees', { student_id: mentions[0].id });
      return;
    }
    this._invoke('ask', { text: echo, mentions: mentions.map(m => m.id) });
  }

  // ── Suggestion / Autocomplete tap ───────────────────────────────────
  async _runAction(item) {
    // Clear input + matches; the user has committed.
    const ta = this.$('textarea'); if (ta) ta.value = '';
    this._matches = [];
    this._rememberAction(item);
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
      this._pushConfirm(item.tool, item.title, prefill, schema, schema.streaming);
    } else if (schema.streaming) {
      this._invokeStream(item.tool, prefill);
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
  _onFormChange(name, value, label) {
    if (!this._pendingForm) return;
    // For mention fields we also keep the human label for display.
    const labels = { ...(this._pendingForm.labels || {}) };
    if (label != null) labels[name] = label;
    this._pendingForm = {
      ...this._pendingForm,
      values: { ...this._pendingForm.values, [name]: value },
      labels,
    };
  }

  _submitForm() {
    if (!this._pendingForm) return;
    const { tool, title, schema, values } = this._pendingForm;
    const missing = (schema.required || []).filter(k => !values[k]);
    if (missing.length) {
      alert('Please fill: ' + missing.join(', '));
      return;
    }
    this._pendingForm = null;
    if (schema.mutating) this._pushConfirm(tool, title, values, schema, schema.streaming);
    else if (schema.streaming) this._invokeStream(tool, values);
    else this._invoke(tool, values);
  }
  _cancelForm() { this._pendingForm = null; }

  // ── Confirmation before mutation ────────────────────────────────────
  _pushConfirm(tool, title, args, schema, streaming) {
    const id = 'c-' + Math.random().toString(36).slice(2, 8);
    this._messages = [...this._messages, {
      id, kind: 'confirm', tool, title, args, schema, streaming: !!streaming,
    }];
    this._scrollBottom();
  }

  _approveConfirm(id) {
    const msg = this._messages.find(m => m.id === id);
    if (!msg) return;
    this._messages = this._messages.filter(m => m.id !== id);
    if (msg.streaming) this._invokeStream(msg.tool, msg.args);
    else this._invoke(msg.tool, msg.args);
  }

  _rejectConfirm(id) {
    this._messages = this._messages.filter(m => m.id !== id);
    this._pushBot({ kind: 'text', text: 'Cancelled — nothing was changed.' });
  }

  // ── Streaming tool invocation (multi-step, SSE) ────────────────────
  //
  // Opens a POST /agent/stream connection, reads named SSE events, and
  // updates the live transcript in real time:
  //   step         → step bar (n/of + label + %)
  //   tool_call    → tool card added to a "streaming" bubble
  //   tool_result  → tool card added
  //   message_start→ starts an assistant bubble (server-rendered HTML)
  //   token        → append text into the last streaming bubble
  //   message_end  → finalise + close connection
  //   error        → red bubble + close connection
  //
  // Design invariant: server closes when done (drops sender); we ALSO
  // abort() on message_end so we never hold an idle socket.
  async _invokeStream(tool, args) {
    // Create an initial "streaming" message that all tool_call / tool_result /
    // message_start events accumulate into. Keeps the transcript tidy.
    const id = 's-' + Math.random().toString(36).slice(2, 6);
    const streamMsg = { id, kind: 'streaming', html: '', tool };
    this._messages = [...this._messages, streamMsg];
    this._busy = true;
    this._stream = { tool, step: null, bubbleId: null, aborter: new AbortController() };
    this._scrollBottom();

    try {
      const resp = await fetch(`${this._agentBase}/stream`, {
        method: 'POST',
        signal: this._stream.aborter.signal,
        headers: { 'Content-Type': 'application/json', 'Accept': 'text/event-stream' },
        body: JSON.stringify({ tool, args }),
      });
      if (!resp.ok || !resp.body) throw new Error('stream http ' + resp.status);
      await this._readSse(resp.body);
    } catch (e) {
      if (e.name !== 'AbortError') {
        this._appendStreamingHtml(id,
          `<div class="tool-card" style="border-color:#ef4444;color:#991b1b;background:#fee2e2">⚠️ ${e.message}</div>`);
      }
    } finally {
      this._busy = false;
      this._stream = null;
      this._scrollBottom();
      // Persist the final transcript (streaming HTML included).
      this._saveState();
    }
  }

  async _readSse(body) {
    const reader = body.getReader();
    const dec = new TextDecoder();
    let buf = '';
    try {
      while (true) {
        const { value, done } = await reader.read();
        if (done) break;
        buf += dec.decode(value, { stream: true });
        let sep;
        while ((sep = buf.indexOf('\n\n')) >= 0) {
          this._handleSseEvent(buf.slice(0, sep));
          buf = buf.slice(sep + 2);
        }
      }
    } catch (e) { if (e?.name !== 'AbortError') throw e; }
    finally { try { reader.releaseLock(); } catch (_) {} }
  }

  _handleSseEvent(chunk) {
    let event = 'message', data = [];
    for (const line of chunk.split('\n')) {
      if (line.startsWith(':')) continue;
      if (line.startsWith('event:')) event = line.slice(6).trim();
      else if (line.startsWith('data:')) data.push(line.slice(5).replace(/^ /, ''));
    }
    const payload = data.join('\n');
    const streamMsg = this._messages[this._messages.length - 1];
    if (!streamMsg || streamMsg.kind !== 'streaming') return;

    switch (event) {
      case 'step': {
        try { this._stream = { ...this._stream, step: JSON.parse(payload) }; }
        catch (_) {}
        break;
      }
      case 'tool_call':
      case 'tool_result':
      case 'message_start': {
        this._appendStreamingHtml(streamMsg.id, payload);
        if (event === 'message_start') {
          // Extract the bubble id so subsequent tokens target it.
          const m = payload.match(/id="([^"]+)"/);
          if (m) this._stream = { ...this._stream, bubbleId: m[1] };
        }
        break;
      }
      case 'token': {
        let id, text;
        try { ({ id, text } = JSON.parse(payload)); } catch { text = payload; }
        this._appendTokenToBubble(streamMsg.id, id || this._stream?.bubbleId, text);
        break;
      }
      case 'message_end': {
        // Server has done its job; close the socket instantly.
        try { this._stream?.aborter?.abort(); } catch (_) {}
        break;
      }
      case 'error': {
        let msg = payload;
        try { msg = JSON.parse(payload).message ?? payload; } catch {}
        this._appendStreamingHtml(streamMsg.id,
          `<div class="tool-card" style="border-color:#ef4444;color:#991b1b;background:#fee2e2">⚠️ ${msg}</div>`);
        try { this._stream?.aborter?.abort(); } catch (_) {}
        break;
      }
    }
  }

  _appendStreamingHtml(streamId, html) {
    this._messages = this._messages.map(m =>
      m.id === streamId ? { ...m, html: m.html + html } : m);
    this._scrollBottom();
  }

  _appendTokenToBubble(streamId, bubbleId, text) {
    if (!text) return;
    // Live DOM append into the specific bubble — cheaper than re-rendering
    // and keeps the type-writer effect smooth.
    const el = this.renderRoot?.querySelector(
      `[data-stream="${streamId}"] #${CSS.escape(bubbleId)}`);
    if (el) {
      el.textContent += text;
      this._scrollBottom();
    } else {
      // Fallback: put it in the html blob so next render picks it up.
      this._appendStreamingHtml(streamId, text);
    }
  }

  _cancelStream() {
    try { this._stream?.aborter?.abort(); } catch (_) {}
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
      // If the tool returned a `ui_action` block, run it client-side.
      // Examples: {ui_action: {action: "navigate", href: "/dsl/dashboard"}}
      if (data.ui_action) this._runUiAction(data.ui_action);
      // Replace pending with result.
      this._messages = this._messages
        .filter(m => m.id !== pid)
        .concat([{ id: 'r-' + pid, kind: data.kind || 'text', ...data }]);
      this._saveState();
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
    // Recent + favorites always show if we have any — most valuable UX
    // real estate. Suggestions come second (proactive from server).
    const hasQuickAccess = this._favorites.length || this._recent.length;
    const hasSuggestions = this._suggestions.length;
    if (!hasQuickAccess && !hasSuggestions) return nothing;
    return html`
      <div class="suggestions">
        ${this._favorites.length ? html`
          <div class="hint">⭐ Pinned</div>
          ${this._favorites.map(f => html`
            <button class="chip" title="Pinned — click to run"
                    @click=${() => this._runAction(f)}
                    @contextmenu=${(e) => { e.preventDefault(); this._toggleFavorite(f); }}>
              ${f.icon ?? ''} ${f.title}
            </button>
          `)}
        ` : nothing}
        ${this._recent.length ? html`
          <div class="hint">🕘 Recent</div>
          ${this._recent.map(r => html`
            <button class="chip" title="Recent — right-click to pin"
                    @click=${() => this._runAction(r)}
                    @contextmenu=${(e) => { e.preventDefault(); this._toggleFavorite(r); }}>
              ${r.icon ?? ''} ${r.title}
            </button>
          `)}
        ` : nothing}
        ${hasSuggestions ? html`
          <div class="hint">${hasQuickAccess ? '💡 Suggested for this page' : 'Try one of these — or type in your own words below'}</div>
          ${this._suggestions.map(s => html`
            <button class="chip"
                    @click=${() => this._runAction(s)}
                    @contextmenu=${(e) => { e.preventDefault(); this._toggleFavorite(s); }}>
              ${s.icon ?? ''} ${s.title}
            </button>
          `)}
        ` : nothing}
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
    if (m.kind === 'streaming') {
      // Server sends pre-styled HTML (tool cards + bubble). We inject via
      // `.innerHTML` because it's trusted (our own server) and we need to
      // preserve the ids so token appends can target specific bubbles.
      return html`<div class="msg-bot card"><div class="bubble">
        <div data-stream=${m.id} .innerHTML=${m.html}></div>
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
                : field.type === 'mention'
                ? html`<div style="position:relative">
                    <input type="text"
                           placeholder=${field.placeholder || 'Type @ to search'}
                           .value=${(f.labels && f.labels[field.name])
                                    ? '@' + f.labels[field.name]
                                    : ''}
                           @focus=${() => this._openMention(`form:${field.name}`, '')}
                           @input=${e => {
                             // Strip leading "@" then re-search.
                             const q = e.target.value.replace(/^@/, '');
                             this._openMention(`form:${field.name}`, q);
                           }} />
                  </div>`
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

  _renderStepBar() {
    if (!this._stream || !this._stream.step) return nothing;
    const { n, of, label } = this._stream.step;
    const pct = Math.round((n / of) * 100);
    return html`
      <div class="step-bar">
        <span>Step ${n}/${of} · ${label}</span>
        <div class="track"><div class="fill" style="width:${pct}%"></div></div>
        <button class="cancel" @click=${() => this._cancelStream()}>Cancel</button>
      </div>
    `;
  }

  _renderMentionStrip() {
    if (!this._composerMentions.length) return nothing;
    return html`
      <div class="mention-strip">
        ${this._composerMentions.map(m => html`
          <span class="mention-chip">
            @${m.label}
            <button @click=${() => this._removeComposerMention(m.id)} title="Remove">×</button>
          </span>
        `)}
      </div>
    `;
  }

  _renderMentionPopover() {
    if (!this._mention || !this._mention.items.length) return nothing;
    return html`
      <div class="mention-popover">
        ${this._mention.items.map((it, i) => html`
          <div class="mention-item ${i === this._mentionCursor ? 'active' : ''}"
               @mouseenter=${() => this._mentionCursor = i}
               @click=${() => this._pickMention(it)}>
            <div class="avatar">${it.icon || '•'}</div>
            <div class="info">
              <div class="name">${it.label}</div>
              <div class="sub">${it.subtitle}</div>
            </div>
            <span class="type">${it.kind}</span>
          </div>
        `)}
      </div>
    `;
  }

  _renderComposer() {
    return html`
      ${this._renderStepBar()}
      ${this._renderMentionStrip()}
      <div class="composer-wrap">
        ${this._renderMentionPopover()}
        <div class="autocomplete ${this._matches.length && !this._mention ? 'open' : ''}">
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
    // Wipe transcript + persisted state, but preserve favorites (user pins).
    this._messages = [];
    this._pendingForm = null;
    this._composerMentions = [];
    this._recent = [];
    this._clearState();
    this._saveState();
  }

  // Dispatch a `ui_action` payload from a tool response — the assistant's
  // way of driving the app (navigate, focus, theme, toast, highlight).
  _runUiAction(a) {
    if (!a || !a.action) return;
    const fn = this._uiTools[a.action];
    if (fn) fn(a);
    else console.warn('[ui-copilot] unknown ui_action', a);
  }

  // Public API for host pages / tests. Kept stable across v1 → v2.
  openPanel()  { this._open(); }
  closePanel() { this._close(); }
}

// Register under BOTH names during the transition, so any HTML that still
// uses `<ui-copilot-v2>` keeps working. New consumers should use
// `<ui-copilot>` (the primary, unified element).
customElements.define('ui-copilot', UICopilotV2);
if (!customElements.get('ui-copilot-v2')) {
  customElements.define('ui-copilot-v2', class extends UICopilotV2 {});
}

// Global .ai-highlight style so `highlight` UI-action has a visible effect
// even on host pages that don't include their own copy.
if (typeof document !== 'undefined' && !document.getElementById('ai-highlight-style')) {
  const s = document.createElement('style');
  s.id = 'ai-highlight-style';
  s.textContent = `
    .ai-highlight {
      outline: 2px solid var(--color-primary, #0a84ff) !important;
      outline-offset: 2px;
      animation: ai-pulse 1.2s ease-in-out;
    }
    @keyframes ai-pulse {
      0%,100% { box-shadow: 0 0 0 0 rgba(10,132,255,.28); }
      50%     { box-shadow: 0 0 0 8px rgba(10,132,255,.28); }
    }
  `;
  document.head.appendChild(s);
}
