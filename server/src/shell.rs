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

/// Static topbar. Content is completely server-authored — swapping
/// via a `topbar` fragment lets pages change breadcrumbs / actions
/// without a full reload.
///
/// Includes a small transition picker on the right so you can try
/// different island-swap animations without opening DevTools. The
/// picker calls `window.ui.setTransition(...)` which is exposed by
/// `ui-app-shell.js`; the choice is persisted in `localStorage`.
fn topbar() -> impl Component {
    // Inline picker: preset + speed slider. Both call into the client
    // runtime (`window.ui.setTransition` / `window.ui.slow`) so a change
    // takes effect on the very next navigation with no reload.
    let transition_picker = Node::raw(
        r#"<div style="display:inline-flex;align-items:center;gap:14px;font-size:12px;opacity:.9;">
             <label style="display:inline-flex;align-items:center;gap:6px;">
               transition
               <select id="ui-transition-select"
                       onchange="window.ui.setTransition(this.value)"
                       style="font-size:12px;padding:4px 6px;border:1px solid var(--color-border,#e5e7eb);
                              border-radius:6px;background:transparent;color:inherit;">
                 <option value="fade">fade (default)</option>
                 <option value="slide-left">slide-left</option>
                 <option value="slide-right">slide-right</option>
                 <option value="slide-up">slide-up</option>
                 <option value="scale">scale</option>
                 <option value="none">none (instant)</option>
               </select>
             </label>
             <label style="display:inline-flex;align-items:center;gap:6px;">
               speed
               <input id="ui-transition-speed" type="range"
                      min="0" max="2000" step="100" value="220"
                      oninput="window.ui.slow(+this.value); document.getElementById('ui-transition-speed-val').textContent = this.value + 'ms'"
                      style="width:120px;">
               <span id="ui-transition-speed-val" style="opacity:.7;min-width:52px;">220ms</span>
             </label>
           </div>
           <script>
             // Sync both controls to whatever is persisted (or defaults).
             (function () {
               try {
                 var sel = document.getElementById('ui-transition-select');
                 var cur = document.documentElement.dataset.uiTransition || 'fade';
                 if (sel) sel.value = cur;

                 var range = document.getElementById('ui-transition-speed');
                 var lab   = document.getElementById('ui-transition-speed-val');
                 var saved = parseInt(localStorage.getItem('erp.transition.speed') || '220', 10);
                 if (range) range.value = String(saved);
                 if (lab)   lab.textContent = saved + 'ms';
               } catch (_) {}
             })();
           </script>"#,
    );

    row()
        .align(Align::Center)
        .justify(Justify::Between)
        .gap(Gap::Md)
        .add(Node::raw(
            r#"<strong style="font-size:16px;">🧩 lit-ui framework</strong>"#,
        ))
        .add(transition_picker)
}

/// Sidebar — the framework will intercept every `<a>` click and swap
/// only the `main` island. Deep-loads (typing /admin in the URL bar)
/// hit the full-page path and see the same chrome.
fn sidebar() -> impl Component {
    let link = |href: &str, label: &str| {
        Node::raw(format!(
            r#"<a href="{href}" style="display:block;padding:10px 14px;color:inherit;text-decoration:none;border-radius:8px;">{label}</a>"#,
            href  = href,
            label = label,
        ))
    };
    column()
        .gap(Gap::Xs)
        .add(Node::raw(r#"<div style="padding:12px 14px;font-size:11px;opacity:.6;text-transform:uppercase;letter-spacing:.08em;">Navigate</div>"#))
        .add(link("/",          "Home"))
        .add(link("/dashboard", "Dashboard"))
        .add(link("/admin",     "Admin"))
        .add(link("/users",     "Users"))
}

/// Placeholder for the copilot pane. Filled in milestone 8.
fn copilot_placeholder() -> impl Component {
    Node::raw(
        r#"<div style="padding:16px;font-size:12px;opacity:.55;">
             <div style="font-weight:600;margin-bottom:6px;">Copilot</div>
             <div>Coming in milestone 8: a lean SSE-driven agent pane that talks to the same fragment protocol.</div>
           </div>"#,
    )
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
pub fn chrome(fragments_html: &str) -> String {
    // Wrap the pre-rendered fragment string in a raw node so it's not
    // re-escaped. The `Node::raw` contract is "this is trusted HTML".
    let main_slot = Node::raw(fragments_html.to_string());

    let shell = app_shell()
        .topbar(topbar())
        .sidebar(sidebar())
        .main(main_slot)
        .copilot(copilot_placeholder());

    page()
        .title("lit-ui framework")
        .assets_base("/lit-components")
        // The shell's grid layout is stable pre-upgrade — no need for
        // the FOUCE spinner, and turning it off saves ~50–200ms on first
        // paint (and eliminates the 1.5s safety-net ceiling).
        .no_fouce_gate()
        .add(shell)
        .render()
}
