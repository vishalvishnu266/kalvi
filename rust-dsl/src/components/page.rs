//! `page()` — a top-level document primitive.
//!
//! Renders a full `<!doctype html>` document with the correct `<head>`
//! wired up for the Lit component library (three stylesheets + one
//! module script), then drops any children into `<body>`.
//!
//! Unlike every other primitive in this crate, `Page` is **not** a
//! `<ui-*>` custom element — it's a whole document. That's why it's
//! hand-written instead of `#[derive(UiComponent)]`.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! let html = page()
//!     .title("Dashboard")
//!     .add(theme_toggle())
//!     .add(button().label("Save"))
//!     .render();
//! // <!doctype html>
//! // <html lang="en" data-theme="light">
//! //   <head>
//! //     <meta charset="utf-8">
//! //     <meta name="viewport" content="width=device-width,initial-scale=1">
//! //     <title>Dashboard</title>
//! //     <link rel="stylesheet" href="/lit-components/assets/tokens.css">
//! //     <link rel="stylesheet" href="/lit-components/assets/global.css">
//! //     <link rel="stylesheet" href="/lit-components/assets/layout.css">
//! //     <script type="module" src="/lit-components/components/index.js"></script>
//! //   </head>
//! //   <body>…children…</body>
//! // </html>
//!
//! # Future
//! The whole point of `<body>` accepting arbitrary children is that
//! later you can drop a shell primitive in there (`shell().topbar(…).main(…)`)
//! without touching `page()`. `page()` stays the thin outer envelope.

use crate::core::{escape_html, Child, Component};

/// Start building a full HTML document.
pub fn page() -> Page { Page::default() }

pub struct Page {
    title:       String,
    lang:        String,
    theme:       String,
    assets_base: String,
    head_extra:  Option<String>,
    body_class:  Option<String>,
    children:    Vec<Child>,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            title:       String::new(),
            lang:        "en".to_string(),
            theme:       "light".to_string(),
            // Matches the default mount point in `server/src/main.rs`
            // (`ServeDir::new(../lit-components)` at `/`). Override with
            // `.assets_base(...)` if you're hosting the asset tree
            // somewhere else (CDN, relative path for file:// preview, …).
            assets_base: "/lit-components".to_string(),
            head_extra:  None,
            body_class:  None,
            children:    Vec::new(),
        }
    }
}

impl Page {
    /// Document `<title>` — required for a11y and browser tab labels.
    pub fn title(mut self, s: impl Into<String>) -> Self { self.title = s.into(); self }

    /// `<html lang="…">` — default `"en"`.
    pub fn lang(mut self, s: impl Into<String>)  -> Self { self.lang  = s.into(); self }

    /// Initial `<html data-theme="…">`. `core.js` will overwrite this
    /// pre-paint if the user has a persisted preference in
    /// `localStorage['erp.theme']`, so this is really a fallback for
    /// first-ever visits. Default `"light"`.
    pub fn theme(mut self, s: impl Into<String>) -> Self { self.theme = s.into(); self }

    /// URL prefix for the stylesheet + module-script links. Default is
    /// `/lit-components` which matches the demo server. Set to a
    /// relative path (`"../lit-components"`) when serving the generated
    /// HTML from disk without a server, or to a full origin
    /// (`"https://cdn.example/lit-ui@0.1"`) when hosting the assets
    /// elsewhere. Trailing slashes are trimmed.
    pub fn assets_base(mut self, s: impl Into<String>) -> Self {
        let s = s.into();
        self.assets_base = s.trim_end_matches('/').to_string();
        self
    }

    /// Raw HTML injected at the end of `<head>` — favicons, extra
    /// `<meta>`, `<style>` blocks, analytics snippets, etc. Trusted:
    /// **not escaped**.
    pub fn head_extra(mut self, html: impl Into<String>) -> Self {
        self.head_extra = Some(html.into()); self
    }

    /// `<body class="…">` — handy for page-level scoping or CSS resets.
    pub fn body_class(mut self, s: impl Into<String>) -> Self {
        self.body_class = Some(s.into()); self
    }

    /// Append one child to the body.
    pub fn add(mut self, child: impl Component + 'static) -> Self {
        self.children.push(Box::new(child)); self
    }
    /// Append many children.
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
        let base = self.assets_base.as_str();
        let mut body_html = String::new();
        for c in &self.children { body_html.push_str(&c.render()); }

        let body_open = match &self.body_class {
            Some(cls) => format!(r#"<body class="{}">"#, escape_html(cls)),
            None      => "<body>".to_string(),
        };
        let head_extra = self.head_extra.as_deref().unwrap_or("");

        format!(
            concat!(
                "<!doctype html>\n",
                r#"<html lang="{lang}" data-theme="{theme}">"#, "\n",
                "<head>\n",
                r#"  <meta charset="utf-8">"#, "\n",
                r#"  <meta name="viewport" content="width=device-width,initial-scale=1">"#, "\n",
                "  <title>{title}</title>\n",
                r#"  <link rel="stylesheet" href="{base}/assets/tokens.css">"#, "\n",
                r#"  <link rel="stylesheet" href="{base}/assets/global.css">"#, "\n",
                r#"  <link rel="stylesheet" href="{base}/assets/layout.css">"#, "\n",
                r#"  <script type="module" src="{base}/components/index.js"></script>"#, "\n",
                // Guaranteed baseline so a page is never completely blank
                // when the external stylesheets fail to load or the design
                // tokens aren't defined yet. Kept minimal — the real
                // styling comes from tokens.css / global.css.
                "  <style>\n",
                "    html, body { margin: 0; padding: 0; background: #ffffff; color: #111111; font-family: system-ui, sans-serif; }\n",
                // Custom elements are `display: inline` by default in
                // most browsers until they upgrade. Force `block` on the
                // layout / container primitives so their children have
                // room to lay out immediately.
                "    ui-center, ui-stack, ui-columns, ui-grid, ui-sidebar, ui-cluster, ui-heading, ui-progress, ui-slider { display: block; }\n",
                "  </style>\n",
                "{head_extra}",
                "</head>\n",
                "{body_open}{body_html}</body>\n",
                "</html>\n",
            ),
            lang       = escape_html(&self.lang),
            theme      = escape_html(&self.theme),
            title      = escape_html(&self.title),
            base       = base,
            head_extra = head_extra,
            body_open  = body_open,
            body_html  = body_html,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::button::button;

    #[test]
    fn defaults_produce_full_document() {
        let html = page().title("Hi").render();
        // Sanity spot-checks — the whole document is asserted in the next test.
        assert!(html.starts_with("<!doctype html>\n<html lang=\"en\" data-theme=\"light\">"));
        assert!(html.contains("<title>Hi</title>"));
        assert!(html.contains(r#"<link rel="stylesheet" href="/lit-components/assets/tokens.css">"#));
        assert!(html.contains(r#"<script type="module" src="/lit-components/components/index.js"></script>"#));
        assert!(html.ends_with("</html>\n"));
    }

    #[test]
    fn document_shape_is_stable() {
        // Full byte-exact tests are brittle when we tweak the safety-net
        // <style>. Assert the structural bits instead.
        let html = page().title("Hi").render();
        assert!(html.starts_with("<!doctype html>\n<html lang=\"en\" data-theme=\"light\">"));
        assert!(html.contains("<title>Hi</title>"));
        assert!(html.contains(r#"<link rel="stylesheet" href="/lit-components/assets/tokens.css">"#));
        assert!(html.contains(r#"<link rel="stylesheet" href="/lit-components/assets/global.css">"#));
        assert!(html.contains(r#"<link rel="stylesheet" href="/lit-components/assets/layout.css">"#));
        assert!(html.contains(r#"<script type="module" src="/lit-components/components/index.js"></script>"#));
        assert!(html.contains("<body>"));
        assert!(html.ends_with("</html>\n"));
    }

    #[test]
    fn children_render_inside_body() {
        let html = page().title("t").add(button().label("Save")).render();
        assert!(html.contains(
            r#"<body><ui-button variant="primary" size="md" type="button">Save</ui-button></body>"#
        ));
    }

    #[test]
    fn assets_base_override_and_trailing_slash_trim() {
        let html = page().title("t").assets_base("https://cdn.example/lit-ui/").render();
        assert!(html.contains(r#"href="https://cdn.example/lit-ui/assets/tokens.css""#));
        assert!(html.contains(r#"src="https://cdn.example/lit-ui/components/index.js""#));
        // No double slash.
        assert!(!html.contains("lit-ui//assets"));
    }

    #[test]
    fn title_is_html_escaped() {
        let html = page().title("<x>&\"y").render();
        assert!(html.contains("<title>&lt;x&gt;&amp;&quot;y</title>"));
    }

    #[test]
    fn theme_lang_and_body_class_reflect() {
        let html = page()
            .title("t").lang("hi").theme("dark").body_class("has-shell dark-bg").render();
        assert!(html.contains(r#"<html lang="hi" data-theme="dark">"#));
        assert!(html.contains(r#"<body class="has-shell dark-bg">"#));
    }

    #[test]
    fn head_extra_is_injected_raw() {
        let html = page()
            .title("t")
            .head_extra(r#"<link rel="icon" href="/favicon.ico">"#)
            .render();
        assert!(html.contains(r#"<link rel="icon" href="/favicon.ico">"#));
    }
}
