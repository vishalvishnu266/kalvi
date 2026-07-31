//! `<ui-tooltip>` typed builder.

use crate::core::{wrap, Attr, Child, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement { Top, Bottom, Left, Right }
impl Placement {
    fn as_str(self) -> &'static str {
        match self { Placement::Top=>"top", Placement::Bottom=>"bottom", Placement::Left=>"left", Placement::Right=>"right" }
    }
}

pub struct Tooltip { text: String, placement: Placement, delay: u32, children: Vec<Child> }
pub fn tooltip(text: impl Into<String>) -> Tooltip {
    Tooltip { text: text.into(), placement: Placement::Top, delay: 250, children: Vec::new() }
}
impl Tooltip {
    pub fn placement(mut self, p: Placement) -> Self { self.placement = p; self }
    pub fn delay(mut self, ms: u32)          -> Self { self.delay = ms; self }
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
}
impl Component for Tooltip {
    fn render(&self) -> String {
        let attrs = [
            Attr::kv("text",      self.text.as_str()),
            Attr::kv("placement", self.placement.as_str()),
            Attr::kv("delay",     self.delay.to_string()),
        ];
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("ui-tooltip", &attrs, &body)
    }
}
