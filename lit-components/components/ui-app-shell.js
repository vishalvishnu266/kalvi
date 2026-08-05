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
//
// Note: no animations. Island swaps are direct DOM mutations for maximum
// snappiness. If a future need arises (e.g. Capacitor/Flutter wrapper),
// reintroduce them at the `applyAll` choke-point in `framework/shell.js`.
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

class UiAppShell extends LitBaseElement {
  static styles = css`
    :host {
      display: grid;
      grid-template-areas:
        "topbar   topbar    topbar"
        "sidebar  main      copilot"
        "toast    toast     toast"
        "modal    modal     modal"
        "bottombar bottombar bottombar";
      /* Desktop: 0px sidebar by default (empty slot reserved for
         per-page power tools), fluid main, no copilot column (copilot
         now floats as a card — see ui-copilot.js). */
      grid-template-columns: var(--shell-sidebar, 0px) 1fr 0px;
      grid-template-rows: auto 1fr auto auto auto;
      min-height: 100dvh;
      background: var(--color-bg, #fff);
      color: var(--color-fg, #111);
    }
    /* Each named slot lives inside a positioned box so we can add
       overflow rules per region without leaking to others. */
    .region { min-width: 0; min-height: 0; }
    .topbar    { grid-area: topbar;    border-bottom: 1px solid var(--color-border, #e5e7eb); padding: 6px 12px; }
    .sidebar   { grid-area: sidebar;   border-right:  1px solid var(--color-border, #e5e7eb); overflow: auto; }
    .main      { grid-area: main;      overflow: auto; }
    .copilot   { grid-area: copilot;   border-left:   1px solid var(--color-border, #e5e7eb); overflow: auto; }
    .toast     { grid-area: toast;     position: sticky; bottom: 0; pointer-events: none; }
    .modal     { grid-area: modal;     position: sticky; bottom: 0; z-index: 100; }
    /* Bottom bar hidden on desktop (mobile tab bar is a mobile-only
       navigation surface). The nested .ui-tab-bar CSS in shell.rs
       drives visibility via a media query. */
    .bottombar { grid-area: bottombar; }
    .toast > ::slotted(*), .modal > ::slotted(*) { pointer-events: auto; }

    /* Mobile layout: single column, activity bar hidden, bottom tab
       bar visible, copilot becomes a bottom sheet driven by the open
       attribute on ui-copilot. */
    @media (max-width: 768px) {
      :host {
        grid-template-areas:
          "topbar"
          "main"
          "toast"
          "modal"
          "bottombar";
        grid-template-columns: 1fr;
      }
      .sidebar { display: none; }
      /* The primary bar is fixed-positioned by its own CSS, so the
         bottombar slot just needs to reserve visual space so the last
         line of content isn't hidden behind it. */
      .bottombar { min-height: 66px; }
      .copilot {
        position: fixed;
        left: 0; right: 0; bottom: 0;
        height: 75dvh;
        border-left: 0;
        border-top: 1px solid var(--color-border, #e5e7eb);
        border-top-left-radius: 18px;
        border-top-right-radius: 18px;
        background: var(--color-bg, #fff);
        box-shadow: 0 -8px 24px rgba(0, 0, 0, .12);
        transform: translateY(100%);
        transition: transform .28s cubic-bezier(.2, .8, .2, 1);
        z-index: 90;
        pointer-events: none;
      }
      .copilot:has(ui-copilot[open]) {
        transform: translateY(0);
        pointer-events: auto;
      }
      .copilot::before {
        content: '';
        display: block;
        width: 40px; height: 4px;
        background: var(--color-border, #e5e7eb);
        border-radius: 2px;
        margin: 8px auto 0;
      }
    }
  `;

  connectedCallback() {
    super.connectedCallback();
    // shell.js already booted eagerly at module load; the call here is
    // a no-op safety net for hosts that load the element without going
    // through the normal component bundle.
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
      <div class="region bottombar"><slot name="bottombar"></slot></div>
    `;
  }
}

if (!customElements.get('ui-app-shell')) {
  customElements.define('ui-app-shell', UiAppShell);
}
