//! Demo web (server-rendered HTML) handler.
//!
//! Same pattern as the JSON API in `src/api/demo.rs` but renders a
//! template that extends `base.html`. Use as a template for future
//! server-rendered modules.

use askama::Template;
use axum::{response::Response, Extension};

use crate::http::TenantScope;
use crate::middleware::auth::SessionUser;
use crate::models::demo::DemoMessage;
use crate::services::demo as demo_svc;
use crate::web::error::{render, WebError};
use crate::web::layout::{visible_nav_items, NavContext, NavItem};

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
    let messages = demo_svc::list_messages(&scope.pool, 50)
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
