// TODO(auth): re-enable — auth is temporarily STUBBED so the portal can be
// previewed without login. To restore, revert this file (see git history).

use axum::{
    body::Body,
    extract::State,
    http::Request as AxumRequest,
    middleware::Next,
    response::Response,
};
use std::collections::HashSet;

use crate::http::AppState;
use crate::middleware::auth::SessionUser;
use crate::services::perm::DEMO_VIEW;

fn stub_session_user() -> SessionUser {
    let mut permissions = HashSet::new();
    permissions.insert(DEMO_VIEW.to_string());

    SessionUser {
        user_id: 0,
        username: "dev".to_string(),
        display: "Dev User".to_string(),
        session_id: 0,
        roles: vec!["admin".to_string()],
        permissions,
    }
}

// TODO(auth): re-enable — currently injects a stub SessionUser and always
// forwards the request.
pub async fn require_session(
    State(_state): State<AppState>,
    mut req: AxumRequest<Body>,
    next: Next,
) -> Response {
    req.extensions_mut().insert(stub_session_user());
    next.run(req).await
}

// TODO(auth): re-enable — currently a no-op passthrough.
pub async fn require_portal_shell(mut req: AxumRequest<Body>, next: Next) -> Response {
    if req.extensions().get::<SessionUser>().is_none() {
        req.extensions_mut().insert(stub_session_user());
    }
    next.run(req).await
}
