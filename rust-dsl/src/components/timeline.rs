//! `<ui-timeline>` + `<ui-timeline-item>` typed builders.

use crate::core::{wrap, Attr, Child, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineTone { Brand, Success, Warning, Danger, Info, Muted }
impl TimelineTone {
    fn as_str(self) -> &'static str {
        match self {
            TimelineTone::Brand   => "brand",
            TimelineTone::Success => "success",
            TimelineTone::Warning => "warning",
            TimelineTone::Danger  => "danger",
            TimelineTone::Info    => "info",
            TimelineTone::Muted   => "muted",
        }
    }
}

pub struct TimelineItem {
    icon: String,
    tone: TimelineTone,
    time: Option<String>,
    children: Vec<Child>,
}
pub fn timeline_item() -> TimelineItem {
    TimelineItem { icon: "activity".into(), tone: TimelineTone::Brand, time: None, children: Vec::new() }
}
impl TimelineItem {
    pub fn icon(mut self, name: impl Into<String>)    -> Self { self.icon = name.into(); self }
    pub fn tone(mut self, t: TimelineTone)            -> Self { self.tone = t; self }
    pub fn time(mut self, s: impl Into<String>)       -> Self { self.time = Some(s.into()); self }
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
}
impl Component for TimelineItem {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("icon", self.icon.as_str()),
            Attr::kv("tone", self.tone.as_str()),
        ];
        if let Some(ref t) = self.time { attrs.push(Attr::kv("time", t.as_str())); }
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("ui-timeline-item", &attrs, &body)
    }
}

pub struct Timeline { items: Vec<TimelineItem> }
pub fn timeline() -> Timeline { Timeline { items: Vec::new() } }
impl Timeline {
    pub fn item(mut self, i: TimelineItem) -> Self { self.items.push(i); self }
    pub fn items<I: IntoIterator<Item = TimelineItem>>(mut self, iter: I) -> Self {
        self.items.extend(iter); self
    }
}
impl Component for Timeline {
    fn render(&self) -> String {
        let body: String = self.items.iter().map(|i| i.render()).collect();
        wrap("ui-timeline", &[] as &[Attr], &body)
    }
}
