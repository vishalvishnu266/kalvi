//! `<ui-data-table>` — the DSL side sets attributes only; the actual rows +
//! columns are pushed to the JS component via an inline `<script>` because
//! `ui-data-table` uses a JS API (`.rows` / `.columns`) rather than
//! declarative children.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! data_table("students")
//!     .searchable().selectable().per_page(10)
//!     .col("name",   "Name",  ColOpts::text().sortable())
//!     .col("grade",  "Grade", ColOpts::text().sortable().center())
//!     .col("status", "Status",
//!         ColOpts::render("(v)=>`<ui-badge tone=\"${v==='present'?'success':'danger'}\">${v}</ui-badge>`"))
//!     .row(&[("name","Aarav"),("grade","5"),("status","present")])
//!     .render();
//! ```

use crate::core::{escape_html, Component};
use std::collections::BTreeMap;

pub struct ColOpts {
    sortable: bool,
    align: Option<&'static str>,   // "center" | "right"
    render_js: Option<String>,     // JS expression, e.g. "(v) => `<b>${v}</b>`"
}
impl ColOpts {
    pub fn text() -> Self { Self { sortable: false, align: None, render_js: None } }
    pub fn render(js: impl Into<String>) -> Self {
        Self { sortable: false, align: None, render_js: Some(js.into()) }
    }
    pub fn sortable(mut self) -> Self { self.sortable = true; self }
    pub fn center(mut self)   -> Self { self.align = Some("center"); self }
    pub fn right(mut self)    -> Self { self.align = Some("right");  self }
}

pub struct DataTable {
    id: String,
    searchable: bool,
    selectable: bool,
    per_page: u32,
    cols: Vec<(String, String, ColOpts)>,     // (key, label, opts)
    rows: Vec<BTreeMap<String, String>>,
}
pub fn data_table(id: impl Into<String>) -> DataTable {
    DataTable {
        id: id.into(), searchable: false, selectable: false, per_page: 10,
        cols: Vec::new(), rows: Vec::new(),
    }
}
impl DataTable {
    pub fn searchable(mut self)          -> Self { self.searchable = true; self }
    pub fn selectable(mut self)          -> Self { self.selectable = true; self }
    pub fn per_page(mut self, n: u32)    -> Self { self.per_page = n; self }
    pub fn col(mut self, key: impl Into<String>, label: impl Into<String>, opts: ColOpts) -> Self {
        self.cols.push((key.into(), label.into(), opts)); self
    }
    pub fn row<I, K, V>(mut self, cells: I) -> Self
    where I: IntoIterator<Item = (K, V)>, K: Into<String>, V: Into<String> {
        let mut m = BTreeMap::new();
        for (k, v) in cells { m.insert(k.into(), v.into()); }
        self.rows.push(m); self
    }
}
impl Component for DataTable {
    fn render(&self) -> String {
        // 1) The element itself
        let flags = format!(
            "{}{} per-page=\"{}\"",
            if self.searchable { " searchable" } else { "" },
            if self.selectable { " selectable" } else { "" },
            self.per_page,
        );
        let mut out = format!(r#"<ui-data-table id="{}"{}></ui-data-table>"#,
            escape_html(&self.id), flags);

        // 2) Inline <script> that hydrates .columns + .rows
        let cols_js: Vec<String> = self.cols.iter().map(|(k, l, o)| {
            let mut parts = vec![
                format!(r#"key: {}"#, json_str(k)),
                format!(r#"label: {}"#, json_str(l)),
            ];
            if o.sortable { parts.push("sortable: true".into()); }
            if let Some(a) = o.align { parts.push(format!(r#"align: "{}""#, a)); }
            if let Some(ref js) = o.render_js { parts.push(format!("render: {}", js)); }
            format!("{{ {} }}", parts.join(", "))
        }).collect();

        let rows_js: Vec<String> = self.rows.iter().map(|r| {
            let pairs: Vec<String> = r.iter()
                .map(|(k, v)| format!("{}: {}", json_str(k), json_str(v)))
                .collect();
            format!("{{ {} }}", pairs.join(", "))
        }).collect();

        out.push_str(&format!(
            r#"<script>(function(){{
  var t = document.getElementById({id_js});
  if (!t) return;
  t.columns = [{cols}];
  t.rows    = [{rows}];
}})();</script>"#,
            id_js = json_str(&self.id),
            cols  = cols_js.join(","),
            rows  = rows_js.join(","),
        ));
        out
    }
}

/// Escape a Rust string into a JS/JSON string literal (with surrounding quotes).
fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"'  => out.push_str(r#"\""#),
            '\\' => out.push_str(r"\\"),
            '\n' => out.push_str(r"\n"),
            '\r' => out.push_str(r"\r"),
            '\t' => out.push_str(r"\t"),
            '<'  => out.push_str(r"\u003c"),  // avoid </script> injection
            '>'  => out.push_str(r"\u003e"),
            '&'  => out.push_str(r"\u0026"),
            c if (c as u32) < 0x20 => out.push_str(&format!(r"\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
