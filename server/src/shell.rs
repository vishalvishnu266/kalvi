//! Layout chrome — everything *around* the current page.
//!
//! [`chrome`] is the `ui_shell::ChromeFn` implementation this app uses:
//! given a rendered fragment batch (which always contains at least the
//! `main` island), wrap it inside a full HTML document with the app
//! shell (topbar + sidebar + main slot + copilot slot).
//!
//! Kept intentionally simple: a topbar with the app title and a
//! sidebar with links to the demo pages. No copilot pane content yet —
//! that lands in milestone 8. The slot is present so the shell layout
//! is stable when we add it.

use lit_ui::core::Node;
use lit_ui::prelude::*;

/// Topbar with brand on the left, launcher trigger in the middle,
/// theme toggle on the right. The launcher trigger is a *fake* input:
/// clicking anywhere opens the real `<ui-launcher>` panel (which owns
/// the actual text field). This gives us OS-like discovery without
/// the complexity of managing two inputs.
fn topbar() -> impl Component {
    Node::raw(
        r#"<div class="ui-topbar">
             <button type="button" class="ui-topbar-launcher-btn"
                     onclick="window.ui.openLauncher()"
                     aria-label="Open launcher (Cmd+K)">
               <ui-icon name="grid" size="18"></ui-icon>
             </button>
             <button type="button" class="ui-topbar-search"
                     onclick="window.ui.openLauncher()"
                     aria-label="Search apps and commands (Cmd+K)">
               <ui-icon name="search" size="16"></ui-icon>
               <span class="ui-topbar-search-placeholder">Search apps, commands, or ask…</span>
               <span class="ui-topbar-kbd">⌘K</span>
             </button>
             <div class="ui-topbar-spacer"></div>
             <button type="button" class="ui-topbar-icon-btn"
                     onclick="var r=document.documentElement;var n=r.dataset.theme==='dark'?'light':'dark';r.dataset.theme=n;try{localStorage.setItem('erp.theme',n)}catch(_){}"
                     aria-label="Toggle theme">
               <ui-icon name="sun" size="18"></ui-icon>
             </button>
           </div>"#,
    )
}

/// Activity bar (desktop). Vertical 56px strip of icon buttons — one
/// per registered [`crate::apps::App`]. Content is 100% server-authored;
/// clicks are intercepted by the shell runtime and turned into island
/// swaps + URL updates.
///
/// The `aria-current="page"` attribute is added on the entry whose
/// route matches `current_path`, so the active state is a plain CSS
/// selector (`[aria-current="page"]`) — no JS.
fn activity_bar(current_path: &str) -> impl Component {
    let mut html = String::from(
        r#"<nav class="ui-activity-bar" aria-label="Apps">"#,
    );
    for app in crate::apps::all() {
        let active = if app.route == current_path {
            r#" aria-current="page""#
        } else {
            ""
        };
        html.push_str(&format!(
            r#"<a class="ui-activity-item" href="{route}" title="{label}" data-app-id="{id}"{active}>
                 <ui-icon name="{icon}" size="22"></ui-icon>
                 <span class="ui-activity-label">{label}</span>
               </a>"#,
            route  = app.route,
            label  = app.label,
            id     = app.id,
            icon   = app.icon,
            active = active,
        ));
    }
    html.push_str("</nav>");
    Node::raw(html)
}

/// Mobile bottom tab bar. Shown only on narrow viewports (CSS in
/// `ui-app-shell.js`). Same wire pattern as the activity bar — icons
/// are `<a>` tags that the shell runtime intercepts.
fn bottom_tab_bar(current_path: &str) -> impl Component {
    let mut html = String::from(
        r#"<nav class="ui-tab-bar" aria-label="Apps">"#,
    );
    for app in crate::apps::mobile_primary() {
        let active = if app.route == current_path {
            r#" aria-current="page""#
        } else {
            ""
        };
        html.push_str(&format!(
            r#"<a class="ui-tab-item" href="{route}" data-app-id="{id}"{active}>
                 <ui-icon name="{icon}" size="22"></ui-icon>
                 <span class="ui-tab-label">{label}</span>
               </a>"#,
            route  = app.route,
            id     = app.id,
            icon   = app.icon,
            label  = app.label,
            active = active,
        ));
    }
    html.push_str("</nav>");
    Node::raw(html)
}

/// The copilot pane. `<ui-copilot>` boots itself on connect: POSTs to
/// `/agent` and streams `<ui-fragment>` envelopes back through the
/// standard applier — same wire format as a normal navigation.
///
/// Marked `open` by default on desktop; on mobile the CSS keeps it
/// slid-off until the FAB toggles the attribute.
fn copilot_pane() -> impl Component {
    Node::raw(r#"<ui-copilot open endpoint="/agent"></ui-copilot>"#)
}

/// The `ChromeFn` passed to `ui_shell::negotiate`. Receives the
/// pre-rendered fragment batch (typically one `<ui-fragment target="main">`)
/// and returns a full HTML document with the shell wrapped around it.
///
/// # Design
///
/// The fragment envelopes are dropped into the `main` slot *raw*, wrapped
/// only in a `<div slot="main">` container. On first paint the
/// `<ui-fragment>` custom element upgrades and applies itself to the
/// `main` island — which is *this same div*. That's a no-op copy in
/// practice (children are re-parented into their own slot host), and it
/// means the exact same code path handles both first paint and
/// subsequent navigations.
pub fn chrome(fragments_html: &str, current_path: &str) -> String {
    // Wrap the pre-rendered fragment string in a raw node so it's not
    // re-escaped. The `Node::raw` contract is "this is trusted HTML".
    let main_slot = Node::raw(fragments_html.to_string());

    // Activity bar (desktop) and bottom tab bar (mobile) render the
    // SAME app list from crate::apps — one source of truth for both.
    let shell = app_shell()
        .topbar(topbar())
        .sidebar(activity_bar(current_path))
        .main(main_slot)
        .copilot(copilot_pane())
        .slot(Region::Custom("bottombar".into()), bottom_tab_bar(current_path));

    // Mount the launcher once. It's a full-viewport fixed overlay
    // (hidden by default), so where we mount it doesn't matter — we
    // just need it in the DOM. Stash it in the modal region so it's
    // grouped with the other overlay-like elements.
    let launcher = Node::raw(r#"<ui-launcher></ui-launcher>"#);

    // A tiny sheet of CSS specific to the topbar + activity bar + tab
    // bar. Kept inline so a change here doesn't need a browser cache
    // bust. Small; not worth splitting out.
    let nav_css = Node::raw(r#"<style>
        /* ─── Topbar ─── */
        .ui-topbar {
          display: flex; align-items: center; gap: 10px;
          height: 44px;
          width: 100%;
        }
        .ui-topbar-launcher-btn,
        .ui-topbar-icon-btn {
          display: grid; place-items: center;
          width: 32px; height: 32px;
          background: transparent; color: inherit;
          border: 0; border-radius: 8px;
          cursor: pointer;
          opacity: .75;
        }
        .ui-topbar-launcher-btn:hover,
        .ui-topbar-icon-btn:hover {
          background: var(--color-surface-2, #f5f5f7);
          opacity: 1;
        }
        .ui-topbar-search {
          display: flex; align-items: center; gap: 8px;
          flex: 1; max-width: 480px;
          padding: 6px 10px;
          background: var(--color-surface-2, #f5f5f7);
          border: 1px solid transparent;
          border-radius: 8px;
          color: inherit; cursor: text;
          font: inherit; font-size: 13px;
          text-align: left;
        }
        .ui-topbar-search:hover { border-color: var(--color-border, #e5e7eb); }
        .ui-topbar-search-placeholder { flex: 1; opacity: .55; }
        .ui-topbar-kbd {
          font-size: 10px; opacity: .5;
          padding: 2px 5px;
          border: 1px solid var(--color-border, #e5e7eb);
          border-radius: 4px;
        }
        .ui-topbar-spacer { flex: 1; }
        @media (max-width: 768px) {
          .ui-topbar-search-placeholder { display: none; }
          .ui-topbar-kbd { display: none; }
          .ui-topbar-search { max-width: none; }
        }
    </style>
    <style>
        /* ─── Desktop activity bar ─── */
        .ui-activity-bar {
          display: flex; flex-direction: column;
          align-items: stretch; gap: 2px;
          padding: 8px 6px;
          height: 100%;
        }
        .ui-activity-item {
          display: flex; flex-direction: column; align-items: center; gap: 2px;
          padding: 8px 4px;
          color: inherit; text-decoration: none;
          border-radius: 8px;
          opacity: .7;
          transition: opacity .12s, background .12s;
        }
        .ui-activity-item:hover { opacity: 1; background: var(--color-surface-2, #f5f5f7); }
        .ui-activity-item[aria-current="page"] {
          opacity: 1;
          background: var(--color-surface-2, #f5f5f7);
          box-shadow: inset 3px 0 0 var(--color-primary, #4f46e5);
        }
        .ui-activity-label {
          font-size: 10px; letter-spacing: .02em;
          text-align: center; line-height: 1.1;
        }

        /* ─── Mobile bottom tab bar ─── */
        .ui-tab-bar {
          display: none;
          justify-content: space-around; align-items: stretch;
          padding: 4px 0 max(4px, env(safe-area-inset-bottom));
          background: var(--color-bg, #fff);
          border-top: 1px solid var(--color-border, #e5e7eb);
        }
        .ui-tab-item {
          flex: 1;
          display: flex; flex-direction: column; align-items: center; gap: 2px;
          padding: 6px 4px;
          color: inherit; text-decoration: none;
          opacity: .55;
        }
        .ui-tab-item[aria-current="page"] {
          opacity: 1; color: var(--color-primary, #4f46e5);
        }
        .ui-tab-label { font-size: 10px; letter-spacing: .01em; }

        @media (max-width: 768px) {
          .ui-tab-bar { display: flex; }
        }
    </style>"#);

    page()
        .title("lit-ui framework")
        .assets_base("/lit-components")
        // The shell's grid layout is stable pre-upgrade — no need for
        // the FOUCE spinner, and turning it off saves ~50–200ms on first
        // paint (and eliminates the 1.5s safety-net ceiling).
        .no_fouce_gate()
        .add(nav_css)
        .add(shell)
        .add(launcher)
        .render()
}
