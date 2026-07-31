//! `Page` — tiny helper that wraps a `Vec<Component>` in a full HTML document
//! and links the vendored Lit component bundle.
//!
//! This is intentionally **framework-agnostic**: it just returns a `String`.
//! Wire it into your Axum / Actix / Rocket / whatever handler by calling
//! `page().add(...).render()` and returning it as `text/html`.

use crate::core::{Child, Component, escape_html};

pub struct Page {
    title: String,
    /// Where the Lit-components folder is mounted on your web server.
    /// Defaults to `/lit-components` — change with [`Page::assets_base`].
    assets_base: String,
    children: Vec<Child>,
}

/// Start a new HTML page.
pub fn page() -> Page {
    Page {
        title: "Untitled".into(),
        assets_base: "/lit-components".into(),
        children: Vec::new(),
    }
}

impl Page {
    pub fn title(mut self, s: impl Into<String>) -> Self { self.title = s.into(); self }

    /// Base URL path to your `lit-components/` folder.
    ///
    /// The rendered page will link three assets from here:
    ///
    /// * `<base>/assets/tokens.css`
    /// * `<base>/assets/global.css`
    /// * `<base>/components/index.js`
    pub fn assets_base(mut self, s: impl Into<String>) -> Self { self.assets_base = s.into(); self }

    /// Append one component to the page body.
    pub fn add(mut self, child: impl Component + 'static) -> Self {
        self.children.push(Box::new(child)); self
    }

    /// Append many components at once.
    pub fn children<I, C>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Component + 'static,
    {
        for c in iter { self.children.push(Box::new(c)); }
        self
    }
}

impl Component for Page {
    fn render(&self) -> String {
        let mut body = String::new();
        for c in &self.children { body.push_str(&c.render()); }

        // The <body> is fluid (100% width). Use `container()` inside for
        // centred max-width sections; use `container().fluid()` (or nothing)
        // for full-bleed layouts like dashboards / kanban boards.
        // FOUCE (Flash Of Undefined Custom Elements) mitigation:
        //   1. Preload the module so the network fetch starts in <head>.
        //   2. Hide <body> with a CSS gate until every custom element used on
        //      the page has been upgraded (customElements.whenDefined).
        //   3. Show a tiny centred spinner while we wait, then reveal.
        //   4. Safety timeout (1500ms) so the page always becomes visible
        //      even if a component fails to register.
        format!(
            r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <title>{title}</title>
  <link rel="stylesheet" href="{base}/assets/tokens.css">
  <link rel="stylesheet" href="{base}/assets/global.css">
  <link rel="stylesheet" href="{base}/assets/layout.css">
  <link rel="modulepreload" href="{base}/components/index.js">
  <!-- Hotwire Turbo Drive — intercepts <a> clicks and swaps <body> without a full reload. -->
  <script type="module" src="https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/dist/turbo.es2017-esm.min.js"></script>
  <style>
    /* Hide the app until custom elements are defined, but show a spinner. */
    body:not(.ce-ready) > *:not(#app-loading) {{ visibility: hidden; }}
    #app-loading {{
      position: fixed; inset: 0;
      display: flex; align-items: center; justify-content: center;
      background: var(--color-bg, #fff);
      z-index: 9999;
      transition: opacity .2s ease;
    }}
    #app-loading.hidden {{ opacity: 0; pointer-events: none; }}
    #app-loading .spinner {{
      width: 36px; height: 36px; border-radius: 50%;
      border: 3px solid var(--color-border, #e5e7eb);
      border-top-color: var(--color-primary, #4f46e5);
      animation: ce-spin .8s linear infinite;
    }}
    @keyframes ce-spin {{ to {{ transform: rotate(360deg); }} }}
  </style>
  <script type="module" src="{base}/components/index.js"></script>
</head>
<body>
  <div id="app-loading" aria-live="polite" aria-busy="true">
    <div class="spinner" role="status" aria-label="Loading"></div>
  </div>
{body}
  <script>
    (function () {{
      var TIMEOUT_MS = 1500;
      function reveal() {{
        if (document.body.classList.contains('ce-ready')) return;
        document.body.classList.add('ce-ready');
        var loader = document.getElementById('app-loading');
        if (loader) {{
          loader.classList.add('hidden');
          setTimeout(function () {{ loader.remove(); }}, 220);
        }}
      }}
      // Prefer the loader's aggregate promise (waits for core + extras
      // + any lazy chunks the current page needs). Falls back to a plain
      // whenDefined scan if index.js hasn't set __lit_ready yet.
      function whenLoaderReady() {{
        if (window.__lit_ready && typeof window.__lit_ready.then === 'function') {{
          return window.__lit_ready;
        }}
        var tags = new Set();
        document.querySelectorAll('*').forEach(function (el) {{
          var t = el.tagName.toLowerCase();
          if (t.indexOf('-') !== -1) tags.add(t);
        }});
        if (!tags.size || !window.customElements) return Promise.resolve();
        return Promise.all(
          Array.from(tags).map(function (t) {{ return customElements.whenDefined(t); }})
        );
      }}
      // Poll briefly for __lit_ready — the module script may not have
      // executed by the time this inline script runs.
      var waited = 0;
      (function tick() {{
        if (window.__lit_ready || waited >= 200) {{
          whenLoaderReady().then(reveal).catch(reveal);
          return;
        }}
        waited += 20;
        setTimeout(tick, 20);
      }})();
      // Safety net: never leave the page hidden.
      setTimeout(reveal, TIMEOUT_MS);
    }})();

    /* ---------- Turbo Drive integration ---------- */
    // On Turbo navigation the browser does NOT reload the page, so our
    // FOUCE gate above only runs once (on the first visit). We still
    // want the loader to scan any lazy tags that appeared on the new
    // page, and we want the loading spinner to briefly show during nav.
    (function () {{
      var loader;
      function showLoader() {{
        loader = document.createElement('div');
        loader.id = 'app-loading';
        loader.setAttribute('aria-live', 'polite');
        loader.setAttribute('aria-busy', 'true');
        loader.innerHTML = '<div class="spinner" role="status" aria-label="Loading"></div>';
        document.body.appendChild(loader);
      }}
      function hideLoader() {{
        if (loader) {{ loader.classList.add('hidden'); setTimeout(function () {{ loader && loader.remove(); loader = null; }}, 220); }}
      }}
      document.addEventListener('turbo:visit',        showLoader);
      document.addEventListener('turbo:before-render', function () {{
        // New body is about to be swapped in — keep it hidden until CEs upgrade.
        document.body.classList.remove('ce-ready');
      }});
      document.addEventListener('turbo:load', function () {{
        // Re-run the "wait for custom elements" logic on the new body.
        var promise = (window.__lit_ready && typeof window.__lit_ready.then === 'function')
          ? window.__lit_ready
          : Promise.resolve();
        promise.then(function () {{
          var tags = new Set();
          document.querySelectorAll('*').forEach(function (el) {{
            var t = el.tagName.toLowerCase();
            if (t.indexOf('-') !== -1) tags.add(t);
          }});
          return Promise.all(Array.from(tags).map(function (t) {{ return customElements.whenDefined(t); }}));
        }}).finally(function () {{
          document.body.classList.add('ce-ready');
          hideLoader();
          // Tiny debug counter so you can visually confirm no full reload.
          window.__turbo_swaps = (window.__turbo_swaps || 0) + 1;
          var badge = document.getElementById('turbo-swap-badge');
          if (badge) badge.textContent = 'Turbo swaps: ' + window.__turbo_swaps;
        }});
      }});
    }})();
  </script>
</body>
</html>
"#,
            title = escape_html(&self.title),
            base  = escape_html(&self.assets_base),
            body  = body,
        )
    }
}
