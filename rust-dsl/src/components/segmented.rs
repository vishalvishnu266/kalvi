//! `<ui-segmented>` typed builder (macOS-style segmented control).

use crate::core::{escape_html, wrap, Attr, Component};

pub struct Segment { pub value: String, pub label: String }
impl Segment {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Segment { value: value.into(), label: label.into() }
    }
}

pub struct Segmented { value: Option<String>, segments: Vec<Segment> }
pub fn segmented() -> Segmented { Segmented { value: None, segments: Vec::new() } }
impl Segmented {
    pub fn value(mut self, v: impl Into<String>) -> Self { self.value = Some(v.into()); self }
    pub fn segment(mut self, s: Segment)         -> Self { self.segments.push(s); self }
    pub fn segments<I: IntoIterator<Item = Segment>>(mut self, iter: I) -> Self {
        self.segments.extend(iter); self
    }
}
impl Component for Segmented {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.value { attrs.push(Attr::kv("value", v.as_str())); }
        let body: String = self.segments.iter()
            .map(|s| format!(r#"<button value="{}">{}</button>"#,
                escape_html(&s.value), escape_html(&s.label)))
            .collect();
        wrap("ui-segmented", &attrs, &body)
    }
}
