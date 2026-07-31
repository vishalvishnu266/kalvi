//! `<ui-drawer>` typed builder.

use crate::core::{wrap, Attr, Child, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerPlacement { Left, Right, Bottom }
impl DrawerPlacement {
    fn as_str(self) -> &'static str {
        match self { DrawerPlacement::Left=>"left", DrawerPlacement::Right=>"right", DrawerPlacement::Bottom=>"bottom" }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerSize { Sm, Md, Lg, Xl }
impl DrawerSize {
    fn as_str(self) -> &'static str {
        match self { DrawerSize::Sm=>"sm", DrawerSize::Md=>"md", DrawerSize::Lg=>"lg", DrawerSize::Xl=>"xl" }
    }
}

pub struct Drawer {
    id: Option<String>,
    title: Option<String>,
    open: bool,
    placement: DrawerPlacement,
    size: DrawerSize,
    children: Vec<Child>,
    footer: Vec<Child>,
}
pub fn drawer() -> Drawer {
    Drawer { id: None, title: None, open: false, placement: DrawerPlacement::Right,
             size: DrawerSize::Md, children: Vec::new(), footer: Vec::new() }
}
impl Drawer {
    pub fn id(mut self, s: impl Into<String>)         -> Self { self.id = Some(s.into()); self }
    pub fn title(mut self, s: impl Into<String>)      -> Self { self.title = Some(s.into()); self }
    pub fn open(mut self)                             -> Self { self.open = true; self }
    pub fn placement(mut self, p: DrawerPlacement)    -> Self { self.placement = p; self }
    pub fn size(mut self, s: DrawerSize)              -> Self { self.size = s; self }
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
    pub fn footer(mut self, c: impl Component + 'static) -> Self { self.footer.push(Box::new(c)); self }
}
impl Component for Drawer {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("placement", self.placement.as_str()),
            Attr::kv("size",      self.size.as_str()),
        ];
        if let Some(ref i) = self.id    { attrs.push(Attr::kv("id",    i.as_str())); }
        if let Some(ref t) = self.title { attrs.push(Attr::kv("title", t.as_str())); }
        if self.open { attrs.push(Attr::flag("open")); }
        let mut body = String::new();
        for c in &self.children { body.push_str(&c.render()); }
        for f in &self.footer {
            body.push_str(r#"<span slot="footer">"#);
            body.push_str(&f.render());
            body.push_str("</span>");
        }
        wrap("ui-drawer", &attrs, &body)
    }
}
