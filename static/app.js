// School ERP — small progressive-enhancement layer.
//
// * Renders Lucide icons after Turbo swaps in new HTML.
// * Toggles dark mode + persists the choice in `localStorage`.
// * Adds ⌘K / Ctrl+K shortcut for the (future) command palette.
// * Hooks so the same page can run inside Hotwire Native without changes.

(function () {
  function renderIcons() {
    if (window.lucide && typeof window.lucide.createIcons === 'function') {
      window.lucide.createIcons();
    }
  }

  // First render + on every Turbo navigation.
  document.addEventListener('DOMContentLoaded', renderIcons);
  document.addEventListener('turbo:load', renderIcons);
  document.addEventListener('turbo:frame-load', renderIcons);

  // Theme toggle.
  document.addEventListener('click', function (e) {
    var t = e.target.closest('[data-theme-toggle]');
    if (!t) return;
    var isDark = document.documentElement.classList.toggle('dark');
    try { localStorage.setItem('theme', isDark ? 'dark' : 'light'); } catch (_) {}
    renderIcons();
  });

  // Keyboard shortcut placeholder.
  document.addEventListener('keydown', function (e) {
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
      e.preventDefault();
      // Placeholder — will open the global search modal in a future screen.
      console.log('⌘K pressed — command palette coming soon');
    }
  });

  // Hotwire Native bridge hook. When embedded in a native shell, the page
  // can post messages up so the native chrome can hide/show. Kept as a
  // no-op stub for now.
  window.NativeBridge = {
    send: function (name, data) {
      if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers[name]) {
        window.webkit.messageHandlers[name].postMessage(data || {});
      } else if (window.AndroidBridge && typeof window.AndroidBridge[name] === 'function') {
        window.AndroidBridge[name](JSON.stringify(data || {}));
      }
    }
  };
})();
