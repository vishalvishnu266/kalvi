use askama::Template;
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
};

use crate::exception::web_error::{render, WebError};
use crate::security::session_user::read_cookie_from_headers;

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
