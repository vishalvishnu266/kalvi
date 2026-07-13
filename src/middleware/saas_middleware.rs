use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use crate::util::{SessionUtil, AppError};

pub async fn saas_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let path = req.uri().path();
    
    let is_saas_login = path == "/saas/login" || path == "/saas/login/";
    
    if path.starts_with("/saas/") && !is_saas_login {
        if !SessionUtil::is_saas_session(req.headers()) {
            return Ok(Redirect::to("/saas/login").into_response());
        }
    }
    
    Ok(next.run(req).await)
}
