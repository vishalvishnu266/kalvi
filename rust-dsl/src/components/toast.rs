//! `<ui-toast-host>` + `<ui-toast>` typed builders.
//!
//! In practice you drop `<ui-toast-host>` once in your page and then fire
//! toasts from JavaScript with `toast('Saved!', { tone: 'success' })`.
//! You can also render a pre-populated toast declaratively for SSR-fed
//! notifications.

use crate::core::{wrap, Attr, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastTone { Info, Success, Warning, Danger }
impl ToastTone {
    fn as_str(self) -> &'static str {
        match self { ToastTone::Info=>"info", ToastTone::Success=>"success",
                     ToastTone::Warning=>"warning", ToastTone::Danger=>"danger" }
    }
}

pub struct Toast {
    tone: ToastTone,
    title: String,
    desc: Option<String>,
    duration: u32,
}
pub fn toast(title: impl Into<String>) -> Toast {
    Toast { tone: ToastTone::Info, title: title.into(), desc: None, duration: 3500 }
}
impl Toast {
    pub fn tone(mut self, t: ToastTone)          -> Self { self.tone = t; self }
    pub fn desc(mut self, s: impl Into<String>)  -> Self { self.desc = Some(s.into()); self }
    pub fn duration(mut self, ms: u32)           -> Self { self.duration = ms; self }
}
impl Component for Toast {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("tone",     self.tone.as_str()),
            Attr::kv("title",    self.title.as_str()),
            Attr::kv("duration", self.duration.to_string()),
        ];
        if let Some(ref d) = self.desc { attrs.push(Attr::kv("desc", d.as_str())); }
        wrap("ui-toast", &attrs, "")
    }
}

/// A container you put once at the end of a page to host `window.toast(...)`
/// notifications.
pub struct ToastHost;
pub fn toast_host() -> ToastHost { ToastHost }
impl Component for ToastHost {
    fn render(&self) -> String { wrap("ui-toast-host", &[] as &[Attr], "") }
}
