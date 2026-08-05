//! Content negotiation: full-page HTML vs fragment-only responses.
//!
//! The rule is deliberately simple, easy to reason about, and easy to
//! trigger from the client:
//!
//! * `Accept: text/vnd.ui-fragments+html` → return the fragment batch as-is.
//! * anything else → return a full HTML document produced by a
//!   caller-supplied "chrome" closure that renders the app shell with
//!   the fragments already inlined into their slots.
//!
//! Keeping the chrome as an injected closure means this crate stays
//! agnostic about which topbar/sidebar/copilot the app wants — those
//! decisions live in the server crate that consumes ui-shell.

use axum::{
    body::Body,
    http::{header, HeaderMap, HeaderValue, Response, StatusCode},
    response::IntoResponse,
};

use crate::fragment::{Fragments, Render};

/// The MIME type that identifies a fragment-only response.
///
/// Chosen with a `vnd.` prefix so it never collides with a standard type
/// and so intermediaries (proxies, browsers) don't try to render it as
/// generic HTML. Suffix `+html` keeps content-sniffing sane.
pub const FRAGMENTS_MIME: &str = "text/vnd.ui-fragments+html";

/// Signature of the "chrome" function: given the pre-rendered fragment
/// batch (already HTML) plus the current request URI path, return a
/// full `<!doctype html>…</html>` document.
///
/// The path enables the chrome to render context-aware regions — most
/// commonly, highlighting the active app in an activity bar / tab bar
/// via `aria-current="page"`.
///
/// Typical implementations build a `lit_ui::components::app_shell::AppShell`
/// with slotted children and let the DSL render it.
pub type ChromeFn = fn(fragments_html: &str, current_path: &str) -> String;

/// Choose the response shape based on the request's `Accept` header.
///
/// - Returns fragments-only when the client asked for
///   [`FRAGMENTS_MIME`].
/// - Otherwise wraps the fragments in the caller-supplied chrome and
///   returns a full HTML document (200 OK, `text/html; charset=utf-8`).
///
/// Pass the current URI path so the chrome can render active-state
/// hints (e.g. `aria-current="page"` on the matching activity-bar
/// entry). If your chrome doesn't care about it, pass any `&str`.
pub fn negotiate(
    headers: &HeaderMap,
    current_path: &str,
    fragments: Fragments,
    chrome: ChromeFn,
) -> Response<Body> {
    if wants_fragments(headers) {
        return fragments.into_response();
    }

    let inner = fragments.render_string();
    let html = chrome(&inner, current_path);
    let mut resp = Response::new(Body::from(html));
    *resp.status_mut() = StatusCode::OK;
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    resp
}

/// True if the `Accept` header explicitly requests the fragment MIME.
///
/// Matches `text/vnd.ui-fragments+html` as a substring — clients are free
/// to append q-values or additional types (`text/html`), we still honour
/// the fragment preference.
pub fn wants_fragments(headers: &HeaderMap) -> bool {
    headers
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.contains(FRAGMENTS_MIME))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_fragment_accept() {
        let mut h = HeaderMap::new();
        h.insert(header::ACCEPT, HeaderValue::from_static(FRAGMENTS_MIME));
        assert!(wants_fragments(&h));
    }

    #[test]
    fn defaults_to_full_page() {
        let mut h = HeaderMap::new();
        h.insert(header::ACCEPT, HeaderValue::from_static("text/html"));
        assert!(!wants_fragments(&h));
    }

    #[test]
    fn accepts_mixed_header() {
        let mut h = HeaderMap::new();
        h.insert(
            header::ACCEPT,
            HeaderValue::from_static("text/vnd.ui-fragments+html, text/html;q=0.9"),
        );
        assert!(wants_fragments(&h));
    }
}
