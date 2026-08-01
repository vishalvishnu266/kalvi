//! `<ui-alert>` typed builder — **persistent page-level** notice.
//!
//! Use for messages that need to stay visible until dismissed or resolved,
//! rather than the transient `toast()`:
//!
//! * "Read-only mode: your role doesn't allow editing fees."
//! * "This term ends in 3 days — plan your exports now."
//! * "New version available — reload to update."
//!
//! For **cross-field validation** inside a form use `form_banner()` instead.
//! For **transient feedback** (save success, save error) use `toast()`.

use crate::core::{escape_html, wrap, Attr, Component};
use crate::components::badge::Tone;
use crate::components::icon::IconName;

pub struct Alert {
    tone: Tone,
    icon: Option<IconName>,
    title: Option<String>,
    message: Option<String>,
    dismissible: bool,
}

pub fn alert() -> Alert {
    Alert { tone: Tone::Info, icon: None, title: None, message: None, dismissible: false }
}

impl Alert {
    pub fn tone(mut self, t: Tone) -> Self { self.tone = t; self }
    pub fn icon(mut self, i: IconName) -> Self { self.icon = Some(i); self }
    pub fn title(mut self, s: impl Into<String>)   -> Self { self.title   = Some(s.into()); self }
    pub fn message(mut self, s: impl Into<String>) -> Self { self.message = Some(s.into()); self }
    pub fn dismissible(mut self) -> Self { self.dismissible = true; self }
}

fn tone_str(t: Tone) -> &'static str {
    match t {
        Tone::Neutral => "neutral",
        Tone::Brand   => "brand",
        Tone::Success => "success",
        Tone::Warning => "warning",
        Tone::Danger  => "danger",
        Tone::Info    => "info",
    }
}

impl Component for Alert {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("tone", tone_str(self.tone))];
        if let Some(ref v) = self.title   { attrs.push(Attr::kv("title",   v.as_str())); }
        if let Some(ref v) = self.message { attrs.push(Attr::kv("message", v.as_str())); }
        if let Some(i) = self.icon        { attrs.push(Attr::kv("icon",    i.as_str())); }
        if self.dismissible { attrs.push(Attr::flag("dismissible")); }
        wrap("ui-alert", &attrs, "")
    }
}

// Re-export escape_html so no `use` clutter is needed if this file grows.
#[allow(dead_code)]
fn _use_escape() { let _ = escape_html("x"); }
