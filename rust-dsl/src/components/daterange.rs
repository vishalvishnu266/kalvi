//! `<ui-daterange>` typed builder.

use crate::core::{wrap, Attr, Component};

pub struct DateRange {
    label: Option<String>,
    from: Option<String>,   // ISO YYYY-MM-DD
    to:   Option<String>,
    hint: Option<String>,
    invalid: bool,
}
pub fn date_range() -> DateRange {
    DateRange { label: None, from: None, to: None, hint: None, invalid: false }
}
impl DateRange {
    pub fn label(mut self, s: impl Into<String>)  -> Self { self.label = Some(s.into()); self }
    pub fn from(mut self, iso: impl Into<String>) -> Self { self.from  = Some(iso.into()); self }
    pub fn to(mut self, iso: impl Into<String>)   -> Self { self.to    = Some(iso.into()); self }
    pub fn hint(mut self, s: impl Into<String>)   -> Self { self.hint  = Some(s.into()); self }
    pub fn invalid(mut self)                      -> Self { self.invalid = true; self }

    /// Field-level error — sets `invalid` and replaces the hint with the
    /// error message so it renders red under the field.
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true; self.hint = Some(msg.into()); self
    }
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}
impl Component for DateRange {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.label { attrs.push(Attr::kv("label", v.as_str())); }
        if let Some(ref v) = self.from  { attrs.push(Attr::kv("from",  v.as_str())); }
        if let Some(ref v) = self.to    { attrs.push(Attr::kv("to",    v.as_str())); }
        if let Some(ref v) = self.hint  { attrs.push(Attr::kv("hint",  v.as_str())); }
        if self.invalid { attrs.push(Attr::flag("invalid")); }
        wrap("ui-daterange", &attrs, "")
    }
}
