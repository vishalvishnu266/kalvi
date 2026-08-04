// Lazy loader — dynamically imports "heavy" component modules only when
// their tag actually appears in the DOM.
//
// How it works:
//   1. LAZY_MAP maps custom-element tag names to the module path that
//      registers them.
//   2. On startup we scan the DOM once and load any lazy tags found.
//   3. We attach a MutationObserver so tags added later (SPA nav,
//      drawers, modals, `innerHTML=`) also trigger a load.
//   4. Each module is loaded AT MOST ONCE — the loader remembers which
//      tags have already been requested.
//   5. Uses `import()` which the browser splits into a separate HTTP/2
//      request per chunk. Vite/Rollup would treeshake further; here we
//      rely on the browser cache — each chunk is cached individually so
//      the second page that uses ui-data-table pays zero cost.
//
// To add a new lazy component:
//   1. Add `'ui-foo': () => import('./ui-foo.js'),` to LAZY_MAP.
//   2. Make sure ui-foo.js has `customElements.define('ui-foo', UiFoo)`.
//   3. Do NOT import it anywhere else — that would defeat the split.

const LAZY_MAP = {
  'ui-data-table':    () => import('./ui-data-table.js'),
  'ui-table':         () => import('./ui-table.js'),
  'ui-kanban':        () => import('./ui-kanban.js'),
  'ui-timeline':      () => import('./ui-timeline.js'),
  'ui-progress':      () => import('./ui-progress.js'),
  'ui-stepper':       () => import('./ui-stepper.js'),
  'ui-inline-edit':   () => import('./ui-inline-edit.js'),
  'ui-command':       () => import('./ui-command.js'),
  'ui-drawer':        () => import('./ui-drawer.js'),
  'ui-dropdown-menu': () => import('./ui-dropdown-menu.js'),
  'ui-file-upload':   () => import('./ui-file-upload.js'),
  'ui-form-banner':   () => import('./ui-form-banner.js'),
  'ui-collapse':      () => import('./ui-collapse.js'),
  'ui-alert':         () => import('./ui-alert.js'),
  'ui-ack-panel':     () => import('./ui-ack-panel.js'),
  // The unified copilot element lives in ui-copilot-v2.js (v2 codebase).
  // Both names lazy-load the same module and register both custom elements.
  'ui-copilot':       () => import('./ui-copilot-v2.js'),
  'ui-copilot-v2':    () => import('./ui-copilot-v2.js'),
};

const requested = new Set();

/**
 * Request a lazy tag. Safe to call multiple times — each module is only
 * fetched once. Returns the import promise (or a resolved promise if the
 * tag is not lazy).
 */
export function requestTag(tag) {
  const t = tag.toLowerCase();
  if (requested.has(t)) return Promise.resolve();
  const loader = LAZY_MAP[t];
  if (!loader) return Promise.resolve();
  requested.add(t);
  return loader().catch((err) => {
    // Allow retry on transient network failures.
    requested.delete(t);
    console.error('[lazy] failed to load', t, err);
  });
}

/** Scan a subtree for any lazy tags and request them in parallel. */
function scan(root) {
  if (!root) return Promise.resolve();
  const promises = [];
  // Root itself may be an element.
  if (root.nodeType === 1 && LAZY_MAP[root.tagName.toLowerCase()]) {
    promises.push(requestTag(root.tagName));
  }
  // querySelectorAll on lazy tag names is faster than walking every node.
  const selector = Object.keys(LAZY_MAP).join(',');
  if (root.querySelectorAll) {
    root.querySelectorAll(selector).forEach((el) => {
      promises.push(requestTag(el.tagName));
    });
  }
  return Promise.all(promises);
}

/**
 * Rescan the current document body for any lazy tags that appeared
 * (e.g. after a Turbo navigation). Returns a promise that resolves
 * once every needed lazy chunk has finished loading.
 *
 * Safe to call any number of times — already-loaded modules are
 * short-circuited by the `requested` Set.
 */
export function rescan() {
  return scan(document.body || document.documentElement);
}

let observer = null;

/**
 * Start observing the document for lazy tags. Returns a promise that
 * resolves once the INITIAL scan (and its imports) have finished — the
 * FOUCE gate awaits this before revealing the page.
 *
 * Re-called safely on Turbo navigation; the observer is (re)attached to
 * the current `document.documentElement` so newly-swapped bodies are
 * covered too.
 */
export function startLazyLoader() {
  if (observer) observer.disconnect();
  observer = new MutationObserver((mutations) => {
    for (const m of mutations) {
      m.addedNodes.forEach((n) => { if (n.nodeType === 1) scan(n); });
    }
  });
  observer.observe(document.documentElement, { childList: true, subtree: true });
  return rescan();
}
