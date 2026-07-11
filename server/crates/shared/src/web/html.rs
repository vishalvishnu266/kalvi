use std::fmt;
use axum::response::{IntoResponse, Response};

/// A wrapper that marks a string as safe to render as HTML.
#[derive(Debug, Clone)]
pub struct Html(pub String);

impl fmt::Display for Html {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoResponse for Html {
    fn into_response(self) -> Response {
        axum::response::Html(self.0).into_response()
    }
}

/// A trait for things that can be converted into HTML.
pub trait IntoHtml {
    fn into_html(self) -> Html;
}

impl IntoHtml for Html {
    fn into_html(self) -> Html {
        self
    }
}

impl IntoHtml for String {
    fn into_html(self) -> Html {
        Html(escape(&self))
    }
}

impl IntoHtml for &str {
    fn into_html(self) -> Html {
        Html(escape(self))
    }
}

/// Escapes special HTML characters to prevent XSS.
pub fn escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// A macro for composing HTML from multiple parts.
#[macro_export]
macro_rules! html {
    ($($part:expr),*) => {{
        let mut s = String::new();
        $(
            s.push_str(&$part.into_html().0);
        )*
        $crate::web::html::Html(s)
    }};
}
