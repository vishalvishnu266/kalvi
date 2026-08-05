//! `<ui-app-shell>` — the layout element that hosts every page in the
//! backend-driven web framework built on top of this DSL.
//!
//! The shell exposes six named slots (the "islands"):
//!
//! | slot name  | typical content                                  |
//! |------------|--------------------------------------------------|
//! | `topbar`   | breadcrumbs, actions, theme toggle               |
//! | `sidebar`  | primary nav                                      |
//! | `main`     | the current page — swapped on every navigation   |
//! | `copilot`  | `<ui-copilot>` chat/agent pane                   |
//! | `toast`    | transient toasts                                 |
//! | `modal`    | modal dialogs                                    |
//!
//! You place children into a slot with `.slot(Region::Main, some_component)`
//! and the DSL wraps them in `<div slot="…">` so the shell picks them up.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! app_shell()
//!     .slot(Region::Topbar, topbar_component())
//!     .slot(Region::Sidebar, sidebar_component())
//!     .slot(Region::Main, dashboard_page())
//!     .slot(Region::Copilot, copilot_component())
//!     .render();
//! ```
//!
//! ## Why this lives in `rust-dsl`
//!
//! Both the initial full-page render *and* fragment responses need to
//! reference the exact same slot names. Putting the wrapper in the DSL
//! guarantees a single source of truth: the [`Region`] enum's string
//! forms are the wire-level `slot` names and match `TARGET_*` on the
//! server side.

use crate::core::{wrap, Attr, Child, Component};

/// The six built-in islands, plus an escape hatch for app-defined ones.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Region {
    /// The primary content area. Default target for link navigation.
    Main,
    /// The copilot pane (right side desktop, bottom sheet mobile).
    Copilot,
    /// Persistent topbar.
    Topbar,
    /// Persistent sidebar.
    Sidebar,
    /// Toast host.
    Toast,
    /// Modal host.
    Modal,
    /// Any custom slot the app has added to its shell.
    Custom(String),
}

impl Region {
    /// The `slot="…"` value that gets emitted on the child wrapper.
    /// Kept in sync with the `TARGET_*` constants in the `ui-shell` crate.
    pub fn as_str(&self) -> &str {
        match self {
            Region::Main    => "main",
            Region::Copilot => "copilot",
            Region::Topbar  => "topbar",
            Region::Sidebar => "sidebar",
            Region::Toast   => "toast",
            Region::Modal   => "modal",
            Region::Custom(s) => s.as_str(),
        }
    }
}

/// `<ui-app-shell>` builder.
///
/// Children are stored per-region in insertion order; `render()` emits
/// them as `<div slot="…">…</div>` wrappers so the shell's slots pick
/// them up correctly.
pub struct AppShell {
    slots: Vec<(Region, Child)>,
}

/// Start building an app shell. Chain `.slot(region, child)` to fill it.
pub fn app_shell() -> AppShell { AppShell { slots: Vec::new() } }

impl AppShell {
    /// Place one child into a named region. Multiple calls to the same
    /// region append in order.
    pub fn slot(mut self, region: Region, child: impl Component + 'static) -> Self {
        self.slots.push((region, Box::new(child)));
        self
    }

    /// Convenience — set the `main` island in one call.
    pub fn main(self, child: impl Component + 'static) -> Self {
        self.slot(Region::Main, child)
    }

    /// Convenience — set the `topbar` island in one call.
    pub fn topbar(self, child: impl Component + 'static) -> Self {
        self.slot(Region::Topbar, child)
    }

    /// Convenience — set the `sidebar` island in one call.
    pub fn sidebar(self, child: impl Component + 'static) -> Self {
        self.slot(Region::Sidebar, child)
    }

    /// Convenience — set the `copilot` island in one call.
    pub fn copilot(self, child: impl Component + 'static) -> Self {
        self.slot(Region::Copilot, child)
    }
}

impl Component for AppShell {
    fn render(&self) -> String {
        // No attributes on the shell itself yet — layout/theme is driven
        // purely by CSS custom properties inherited from :root.
        let attrs: [Attr; 0] = [];

        let mut body = String::new();
        for (region, child) in &self.slots {
            // A light-DOM wrapper carries the `slot=` attribute so the
            // child component can be a web component with its own shadow
            // root — the slot attribution is preserved either way.
            body.push_str(r#"<div slot=""#);
            body.push_str(region.as_str());
            body.push_str(r#"">"#);
            body.push_str(&child.render());
            body.push_str("</div>");
        }

        wrap("ui-app-shell", &attrs, &body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Node;

    #[test]
    fn emits_slotted_children() {
        let html = app_shell()
            .main(Node::raw("<p>hi</p>"))
            .topbar(Node::raw("<b>x</b>"))
            .render();
        assert!(html.starts_with("<ui-app-shell"));
        assert!(html.contains(r#"<div slot="main"><p>hi</p></div>"#));
        assert!(html.contains(r#"<div slot="topbar"><b>x</b></div>"#));
    }

    #[test]
    fn custom_region_uses_its_name() {
        let html = app_shell()
            .slot(Region::Custom("breadcrumbs".into()), Node::raw("x"))
            .render();
        assert!(html.contains(r#"slot="breadcrumbs""#));
    }
}
