//! `GET /apps` — the OS-style launcher page.
//!
//! Renders every registered [`crate::apps::App`] as a colored icon
//! tile in a responsive grid. Clicking a tile navigates via the shell
//! runtime (island swap + URL update, no reload).
//!
//! Style: iOS-home hybrid.
//!   * 3-column grid on phone (each tile ~28% viewport width)
//!   * 5-column grid on tablet+
//!   * Icons are colored rounded squares (background = 18% opacity of
//!     the app's brand color, glyph = full saturation of the same)
//!   * Labels under each icon, ~12px, centered

use axum::{http::HeaderMap, response::Response};
use lit_ui::core::Node;
use lit_ui::prelude::*;
use ui_shell::{negotiate, Fragment, Fragments, Target};

use crate::apps;
use crate::shell::chrome;

pub async fn handler(headers: HeaderMap) -> Response {
    let mut grid = String::from(
        r#"<div class="ui-apps-page">
             <h1 class="ui-apps-title">Apps</h1>
             <div class="ui-apps-grid" role="list">"#,
    );

    for app in apps::all() {
        // The tile background is an rgba() built from the app's hex
        // color at 18% alpha. Straight hex substitution keeps this
        // simple (no color-mixing math needed).
        grid.push_str(&format!(
            r#"<a class="ui-apps-tile" href="{route}" role="listitem" data-app-id="{id}">
                 <span class="ui-apps-icon" style="background:{color}22;color:{color};">
                   <ui-icon name="{icon}" size="28"></ui-icon>
                 </span>
                 <span class="ui-apps-label">{label}</span>
               </a>"#,
            route = app.route,
            id    = app.id,
            color = app.color,
            icon  = app.icon,
            label = app.label,
        ));
    }

    grid.push_str("</div></div>");

    // Inline the CSS with the page — it's small and only used here.
    grid.push_str(
        r#"<style>
          .ui-apps-page { max-width: 960px; margin: 0 auto; padding: 24px 20px 48px; }
          .ui-apps-title {
            font-size: 22px; font-weight: 600; letter-spacing: -0.01em;
            margin: 8px 0 24px;
          }
          .ui-apps-grid {
            display: grid;
            gap: 20px 12px;
            grid-template-columns: repeat(3, 1fr);
          }
          @media (min-width: 640px) { .ui-apps-grid { grid-template-columns: repeat(5, 1fr); } }
          @media (min-width: 960px) { .ui-apps-grid { grid-template-columns: repeat(6, 1fr); } }

          .ui-apps-tile {
            display: flex; flex-direction: column; align-items: center; gap: 8px;
            padding: 4px;
            color: inherit; text-decoration: none;
            border-radius: 14px;
            transition: transform .08s ease;
          }
          .ui-apps-tile:hover { transform: scale(1.04); }
          .ui-apps-tile:active { transform: scale(.96); }

          .ui-apps-icon {
            display: grid; place-items: center;
            width: 64px; height: 64px;
            border-radius: 18px;
            box-shadow: 0 1px 2px rgba(0,0,0,.04),
                        inset 0 1px 0 rgba(255,255,255,.35);
          }
          .ui-apps-label {
            font-size: 12px; text-align: center; line-height: 1.2;
            max-width: 84px;
          }
        </style>"#,
    );

    let body = Node::raw(grid);
    let frags = Fragments::new().push(Fragment::replace(Target::Main, body));
    negotiate(&headers, "/apps", frags, chrome)
}
