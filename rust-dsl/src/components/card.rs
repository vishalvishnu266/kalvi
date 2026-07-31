//! `<ui-card>` — a container with an optional header, an "actions" slot,
//! and a default body slot.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! let html = card()
//!     .title("Attendance")
//!     .subtitle("This week")
//!     .padded()
//!     .action(button().label("See all").variant(Variant::Ghost))
//!     .add(input().label("Search"))
//!     .render();
//! ```

use crate::core::{wrap, Attr, Child, Component};

/// `<ui-card>` builder.
pub struct Card {
    title: Option<String>,
    subtitle: Option<String>,
    padded: bool,
    flush: bool,
    actions: Vec<Child>,
    children: Vec<Child>,
}

/// Start building a new card.
pub fn card() -> Card {
    Card {
        title: None, subtitle: None,
        padded: false, flush: false,
        actions: Vec::new(), children: Vec::new(),
    }
}

impl Card {
    pub fn title(mut self, s: impl Into<String>)    -> Self { self.title    = Some(s.into()); self }
    pub fn subtitle(mut self, s: impl Into<String>) -> Self { self.subtitle = Some(s.into()); self }
    pub fn padded(mut self) -> Self { self.padded = true; self }
    pub fn flush(mut self)  -> Self { self.flush  = true; self }

    /// Add an element to the header's "actions" slot (top-right of the card).
    pub fn action(mut self, child: impl Component + 'static) -> Self {
        self.actions.push(Box::new(child)); self
    }

    /// Add one child to the card body.
    pub fn add(mut self, child: impl Component + 'static) -> Self {
        self.children.push(Box::new(child)); self
    }

    /// Add many children to the card body.
    pub fn children<I, C>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Component + 'static,
    {
        for c in iter { self.children.push(Box::new(c)); }
        self
    }
}

impl Component for Card {
    fn render(&self) -> String {
        let mut attrs: Vec<Attr> = Vec::new();
        if let Some(ref t) = self.title    { attrs.push(Attr::kv("title",    t.as_str())); }
        if let Some(ref s) = self.subtitle { attrs.push(Attr::kv("subtitle", s.as_str())); }
        if self.padded { attrs.push(Attr::flag("padded")); }
        if self.flush  { attrs.push(Attr::flag("flush")); }

        let mut body = String::new();

        // Slot each action into the header's `actions` named slot.
        // Slotted children go into the light DOM, so we wrap each one in a
        // <span slot="actions"> to preserve the slot attribution even if the
        // action is itself a web component that renders shadow DOM.
        for a in &self.actions {
            body.push_str(r#"<span slot="actions">"#);
            body.push_str(&a.render());
            body.push_str("</span>");
        }

        // Regular children go into the default slot.
        for c in &self.children {
            body.push_str(&c.render());
        }

        wrap("ui-card", &attrs, &body)
    }
}
