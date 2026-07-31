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
        format!(
            r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <title>{title}</title>
  <link rel="stylesheet" href="{base}/assets/tokens.css">
  <link rel="stylesheet" href="{base}/assets/global.css">
  <script type="module" src="{base}/components/index.js"></script>
</head>
<body>
{body}
</body>
</html>
"#,
            title = escape_html(&self.title),
            base  = escape_html(&self.assets_base),
            body  = body,
        )
    }
}
