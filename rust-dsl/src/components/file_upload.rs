//! `<ui-file-upload>` typed builder.

use crate::core::{wrap, Attr, Component};

pub struct FileUpload {
    label: Option<String>,
    accept: Option<String>,
    multiple: bool,
    max_size: Option<u64>,
    name: Option<String>,
    hint: Option<String>,
    invalid: bool,
}
pub fn file_upload() -> FileUpload {
    FileUpload { label: None, accept: None, multiple: false, max_size: None,
                 name: None, hint: None, invalid: false }
}
impl FileUpload {
    pub fn label(mut self, s: impl Into<String>)    -> Self { self.label  = Some(s.into()); self }
    pub fn accept(mut self, s: impl Into<String>)   -> Self { self.accept = Some(s.into()); self }
    pub fn multiple(mut self)                       -> Self { self.multiple = true; self }
    pub fn max_size(mut self, bytes: u64)           -> Self { self.max_size = Some(bytes); self }
    pub fn name(mut self, s: impl Into<String>)     -> Self { self.name = Some(s.into()); self }
    pub fn hint(mut self, s: impl Into<String>)     -> Self { self.hint = Some(s.into()); self }
    pub fn invalid(mut self)                        -> Self { self.invalid = true; self }

    /// Field-level error — sets `invalid` and replaces the hint with the
    /// error message so it renders red under the drop-zone.
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.invalid = true; self.hint = Some(msg.into()); self
    }
    pub fn maybe_error(self, msg: Option<impl Into<String>>) -> Self {
        match msg { Some(m) => self.error(m), None => self }
    }
}
impl Component for FileUpload {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.label  { attrs.push(Attr::kv("label",  v.as_str())); }
        if let Some(ref v) = self.accept { attrs.push(Attr::kv("accept", v.as_str())); }
        if let Some(ref v) = self.name   { attrs.push(Attr::kv("name",   v.as_str())); }
        if let Some(ref v) = self.hint   { attrs.push(Attr::kv("hint",   v.as_str())); }
        if let Some(m)     = self.max_size { attrs.push(Attr::kv("max-size", m.to_string())); }
        if self.multiple { attrs.push(Attr::flag("multiple")); }
        if self.invalid  { attrs.push(Attr::flag("invalid")); }
        wrap("ui-file-upload", &attrs, "")
    }
}
