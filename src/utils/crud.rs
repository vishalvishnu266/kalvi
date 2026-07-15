//! Shared helpers used by CRUD controllers (students, teachers, staff, …).
//!
//! The intent is to avoid copy-pasting the same 5 helper functions into every
//! controller. New modules should prefer these over rolling their own.

use axum::http::HeaderMap;

/// Trim a form string. Returns `None` if the trimmed value is empty; otherwise
/// returns `Some(trimmed_string)`. Useful when writing `Option<String>` DB
/// columns from HTML form fields.
pub fn opt(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

/// True if the current request was issued by a Turbo Frame (i.e. carries the
/// `Turbo-Frame` header). Used to render only the frame body instead of a full
/// page on subsequent frame-scoped navigations.
pub fn is_turbo_frame(headers: &HeaderMap) -> bool {
    headers.get("Turbo-Frame").is_some()
}

/// Translate a `sqlx::Error` into a user-friendly, short message suitable for
/// showing in a form-level error banner. Pattern-matches on well-known SQLite
/// error strings; falls back to a generic "Database error: <msg>".
///
/// Callers can pass a list of `(needle, replacement)` overrides for
/// column-specific UNIQUE constraint messages.
///
/// # Example
/// ```ignore
/// friendly_db_error(&e, &[("admission_no", "Admission number already exists.")])
/// ```
pub fn friendly_db_error(e: &sqlx::Error, unique_overrides: &[(&str, &str)]) -> String {
    let s = e.to_string();
    if s.contains("UNIQUE") {
        for (needle, msg) in unique_overrides {
            if s.contains(needle) {
                return (*msg).to_string();
            }
        }
        return "That value must be unique — please try another.".to_string();
    }
    if s.contains("FOREIGN KEY") {
        return "This record references data that no longer exists.".to_string();
    }
    if s.contains("CHECK constraint") {
        return "One of the fields has an invalid value.".to_string();
    }
    format!("Database error: {}", s)
}

/// Wrap a rendered HTML fragment inside a `<turbo-frame id="...">` so Turbo
/// can perform an in-place swap.
pub fn wrap_turbo_frame(frame_id: &str, inner_html: String) -> String {
    format!(r#"<turbo-frame id="{}">{}</turbo-frame>"#, frame_id, inner_html)
}

// NOTE: In Phase 0.3 we plan to introduce a `PageContext { user, tenant,
// student_count }` struct and a `render_page(ctx, tpl)` helper so every new
// controller stops repeating `user.full_name.clone()`, `user.initials()`,
// and the students COUNT query. Left as-is for now to keep 0.2 focused.
