//! `<ui-inline-edit>` typed builder.

use crate::core::{wrap, Attr, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineKind { Text, Number, Textarea }
impl InlineKind {
    fn as_str(self) -> &'static str {
        match self { InlineKind::Text=>"text", InlineKind::Number=>"number", InlineKind::Textarea=>"textarea" }
    }
}

pub struct InlineEdit {
    value: String,
    name: Option<String>,
    placeholder: Option<String>,
    kind: InlineKind,
    required: bool,
    readonly: bool,
}
pub fn inline_edit(value: impl Into<String>) -> InlineEdit {
    InlineEdit { value: value.into(), name: None, placeholder: None,
                 kind: InlineKind::Text, required: false, readonly: false }
}
impl InlineEdit {
    pub fn name(mut self, s: impl Into<String>)        -> Self { self.name = Some(s.into()); self }
    pub fn placeholder(mut self, s: impl Into<String>) -> Self { self.placeholder = Some(s.into()); self }
    pub fn kind(mut self, k: InlineKind)               -> Self { self.kind = k; self }
    pub fn required(mut self) -> Self { self.required = true; self }
    pub fn readonly(mut self) -> Self { self.readonly = true; self }
}
impl Component for InlineEdit {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("value", self.value.as_str()),
            Attr::kv("type",  self.kind.as_str()),
        ];
        if let Some(ref n) = self.name        { attrs.push(Attr::kv("name",        n.as_str())); }
        if let Some(ref p) = self.placeholder { attrs.push(Attr::kv("placeholder", p.as_str())); }
        if self.required { attrs.push(Attr::flag("required")); }
        if self.readonly { attrs.push(Attr::flag("readonly")); }
        wrap("ui-inline-edit", &attrs, "")
    }
}
