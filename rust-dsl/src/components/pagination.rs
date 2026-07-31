//! `<ui-pagination>` typed builder.

use crate::core::{wrap, Attr, Component};

pub struct Pagination { page: u32, total: u32, per_page: u32, siblings: u32 }
pub fn pagination(page: u32, total: u32) -> Pagination {
    Pagination { page, total, per_page: 10, siblings: 1 }
}
impl Pagination {
    pub fn per_page(mut self, n: u32) -> Self { self.per_page = n; self }
    pub fn siblings(mut self, n: u32) -> Self { self.siblings = n; self }
}
impl Component for Pagination {
    fn render(&self) -> String {
        let attrs = [
            Attr::kv("page",     self.page.to_string()),
            Attr::kv("total",    self.total.to_string()),
            Attr::kv("per-page", self.per_page.to_string()),
            Attr::kv("siblings", self.siblings.to_string()),
        ];
        wrap("ui-pagination", &attrs, "")
    }
}
