//! `<ui-skeleton>` typed builder.

use crate::core::{wrap, Attr, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape { Line, Rect, Circle }
impl Shape {
    fn as_str(self) -> &'static str {
        match self { Shape::Line=>"line", Shape::Rect=>"rect", Shape::Circle=>"circle" }
    }
}

pub struct Skeleton { shape: Shape, width: Option<String>, height: Option<String>, lines: u32 }
pub fn skeleton() -> Skeleton {
    Skeleton { shape: Shape::Line, width: None, height: None, lines: 1 }
}
impl Skeleton {
    pub fn shape(mut self, s: Shape)                  -> Self { self.shape  = s; self }
    pub fn width(mut self, w: impl Into<String>)      -> Self { self.width  = Some(w.into()); self }
    pub fn height(mut self, h: impl Into<String>)     -> Self { self.height = Some(h.into()); self }
    pub fn lines(mut self, n: u32)                    -> Self { self.lines  = n; self }
}
impl Component for Skeleton {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("shape", self.shape.as_str())];
        if let Some(ref w) = self.width  { attrs.push(Attr::kv("width",  w.as_str())); }
        if let Some(ref h) = self.height { attrs.push(Attr::kv("height", h.as_str())); }
        if self.lines != 1               { attrs.push(Attr::kv("lines",  self.lines.to_string())); }
        wrap("ui-skeleton", &attrs, "")
    }
}
