//! `<ui-dropdown-menu>` typed builder.

use crate::core::{escape_html, wrap, Attr, Child, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAlign { Start, End }
impl MenuAlign {
    fn as_str(self) -> &'static str {
        match self { MenuAlign::Start=>"start", MenuAlign::End=>"end" }
    }
}

pub struct MenuItem {
    pub label: String,
    pub href: Option<String>,
    pub danger: bool,
    pub disabled: bool,
}
impl MenuItem {
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self { label: label.into(), href: Some(href.into()), danger: false, disabled: false }
    }
    pub fn action(label: impl Into<String>) -> Self {
        Self { label: label.into(), href: None, danger: false, disabled: false }
    }
    pub fn danger(mut self)   -> Self { self.danger = true; self }
    pub fn disabled(mut self) -> Self { self.disabled = true; self }
}

/// Either a menu item or a `<hr>` divider.
pub enum MenuEntry { Item(MenuItem), Divider }
impl MenuEntry {
    pub fn item(i: MenuItem) -> Self { MenuEntry::Item(i) }
    pub fn divider() -> Self { MenuEntry::Divider }
}

pub struct DropdownMenu {
    align: MenuAlign,
    trigger: Option<Child>,
    entries: Vec<MenuEntry>,
}
pub fn dropdown_menu() -> DropdownMenu {
    DropdownMenu { align: MenuAlign::Start, trigger: None, entries: Vec::new() }
}
impl DropdownMenu {
    pub fn align(mut self, a: MenuAlign) -> Self { self.align = a; self }
    pub fn trigger(mut self, c: impl Component + 'static) -> Self {
        self.trigger = Some(Box::new(c)); self
    }
    pub fn item(mut self, i: MenuItem)   -> Self { self.entries.push(MenuEntry::Item(i)); self }
    pub fn divider(mut self)             -> Self { self.entries.push(MenuEntry::Divider); self }
}
impl Component for DropdownMenu {
    fn render(&self) -> String {
        let attrs = [Attr::kv("align", self.align.as_str())];
        let mut body = String::new();
        if let Some(ref t) = self.trigger {
            // The Lit component looks for a child whose slot="trigger",
            // so we wrap whatever the user passes with slot="trigger" on a
            // <div> — the Lit component then propagates clicks through it.
            body.push_str(r#"<div slot="trigger" style="display:contents;">"#);
            body.push_str(&t.render());
            body.push_str("</div>");
        }
        for e in &self.entries {
            match e {
                MenuEntry::Divider => body.push_str("<hr>"),
                MenuEntry::Item(i) => {
                    let tone = if i.danger { r#" data-tone="danger""# } else { "" };
                    let dis  = if i.disabled { " disabled" } else { "" };
                    match &i.href {
                        Some(h) => body.push_str(&format!(
                            r#"<a href="{}"{}{}>{}</a>"#,
                            escape_html(h), tone, dis, escape_html(&i.label))),
                        None => body.push_str(&format!(
                            r#"<button{}{}>{}</button>"#,
                            tone, dis, escape_html(&i.label))),
                    }
                }
            }
        }
        wrap("ui-dropdown-menu", &attrs, &body)
    }
}
