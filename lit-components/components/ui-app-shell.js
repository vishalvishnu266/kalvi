// -----------------------------------------------------------------------------
// <ui-app-shell> — the ONLY layout element in the framework.
//
// Six named slots (a.k.a. "islands"). The server places initial content
// into them via `slot="…"` on the child elements; runtime navigation
// swaps them via <ui-fragment target="…"> envelopes.
//
// Layout:
//
//   ┌──────────────────────────────────────────────────┐
//   │                    topbar                         │
//   ├─────────┬────────────────────────────┬────────────┤
//   │         │                            │            │
//   │ sidebar │           main             │  copilot   │
//   │         │                            │            │
//   ├─────────┴────────────────────────────┴────────────┤
//   │                     toast host                    │
//   │                     modal host                    │
//   └──────────────────────────────────────────────────┘
//
// On narrow viewports (<768px) sidebar and copilot become slide-over
// panels — but that CSS work lands in milestone 9 (mobile polish).
// -----------------------------------------------------------------------------

import { LitBaseElement, html, css } from './base.js';
import { bootShell, registerSideEffect } from '../framework/shell.js';

// ---- Built-in side effects -------------------------------------------------
// Registered once, at module load. Servers can trigger these by emitting:
//   <ui-fragment target="__side_effect__" kind="theme">{"theme":"dark"}</ui-fragment>
registerSideEffect('theme', ({ theme } = {}) => {
  if (theme === 'dark' || theme === 'light') {
    document.documentElement.dataset.theme = theme;
    try { localStorage.setItem('erp.theme', theme); } catch { /* ignore */ }
  }
});
registerSideEffect('focus', ({ selector } = {}) => {
  if (!selector) return;
  const el = document.querySelector(selector);
  if (el && 'focus' in el) el.focus();
});
registerSideEffect('scroll', ({ selector = 'body', top = 0 } = {}) => {
  const el = document.querySelector(selector);
  if (el) el.scrollTo({ top, behavior: 'smooth' });
});

// ---- View-transition keyframes --------------------------------------------
// The `::view-transition-*` pseudo-elements live on the document root,
// not inside our shadow tree, so we install a document-level <style> tag
// once. This gives the `main` island a soft fade + tiny slide on every
// island swap. Browsers without View Transitions ignore this entirely.
let transitionStyleInstalled = false;
function installTransitionStyle() {
  if (transitionStyleInstalled) return;
  transitionStyleInstalled = true;
  const style = document.createElement('style');
  style.dataset.uiShell = 'view-transitions';
  style.textContent = `
    @keyframes ui-fade-in  { from { opacity: 0; transform: translateY(4px); } }
    @keyframes ui-fade-out { to   { opacity: 0; transform: translateY(-4px); } }
    ::view-transition-old(shell-main) {
      animation: ui-fade-out .18s ease-in both;
    }
    ::view-transition-new(shell-main) {
      animation: ui-fade-in  .22s ease-out both;
    }
  `;
  document.head.appendChild(style);
}

class UiAppShell extends LitBaseElement {
  static styles = css`
    :host {
      display: grid;
      grid-template-areas:
        "topbar  topbar   topbar"
        "sidebar main     copilot"
        "toast   toast    toast"
        "modal   modal    modal";
      grid-template-columns: var(--shell-sidebar, 240px) 1fr var(--shell-copilot, 360px);
      grid-template-rows: auto 1fr auto auto;
      min-height: 100dvh;
      background: var(--color-bg, #fff);
      color: var(--color-fg, #111);
    }
    /* Each named slot lives inside a positioned box so we can add
       transitions / overflow rules per region without leaking to others. */
    .region { min-width: 0; min-height: 0; }
    .topbar   { grid-area: topbar;   border-bottom: 1px solid var(--color-border, #e5e7eb); }
    .sidebar  { grid-area: sidebar;  border-right:  1px solid var(--color-border, #e5e7eb); overflow: auto; }
    .main     { grid-area: main;     overflow: auto; view-transition-name: shell-main; }
    .copilot  { grid-area: copilot;  border-left:   1px solid var(--color-border, #e5e7eb); overflow: auto; }
    .toast    { grid-area: toast;    position: sticky; bottom: 0; pointer-events: none; }
    .modal    { grid-area: modal;    position: sticky; bottom: 0; z-index: 100; }
    .toast > ::slotted(*), .modal > ::slotted(*) { pointer-events: auto; }

    /* Mobile: collapse into a single column, hide sidebar/copilot by
       default (they'll be revealed via ui-fragment swaps + open state
       once the mobile shell is designed in milestone 9). */
    @media (max-width: 768px) {
      :host {
        grid-template-areas:
          "topbar"
          "main"
          "toast"
          "modal";
        grid-template-columns: 1fr;
      }
      .sidebar, .copilot { display: none; }
    }
  `;

  connectedCallback() {
    super.connectedCallback();
    installTransitionStyle();
    bootShell();
  }

  render() {
    return html`
      <div class="region topbar"><slot name="topbar"></slot></div>
      <div class="region sidebar"><slot name="sidebar"></slot></div>
      <div class="region main"><slot name="main"></slot></div>
      <div class="region copilot"><slot name="copilot"></slot></div>
      <div class="region toast"><slot name="toast"></slot></div>
      <div class="region modal"><slot name="modal"></slot></div>
    `;
  }
}

if (!customElements.get('ui-app-shell')) {
  customElements.define('ui-app-shell', UiAppShell);
}
