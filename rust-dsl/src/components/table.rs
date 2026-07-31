//! Simple `<ui-table>` wrapper around a `<table>` you build by hand.
//!
//! For sortable/filterable/paged tables use [`crate::components::data_table`]
//! instead — it takes typed `columns` + `rows` and does the work.

use crate::core::{escape_html, wrap, Attr, Component};

pub struct Column {
    pub key: &'static str,
    pub label: &'static str,
    pub align: Align,
}
#[derive(Debug, Clone, Copy)]
pub enum Align { Left, Center, Right }
impl Align {
    fn css(self) -> &'static str {
        match self { Align::Left => "left", Align::Center => "center", Align::Right => "right" }
    }
}

pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
}
pub fn table() -> Table { Table { columns: Vec::new(), rows: Vec::new() } }
impl Table {
    pub fn column(mut self, key: &'static str, label: &'static str) -> Self {
        self.columns.push(Column { key, label, align: Align::Left }); self
    }
    pub fn column_aligned(mut self, key: &'static str, label: &'static str, a: Align) -> Self {
        self.columns.push(Column { key, label, align: a }); self
    }
    pub fn row<I, S>(mut self, cells: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.rows.push(cells.into_iter().map(Into::into).collect()); self
    }
}
impl Component for Table {
    fn render(&self) -> String {
        let mut inner = String::from("<table><thead><tr>");
        for c in &self.columns {
            inner.push_str(&format!(
                r#"<th style="text-align:{}">{}</th>"#,
                c.align.css(), escape_html(c.label)
            ));
        }
        inner.push_str("</tr></thead><tbody>");
        for r in &self.rows {
            inner.push_str("<tr>");
            for (i, cell) in r.iter().enumerate() {
                let align = self.columns.get(i).map(|c| c.align).unwrap_or(Align::Left);
                inner.push_str(&format!(
                    r#"<td style="text-align:{}">{}</td>"#,
                    align.css(), cell
                ));
            }
            inner.push_str("</tr>");
        }
        inner.push_str("</tbody></table>");
        wrap("ui-table", &[] as &[Attr], &inner)
    }
}
