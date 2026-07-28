use axum::{
    body::Body,
    http::Request as AxumRequest,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use crate::security::session_user::SessionUser;

pub async fn require_staff_shell(req: AxumRequest<Body>, next: Next) -> Response {
    if req.extensions().get::<SessionUser>().is_some() {
        return next.run(req).await;
    }
    let path = req.uri().path();
    let tenant = path.split('/').nth(2).unwrap_or("default");
    Redirect::to(&format!("/web/{}/login", tenant)).into_response()
}
