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
    /// When true, a `<ui-copilot>` element is appended just before `</body>`
    /// so the floating chat button appears on every page.
    with_copilot: bool,
    copilot_session_url: String,
    copilot_stream_url:  String,
    copilot_history_url: String,
    copilot_label:       String,
}

/// Start a new HTML page.
pub fn page() -> Page {
    Page {
        title: "Untitled".into(),
        assets_base: "/lit-components".into(),
        children: Vec::new(),
        with_copilot: false,
        copilot_session_url: "/copilot/session".into(),
        copilot_stream_url:  "/copilot/message".into(),
        copilot_history_url: "/copilot/history".into(),
        copilot_label:       "ERP Copilot".into(),
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

    /// Mount a `<ui-copilot>` floating chat button on this page.
    ///
    /// The element sits OUTSIDE any Hotwire Turbo frame so it survives page
    /// navigations without unmounting the current conversation.
    pub fn with_copilot(mut self) -> Self { self.with_copilot = true; self }

    /// Override the Copilot session endpoint (default `/copilot/session`).
    pub fn copilot_session_url(mut self, s: impl Into<String>) -> Self { self.copilot_session_url = s.into(); self.with_copilot = true; self }
    /// Override the Copilot SSE stream endpoint (default `/copilot/message`).
    pub fn copilot_stream_url (mut self, s: impl Into<String>) -> Self { self.copilot_stream_url  = s.into(); self.with_copilot = true; self }
    /// Override the Copilot history endpoint (default `/copilot/history`).
    pub fn copilot_history_url(mut self, s: impl Into<String>) -> Self { self.copilot_history_url = s.into(); self.with_copilot = true; self }
    /// Override the Copilot panel title (default `"ERP Copilot"`).
    pub fn copilot_label      (mut self, s: impl Into<String>) -> Self { self.copilot_label       = s.into(); self.with_copilot = true; self }
}

impl Component for Page {
    fn render(&self) -> String {
        let mut body = String::new();
        for c in &self.children { body.push_str(&c.render()); }
        if self.with_copilot {
            body.push_str(&format!(
                r#"<ui-copilot session-url="{s}" stream-url="{m}" history-url="{h}" label="{l}"></ui-copilot>"#,
                s = escape_html(&self.copilot_session_url),
                m = escape_html(&self.copilot_stream_url),
                h = escape_html(&self.copilot_history_url),
                l = escape_html(&self.copilot_label),
            ));
        }

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
  <!-- Hotwire Turbo Drive: required for Hotwire Native — turns page
       navigations into HTML body swaps that the native shell can treat
       as native screens. No custom lifecycle code needed. -->
  <script type="module" src="https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/dist/turbo.es2017-esm.min.js"></script>
  <style>
    /* Breathing room at the top/sides of every DSL page. Individual pages
       can override by wrapping content in their own container/layout. On
       mobile we shrink the horizontal gutters so cards don't feel cramped
       and keep the safe-area inset for iOS notch. */
    body {{
      padding: 24px 24px 32px;
      padding-top:    max(24px, env(safe-area-inset-top));
      padding-left:   max(24px, env(safe-area-inset-left));
      padding-right:  max(24px, env(safe-area-inset-right));
      padding-bottom: max(32px, env(safe-area-inset-bottom));
      box-sizing: border-box;
    }}
    @media (max-width: 640px) {{
      body {{ padding: 16px 12px 24px; }}
    }}

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
      setTimeout(function () {{
        // Diagnostic — if we hit the safety net it means some custom
        // element failed to upgrade in time. Log the offenders so the
        // occasional "page didn't render" bug is debuggable instead of
        // silent. Only fires if we're forced to reveal via the timeout.
        if (!document.body.classList.contains('ce-ready')) {{
          try {{
            var missing = [];
            document.querySelectorAll('*').forEach(function (el) {{
              var t = el.tagName.toLowerCase();
              if (t.indexOf('-') !== -1 && !customElements.get(t)) {{
                missing.push(t);
              }}
            }});
            if (missing.length) {{
              console.warn('[lit-ui] FOUCE safety-net fired — un-upgraded custom elements:',
                Array.from(new Set(missing)).sort());
            }}
          }} catch (_) {{}}
        }}
        reveal();
      }}, TIMEOUT_MS);
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
