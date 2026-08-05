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
  // Durations are driven by a CSS custom property so we can slow the
  // whole system down for demoing / debugging (e.g.
  // `document.documentElement.style.setProperty('--ui-t', '2s')` or
  // `window.ui.slow(2000)`). The exit uses ~75% of the enter duration
  // so the two animations don't stack visibly.
  style.textContent = `
    :root { --ui-t: 220ms; --ui-t-out: 160ms; }

    /* ─── Shared keyframes ─── */
    @keyframes ui-fade-in       { from { opacity: 0; } }
    @keyframes ui-fade-out      { to   { opacity: 0; } }
    @keyframes ui-drift-in      { from { opacity: 0; transform: translateY(8px);  } }
    @keyframes ui-drift-out     { to   { opacity: 0; transform: translateY(-8px); } }
    @keyframes ui-slide-in-r    { from { transform: translateX( 40px); opacity: 0; } }
    @keyframes ui-slide-out-l   { to   { transform: translateX(-40px); opacity: 0; } }
    @keyframes ui-slide-in-l    { from { transform: translateX(-40px); opacity: 0; } }
    @keyframes ui-slide-out-r   { to   { transform: translateX( 40px); opacity: 0; } }
    @keyframes ui-slide-in-up   { from { transform: translateY( 40px); opacity: 0; } }
    @keyframes ui-slide-out-up  { to   { transform: translateY(-40px); opacity: 0; } }
    @keyframes ui-scale-in      { from { transform: scale(0.94); opacity: 0; } }
    @keyframes ui-scale-out     { to   { transform: scale(1.06); opacity: 0; } }
    /* 3D flip needs perspective on the pseudo-element parent — set on
       ::view-transition-group below. Old = front face rotating away,
       new = back face rotating into view. Backface-visibility hides the
       mirrored content mid-rotation. */
    @keyframes ui-flip-out-y {
      from { transform: rotateY(0deg);   opacity: 1; }
      to   { transform: rotateY(-90deg); opacity: 0; }
    }
    @keyframes ui-flip-in-y {
      from { transform: rotateY( 90deg); opacity: 0; }
      to   { transform: rotateY(  0deg); opacity: 1; }
    }
    @keyframes ui-flip-out-x {
      from { transform: rotateX(0deg);   opacity: 1; }
      to   { transform: rotateX(-90deg); opacity: 0; }
    }
    @keyframes ui-flip-in-x {
      from { transform: rotateX( 90deg); opacity: 0; }
      to   { transform: rotateX(  0deg); opacity: 1; }
    }

    /* ─── Preset: fade (default) ─── */
    html[data-ui-transition="fade"] ::view-transition-old(shell-main),
    html:not([data-ui-transition]) ::view-transition-old(shell-main) {
      animation: ui-drift-out var(--ui-t-out) ease-in both;
    }
    html[data-ui-transition="fade"] ::view-transition-new(shell-main),
    html:not([data-ui-transition]) ::view-transition-new(shell-main) {
      animation: ui-drift-in var(--ui-t) ease-out both;
    }

    /* ─── Preset: slide-left (forward nav feel) ─── */
    html[data-ui-transition="slide-left"] ::view-transition-old(shell-main) {
      animation: ui-slide-out-l var(--ui-t-out) ease-in both;
    }
    html[data-ui-transition="slide-left"] ::view-transition-new(shell-main) {
      animation: ui-slide-in-r var(--ui-t) cubic-bezier(.2,.8,.2,1) both;
    }

    /* ─── Preset: slide-right (back nav feel) ─── */
    html[data-ui-transition="slide-right"] ::view-transition-old(shell-main) {
      animation: ui-slide-out-r var(--ui-t-out) ease-in both;
    }
    html[data-ui-transition="slide-right"] ::view-transition-new(shell-main) {
      animation: ui-slide-in-l var(--ui-t) cubic-bezier(.2,.8,.2,1) both;
    }

    /* ─── Preset: slide-up (bottom-sheet feel) ─── */
    html[data-ui-transition="slide-up"] ::view-transition-old(shell-main) {
      animation: ui-slide-out-up var(--ui-t-out) ease-in both;
    }
    html[data-ui-transition="slide-up"] ::view-transition-new(shell-main) {
      animation: ui-slide-in-up var(--ui-t) cubic-bezier(.2,.8,.2,1) both;
    }

    /* ─── Preset: scale (subtle zoom) ─── */
    html[data-ui-transition="scale"] ::view-transition-old(shell-main) {
      animation: ui-scale-out var(--ui-t-out) ease-in both;
    }
    html[data-ui-transition="scale"] ::view-transition-new(shell-main) {
      animation: ui-scale-in var(--ui-t) ease-out both;
    }

    /* ─── Preset: flip-y (horizontal card flip — feels like flipping to
       the "back" of a card). Perspective is set on the group so both
       the old and new snapshots share the same vanishing point. Both
       animations run on top of each other in the same group by default;
       we split the flip into two halves via animation-delay so the old
       face rotates out before the new one rotates in. */
    html[data-ui-transition="flip-y"] ::view-transition-group(shell-main) {
      perspective: 1200px;
      transform-style: preserve-3d;
    }
    html[data-ui-transition="flip-y"] ::view-transition-old(shell-main) {
      animation: ui-flip-out-y calc(var(--ui-t) * 0.5) ease-in both;
      backface-visibility: hidden;
    }
    html[data-ui-transition="flip-y"] ::view-transition-new(shell-main) {
      animation: ui-flip-in-y calc(var(--ui-t) * 0.5) ease-out both;
      animation-delay: calc(var(--ui-t) * 0.5);
      backface-visibility: hidden;
    }

    /* ─── Preset: flip-x (vertical flip — like flipping a page top→bottom). */
    html[data-ui-transition="flip-x"] ::view-transition-group(shell-main) {
      perspective: 1200px;
      transform-style: preserve-3d;
    }
    html[data-ui-transition="flip-x"] ::view-transition-old(shell-main) {
      animation: ui-flip-out-x calc(var(--ui-t) * 0.5) ease-in both;
      backface-visibility: hidden;
    }
    html[data-ui-transition="flip-x"] ::view-transition-new(shell-main) {
      animation: ui-flip-in-x calc(var(--ui-t) * 0.5) ease-out both;
      animation-delay: calc(var(--ui-t) * 0.5);
      backface-visibility: hidden;
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
  const allowed = new Set(['fade', 'slide-left', 'slide-right', 'slide-up', 'scale', 'flip-y', 'flip-x', 'none']);
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
    const savedSpeed = localStorage.getItem('erp.transition.speed');
    if (savedSpeed) applyTransitionSpeed(parseInt(savedSpeed, 10));
  } catch { /* ignore */ }
}

/**
 * Slow-motion mode. `ms` is the ENTER duration; the EXIT is set to 75%.
 * Persisted so it survives a reload — call `slow(0)` to reset.
 *
 * Exposed as `window.ui.slow(ms)` (see below) so you can eyeball a
 * preset at 2000ms, then dial it back to production speed.
 */
export function applyTransitionSpeed(ms) {
  const root = document.documentElement;
  if (!ms || ms <= 0) {
    root.style.removeProperty('--ui-t');
    root.style.removeProperty('--ui-t-out');
    try { localStorage.removeItem('erp.transition.speed'); } catch { /* ignore */ }
    return;
  }
  root.style.setProperty('--ui-t', ms + 'ms');
  root.style.setProperty('--ui-t-out', Math.round(ms * 0.75) + 'ms');
  try { localStorage.setItem('erp.transition.speed', String(ms)); } catch { /* ignore */ }
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
  // Slow-mo helper for demoing / debugging transitions.
  //   window.ui.slow(2000) → 2s per swap
  //   window.ui.slow(0)    → back to default
  slow: applyTransitionSpeed,
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
