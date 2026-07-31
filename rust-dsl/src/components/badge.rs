//! `<ui-badge>` typed builder.

use crate::core::{escape_html, wrap, Attr, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone { Neutral, Brand, Success, Warning, Danger, Info }
impl Tone {
    fn as_str(self) -> &'static str {
        match self {
            Tone::Neutral => "neutral",
            Tone::Brand   => "brand",
            Tone::Success => "success",
            Tone::Warning => "warning",
            Tone::Danger  => "danger",
            Tone::Info    => "info",
        }
    }
}

pub struct Badge { label: String, tone: Tone, dot: bool }
pub fn badge(label: impl Into<String>) -> Badge {
    Badge { label: label.into(), tone: Tone::Neutral, dot: false }
}
impl Badge {
    pub fn tone(mut self, t: Tone) -> Self { self.tone = t; self }
    pub fn dot(mut self)           -> Self { self.dot = true; self }
}
impl Component for Badge {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if self.tone != Tone::Neutral { attrs.push(Attr::kv("tone", self.tone.as_str())); }
        if self.dot                   { attrs.push(Attr::flag("dot")); }
        wrap("ui-badge", &attrs, &escape_html(&self.label))
    }
}
