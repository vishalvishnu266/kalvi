/* Shared boot script for the UI preview pages.
 *
 * 1. Resolves the theme (light/dark) BEFORE first paint so there is no
 *    flash of the wrong theme when the user has picked dark mode.
 * 2. Wires a `data-theme-toggle` button (if present) to flip the theme.
 * 3. Renders Lucide icons once the DOM is ready.
 */
(function () {
  // ── Pre-paint theme resolver ────────────────────────────────────────
  try {
    var stored = localStorage.getItem('theme');
    var wantsDark =
      stored === 'dark' ||
      (!stored && window.matchMedia('(prefers-color-scheme: dark)').matches);
    if (wantsDark) document.documentElement.classList.add('dark');
  } catch (_) {}

  // ── Theme toggle button ─────────────────────────────────────────────
  document.addEventListener('click', function (e) {
    var btn = e.target.closest && e.target.closest('[data-theme-toggle]');
    if (!btn) return;
    var isDark = document.documentElement.classList.toggle('dark');
    try { localStorage.setItem('theme', isDark ? 'dark' : 'light'); } catch (_) {}
  });

  // ── Lucide icons ────────────────────────────────────────────────────
  function renderIcons() {
    if (window.lucide && typeof window.lucide.createIcons === 'function') {
      window.lucide.createIcons();
    }
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', renderIcons);
  } else {
    renderIcons();
  }
})();
