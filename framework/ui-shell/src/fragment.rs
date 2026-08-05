//! The [`Fragment`] envelope — the single wire format for every island
//! swap in the framework.
//!
//! A fragment carries three things:
//!
//! 1. A [`Target`] — which named island it applies to.
//! 2. An [`Action`] — how to apply it (replace, append, prepend, remove, update).
//! 3. A body — arbitrary HTML, typically built by `rust-dsl`.
//!
//! [`Fragments`] is just a batch (order-preserved). It implements
//! [`axum::response::IntoResponse`] so a route handler can `return frags;`.

use axum::{
    body::Body,
    http::{header, HeaderValue, Response, StatusCode},
    response::IntoResponse,
};
use lit_ui::core::Component;

use crate::negotiation::FRAGMENTS_MIME;

/// A named island in the shell. Strings match the `slot` name in
/// `<ui-app-shell>`.
///
/// [`Target::Custom`] lets an app extend the shell with new islands
/// without changing the framework.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// The primary content island.
    Main,
    /// The copilot pane.
    Copilot,
    /// The persistent topbar.
    Topbar,
    /// The persistent sidebar.
    Sidebar,
    /// The toast host.
    Toast,
    /// The modal host.
    Modal,
    /// Any other named slot the app has added to its shell.
    Custom(String),
}

impl Target {
    /// The wire-level name emitted as the `target=".."` attribute.
    pub fn as_str(&self) -> &str {
        match self {
            Target::Main => crate::TARGET_MAIN,
            Target::Copilot => crate::TARGET_COPILOT,
            Target::Topbar => crate::TARGET_TOPBAR,
            Target::Sidebar => crate::TARGET_SIDEBAR,
            Target::Toast => crate::TARGET_TOAST,
            Target::Modal => crate::TARGET_MODAL,
            Target::Custom(s) => s.as_str(),
        }
    }
}

/// How the client should apply a fragment to its target island.
///
/// Kept intentionally small — extend only when a real UX need appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Replace the entire target island's children with the fragment body.
    /// This is the default and matches "island navigation".
    Replace,
    /// Append the fragment body as the last children of the target.
    Append,
    /// Prepend the fragment body as the first children of the target.
    Prepend,
    /// Remove the target island's children. Body is ignored.
    Remove,
    /// Same as [`Replace`] but sets `.innerHTML` directly, preserving the
    /// target element (useful when the target itself carries handlers).
    Update,
}

impl Action {
    /// The wire-level name emitted as the `action=".."` attribute.
    pub fn as_str(&self) -> &str {
        match self {
            Action::Replace => "replace",
            Action::Append => "append",
            Action::Prepend => "prepend",
            Action::Remove => "remove",
            Action::Update => "update",
        }
    }
}

/// A single `<ui-fragment>` envelope.
///
/// Prefer the constructor helpers ([`Fragment::replace`], [`append`],
/// etc.) — they read like English at the call site.
#[derive(Debug, Clone)]
pub struct Fragment {
    pub target: Target,
    pub action: Action,
    /// Pre-rendered HTML body. Use [`Fragment::replace`] with a DSL
    /// component if you want the crate to call `.render()` for you.
    pub body: String,
}

impl Fragment {
    /// Build a `replace` fragment from any `lit-ui` [`Component`].
    pub fn replace<C: Component>(target: Target, body: C) -> Self {
        Self { target, action: Action::Replace, body: body.render() }
    }

    /// Build an `append` fragment (e.g. push a toast onto the toast host).
    pub fn append<C: Component>(target: Target, body: C) -> Self {
        Self { target, action: Action::Append, body: body.render() }
    }

    /// Build a `prepend` fragment.
    pub fn prepend<C: Component>(target: Target, body: C) -> Self {
        Self { target, action: Action::Prepend, body: body.render() }
    }

    /// Build a `remove` fragment (body is ignored on the client).
    pub fn remove(target: Target) -> Self {
        Self { target, action: Action::Remove, body: String::new() }
    }

    /// Build an `update` (innerHTML) fragment.
    pub fn update<C: Component>(target: Target, body: C) -> Self {
        Self { target, action: Action::Update, body: body.render() }
    }

    /// Escape hatch — build from an already-rendered HTML string.
    /// Prefer the DSL-based constructors; use this only for legacy HTML
    /// or when composing fragments returned from a subsystem.
    pub fn from_html(target: Target, action: Action, html: impl Into<String>) -> Self {
        Self { target, action, body: html.into() }
    }
}

/// A batch of fragments applied in order.
///
/// Implements [`IntoResponse`] so a route handler can return it directly.
#[derive(Debug, Clone, Default)]
pub struct Fragments(pub Vec<Fragment>);

impl Fragments {
    /// Create an empty batch. Chain [`Fragments::push`] to add envelopes.
    pub fn new() -> Self { Self(Vec::new()) }

    /// Add a fragment to the batch. Order is preserved on the wire.
    pub fn push(mut self, f: Fragment) -> Self { self.0.push(f); self }

    /// Extend the batch from any iterator of fragments.
    pub fn extend<I: IntoIterator<Item = Fragment>>(mut self, iter: I) -> Self {
        self.0.extend(iter);
        self
    }

    /// True when there are no fragments to send.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

/// Renders a fragment or fragment batch to the on-the-wire representation.
///
/// Kept as its own trait (rather than a `Display` impl) so we can add
/// alternate encodings (e.g. JSON) later without breaking callers.
pub trait Render {
    /// Append the wire representation to `out`.
    fn render_into(&self, out: &mut String);

    /// Convenience: allocate and return the full wire representation.
    fn render_string(&self) -> String {
        let mut s = String::new();
        self.render_into(&mut s);
        s
    }
}

impl Render for Fragment {
    fn render_into(&self, out: &mut String) {
        // No user-controlled values reach an attribute here — targets are
        // enum-driven (or `Custom(String)` under the app's control) and
        // action is enum-driven. Bodies are already HTML by contract.
        out.push_str("<ui-fragment target=\"");
        out.push_str(self.target.as_str());
        out.push_str("\" action=\"");
        out.push_str(self.action.as_str());
        out.push_str("\">");
        if !matches!(self.action, Action::Remove) {
            out.push_str(&self.body);
        }
        out.push_str("</ui-fragment>");
    }
}

impl Render for Fragments {
    fn render_into(&self, out: &mut String) {
        for f in &self.0 {
            f.render_into(out);
        }
    }
}

impl IntoResponse for Fragment {
    fn into_response(self) -> Response<Body> {
        Fragments::new().push(self).into_response()
    }
}

impl IntoResponse for Fragments {
    fn into_response(self) -> Response<Body> {
        let body = self.render_string();
        let mut resp = Response::new(Body::from(body));
        *resp.status_mut() = StatusCode::OK;
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(FRAGMENTS_MIME),
        );
        resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RawHtml(&'static str);
    impl Component for RawHtml {
        fn render(&self) -> String { self.0.to_string() }
    }

    #[test]
    fn renders_single_replace() {
        let f = Fragment::replace(Target::Main, RawHtml("<p>hi</p>"));
        assert_eq!(
            f.render_string(),
            r#"<ui-fragment target="main" action="replace"><p>hi</p></ui-fragment>"#
        );
    }

    #[test]
    fn remove_omits_body() {
        let f = Fragment::remove(Target::Modal);
        assert_eq!(
            f.render_string(),
            r#"<ui-fragment target="modal" action="remove"></ui-fragment>"#
        );
    }

    #[test]
    fn batch_preserves_order() {
        let batch = Fragments::new()
            .push(Fragment::replace(Target::Main, RawHtml("A")))
            .push(Fragment::append(Target::Toast, RawHtml("B")));
        let s = batch.render_string();
        assert!(s.starts_with(r#"<ui-fragment target="main""#));
        assert!(s.contains(r#"<ui-fragment target="toast" action="append">B"#));
    }

    #[test]
    fn custom_target_roundtrips() {
        let f = Fragment::from_html(
            Target::Custom("breadcrumbs".into()),
            Action::Update,
            "<span>x</span>",
        );
        assert!(f.render_string().contains(r#"target="breadcrumbs""#));
    }
}
