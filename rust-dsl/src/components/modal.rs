//! `<ui-modal>` typed builder.

use crate::core::{wrap, Attr, Child, Component};

pub struct Modal {
    id: Option<String>,
    title: Option<String>,
    open: bool,
    children: Vec<Child>,
    footer: Vec<Child>,
}
pub fn modal() -> Modal {
    Modal { id: None, title: None, open: false, children: Vec::new(), footer: Vec::new() }
}
impl Modal {
    pub fn id(mut self, s: impl Into<String>)      -> Self { self.id = Some(s.into()); self }
    pub fn title(mut self, s: impl Into<String>)   -> Self { self.title = Some(s.into()); self }
    pub fn open(mut self)                          -> Self { self.open = true; self }
    pub fn add(mut self, c: impl Component + 'static)    -> Self { self.children.push(Box::new(c)); self }
    pub fn footer(mut self, c: impl Component + 'static) -> Self { self.footer.push(Box::new(c)); self }
}
impl Component for Modal {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref i) = self.id    { attrs.push(Attr::kv("id",    i.as_str())); }
        if let Some(ref t) = self.title { attrs.push(Attr::kv("title", t.as_str())); }
        if self.open { attrs.push(Attr::flag("open")); }
        let mut body = String::new();
        for c in &self.children { body.push_str(&c.render()); }
        for f in &self.footer {
            body.push_str(r#"<span slot="footer">"#);
            body.push_str(&f.render());
            body.push_str("</span>");
        }
        wrap("ui-modal", &attrs, &body)
    }
}
