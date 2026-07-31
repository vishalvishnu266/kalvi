//! `<ui-list-item>` typed builder.

use crate::core::{wrap, Attr, Child, Component};

pub struct ListItem {
    title: String,
    subtitle: Option<String>,
    trailing_text: Option<String>,
    clickable: bool,
    leading: Vec<Child>,
    trailing: Vec<Child>,
}
pub fn list_item(title: impl Into<String>) -> ListItem {
    ListItem {
        title: title.into(), subtitle: None, trailing_text: None,
        clickable: false, leading: Vec::new(), trailing: Vec::new(),
    }
}
impl ListItem {
    pub fn subtitle(mut self, s: impl Into<String>) -> Self { self.subtitle = Some(s.into()); self }
    pub fn trailing_text(mut self, s: impl Into<String>) -> Self { self.trailing_text = Some(s.into()); self }
    pub fn clickable(mut self) -> Self { self.clickable = true; self }
    pub fn leading(mut self, c: impl Component + 'static)  -> Self { self.leading.push(Box::new(c)); self }
    pub fn trailing(mut self, c: impl Component + 'static) -> Self { self.trailing.push(Box::new(c)); self }
}
impl Component for ListItem {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("title", self.title.as_str())];
        if let Some(ref s) = self.subtitle       { attrs.push(Attr::kv("subtitle", s.as_str())); }
        if let Some(ref s) = self.trailing_text  { attrs.push(Attr::kv("trailing", s.as_str())); }
        if self.clickable { attrs.push(Attr::flag("clickable")); }

        let mut body = String::new();
        for c in &self.leading  { body.push_str(&format!(r#"<span slot="leading">{}</span>"#,  c.render())); }
        for c in &self.trailing { body.push_str(&format!(r#"<span slot="trailing">{}</span>"#, c.render())); }
        wrap("ui-list-item", &attrs, &body)
    }
}
