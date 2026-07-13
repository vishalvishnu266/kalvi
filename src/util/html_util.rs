use once_cell::sync::Lazy;
use regex::Regex;

pub static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z0-9-]+$").unwrap());

pub fn escape_html(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }
    escaped
}
