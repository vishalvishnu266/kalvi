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

pub trait IntoHtml {
    fn into_html(self) -> axum::response::Html<String>;
}

impl IntoHtml for String {
    fn into_html(self) -> axum::response::Html<String> {
        axum::response::Html(self)
    }
}

pub fn is_reserved_slug(slug: &str) -> bool {
    let reserved = ["saas", "api", "web", "health", "contact", "login", "registration"];
    reserved.contains(&slug) || slug.is_empty()
}

pub fn is_valid_slug(slug: &str) -> bool {
    if slug.is_empty() || is_reserved_slug(slug) {
        return false;
    }
    
    // Manual check instead of Regex: lowercase, numbers, and hyphens only
    slug.chars().all(|c| {
        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'
    })
}
