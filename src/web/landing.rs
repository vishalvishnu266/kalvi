//! Public marketing / landing page for `/`.
//!
//! * If the visitor already has an `erp_tenant` session cookie, we redirect
//!   them straight into their tenant app shell at `/web/{tenant}/`.
//! * Otherwise we render a small SaaS landing page with a "Sign in" CTA.

use askama::Template;
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
};

use crate::web::auth::read_cookie_from_headers;
use crate::web::error::{render, WebError};

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingPage;

pub async fn index(headers: HeaderMap) -> Result<Response, WebError> {
    if let Some(tenant) = read_cookie_from_headers(&headers, "erp_tenant") {
        let location = format!("/web/{}/", tenant);
        return Ok(Redirect::to(&location).into_response());
    }
    render(&LandingPage)
}
