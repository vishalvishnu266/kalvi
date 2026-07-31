//! `<ui-icon>` typed builder.

use crate::core::{wrap, Attr, Component};

pub struct Icon { name: String, size: u32 }
pub fn icon(name: impl Into<String>) -> Icon { Icon { name: name.into(), size: 18 } }
impl Icon {
    pub fn size(mut self, px: u32) -> Self { self.size = px; self }
}
impl Component for Icon {
    fn render(&self) -> String {
        let attrs = [
            Attr::kv("name", self.name.as_str()),
            Attr::kv("size", self.size.to_string()),
        ];
        wrap("ui-icon", &attrs, "")
    }
}
