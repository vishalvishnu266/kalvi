//! `<ui-avatar>` typed builder.

use crate::core::{wrap, Attr, Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize { Sm, Md, Lg, Xl }
impl AvatarSize {
    fn as_str(self) -> &'static str {
        match self { AvatarSize::Sm=>"sm", AvatarSize::Md=>"md", AvatarSize::Lg=>"lg", AvatarSize::Xl=>"xl" }
    }
}

pub struct Avatar { name: String, src: Option<String>, size: AvatarSize }
pub fn avatar(name: impl Into<String>) -> Avatar {
    Avatar { name: name.into(), src: None, size: AvatarSize::Md }
}
impl Avatar {
    pub fn src(mut self, url: impl Into<String>) -> Self { self.src = Some(url.into()); self }
    pub fn size(mut self, s: AvatarSize) -> Self { self.size = s; self }
}
impl Component for Avatar {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("name", self.name.as_str()),
            Attr::kv("size", self.size.as_str()),
        ];
        if let Some(ref s) = self.src { attrs.push(Attr::kv("src", s.as_str())); }
        wrap("ui-avatar", &attrs, "")
    }
}
