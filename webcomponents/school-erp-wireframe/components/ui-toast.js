import { BaseElement, attr } from './base.js';

/**
 * <ui-toast> — a single toast card. Rarely used on its own; use the
 * global helper `toast('Message', { tone: 'success' })` instead.
 *
 * <ui-toast-host> — the container, mounted once by the helper.
 */

class UIToast extends BaseElement {
  static styles = `
    :host {
      display: block;
      min-width: 280px; max-width: 420px;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      box-shadow: var(--shadow-lg);
      padding: 12px 14px;
      display: grid;
      grid-template-columns: auto 1fr auto;
      gap: 10px; align-items: start;
      animation: in var(--dur-med) var(--ease-spring);
      backdrop-filter: var(--chrome-blur);
      -webkit-backdrop-filter: var(--chrome-blur);
    }
    :host([leaving]) { animation: out var(--dur-med) var(--ease) forwards; }
    @keyframes in  { from { transform: translateX(24px); opacity: 0; } to { transform: none; opacity: 1; } }
    @keyframes out { to   { transform: translateX(24px); opacity: 0; } }

    .icon {
      width: 28px; height: 28px; border-radius: 8px;
      display: grid; place-items: center;
      background: var(--color-primary-soft);
      color: var(--color-primary);
    }
    :host([tone="success"]) .icon { background: color-mix(in srgb, var(--color-success) 15%, transparent); color: var(--color-success); }
    :host([tone="warning"]) .icon { background: color-mix(in srgb, var(--color-warning) 15%, transparent); color: var(--color-warning); }
    :host([tone="danger"])  .icon { background: color-mix(in srgb, var(--color-danger)  15%, transparent); color: var(--color-danger);  }
    :host([tone="info"])    .icon { background: color-mix(in srgb, var(--color-info)    15%, transparent); color: var(--color-info); }

    .body .t { font-size: var(--fs-sm); font-weight: var(--fw-semibold); color: var(--color-text); }
    .body .d { font-size: var(--fs-xs); color: var(--color-text-muted); margin-top: 2px; }

    .close {
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-text-subtle); width: 22px; height: 22px;
      border-radius: 4px; display: grid; place-items: center;
    }
    .close:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); color: var(--color-text); }

    /* Progress bar showing auto-dismiss */
    .bar {
      grid-column: 1 / -1; height: 2px; margin-top: 8px;
      background: var(--color-surface-alt); border-radius: 999px; overflow: hidden;
    }
    .bar > span {
      display: block; height: 100%; width: 100%;
      background: var(--color-primary);
      transform-origin: left;
      animation: shrink var(--dur, 3500ms) linear forwards;
    }
    :host([tone="success"]) .bar > span { background: var(--color-success); }
    :host([tone="warning"]) .bar > span { background: var(--color-warning); }
    :host([tone="danger"])  .bar > span { background: var(--color-danger); }
    :host([tone="info"])    .bar > span { background: var(--color-info); }
    @keyframes shrink { to { transform: scaleX(0); } }
  `;
  render() {
    const tone  = attr(this, 'tone', 'info');
    const title = attr(this, 'title', '');
    const desc  = attr(this, 'desc', '');
    const dur   = attr(this, 'duration', '3500');
    const iconName = { success: 'check', warning: 'bell', danger: 'x', info: 'activity' }[tone] || 'activity';
    return `
      <div class="icon"><ui-icon name="${iconName}" size="16"></ui-icon></div>
      <div class="body">
        <div class="t">${title}</div>
        ${desc ? `<div class="d">${desc}</div>` : ''}
      </div>
      <button class="close" title="Dismiss"><ui-icon name="x" size="14"></ui-icon></button>
      <div class="bar" style="--dur:${parseInt(dur,10)}ms"><span></span></div>
    `;
  }
  afterRender() {
    this.$('.close').addEventListener('click', () => this.dismiss());
    const dur = parseInt(attr(this, 'duration', '3500'), 10);
    if (dur > 0) this._t = setTimeout(() => this.dismiss(), dur);
  }
  dismiss() {
    clearTimeout(this._t);
    this.setAttribute('leaving', '');
    setTimeout(() => this.remove(), 240);
  }
}
customElements.define('ui-toast', UIToast);

/* ----- Host container: mount once, stack toasts bottom-right ----- */
class UIToastHost extends BaseElement {
  static styles = `
    :host {
      position: fixed; z-index: 9999;
      right: 16px; bottom: 16px;
      display: flex; flex-direction: column-reverse; gap: 10px;
      pointer-events: none;
    }
    ::slotted(*) { pointer-events: auto; }
    @media (max-width: 640px) {
      :host { right: 8px; bottom: calc(var(--bottomnav-h, 62px) + 16px + env(safe-area-inset-bottom, 0)); left: 8px; }
    }
  `;
  render() { return `<slot></slot>`; }
}
customElements.define('ui-toast-host', UIToastHost);

/* ----- Global helper: window.toast('Saved', { tone: 'success' }) ----- */
function ensureHost() {
  let host = document.querySelector('ui-toast-host');
  if (!host) { host = document.createElement('ui-toast-host'); document.body.appendChild(host); }
  return host;
}
export function toast(title, opts = {}) {
  const el = document.createElement('ui-toast');
  el.setAttribute('title', title);
  if (opts.desc)     el.setAttribute('desc',     opts.desc);
  if (opts.tone)     el.setAttribute('tone',     opts.tone);
  if (opts.duration) el.setAttribute('duration', String(opts.duration));
  ensureHost().appendChild(el);
  return el;
}
// expose on window so plain HTML pages can use it without imports
if (typeof window !== 'undefined') window.toast = toast;
