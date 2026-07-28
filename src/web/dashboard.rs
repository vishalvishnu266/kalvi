//! Tenant dashboard — the post-login home. Renders a launcher grid
//! filtered by the caller's permissions.
//!
//! Business tiles have been stripped; only the `demo` tile ships out
//! of the box. Add one `Tile` per module here as you build them.

use askama::Template;
use axum::{response::Response, Extension};

use crate::http::TenantScope;
use crate::middleware::auth::SessionUser;
use crate::services::perm::*;
use crate::web::error::{render, WebError};
use crate::web::layout::{visible_nav_items, NavContext, NavItem};

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
        // Shoelace / Bootstrap icon name (see https://icons.getbootstrap.com/).
        icon: "stars",
        // Semantic tile class defined in `static/app-shell.css` — replaces the
        // old Tailwind `from-*/to-*` dynamic gradient string.
        gradient: "tile-violet",
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
