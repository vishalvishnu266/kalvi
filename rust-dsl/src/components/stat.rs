//! `<ui-stat>` typed builder (KPI card).

use crate::core::{wrap, Attr, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend { Up, Down, Flat }
impl Trend {
    fn as_str(self) -> &'static str {
        match self { Trend::Up => "up", Trend::Down => "down", Trend::Flat => "flat" }
    }
}

pub struct Stat {
    label: String, value: String, delta: Option<String>,
    trend: Trend, icon: String,
}
pub fn stat(label: impl Into<String>, value: impl Into<String>) -> Stat {
    Stat { label: label.into(), value: value.into(),
           delta: None, trend: Trend::Flat, icon: "activity".into() }
}
impl Stat {
    pub fn delta(mut self, s: impl Into<String>) -> Self { self.delta = Some(s.into()); self }
    pub fn trend(mut self, t: Trend)              -> Self { self.trend = t; self }
    pub fn icon(mut self, name: impl Into<String>) -> Self { self.icon = name.into(); self }
}
impl Component for Stat {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("label", self.label.as_str()),
            Attr::kv("value", self.value.as_str()),
            Attr::kv("trend", self.trend.as_str()),
            Attr::kv("icon",  self.icon.as_str()),
        ];
        if let Some(ref d) = self.delta { attrs.push(Attr::kv("delta", d.as_str())); }
        wrap("ui-stat", &attrs, "")
    }
}
