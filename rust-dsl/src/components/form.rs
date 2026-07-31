//! `<ui-form>` typed builder.

use crate::core::{wrap, Attr, Child, Component};

pub struct Form {
    action: Option<String>,
    method: String,      // "get" | "post"
    novalidate: bool,
    inline: bool,
    fields: Vec<Child>,
    actions: Vec<Child>,
}
pub fn form() -> Form {
    Form {
        action: None, method: "post".into(),
        novalidate: false, inline: false,
        fields: Vec::new(), actions: Vec::new(),
    }
}
impl Form {
    pub fn action(mut self, s: impl Into<String>) -> Self { self.action = Some(s.into()); self }
    pub fn method(mut self, s: impl Into<String>) -> Self { self.method = s.into(); self }
    pub fn novalidate(mut self) -> Self { self.novalidate = true; self }
    pub fn inline(mut self)     -> Self { self.inline     = true; self }

    /// Add a field (input, select, checkbox, …) to the form body.
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.fields.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.fields.push(Box::new(c)); } self
    }
    /// Add a button (or anything) into the form's "actions" slot.
    pub fn action_btn(mut self, c: impl Component + 'static) -> Self {
        self.actions.push(Box::new(c)); self
    }
}
impl Component for Form {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("method", self.method.as_str())];
        if let Some(ref a) = self.action { attrs.push(Attr::kv("action", a.as_str())); }
        if self.novalidate { attrs.push(Attr::flag("novalidate")); }
        if self.inline     { attrs.push(Attr::flag("inline")); }

        let mut body: String = self.fields.iter().map(|c| c.render()).collect();
        for c in &self.actions {
            body.push_str(r#"<span slot="actions">"#);
            body.push_str(&c.render());
            body.push_str("</span>");
        }
        wrap("ui-form", &attrs, &body)
    }
}
