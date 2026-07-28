/* Shared boot script for the Shoelace UI preview.
 *
 * Handles two independent axes:
 *   1. Colour mode (light / dark)      → `<html class="sl-theme-dark">`
 *   2. Brand theme (regal / indigo / …) → `<html data-theme="regal">`
 *
 * Both are applied pre-paint (no FOUC) and persisted in localStorage.
 *
 * Wire-up patterns supported in markup:
 *   <sl-icon-button data-theme-toggle>…</sl-icon-button>   flips light/dark
 *   <sl-select data-brand-picker>                          set brand theme
 *      <sl-option value="regal">Regal</sl-option>
 *      …
 *   </sl-select>
 */
(function () {
  const KEY_MODE  = 'sl-color-mode';
  const KEY_BRAND = 'sl-brand-theme';
  const KEY_FONT  = 'sl-font-pack';
  const DARK_CLS  = 'sl-theme-dark';
  const DEFAULT_BRAND = 'regal';
  const DEFAULT_FONT  = 'inter';

  const BRAND_THEMES = [
    { value: 'regal',    label: 'Regal (purple + gold)' },
    { value: 'indigo',   label: 'Indigo (fresh)' },
    { value: 'edu-blue', label: 'Edu Blue (classic)' },
    { value: 'emerald',  label: 'Emerald (warm)' },
    { value: 'rose',     label: 'Rose (playful)' },
  ];
  const FONT_PACKS = [
    { value: 'inter',    label: 'Inter + Space Grotesk' },
    { value: 'manrope',  label: 'Manrope' },
    { value: 'dm-sans',  label: 'DM Sans' },
    { value: 'jakarta',  label: 'Plus Jakarta Sans' },
    { value: 'geist',    label: 'Geist' },
    { value: 'outfit',   label: 'Outfit' },
  ];
  // Expose for pages that want to render a picker UI.
  window.APP_THEMES = BRAND_THEMES;
  window.APP_FONTS  = FONT_PACKS;

  function applyMode(mode) {
    document.documentElement.classList.toggle(DARK_CLS, mode === 'dark');
    document.documentElement.classList.toggle('dark',    mode === 'dark');
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
    const btn = e.target.closest?.('[data-theme-toggle]');
    if (!btn) return;
    const nowDark = !document.documentElement.classList.contains(DARK_CLS);
    applyMode(nowDark ? 'dark' : 'light');
    try { localStorage.setItem(KEY_MODE, nowDark ? 'dark' : 'light'); } catch (_) {}
  });

  // ── Pickers ─────────────────────────────────────────────────────────
  function populate(sel, items, currentValue, onChange) {
    if (!sel.querySelector('sl-option')) {
      for (const t of items) {
        const opt = document.createElement('sl-option');
        opt.value = t.value;
        opt.textContent = t.label;
        sel.appendChild(opt);
      }
    }
    sel.value = currentValue;
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
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', wirePickers);
  } else {
    wirePickers();
  }
})();
