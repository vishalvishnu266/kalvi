//! `<ui-fragment>` — the wire-level envelope that tells the shell where
//! to place a chunk of HTML.
//!
//! The server usually builds `Fragment` values via the `ui-shell` crate
//! (which owns the HTTP integration), but exposing a DSL wrapper here
//! lets pages compose fragments *inline* alongside normal components —
//! useful for pages that want to emit multiple envelopes in a single
//! response body without leaving the DSL.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! let html = (
//!     fragment().target("main").action(FragmentAction::Replace)
//!         .add(card().title("Dashboard")),
//! ).render();
//! ```

use crate::core::{wrap, Attr, Child, Component};

/// The set of actions the client runtime knows how to apply.
///
/// Kept in lock-step with the `Action` enum in `framework/ui-shell/src/fragment.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FragmentAction {
    /// Replace the target island's children with the fragment body.
    Replace,
    /// Append the body as the last children of the target.
    Append,
    /// Prepend the body as the first children of the target.
    Prepend,
    /// Remove the target island's children. Body is ignored on the client.
    Remove,
    /// Set `.innerHTML` directly, preserving the target element.
    Update,
}

impl FragmentAction {
    /// The `action="…"` value emitted on the wire.
    pub fn as_str(self) -> &'static str {
        match self {
            FragmentAction::Replace => "replace",
            FragmentAction::Append  => "append",
            FragmentAction::Prepend => "prepend",
            FragmentAction::Remove  => "remove",
            FragmentAction::Update  => "update",
        }
    }
}

/// `<ui-fragment>` builder.
pub struct Fragment {
    target: String,
    action: FragmentAction,
    children: Vec<Child>,
}

/// Start building a fragment. Default target is `main`, default action is `replace`.
pub fn fragment() -> Fragment {
    Fragment {
        target: "main".into(),
        action: FragmentAction::Replace,
        children: Vec::new(),
    }
}

impl Fragment {
    /// Set the target island (see `Region::as_str()` for well-known names).
    pub fn target(mut self, t: impl Into<String>) -> Self { self.target = t.into(); self }

    /// Set the swap action.
    pub fn action(mut self, a: FragmentAction) -> Self { self.action = a; self }

    /// Add one child to the fragment body.
    pub fn add(mut self, child: impl Component + 'static) -> Self {
        self.children.push(Box::new(child)); self
    }

    /// Add many children.
    pub fn children<I, C>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Component + 'static,
    {
        for c in iter { self.children.push(Box::new(c)); }
        self
    }
}

impl Component for Fragment {
    fn render(&self) -> String {
        let attrs = [
            Attr::kv("target", self.target.as_str()),
            Attr::kv("action", self.action.as_str()),
        ];
        let mut body = String::new();
        if !matches!(self.action, FragmentAction::Remove) {
            for c in &self.children { body.push_str(&c.render()); }
        }
        wrap("ui-fragment", &attrs, &body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Node;

    #[test]
    fn defaults_to_main_replace() {
        let html = fragment().add(Node::raw("<p>x</p>")).render();
        assert!(html.contains(r#"target="main""#));
        assert!(html.contains(r#"action="replace""#));
        assert!(html.contains("<p>x</p>"));
    }

    #[test]
    fn remove_omits_body() {
        let html = fragment().action(FragmentAction::Remove).add(Node::raw("ignored")).render();
        assert!(!html.contains("ignored"));
    }
}
