//! `<ui-daterange>` typed builder.

use crate::core::{wrap, Attr, Component};

pub struct DateRange {
    label: Option<String>,
    from: Option<String>,   // ISO YYYY-MM-DD
    to:   Option<String>,
}
pub fn date_range() -> DateRange { DateRange { label: None, from: None, to: None } }
impl DateRange {
    pub fn label(mut self, s: impl Into<String>)  -> Self { self.label = Some(s.into()); self }
    pub fn from(mut self, iso: impl Into<String>) -> Self { self.from  = Some(iso.into()); self }
    pub fn to(mut self, iso: impl Into<String>)   -> Self { self.to    = Some(iso.into()); self }
}
impl Component for DateRange {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.label { attrs.push(Attr::kv("label", v.as_str())); }
        if let Some(ref v) = self.from  { attrs.push(Attr::kv("from",  v.as_str())); }
        if let Some(ref v) = self.to    { attrs.push(Attr::kv("to",    v.as_str())); }
        wrap("ui-daterange", &attrs, "")
    }
}
