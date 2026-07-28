/* Boot script — resolves colour mode + brand theme + font pack pre-paint.
 *
 * Three independent axes, each persisted in localStorage:
 *   1. Colour mode  → `<html class="sl-theme-dark">` (and `.dark` for compat)
 *   2. Brand theme  → `<html data-theme="indigo">`
 *   3. Font pack    → `<html data-font="system">`
 *
 * Wire markup:
 *   <sl-icon-button data-theme-toggle>…</sl-icon-button>   flips light/dark
 *   <sl-select data-brand-picker size="small" hoist></sl-select>
 *   <sl-select data-font-picker  size="small" hoist></sl-select>
 *
 * Pickers auto-populate from `BRAND_THEMES` / `FONT_PACKS` below.
 */
(function () {
  const KEY_MODE  = 'sl-color-mode';
  const KEY_BRAND = 'sl-brand-theme';
  const KEY_FONT  = 'sl-font-pack';
  const DARK_CLS  = 'sl-theme-dark';
  const DEFAULT_BRAND = 'indigo';
  const DEFAULT_FONT  = 'system';

  const BRAND_THEMES = [
    { value: 'indigo',   label: 'Indigo (default)' },
    { value: 'regal',    label: 'Regal (purple + gold)' },
    { value: 'edu-blue', label: 'Edu Blue (classic)' },
    { value: 'emerald',  label: 'Emerald (warm)' },
    { value: 'rose',     label: 'Rose (playful)' },
  ];
  const FONT_PACKS = [
    { value: 'system',   label: 'System (native)' },
    { value: 'inter',    label: 'Inter + Space Grotesk' },
    { value: 'manrope',  label: 'Manrope' },
    { value: 'dm-sans',  label: 'DM Sans' },
    { value: 'jakarta',  label: 'Plus Jakarta Sans' },
    { value: 'outfit',   label: 'Outfit' },
  ];

  // Expose so pages can render a custom picker if they want.
  window.APP_THEMES = BRAND_THEMES;
  window.APP_FONTS  = FONT_PACKS;

  function applyMode(mode) {
    document.documentElement.classList.toggle(DARK_CLS, mode === 'dark');
    document.documentElement.classList.toggle('dark',   mode === 'dark');
  }
  function applyBrand(brand) {
    document.documentElement.dataset.theme = brand;
  }
  function applyFont(font) {
    document.documentElement.dataset.font = font;
  }

  // ── Pre-paint resolvers ────────────────────────────────────────────
  try {
    const storedMode = localStorage.getItem(KEY_MODE);
    const wantsDark = storedMode === 'dark' ||
      (!storedMode && window.matchMedia('(prefers-color-scheme: dark)').matches);
    applyMode(wantsDark ? 'dark' : 'light');
  } catch (_) {}
  try {
    applyBrand(localStorage.getItem(KEY_BRAND) || DEFAULT_BRAND);
  } catch (_) {
    applyBrand(DEFAULT_BRAND);
  }
  try {
    applyFont(localStorage.getItem(KEY_FONT) || DEFAULT_FONT);
  } catch (_) {
    applyFont(DEFAULT_FONT);
  }

  // ── Colour-mode toggle ─────────────────────────────────────────────
  document.addEventListener('click', function (e) {
    const btn = e.target.closest && e.target.closest('[data-theme-toggle]');
    if (!btn) return;
    const nowDark = !document.documentElement.classList.contains(DARK_CLS);
    applyMode(nowDark ? 'dark' : 'light');
    try { localStorage.setItem(KEY_MODE, nowDark ? 'dark' : 'light'); } catch (_) {}
  });

  // ── Picker wiring ──────────────────────────────────────────────────
  //
  // Shoelace's <sl-select> upgrades asynchronously from the CDN autoloader,
  // so we can't just call `sel.value = …` on first tick. Use a small polling
  // loop with `customElements.whenDefined('sl-select')` before wiring.
  function populate(sel, items, currentValue, onChange) {
    // Populate options once.
    if (!sel.querySelector('sl-option')) {
      for (const t of items) {
        const opt = document.createElement('sl-option');
        opt.value = t.value;
        opt.textContent = t.label;
        sel.appendChild(opt);
      }
    }
    // updateComplete is a Lit lifecycle promise Shoelace exposes.
    const setValue = () => { sel.value = currentValue; };
    if (sel.updateComplete) { sel.updateComplete.then(setValue); }
    else { setValue(); }

    sel.addEventListener('sl-change', () => onChange(sel.value));
  }

  function wirePickers() {
    document.querySelectorAll('[data-brand-picker]').forEach((sel) => {
      populate(sel, BRAND_THEMES,
        document.documentElement.dataset.theme || DEFAULT_BRAND,
        (val) => {
          applyBrand(val);
          try { localStorage.setItem(KEY_BRAND, val); } catch (_) {}
        });
    });
    document.querySelectorAll('[data-font-picker]').forEach((sel) => {
      populate(sel, FONT_PACKS,
        document.documentElement.dataset.font || DEFAULT_FONT,
        (val) => {
          applyFont(val);
          try { localStorage.setItem(KEY_FONT, val); } catch (_) {}
        });
    });
  }

  function whenReady(cb) {
    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', cb);
    } else {
      cb();
    }
  }
  whenReady(function () {
    if (window.customElements && customElements.whenDefined) {
      customElements.whenDefined('sl-select').then(wirePickers).catch(wirePickers);
    } else {
      wirePickers();
    }
  });
})();
