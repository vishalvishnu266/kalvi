use askama::Template;
use axum::{response::Response, Extension};

use crate::exception::web_error::{render, WebError};
use crate::security::permissions::*;
use crate::security::session_user::SessionUser;
use crate::security::tenant_scope::TenantScope;
use crate::view::layout::{visible_nav_items, NavContext, NavItem};

#[derive(Template)]
#[template(path = "dashboard.html")]
struct MenuPage<'a> {
    nav: &'a NavContext,
    nav_items: Vec<&'static NavItem>,
    tiles: Vec<&'static Tile>,
}

pub struct Tile {
    pub href: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub gradient: &'static str,
    pub perm: Option<&'static [&'static str]>,
}

fn tiles() -> &'static [Tile] {
    &[Tile {
        href: "demo",
        label: "Demo",
        description: "Reference module wiring",
        icon: "sparkles",
        gradient: "from-violet-500 to-purple-600",
        perm: Some(&[DEMO_VIEW]),
    }]
}

fn visible_tiles(session: &SessionUser) -> Vec<&'static Tile> {
    tiles()
        .iter()
        .filter(|t| match t.perm {
            None => true,
            Some(codes) => session.any_of(codes),
        })
        .collect()
}

pub async fn index(
    scope: TenantScope,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    let nav = NavContext::new(
        session.display.clone(),
        scope.tenant.as_str().to_string(),
        "dashboard",
        "Home",
    );
    let nav_items = visible_nav_items(&session);
    let tiles = visible_tiles(&session);
    render(&MenuPage {
        nav: &nav,
        nav_items,
        tiles,
    })
}
