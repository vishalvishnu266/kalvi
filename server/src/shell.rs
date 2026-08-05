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

/// Topbar — brand on the left, primary bar (Apps / Search / AI /
/// Profile) on the right. Everything else that used to live here (the
/// wide search field, the standalone launcher trigger) has been
/// consolidated into the primary bar. See `<ui-primary-bar>`.
fn topbar() -> impl Component {
    Node::raw(
        r#"<div class="ui-topbar">
             <strong class="ui-topbar-brand">🧩 lit-ui</strong>
             <div class="ui-topbar-spacer"></div>
             <!-- The primary bar is a mobile-only fixed-bottom bar on
                  narrow viewports; on desktop the same element renders
                  inline here at the top-right. Same element, one CSS
                  media query flips its placement. -->
             <ui-primary-bar data-slot="topbar"></ui-primary-bar>
           </div>"#,
    )
}

/// The copilot pane. Now a floating popover (desktop) / bottom sheet
/// (mobile) — hidden by default, opened by the AI icon in the primary
/// bar. Placed once at the shell level so its state survives page
/// swaps.
fn copilot_pane() -> impl Component {
    Node::raw(r#"<ui-copilot endpoint="/agent"></ui-copilot>"#)
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

    // The shell no longer owns navigation — that moves to the primary
    // bar (top-right desktop, bottom mobile) and the /apps launcher
    // page. The sidebar slot is empty by default; individual pages can
    // fill it with page-specific power tools later.
    // The bottombar slot mounts a MOBILE-ONLY <ui-primary-bar>. Same
    // element as the topbar's; CSS decides which one is visible.
    let _ = current_path; // no longer used here — active state lives in /apps
    let shell = app_shell()
        .topbar(topbar())
        .main(main_slot)
        .copilot(copilot_pane())
        .slot(Region::Custom("bottombar".into()), Node::raw(
            r#"<ui-primary-bar data-slot="bottom"></ui-primary-bar>"#,
        ));

    // Mount the launcher once. It's a full-viewport fixed overlay
    // (hidden by default), so where we mount it doesn't matter — we
    // just need it in the DOM. Stash it in the modal region so it's
    // grouped with the other overlay-like elements.
    let launcher = Node::raw(r#"<ui-launcher></ui-launcher>"#);

    // Topbar shell styles. The primary bar owns its own styling
    // (see ui-primary-bar.js) so this stays minimal.
    let nav_css = Node::raw(r#"<style>
        .ui-topbar {
          display: flex; align-items: center; gap: 10px;
          height: 44px;
          width: 100%;
        }
        .ui-topbar-brand {
          font-size: 14px; font-weight: 600;
          letter-spacing: -0.01em;
          opacity: .85;
        }
        .ui-topbar-spacer { flex: 1; }
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
