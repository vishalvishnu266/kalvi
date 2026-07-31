// Entry file — import once per page.
//
//   <link rel="modulepreload" href="/lit-components/components/index.js">
//   <script type="module" src="/lit-components/components/index.js"></script>
//
// Three-tier bundle strategy (see also core.js / extras.js / lazy.js):
//
//   1. core.js    — critical, first-paint components + theme restore.
//                   Awaited synchronously.
//   2. extras.js  — common controls (input, form, list-item, modal, ...).
//                   Kicked off in parallel with core, awaited before the
//                   FOUCE gate opens.
//   3. lazy.js    — heavy components (data-table, kanban, timeline,
//                   drawer, ...). Loaded on-demand per tag via a
//                   MutationObserver + startup DOM scan.
//
// The FOUCE gate in the server-rendered <body> looks for
//   window.__lit_ready
// and awaits it. We expose that promise here.

import './core.js';
import { startLazyLoader, rescan } from './lazy.js';

// Kick off `extras.js` immediately (parallel with the lazy scan below).
// It's a single dynamic import so the browser fetches it as its own chunk
// after `core.js` — which is what we want, because the shell can already
// paint before the extras land.
const extrasPromise = import('./extras.js').catch((err) => {
  console.error('[extras] failed to load', err);
});

/** Wait for every custom-element tag currently in the DOM to be upgraded. */
function whenAllDefined() {
  const tags = new Set();
  document.querySelectorAll('*').forEach((el) => {
    const t = el.tagName.toLowerCase();
    if (t.includes('-')) tags.add(t);
  });
  return Promise.all(
    Array.from(tags).map((t) => customElements.whenDefined(t))
  );
}

/**
 * Build a fresh "page is ready" promise for the CURRENT document body.
 *
 * Call this on first load AND on every Turbo navigation. It:
 *   1. (Re)starts the lazy loader — re-attaches the MutationObserver to
 *      the (possibly new) document element.
 *   2. Waits for extras.js to finish loading (once, cached after).
 *   3. Waits for every custom-element tag on the new body to upgrade.
 *
 * Exposed as `window.__lit_ready_now` so the inline FOUCE gate and the
 * Turbo integration in page.rs can always get a FRESH promise instead of
 * the stale one-shot promise.
 */
function pageReady() {
  return Promise.all([
    extrasPromise,
    startLazyLoader(), // re-scans body + re-attaches observer
  ]).then(whenAllDefined);
}

// Fresh-promise factory — call this on every navigation.
window.__lit_ready_now = pageReady;

// Back-compat: some code (the inline gate on first paint) still awaits
// this. On first load it's the same promise pageReady() produces.
window.__lit_ready = pageReady();
