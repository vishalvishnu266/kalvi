//! `<ui-avatar-group>` typed builder.

use crate::core::{wrap, Attr, Component};
use crate::components::avatar::{Avatar, AvatarSize};

pub struct AvatarGroup { max: u32, size: AvatarSize, avatars: Vec<Avatar> }
pub fn avatar_group() -> AvatarGroup {
    AvatarGroup { max: 4, size: AvatarSize::Md, avatars: Vec::new() }
}
impl AvatarGroup {
    pub fn max(mut self, n: u32)       -> Self { self.max = n; self }
    pub fn size(mut self, s: AvatarSize) -> Self { self.size = s; self }
    pub fn add(mut self, a: Avatar)    -> Self { self.avatars.push(a); self }
    pub fn avatars<I: IntoIterator<Item = Avatar>>(mut self, iter: I) -> Self {
        self.avatars.extend(iter); self
    }
}
impl Component for AvatarGroup {
    fn render(&self) -> String {
        let attrs = [
            Attr::kv("max",  self.max.to_string()),
            Attr::kv("size", match self.size {
                AvatarSize::Sm => "sm", AvatarSize::Md => "md",
                AvatarSize::Lg => "lg", AvatarSize::Xl => "xl",
            }),
        ];
        let body: String = self.avatars.iter().map(|a| a.render()).collect();
        wrap("ui-avatar-group", &attrs, &body)
    }
}
