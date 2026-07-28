use askama::Template;
use axum::{response::Response, Extension};

use crate::entity::demo_message::DemoMessage;
use crate::exception::web_error::{render, WebError};
use crate::security::session_user::SessionUser;
use crate::security::tenant_scope::TenantScope;
use crate::service::demo_service;
use crate::view::layout::{visible_nav_items, NavContext, NavItem};

#[derive(Template)]
#[template(path = "demo/index.html")]
struct DemoPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    messages: Vec<DemoMessage>,
}

pub async fn index(
    scope: TenantScope,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    let messages = demo_service::list_messages(&scope.pool, 50)
        .await
        .unwrap_or_default();

    let nav = NavContext::new(
        session.display.clone(),
        scope.tenant.as_str().to_string(),
        "demo",
        "Demo",
    );
    let nav_items = visible_nav_items(&session);

    render(&DemoPage {
        nav: &nav,
        nav_items,
        messages,
    })
}
