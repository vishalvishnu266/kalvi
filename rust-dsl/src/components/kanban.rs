//! `<ui-kanban>` + `<ui-kanban-column>` + `<ui-kanban-card>` typed builders.

use crate::core::{escape_html, wrap, Attr, Child, Component};

pub struct KanbanCard { id: Option<String>, children: Vec<Child> }
pub fn kanban_card() -> KanbanCard { KanbanCard { id: None, children: Vec::new() } }
impl KanbanCard {
    pub fn id(mut self, s: impl Into<String>) -> Self { self.id = Some(s.into()); self }
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.children.push(Box::new(c)); self }
    pub fn text(mut self, s: impl Into<String>) -> Self {
        self.children.push(Box::new(crate::core::Node::text(s.into()))); self
    }
}
impl Component for KanbanCard {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref i) = self.id { attrs.push(Attr::kv("id", i.as_str())); }
        let body: String = self.children.iter().map(|c| c.render()).collect();
        wrap("ui-kanban-card", &attrs, &body)
    }
}

pub struct KanbanColumn { title: String, cards: Vec<KanbanCard> }
pub fn kanban_column(title: impl Into<String>) -> KanbanColumn {
    KanbanColumn { title: title.into(), cards: Vec::new() }
}
impl KanbanColumn {
    pub fn add(mut self, c: KanbanCard) -> Self { self.cards.push(c); self }
    pub fn cards<I: IntoIterator<Item = KanbanCard>>(mut self, iter: I) -> Self {
        self.cards.extend(iter); self
    }
}
impl Component for KanbanColumn {
    fn render(&self) -> String {
        let attrs = [Attr::kv("title", self.title.as_str())];
        let body: String = self.cards.iter().map(|c| c.render()).collect();
        wrap("ui-kanban-column", &attrs, &body)
    }
    // NB: the `title` on the parent element is used by the JS component
    // for the header text; escape_html is applied by Attr::render().
}

pub struct Kanban { columns: Vec<KanbanColumn> }
pub fn kanban() -> Kanban { Kanban { columns: Vec::new() } }
impl Kanban {
    pub fn column(mut self, c: KanbanColumn) -> Self { self.columns.push(c); self }
    pub fn columns<I: IntoIterator<Item = KanbanColumn>>(mut self, iter: I) -> Self {
        self.columns.extend(iter); self
    }
}
impl Component for Kanban {
    fn render(&self) -> String {
        let body: String = self.columns.iter().map(|c| c.render()).collect();
        // `wrap` used just for consistency; no attrs.
        let _ = escape_html; // silence unused-import if compiled alone
        wrap("ui-kanban", &[] as &[Attr], &body)
    }
}
