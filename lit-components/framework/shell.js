// -----------------------------------------------------------------------------
// framework/shell.js — the browser-side runtime for the ui-app-shell
// fragment protocol. Zero framework dependencies (no Lit); ~200 LOC.
//
// Responsibilities:
//   1. Intercept same-origin <a> clicks and <form> submits.
//   2. Fetch the target URL with `Accept: text/vnd.ui-fragments+html`
//      so the server returns fragments (not a full page).
//   3. Parse the response and apply every <ui-fragment target action>
//      element it contains to the corresponding island.
//   4. Sync the URL with pushState + handle back/forward via popstate.
//
// The client never invents new behaviour — every visible change is
// server-authored. That keeps the whole app driveable by the copilot /
// agent through the exact same pipeline.
// -----------------------------------------------------------------------------

/** MIME the server uses to signal "fragments only, no shell". */
export const FRAGMENTS_MIME = 'text/vnd.ui-fragments+html';

/** Special target the server uses for client-only side effects. */
const SIDE_EFFECT_TARGET = '__side_effect__';

/** Registry of pluggable side-effect handlers, keyed by `kind`. */
const sideEffects = Object.create(null);

/** Register a side-effect handler (called from ui-app-shell.js on boot). */
export function registerSideEffect(kind, handler) {
  sideEffects[kind] = handler;
}

/**
 * Look up an island element (with slot="target") anywhere in the document.
 * Islands live as slotted children of <ui-app-shell>.
 */
function findIsland(target) {
  return document.querySelector(`[slot="${CSS.escape(target)}"]`);
}

/**
 * Apply a single fragment to its island.
 *
 * `action` semantics mirror the server-side enum in
 * `framework/ui-shell/src/fragment.rs` — keep the two in sync.
 */
export function applyFragment(fragEl) {
  const target = fragEl.getAttribute('target');
  const action = (fragEl.getAttribute('action') || 'replace').toLowerCase();

  // Side effects don't touch an island — they run a registered handler.
  if (target === SIDE_EFFECT_TARGET) {
    const kind = fragEl.getAttribute('kind') || '';
    let payload = {};
    try { payload = JSON.parse(fragEl.textContent || '{}'); } catch { /* ignore */ }
    const handler = sideEffects[kind];
    if (handler) handler(payload);
    else console.warn('[shell] unknown side-effect kind:', kind);
    return;
  }

  const island = findIsland(target);
  if (!island) {
    console.warn('[shell] no island for target:', target);
    return;
  }

  switch (action) {
    case 'replace':
      island.replaceChildren(...fragEl.childNodes);
      break;
    case 'append':
      island.append(...fragEl.childNodes);
      break;
    case 'prepend':
      island.prepend(...fragEl.childNodes);
      break;
    case 'remove':
      island.replaceChildren();
      break;
    case 'update':
      island.innerHTML = fragEl.innerHTML;
      break;
    default:
      console.warn('[shell] unknown action:', action);
  }
}

/**
 * Apply every <ui-fragment> found under `root` (which is typically a
 * DocumentFragment from `Range.createContextualFragment(html)`).
 */
export function applyAll(root) {
  root.querySelectorAll('ui-fragment').forEach(applyFragment);
}

/**
 * Run `mutator` inside a View Transition when the browser supports it,
 * so island swaps get a smooth crossfade (or a custom animation defined
 * in CSS via `::view-transition-*`). On unsupported browsers, or when
 * transitions are disabled via `window.__ui_disable_transitions = true`,
 * or under `prefers-reduced-motion`, it runs the mutator synchronously.
 *
 * Kept as a single choke-point so we can benchmark & tune from one place.
 */
function withTransition(mutator) {
  const disabled =
    window.__ui_disable_transitions === true ||
    typeof document.startViewTransition !== 'function' ||
    matchMedia('(prefers-reduced-motion: reduce)').matches;
  if (disabled) { mutator(); return; }
  // Wrap in requestAnimationFrame so the browser has already flushed any
  // in-flight work before the snapshot — noticeably smoother on Chrome.
  requestAnimationFrame(() => document.startViewTransition(() => mutator()));
}

/**
 * Fetch a URL as a fragment response and apply the result.
 *
 * `opts.push` (default true) controls whether the URL bar is updated.
 * `opts.method` / `opts.body` let form submits reuse this code path.
 */
export async function navigate(url, opts = {}) {
  const { push = true, method = 'GET', body = null } = opts;

  const res = await fetch(url, {
    method,
    body,
    headers: { Accept: FRAGMENTS_MIME },
    credentials: 'same-origin',
    redirect: 'follow',
  });

  // 4xx/5xx: fall back to a hard nav so error pages render normally.
  if (!res.ok) { window.location.assign(url); return; }

  // If the server returned a full page (e.g. after a login redirect),
  // give up on partial swap and reload cleanly. Simple heuristic on the
  // response content-type keeps this correct 99% of the time.
  const ct = res.headers.get('content-type') || '';
  if (!ct.includes('ui-fragments')) { window.location.assign(url); return; }

  const html = await res.text();
  const tpl = document.createElement('template');
  tpl.innerHTML = html;

  // Animate the swap. The View Transitions API snapshots the DOM before
  // + after the mutation and interpolates between them — no per-island
  // animation code required. Fine-tune via CSS `::view-transition-*`.
  withTransition(() => applyAll(tpl.content));

  if (push && method === 'GET') {
    // Use the FINAL URL (post-redirect) so bookmarks / back button stay honest.
    history.pushState({ shell: true }, '', res.url || url);
  }

  // Let interested code (analytics, focus management, mobile shell) know.
  document.dispatchEvent(new CustomEvent('ui:navigated', {
    detail: { url: res.url || url, method },
  }));
}

// ---- Global interception ---------------------------------------------------
//
// One document-level listener each for click + submit. We deliberately
// keep the rules simple so it is easy to reason about "when does this
// intercept?":
//
//   * Same-origin only.
//   * <a> without target=_blank, download, or [data-nav="off"].
//   * <form> without target=_blank or [data-nav="off"].
//   * Modifier keys (ctrl/meta/shift/alt) opt out (browser default nav).
//   * Middle click opts out.
// ---------------------------------------------------------------------------

function isSameOrigin(url) {
  try { return new URL(url, location.href).origin === location.origin; }
  catch { return false; }
}

function shouldInterceptClick(e, anchor) {
  if (e.defaultPrevented) return false;
  if (e.button !== 0) return false;
  if (e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return false;
  if (anchor.target && anchor.target !== '_self') return false;
  if (anchor.hasAttribute('download')) return false;
  if (anchor.getAttribute('data-nav') === 'off') return false;
  if (!isSameOrigin(anchor.href)) return false;
  return true;
}

/**
 * Run `fn` with the given transition preset applied only for its
 * duration. Used to honour a link's `data-transition="…"` attribute
 * without disturbing the persisted global preset.
 */
async function withPreset(preset, fn) {
  if (!preset) return fn();
  const root = document.documentElement;
  const prev = root.dataset.uiTransition;
  root.dataset.uiTransition = preset;
  try { await fn(); }
  finally {
    if (prev == null) delete root.dataset.uiTransition;
    else root.dataset.uiTransition = prev;
  }
}

function installInterceptors() {
  document.addEventListener('click', (e) => {
    const a = e.target instanceof Element ? e.target.closest('a[href]') : null;
    if (!a) return;
    if (!shouldInterceptClick(e, a)) return;
    e.preventDefault();
    // Per-link transition override: <a data-transition="slide-left" ...>
    const preset = a.getAttribute('data-transition');
    withPreset(preset, () => navigate(a.href));
  });

  document.addEventListener('submit', (e) => {
    const form = e.target;
    if (!(form instanceof HTMLFormElement)) return;
    if (form.getAttribute('data-nav') === 'off') return;
    if (form.target && form.target !== '_self') return;

    const action = form.action || location.href;
    if (!isSameOrigin(action)) return;

    e.preventDefault();
    const method = (form.method || 'GET').toUpperCase();
    if (method === 'GET') {
      const url = new URL(action);
      new FormData(form).forEach((v, k) => url.searchParams.append(k, String(v)));
      navigate(url.toString());
    } else {
      navigate(action, { method, body: new FormData(form), push: false });
    }
  });

  window.addEventListener('popstate', () => {
    // Back/forward: re-fetch the current URL as a fragment. Never push
    // (the browser has already updated the URL).
    navigate(location.href, { push: false });
  });
}

/** Wire everything up. Idempotent; safe to call more than once. */
let installed = false;
export function bootShell() {
  if (installed) return;
  installed = true;
  installInterceptors();
  // Also apply any fragments the server may have emitted directly into
  // the initial document (rare, but useful for early toasts).
  applyAll(document);
}

// Expose a tiny window API for the copilot + devtools consoles.
window.ui = Object.assign(window.ui || {}, {
  navigate,
  applyFragment,
  applyAll,
  registerSideEffect,
});
