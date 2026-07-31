//! `<ui-stepper>` typed builder.

use crate::core::{escape_html, wrap, Attr, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepperOrientation { Horizontal, Vertical }
impl StepperOrientation {
    fn as_str(self) -> &'static str {
        match self { StepperOrientation::Horizontal=>"horizontal", StepperOrientation::Vertical=>"vertical" }
    }
}

pub struct Stepper {
    current: u32,
    orientation: StepperOrientation,
    clickable: bool,
    steps: Vec<String>,
}
pub fn stepper() -> Stepper {
    Stepper { current: 0, orientation: StepperOrientation::Horizontal, clickable: false, steps: Vec::new() }
}
impl Stepper {
    pub fn current(mut self, n: u32)                 -> Self { self.current = n; self }
    pub fn orientation(mut self, o: StepperOrientation) -> Self { self.orientation = o; self }
    pub fn clickable(mut self)                       -> Self { self.clickable = true; self }
    pub fn step(mut self, label: impl Into<String>)  -> Self { self.steps.push(label.into()); self }
    pub fn steps<I, S>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.steps.extend(iter.into_iter().map(Into::into)); self
    }
}
impl Component for Stepper {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("current",     self.current.to_string()),
            Attr::kv("orientation", self.orientation.as_str()),
        ];
        if self.clickable { attrs.push(Attr::flag("clickable")); }
        let body: String = self.steps.iter()
            .map(|s| format!("<span>{}</span>", escape_html(s)))
            .collect();
        wrap("ui-stepper", &attrs, &body)
    }
}
