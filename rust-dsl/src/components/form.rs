//! `<ui-form>` typed builder.

use crate::components::button::{button, Variant};
use crate::components::icon::Icons;
use crate::core::{wrap, Attr, Child, Component};

pub struct Form {
    action: Option<String>,
    method: String,      // "get" | "post"
    novalidate: bool,
    inline: bool,
    banner: Option<Child>,   // optional <ui-form-banner> at the top
    fields: Vec<Child>,
    actions: Vec<Child>,
}
pub fn form() -> Form {
    Form {
        action: None, method: "post".into(),
        novalidate: false, inline: false, banner: None,
        fields: Vec::new(), actions: Vec::new(),
    }
}
impl Form {
    pub fn action(mut self, s: impl Into<String>) -> Self { self.action = Some(s.into()); self }
    pub fn method(mut self, s: impl Into<String>) -> Self { self.method = s.into(); self }
    pub fn novalidate(mut self) -> Self { self.novalidate = true; self }
    pub fn inline(mut self)     -> Self { self.inline     = true; self }

    /// Add a field (input, select, checkbox, …) to the form body.
    pub fn add(mut self, c: impl Component + 'static) -> Self { self.fields.push(Box::new(c)); self }
    pub fn children<I, C>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = C>, C: Component + 'static {
        for c in iter { self.fields.push(Box::new(c)); } self
    }
    /// Add a button (or anything) into the form's "actions" slot.
    pub fn action_btn(mut self, c: impl Component + 'static) -> Self {
        self.actions.push(Box::new(c)); self
    }

    /// Standard "Cancel" + "Save"-style footer. Convention: the destructive-
    /// safe (secondary) button on the left, the primary action on the right,
    /// primary button carries the check icon.
    ///
    /// ```ignore
    /// form().action("/students").method("post")
    ///     .add(input().label("Name"))
    ///     .save_cancel("Save student");     // → [Cancel] [Save student ✓]
    /// ```
    ///
    /// Pass a custom primary label (e.g. `"Create invoice"`) so it stays
    /// verb-first. Cancel is always labelled `"Cancel"` — that's the
    /// convention.
    pub fn save_cancel(self, save_label: impl Into<String>) -> Self {
        self.action_btn(button().label("Cancel").variant(Variant::Secondary))
            .action_btn(button().label(save_label).variant(Variant::Primary).icon(Icons::CHECK))
    }

    /// Attach a `form_banner()` at the top of the form. This is the
    /// **standard place for form-level errors** — cross-field business
    /// rules ("guardian email must differ from student email"), server-
    /// side rejections, or success confirmations after a save.
    ///
    /// ```ignore
    /// form().action("/students").method("post")
    ///     .banner(form_banner().tone(Tone::Danger)
    ///         .message("Guardian email must be different from the student's email."))
    ///     .add(input().label("Student email"))
    ///     .add(input().label("Guardian email"))
    ///     .save_cancel("Save");
    /// ```
    pub fn banner(mut self, b: impl Component + 'static) -> Self {
        self.banner = Some(Box::new(b)); self
    }

    /// Convenience — accepts an `Option<FormBanner>` so server code doesn't
    /// need a `match` for the "no banner" case. `None` = no banner.
    pub fn maybe_banner(self, b: Option<impl Component + 'static>) -> Self {
        match b { Some(bx) => self.banner(bx), None => self }
    }
}
impl Component for Form {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("method", self.method.as_str())];
        if let Some(ref a) = self.action { attrs.push(Attr::kv("action", a.as_str())); }
        if self.novalidate { attrs.push(Attr::flag("novalidate")); }
        if self.inline     { attrs.push(Attr::flag("inline")); }

        // Banner slot (renders at the top of the form).
        let mut body = String::new();
        if let Some(ref b) = self.banner {
            body.push_str(r#"<span slot="banner">"#);
            body.push_str(&b.render());
            body.push_str("</span>");
        }
        body.push_str(&self.fields.iter().map(|c| c.render()).collect::<String>());
        for c in &self.actions {
            body.push_str(r#"<span slot="actions">"#);
            body.push_str(&c.render());
            body.push_str("</span>");
        }
        wrap("ui-form", &attrs, &body)
    }
}
