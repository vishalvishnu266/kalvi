//! `<ui-button>` — Vaadin-style typed builder.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! let html = button()
//!     .label("Save")
//!     .variant(Variant::Primary)
//!     .size(Size::Md)
//!     .icon("check")
//!     .render();
//! ```

use crate::core::{escape_html, wrap, Attr, Child, Component};

/// Visual variant of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Primary,
    Secondary,
    Ghost,
    Danger,
}
impl Variant {
    fn as_str(self) -> &'static str {
        match self {
            Variant::Primary   => "primary",
            Variant::Secondary => "secondary",
            Variant::Ghost     => "ghost",
            Variant::Danger    => "danger",
        }
    }
}

/// Button size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size { Sm, Md, Lg }
impl Size {
    fn as_str(self) -> &'static str {
        match self { Size::Sm => "sm", Size::Md => "md", Size::Lg => "lg" }
    }
}

/// `<ui-button>` builder.
///
/// Prefer the free function [`button()`] over `Button::default()` — it reads
/// better in a chain: `button().label("Save").variant(Variant::Primary)`.
pub struct Button {
    label: String,
    variant: Variant,
    size: Size,
    icon: Option<String>,
    full: bool,
    disabled: bool,
    children: Vec<Child>,
}

/// Start building a new button. See [`Button`] for the full method surface.
pub fn button() -> Button {
    Button {
        label: String::new(),
        variant: Variant::Primary,
        size: Size::Md,
        icon: None,
        full: false,
        disabled: false,
        children: Vec::new(),
    }
}

impl Button {
    /// Set the button's text (slotted as the button's default child).
    ///
    /// Prefer this over `.add(...)` for simple labels; use `.add()` when you
    /// need mixed inline content (icons, badges, formatting).
    pub fn label(mut self, s: impl Into<String>) -> Self {
        self.label = s.into();
        self
    }

    pub fn variant(mut self, v: Variant) -> Self { self.variant = v; self }
    pub fn size(mut self, s: Size)       -> Self { self.size = s; self }
    pub fn icon(mut self, name: impl Into<String>) -> Self {
        self.icon = Some(name.into()); self
    }
    pub fn full(mut self)     -> Self { self.full = true; self }
    pub fn disabled(mut self) -> Self { self.disabled = true; self }

    /// Add a single child (icon, badge, span, another component…).
    pub fn add(mut self, child: impl Component + 'static) -> Self {
        self.children.push(Box::new(child)); self
    }

    /// Add many children in one call.
    pub fn children<I, C>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Component + 'static,
    {
        for c in iter { self.children.push(Box::new(c)); }
        self
    }
}

impl Component for Button {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("variant", self.variant.as_str()),
            Attr::kv("size",    self.size.as_str()),
        ];
        if let Some(ref i) = self.icon { attrs.push(Attr::kv("icon", i.as_str())); }
        if self.full     { attrs.push(Attr::flag("full")); }
        if self.disabled { attrs.push(Attr::flag("disabled")); }

        let mut body = String::new();
        if !self.label.is_empty() { body.push_str(&escape_html(&self.label)); }
        for c in &self.children { body.push_str(&c.render()); }

        wrap("ui-button", &attrs, &body)
    }
}
