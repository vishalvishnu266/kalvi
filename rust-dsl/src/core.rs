//! Base traits + shared helpers used by every component.
//!
//! There are only two ideas here:
//!
//! 1. [`Component`] — anything that can render itself into an HTML `String`.
//! 2. [`Node`] — an already-rendered chunk of HTML (or plain text) that can
//!    stand in for a `Component`, so you can freely mix raw strings and
//!    typed builders under the same `Vec<Box<dyn Component>>`.

use std::fmt;

/// Every UI element implements this. Renders to an HTML string.
///
/// The returned `String` is expected to be trusted — components themselves
/// must escape any user-supplied text via [`escape_html`].
pub trait Component {
    fn render(&self) -> String;
}

/// Blanket `Display` support via [`RenderExt`] rather than a blanket impl
/// (which would conflict with foreign types).
pub trait RenderExt {
    /// Convenience: render into a `String` (identical to `render`).
    fn to_html(&self) -> String;
}

impl<T: Component + ?Sized> RenderExt for T {
    fn to_html(&self) -> String {
        self.render()
    }
}

// ---------------------------------------------------------------------------
// Raw / text nodes
// ---------------------------------------------------------------------------

/// A pre-rendered chunk of markup, or a plain-text run.
///
/// * `Node::text("Hi")` renders escaped: `Hi`
/// * `Node::raw("<b>Hi</b>")` renders verbatim: `<b>Hi</b>`
///
/// Use `Node` when you want to slot a `&str`, `String`, or hand-written HTML
/// into a component tree without wrapping it in another builder.
pub enum Node {
    Text(String),
    Raw(String),
}

impl Node {
    pub fn text(s: impl Into<String>) -> Self { Node::Text(s.into()) }
    pub fn raw(s: impl Into<String>)  -> Self { Node::Raw(s.into()) }
}

impl Component for Node {
    fn render(&self) -> String {
        match self {
            Node::Text(t) => escape_html(t),
            Node::Raw(r)  => r.clone(),
        }
    }
}

// Owned String → escaped text is convenient inside .add(...) calls.
// (No impl for &str — trait objects need 'static, so use `Node::text("…")`
// or `.to_owned()` if you have a borrowed slice.)
impl Component for String { fn render(&self) -> String { escape_html(self) } }

// ---------------------------------------------------------------------------
// Boxed child helper
// ---------------------------------------------------------------------------

/// Alias for the boxed trait object stored inside containers.
pub type Child = Box<dyn Component>;

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.render())
    }
}

// ---------------------------------------------------------------------------
// HTML escaping (attributes AND text)
// ---------------------------------------------------------------------------

/// Escape a string for safe insertion into HTML text content or attributes.
///
/// The tag renderers use this on every user-supplied value.
pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&'  => out.push_str("&amp;"),
            '<'  => out.push_str("&lt;"),
            '>'  => out.push_str("&gt;"),
            '"'  => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _    => out.push(ch),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Tag rendering helpers (used by every component)
// ---------------------------------------------------------------------------

/// One HTML attribute — either a `key="value"` pair or a boolean flag.
///
/// Boolean attributes render as just the key (e.g. `required`), matching
/// the HTML boolean-attribute convention that Lit components expect.
pub enum Attr {
    KV(&'static str, String),
    Flag(&'static str),
}

impl Attr {
    pub fn kv(k: &'static str, v: impl Into<String>) -> Self { Attr::KV(k, v.into()) }
    pub fn flag(k: &'static str) -> Self { Attr::Flag(k) }

    fn render(&self) -> String {
        match self {
            Attr::KV(k, v) => format!(" {}=\"{}\"", k, escape_html(v)),
            Attr::Flag(k)  => format!(" {}", k),
        }
    }
}

/// Render an open tag with its attributes.
pub fn open(tag: &str, attrs: &[Attr]) -> String {
    let mut s = String::from("<");
    s.push_str(tag);
    for a in attrs { s.push_str(&a.render()); }
    s.push('>');
    s
}

/// Render a full container: `<tag attrs...>children</tag>`.
pub fn wrap(tag: &str, attrs: &[Attr], body: &str) -> String {
    format!("{}{}</{}>", open(tag, attrs), body, tag)
}
