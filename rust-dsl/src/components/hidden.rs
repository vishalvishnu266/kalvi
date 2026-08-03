//! Plain `<input type="hidden">` builder.
//!
//! Hidden fields are essential for server-rendered ERP forms:
//!
//! * **CSRF tokens** — every non-GET form must carry one.
//! * **Method overrides** — `_method=PUT` / `_method=DELETE` for HTML forms
//!   that only speak GET/POST.
//! * **Row / tenant / audit ids** — often needed on edit forms so the
//!   server knows which record is being updated.
//! * **Return-to URLs** — where to redirect after a successful save.
//!
//! Unlike the other `Input` variants this is a *native* `<input>` (not a
//! `<ui-input>` web component) because it has no visual surface and we
//! want the browser's form serialiser to pick it up automatically.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! form().action("/students").method("post")
//!     .csrf(&csrf_token)                     // convenience helper
//!     .add(hidden("student_id", "42"))       // raw hidden field
//!     .add(input().label("Full name").name("name"))
//!     .save_cancel("Save");
//! ```
//!
//! For PUT/PATCH/DELETE via a POST form:
//!
//! ```ignore
//! form().action("/students/42").method("post")
//!     .csrf(&csrf_token)
//!     .method_override("DELETE")
//!     .action_btn(button().label("Delete").variant(Variant::Danger).submit());
//! ```

use crate::core::{escape_html, Component};

/// A hidden form field. Renders as a native `<input type="hidden">`.
pub struct Hidden {
    name: String,
    value: String,
}

/// Start building a hidden field. Both `name` and `value` are required —
/// a hidden field with no name is dead weight.
pub fn hidden(name: impl Into<String>, value: impl Into<String>) -> Hidden {
    Hidden { name: name.into(), value: value.into() }
}

impl Hidden {
    /// Rename the field (rarely needed after construction, but symmetric
    /// with the other builders).
    pub fn name(mut self, s: impl Into<String>) -> Self { self.name = s.into(); self }
    /// Replace the field value.
    pub fn value(mut self, s: impl Into<String>) -> Self { self.value = s.into(); self }
}

impl Component for Hidden {
    fn render(&self) -> String {
        format!(
            r#"<input type="hidden" name="{}" value="{}">"#,
            escape_html(&self.name),
            escape_html(&self.value),
        )
    }
}
