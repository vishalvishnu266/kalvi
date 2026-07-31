//! `<ui-breadcrumb>` typed builder.

use crate::core::{escape_html, wrap, Attr, Component};

pub struct Crumb { pub label: String, pub href: Option<String> }
impl Crumb {
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Crumb { label: label.into(), href: Some(href.into()) }
    }
    pub fn current(label: impl Into<String>) -> Self {
        Crumb { label: label.into(), href: None }
    }
}

pub struct Breadcrumb { separator: String, collapse: bool, items: Vec<Crumb> }
pub fn breadcrumb() -> Breadcrumb {
    Breadcrumb { separator: "/".into(), collapse: false, items: Vec::new() }
}
impl Breadcrumb {
    pub fn separator(mut self, s: impl Into<String>) -> Self { self.separator = s.into(); self }
    pub fn collapse(mut self) -> Self { self.collapse = true; self }
    pub fn item(mut self, c: Crumb) -> Self { self.items.push(c); self }
    pub fn items<I: IntoIterator<Item = Crumb>>(mut self, iter: I) -> Self {
        self.items.extend(iter); self
    }
}
impl Component for Breadcrumb {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("separator", self.separator.as_str())];
        if self.collapse { attrs.push(Attr::flag("collapse")); }
        let body: String = self.items.iter().map(|c| match &c.href {
            Some(h) => format!(r#"<a href="{}">{}</a>"#, escape_html(h), escape_html(&c.label)),
            None    => format!(r#"<span>{}</span>"#, escape_html(&c.label)),
        }).collect();
        wrap("ui-breadcrumb", &attrs, &body)
    }
}
