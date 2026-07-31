//! `<ui-progress>` typed builder (linear + circular).

use crate::core::{wrap, Attr, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgShape { Linear, Circle }
impl ProgShape {
    fn as_str(self) -> &'static str {
        match self { ProgShape::Linear => "linear", ProgShape::Circle => "circle" }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgTone { Brand, Success, Warning, Danger, Info }
impl ProgTone {
    fn as_str(self) -> &'static str {
        match self {
            ProgTone::Brand => "brand", ProgTone::Success => "success",
            ProgTone::Warning => "warning", ProgTone::Danger => "danger",
            ProgTone::Info => "info",
        }
    }
}

pub struct Progress {
    value: u32, max: u32, shape: ProgShape, size: u32, tone: ProgTone,
    label: Option<String>, show_value: bool, indeterminate: bool,
}
pub fn progress(value: u32) -> Progress {
    Progress { value, max: 100, shape: ProgShape::Linear, size: 56,
               tone: ProgTone::Brand, label: None, show_value: false, indeterminate: false }
}
impl Progress {
    pub fn max(mut self, n: u32)              -> Self { self.max = n; self }
    pub fn shape(mut self, s: ProgShape)      -> Self { self.shape = s; self }
    pub fn size(mut self, px: u32)            -> Self { self.size = px; self }
    pub fn tone(mut self, t: ProgTone)        -> Self { self.tone = t; self }
    pub fn label(mut self, s: impl Into<String>) -> Self { self.label = Some(s.into()); self }
    pub fn show_value(mut self)               -> Self { self.show_value = true; self }
    pub fn indeterminate(mut self)            -> Self { self.indeterminate = true; self }
}
impl Component for Progress {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("value", self.value.to_string()),
            Attr::kv("max",   self.max.to_string()),
            Attr::kv("shape", self.shape.as_str()),
            Attr::kv("size",  self.size.to_string()),
            Attr::kv("tone",  self.tone.as_str()),
        ];
        if let Some(ref l) = self.label { attrs.push(Attr::kv("label", l.as_str())); }
        if self.show_value    { attrs.push(Attr::flag("show-value")); }
        if self.indeterminate { attrs.push(Attr::flag("indeterminate")); }
        wrap("ui-progress", &attrs, "")
    }
}
