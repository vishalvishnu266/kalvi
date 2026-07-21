//! Public marketing / landing page for `/`.
//!
//! * If the visitor already has an `erp_tenant` session cookie, we redirect
//!   them straight into their tenant app shell at `/web/{tenant}/`.
//! * Otherwise we render a small SaaS landing page with a "Sign in" CTA
//!   pointing to `/web/login`.

use askama::Template;
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};

use crate::web::auth::read_cookie_from_headers;
use crate::web::error::{render, WebError};

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingPage;

pub fn routes() -> Router {
    Router::new().route("/", get(index))
}

async fn index(headers: HeaderMap) -> Result<Response, WebError> {
    // If already signed in, jump straight into the tenant shell.
    if let Some(tenant) = read_cookie_from_headers(&headers, "erp_tenant") {
        let location = format!("/web/{}/", tenant);
        return Ok(Redirect::to(&location).into_response());
    }
    render(&LandingPage)
}
