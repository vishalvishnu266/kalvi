//! `<ui-form>` typed builder.

use crate::components::button::{button, Variant};
use crate::components::hidden::hidden;
use crate::components::icon::Icons;
use crate::core::{wrap, Attr, Child, Component};

/// Where the actions row should stick when the form scrolls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StickyActions {
    /// Not sticky — the actions row scrolls with the fields (default).
    Off,
    /// Pin the actions row to the top of the viewport. Good when Save
    /// is the primary intent and the user is likely already scrolling
    /// down into more optional fields (long edit forms).
    Top,
    /// Pin the actions row to the bottom of the viewport. The most
    /// common ERP pattern: users fill top-to-bottom, and Save is
    /// always within thumb reach on mobile and one click away on
    /// desktop, no matter how long the form is.
    Bottom,
}
impl StickyActions {
    fn as_str(self) -> Option<&'static str> {
        match self {
            StickyActions::Off    => None,
            StickyActions::Top    => Some("top"),
            StickyActions::Bottom => Some("bottom"),
        }
    }
}

pub struct Form {
    action: Option<String>,
    method: String,      // "get" | "post"
    novalidate: bool,
    inline: bool,
    sticky: StickyActions,
    sticky_offset: Option<String>,
    banner: Option<Child>,   // optional <ui-form-banner> at the top
    fields: Vec<Child>,
    actions: Vec<Child>,
}
pub fn form() -> Form {
    Form {
        action: None, method: "post".into(),
        novalidate: false, inline: false,
        sticky: StickyActions::Off, sticky_offset: None,
        banner: None,
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
            .action_btn(button().label(save_label).variant(Variant::Primary)
                        .icon(Icons::CHECK).submit())
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

    // ---------------------------------------------------------------
    // Hidden-field conveniences (CSRF, method override, arbitrary
    // hidden values).
    // ---------------------------------------------------------------

    /// Inject a CSRF token as a hidden field.
    ///
    /// The default field name is `_csrf` — override via
    /// [`Form::csrf_named`] if your server expects `authenticity_token`,
    /// `csrfmiddlewaretoken`, `__RequestVerificationToken`, etc.
    ///
    /// ```ignore
    /// form().action("/students").method("post")
    ///     .csrf(&ctx.csrf_token)
    ///     .add(input().label("Name").name("name"))
    ///     .save_cancel("Save");
    /// ```
    pub fn csrf(self, token: impl Into<String>) -> Self {
        self.csrf_named("_csrf", token)
    }

    /// Same as [`Form::csrf`] but with a custom field name (e.g.
    /// `authenticity_token` for Rails, `csrfmiddlewaretoken` for Django).
    pub fn csrf_named(self, name: impl Into<String>, token: impl Into<String>) -> Self {
        self.add(hidden(name, token))
    }

    /// Inject a `_method` hidden field so a POST form can express PUT,
    /// PATCH, or DELETE. Standard convention used by Rails, Laravel,
    /// Axum's `MethodOverrideLayer`, etc.
    ///
    /// ```ignore
    /// form().action("/students/42").method("post")
    ///     .csrf(&token)
    ///     .method_override("DELETE")
    ///     .action_btn(button().label("Delete").variant(Variant::Danger).submit());
    /// ```
    pub fn method_override(self, http_verb: impl Into<String>) -> Self {
        self.add(hidden("_method", http_verb))
    }

    /// Add a raw hidden field. Thin sugar over
    /// `.add(hidden(name, value))` — kept so the intent reads clearly at
    /// the call site.
    pub fn hidden(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.add(hidden(name, value))
    }

    // ---------------------------------------------------------------
    // Sticky action bar — keep Save/Cancel visible while the user
    // scrolls a long form.
    // ---------------------------------------------------------------

    /// Pin the actions row (Save / Cancel / Delete …) so it stays on
    /// screen while the user scrolls. Uses CSS `position: sticky`
    /// under the hood — no scroll listeners, no layout thrash, and
    /// short forms behave exactly as before (the row only pins when
    /// it would otherwise scroll off).
    ///
    /// The **bottom** placement is the recommended default for ERP
    /// edit/create forms (Save always within one click), while
    /// **top** is useful for review/approval forms where the primary
    /// action sits above the header.
    ///
    /// ```ignore
    /// form().action("/students").method("post")
    ///     .csrf(&token)
    ///     .sticky_actions(StickyActions::Bottom)   // Save follows the scroll
    ///     .add(form_section("Personal").open().add(input().label("Name")))
    ///     .add(form_section("Address").add(input().label("Street")))
    ///     .save_cancel("Save student");
    /// ```
    pub fn sticky_actions(mut self, placement: StickyActions) -> Self {
        self.sticky = placement; self
    }

    /// Sugar for `sticky_actions(StickyActions::Bottom)`.
    pub fn sticky_bottom(self) -> Self { self.sticky_actions(StickyActions::Bottom) }
    /// Sugar for `sticky_actions(StickyActions::Top)`.
    pub fn sticky_top(self)    -> Self { self.sticky_actions(StickyActions::Top) }

    /// Optional CSS length pushed into `--sticky-offset` on the host
    /// so the pinned bar sits below your app-shell topbar (or above
    /// your bottom-nav on mobile) instead of overlapping it.
    ///
    /// ```ignore
    /// form()
    ///     .sticky_bottom()
    ///     .sticky_offset("var(--app-bottomnav-h, 0px)")   // sit above bottom-nav
    /// ```
    pub fn sticky_offset(mut self, css_length: impl Into<String>) -> Self {
        self.sticky_offset = Some(css_length.into()); self
    }
}
impl Component for Form {
    fn render(&self) -> String {
        let mut attrs = vec![Attr::kv("method", self.method.as_str())];
        if let Some(ref a) = self.action { attrs.push(Attr::kv("action", a.as_str())); }
        if self.novalidate { attrs.push(Attr::flag("novalidate")); }
        if self.inline     { attrs.push(Attr::flag("inline")); }
        if let Some(s) = self.sticky.as_str() { attrs.push(Attr::kv("sticky", s)); }
        if let Some(ref o) = self.sticky_offset {
            attrs.push(Attr::kv("sticky-offset", o.as_str()));
        }

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
