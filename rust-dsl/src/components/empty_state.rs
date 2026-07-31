//! `<ui-empty-state>` typed builder.

use crate::core::{wrap, Attr, Child, Component};

pub struct EmptyState {
    icon: String,
    title: String,
    description: Option<String>,
    compact: bool,
    actions: Vec<Child>,
    children: Vec<Child>,
}
pub fn empty_state(title: impl Into<String>) -> EmptyState {
    EmptyState { icon: "library".into(), title: title.into(), description: None,
                 compact: false, actions: Vec::new(), children: Vec::new() }
}
impl EmptyState {
    pub fn icon(mut self, name: impl Into<String>)        -> Self { self.icon = name.into(); self }
    pub fn description(mut self, d: impl Into<String>)    -> Self { self.description = Some(d.into()); self }
    pub fn compact(mut self)                              -> Self { self.compact = true; self }
    pub fn action(mut self, c: impl Component + 'static)  -> Self { self.actions.push(Box::new(c)); self }
    pub fn add(mut self, c: impl Component + 'static)     -> Self { self.children.push(Box::new(c)); self }
}
impl Component for EmptyState {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("icon",  self.icon.as_str()),
            Attr::kv("title", self.title.as_str()),
        ];
        if let Some(ref d) = self.description { attrs.push(Attr::kv("description", d.as_str())); }
        if self.compact { attrs.push(Attr::flag("compact")); }
        let mut body = String::new();
        for c in &self.children { body.push_str(&c.render()); }
        for a in &self.actions {
            body.push_str(r#"<span slot="actions">"#);
            body.push_str(&a.render());
            body.push_str("</span>");
        }
        wrap("ui-empty-state", &attrs, &body)
    }
}
