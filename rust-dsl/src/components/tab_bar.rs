//! `<ui-tab-bar>` typed builder.

use crate::core::{escape_html, wrap, Attr, Component};

pub struct Tab { pub label: String, pub href: String, pub active: bool }
impl Tab {
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Tab { label: label.into(), href: href.into(), active: false }
    }
    pub fn active(mut self) -> Self { self.active = true; self }
}

pub struct TabBar { tabs: Vec<Tab> }
pub fn tab_bar() -> TabBar { TabBar { tabs: Vec::new() } }
impl TabBar {
    pub fn tab(mut self, t: Tab) -> Self { self.tabs.push(t); self }
    pub fn tabs<I: IntoIterator<Item = Tab>>(mut self, iter: I) -> Self {
        self.tabs.extend(iter); self
    }
}
impl Component for TabBar {
    fn render(&self) -> String {
        let body: String = self.tabs.iter().map(|t| {
            let act = if t.active { " data-active" } else { "" };
            format!(r#"<a href="{}"{}>{}</a>"#, escape_html(&t.href), act, escape_html(&t.label))
        }).collect();
        wrap("ui-tab-bar", &[] as &[Attr], &body)
    }
}
