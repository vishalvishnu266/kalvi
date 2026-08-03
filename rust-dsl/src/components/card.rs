//! `<ui-card>` — a container with an optional header, an "actions" slot,
//! and a default body slot.
//!
//! ## Padding is on by default
//!
//! Every real usage in the codebase wanted a padded card, so `card()` is
//! **padded by default**. Only opt out when you need edge-to-edge content
//! (a full-bleed image, an embedded table/kanban with its own chrome):
//!
//! ```ignore
//! card().add(input().label("Name"))                 // padded — normal case
//! card().flush().add(table)                         // flush  — table borders reach the edge
//! ```
//!
//! `.padded()` is kept as a deprecated no-op for backwards compatibility —
//! existing call sites keep compiling. Sweep them at your leisure.

use crate::core::{wrap, Attr, Child, Component};

/// `<ui-card>` builder.
pub struct Card {
    title: Option<String>,
    subtitle: Option<String>,
    /// Whether to render internal body padding. Defaults to `true`.
    padded: bool,
    /// `flush` is the explicit opposite of `padded` on the underlying web
    /// component. We keep the attribute so the CSS can still target it
    /// (some Lit implementations use `flush` as the "on" signal), but the
    /// Rust builder now drives it purely via `padded`.
    flush: bool,
    /// Drop the card's default `overflow: hidden` so a `<ui-form sticky>`
    /// child can pin its action bar against the viewport instead of being
    /// trapped inside the card. Required whenever you host a form with
    /// `sticky_bottom()` / `sticky_top()` inside a card.
    sticky_friendly: bool,
    actions: Vec<Child>,
    children: Vec<Child>,
}

/// Start building a new card. **Padded by default** — call [`Card::flush`]
/// to remove the body padding for edge-to-edge content.
pub fn card() -> Card {
    Card {
        title: None, subtitle: None,
        padded: true, flush: false,       // ← default flipped: padded on
        sticky_friendly: false,
        actions: Vec::new(), children: Vec::new(),
    }
}

impl Card {
    pub fn title(mut self, s: impl Into<String>)    -> Self { self.title    = Some(s.into()); self }
    pub fn subtitle(mut self, s: impl Into<String>) -> Self { self.subtitle = Some(s.into()); self }

    /// **Deprecated.** Cards are padded by default now — this is a no-op
    /// kept only so existing call sites don't need to be edited in one
    /// go. New code should just write `card()`.
    #[deprecated(note = "cards are padded by default; remove the `.padded()` call")]
    pub fn padded(self) -> Self { self }

    /// Remove the body padding — use for full-bleed images, embedded
    /// tables, kanbans, or anything else that must touch the card edge.
    /// Also clears the `padded` flag so both attributes are consistent.
    pub fn flush(mut self) -> Self {
        self.padded = false;
        self.flush  = true;
        self
    }

    /// Drop the card's default `overflow: hidden` clipping so a
    /// `<ui-form>` inside it can use `sticky_bottom()` / `sticky_top()`
    /// and pin its action bar against the viewport rather than being
    /// trapped inside the card.
    ///
    /// ```ignore
    /// card().title("Edit student").sticky_friendly()
    ///     .add(form()
    ///         .sticky_bottom()
    ///         .csrf(&token)
    ///         .add(input().label("Name"))
    ///         .save_cancel("Save"))
    /// ```
    pub fn sticky_friendly(mut self) -> Self {
        self.sticky_friendly = true; self
    }

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
        if self.sticky_friendly { attrs.push(Attr::flag("sticky-friendly")); }

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
