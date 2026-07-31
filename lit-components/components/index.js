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
import { startLazyLoader } from './lazy.js';

// Kick off `extras.js` immediately (parallel with the lazy scan below).
// It's a single dynamic import so the browser fetches it as its own chunk
// after `core.js` — which is what we want, because the shell can already
// paint before the extras land.
const extrasPromise = import('./extras.js').catch((err) => {
  console.error('[extras] failed to load', err);
});

// Kick off the lazy loader: scans the DOM for heavy tags right now and
// starts watching for future additions. The returned promise resolves
// when the INITIAL batch is done.
const lazyPromise = startLazyLoader();

// Public "everything the current page needs is defined" promise.
// The FOUCE gate uses this to decide when to reveal <body>.
window.__lit_ready = Promise.all([extrasPromise, lazyPromise]).then(() => {
  // Also wait for every custom-element tag currently in the DOM to be
  // upgraded — this is what guarantees no visible flash.
  const tags = new Set();
  document.querySelectorAll('*').forEach((el) => {
    const t = el.tagName.toLowerCase();
    if (t.includes('-')) tags.add(t);
  });
  return Promise.all(
    Array.from(tags).map((t) => customElements.whenDefined(t))
  );
});
