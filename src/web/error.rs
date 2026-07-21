//! Error → HTML response mapping for the web layer.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::services::ServiceError;

/// A minimal wrapper so `?` works in web handlers.
pub struct WebError(pub StatusCode, pub String);

impl WebError {
    pub fn bad(msg: impl Into<String>) -> Self { Self(StatusCode::BAD_REQUEST, msg.into()) }
}

impl From<ServiceError> for WebError {
    fn from(e: ServiceError) -> Self {
        let sc = match &e {
            ServiceError::NotFound        => StatusCode::NOT_FOUND,
            ServiceError::Validation(_)   => StatusCode::BAD_REQUEST,
            ServiceError::Conflict(_)     => StatusCode::CONFLICT,
            ServiceError::Unauthorized    => StatusCode::UNAUTHORIZED,
            ServiceError::Forbidden(_)    => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self(sc, e.to_string())
    }
}

impl From<askama::Error> for WebError {
    fn from(e: askama::Error) -> Self { Self(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let WebError(sc, msg) = self;
        // For the SPA-style shell we render a small HTML error page.
        let body = format!(
            "<div style=\"font-family:-apple-system,BlinkMacSystemFont,sans-serif;\
                        padding:40px;max-width:600px;margin:auto;color:#111\">\
             <h1 style=\"font-size:20px;margin:0 0 8px\">Something went wrong</h1>\
             <p style=\"color:#555\">{}</p></div>",
            html_escape(&msg)
        );
        (sc, [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")], body).into_response()
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Render an askama template into an `axum` HTML response.
pub fn render<T: askama::Template>(t: &T) -> Result<Response, WebError> {
    let html = t.render()?;
    Ok((
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html,
    ).into_response())
}
