//! Custom askama filters used across templates.
//!
//! Askama looks up unknown filters in `crate::filters` **or** the module
//! path configured via `#[template]`. We wire them into templates via the
//! standard `use crate::web::filters as filters;` — but askama actually
//! finds them by the `filters` module inside the template's crate root
//! after `pub use`. To keep it simple every template that needs one of these
//! filters gets it via the shim below.

/// Two-decimal money display, e.g. `1234.5` → `1,234.50`. Locale-agnostic.
pub fn money(v: &f64) -> askama::Result<String> {
    let n = *v;
    let sign = if n < 0.0 { "-" } else { "" };
    let n = n.abs();
    let whole = n.trunc() as i64;
    let cents = ((n - whole as f64) * 100.0).round() as i64;
    // Thousands separator with commas.
    let s = whole.to_string();
    let bytes = s.as_bytes();
    let mut with_sep = String::new();
    for (i, ch) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 {
            with_sep.push(',');
        }
        with_sep.push(*ch as char);
    }
    Ok(format!("{sign}{with_sep}.{cents:02}"))
}
