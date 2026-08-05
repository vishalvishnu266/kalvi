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

// ---- View-transition presets ----------------------------------------------
//
// The `::view-transition-*` pseudo-elements live on the document root,
// not inside our shadow tree, so we install a document-level <style> tag
// once, keyed on `<html data-ui-transition="…">`. Change the attribute
// (or call `window.ui.setTransition(name)`) to pick a preset without
// reloading. Individual links can override per-nav by adding
// `data-transition="…"` — the shell.js interceptor reads it and swaps
// the attribute for the duration of that navigation.
//
// Presets:
//   • `fade`         (default) — soft crossfade + 4px vertical drift
//   • `slide-left`   — new page slides in from the right, old slides out left
//   • `slide-right`  — mirror of slide-left (feels like "back")
//   • `slide-up`     — new page rises from the bottom
//   • `scale`        — subtle zoom in + fade
//   • `none`         — instant swap, keeps the View Transition scaffolding
//                      (still smoother than raw DOM mutation in Chrome)
//
// Durations are intentionally short (160–260ms) so the app feels snappy.
let transitionStyleInstalled = false;
function installTransitionStyle() {
  if (transitionStyleInstalled) return;
  transitionStyleInstalled = true;
  const style = document.createElement('style');
  style.dataset.uiShell = 'view-transitions';
  style.textContent = `
    /* ─── Shared keyframes ─── */
    @keyframes ui-fade-in       { from { opacity: 0; } }
    @keyframes ui-fade-out      { to   { opacity: 0; } }
    @keyframes ui-drift-in      { from { opacity: 0; transform: translateY(4px);  } }
    @keyframes ui-drift-out     { to   { opacity: 0; transform: translateY(-4px); } }
    @keyframes ui-slide-in-r    { from { transform: translateX( 24px); opacity: 0; } }
    @keyframes ui-slide-out-l   { to   { transform: translateX(-24px); opacity: 0; } }
    @keyframes ui-slide-in-l    { from { transform: translateX(-24px); opacity: 0; } }
    @keyframes ui-slide-out-r   { to   { transform: translateX( 24px); opacity: 0; } }
    @keyframes ui-slide-in-up   { from { transform: translateY( 20px); opacity: 0; } }
    @keyframes ui-slide-out-up  { to   { transform: translateY(-20px); opacity: 0; } }
    @keyframes ui-scale-in      { from { transform: scale(0.98); opacity: 0; } }
    @keyframes ui-scale-out     { to   { transform: scale(1.02); opacity: 0; } }

    /* ─── Preset: fade (default) ─── */
    html[data-ui-transition="fade"] ::view-transition-old(shell-main),
    html:not([data-ui-transition]) ::view-transition-old(shell-main) {
      animation: ui-drift-out .16s ease-in both;
    }
    html[data-ui-transition="fade"] ::view-transition-new(shell-main),
    html:not([data-ui-transition]) ::view-transition-new(shell-main) {
      animation: ui-drift-in  .22s ease-out both;
    }

    /* ─── Preset: slide-left (forward nav feel) ─── */
    html[data-ui-transition="slide-left"] ::view-transition-old(shell-main) {
      animation: ui-slide-out-l .20s ease-in both;
    }
    html[data-ui-transition="slide-left"] ::view-transition-new(shell-main) {
      animation: ui-slide-in-r  .26s cubic-bezier(.2,.8,.2,1) both;
    }

    /* ─── Preset: slide-right (back nav feel) ─── */
    html[data-ui-transition="slide-right"] ::view-transition-old(shell-main) {
      animation: ui-slide-out-r .20s ease-in both;
    }
    html[data-ui-transition="slide-right"] ::view-transition-new(shell-main) {
      animation: ui-slide-in-l  .26s cubic-bezier(.2,.8,.2,1) both;
    }

    /* ─── Preset: slide-up (bottom-sheet feel) ─── */
    html[data-ui-transition="slide-up"] ::view-transition-old(shell-main) {
      animation: ui-slide-out-up .18s ease-in both;
    }
    html[data-ui-transition="slide-up"] ::view-transition-new(shell-main) {
      animation: ui-slide-in-up  .26s cubic-bezier(.2,.8,.2,1) both;
    }

    /* ─── Preset: scale (subtle zoom) ─── */
    html[data-ui-transition="scale"] ::view-transition-old(shell-main) {
      animation: ui-scale-out .16s ease-in both;
    }
    html[data-ui-transition="scale"] ::view-transition-new(shell-main) {
      animation: ui-scale-in  .22s ease-out both;
    }

    /* ─── Preset: none (instant, no animation) ─── */
    html[data-ui-transition="none"] ::view-transition-old(shell-main),
    html[data-ui-transition="none"] ::view-transition-new(shell-main) {
      animation: none;
    }
  `;
  document.head.appendChild(style);
}

/**
 * Pick a transition preset globally. Persisted so a reload keeps it.
 * Available: fade | slide-left | slide-right | slide-up | scale | none.
 *
 * Also exposed as `window.ui.setTransition(name)` (see shell.js) so it
 * can be flipped live from DevTools.
 */
export function setTransitionPreset(name) {
  const allowed = new Set(['fade', 'slide-left', 'slide-right', 'slide-up', 'scale', 'none']);
  if (!allowed.has(name)) {
    console.warn('[shell] unknown transition preset:', name, '— allowed:', [...allowed]);
    return;
  }
  document.documentElement.dataset.uiTransition = name;
  try { localStorage.setItem('erp.transition', name); } catch { /* ignore */ }
}

/** Restore the preset previously chosen via setTransitionPreset(). */
function restoreTransitionPreset() {
  try {
    const saved = localStorage.getItem('erp.transition');
    if (saved) document.documentElement.dataset.uiTransition = saved;
  } catch { /* ignore */ }
}

// ---- Eager module-load setup ---------------------------------------------
//
// The style + preset switcher have to be available BEFORE the element
// upgrades — otherwise the topbar's <select onchange="window.ui.setTransition(..)">
// throws on first interaction and the very first navigation has no
// animation because the ::view-transition CSS hasn't been installed yet.
//
// Both operations are idempotent and touch only document-level state.
installTransitionStyle();
restoreTransitionPreset();
window.ui = Object.assign(window.ui || {}, {
  setTransition: setTransitionPreset,
});

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
    // Everything else (transition style, window.ui.setTransition, boot)
    // is already set up at module load — see below the class definition.
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
