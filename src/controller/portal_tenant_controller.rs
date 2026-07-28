use askama::Template;
use axum::{response::Response, Extension};

use crate::exception::web_error::{render, WebError};
use crate::security::session_user::SessionUser;
use crate::security::tenant_scope::TenantScope;

#[derive(Template)]
#[template(path = "portal/index.html")]
struct PortalHome<'a> {
    tenant_id: &'a str,
    user_display: &'a str,
}

pub async fn index(
    scope: TenantScope,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    render(&PortalHome {
        tenant_id: scope.tenant.as_str(),
        user_display: &session.display,
    })
}
