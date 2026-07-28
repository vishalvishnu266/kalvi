// School ERP — small progressive-enhancement layer for the Shoelace shell.
//
// Shoelace itself handles component upgrades via the CDN autoloader; we only
// need a tiny bridge for:
//   * ⌘K / Ctrl+K global search shortcut (focuses the topbar/spotlight input).
//   * Hotwire Native message-passing bridge (no-op stub when not embedded).
//
// Dark mode + brand theme live in `boot.js` (must run pre-paint).

(function () {
  // Global ⌘K / Ctrl+K → focus any `[data-search-input]` on the page.
  document.addEventListener('keydown', function (e) {
    if ((e.metaKey || e.ctrlKey) && e.key && e.key.toLowerCase() === 'k') {
      var input = document.querySelector('[data-search-input]');
      if (input) {
        e.preventDefault();
        if (typeof input.focus === 'function') input.focus();
        if (typeof input.select === 'function') input.select();
      }
    }
  });

  // Hotwire Native bridge — no-op unless embedded in a native shell.
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
