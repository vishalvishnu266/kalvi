use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use crate::util::SessionUtil;

pub async fn saas_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    
    // SaaS Owner paths - requires "saas_" session
    // Allow access to login, but protect everything else under /saas/
    let is_saas_login = path == "/saas/login" || path == "/saas/login/";
    
    if path.starts_with("/saas/") && !is_saas_login {
        let session_id = SessionUtil::get_session_id(req.headers());
        match session_id {
            Some(sid) if sid.starts_with("saas_") => {},
            _ => return Ok(Redirect::to("/saas/login").into_response()),
        }
    }
    
    Ok(next.run(req).await)
}
