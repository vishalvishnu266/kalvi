//! `<ui-ack-panel>` — **slide-in acknowledgment panel**.
//!
//! A drawer-style panel that slides in from the left or right of the
//! viewport carrying a single semantic message + a required ACK button.
//! Unlike a normal drawer:
//!
//!   * It's **modal** — a scrim behind it prevents interaction with the
//!     rest of the page.
//!   * The user MUST press the acknowledgment button to dismiss it (no
//!     ESC-to-close, no click-outside-to-close). That's what makes it an
//!     *acknowledgment* rather than an *alert*.
//!   * Semantic-tone-driven: `Info`, `Warning`, `Danger` — the whole
//!     accent bar + icon color come from the tone.
//!
//! ## Where it fits in the error UX
//!
//! | Scope | Surface | Component |
//! |---|---|---|
//! | Field-level | Under the field | `input().error(...)` |
//! | Form-level | Banner at top of form | `form_banner()` |
//! | Page-level, persistent | Coloured box at top of page | `alert()` |
//! | Page-level, transient | Toast (auto-dismissing) | `toast()` |
//! | **Blocking acknowledgment** — user MUST see + confirm | **Slide-in panel** | **`ack_panel()` ← this file** |
//!
//! ## Usage
//!
//! ```ignore
//! ack_panel()
//!     .id("ack-fees-locked")
//!     .placement(AckPlacement::Right)
//!     .tone(Tone::Warning)
//!     .icon(Icons::WARNING)
//!     .title("Fees module is locked")
//!     .message("Editing is disabled until the term audit completes on Aug 15. \
//!               Contact admin@school for urgent adjustments.")
//!     .ack_label("I understand")
//! ```
//!
//! Because it's slide-in, you typically render it as `.open()` on page
//! load (for a page-blocking notice) or trigger it from JS via
//! `document.getElementById('ack-fees-locked').open()`.

use crate::core::{wrap, Attr, Component};
use crate::components::badge::Tone;
use crate::components::icon::IconName;

/// Which edge the panel slides in from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AckPlacement { Left, Right }

pub struct AckPanel {
    id: Option<String>,
    tone: Tone,
    placement: AckPlacement,
    icon: Option<IconName>,
    title: Option<String>,
    message: Option<String>,
    ack_label: String,
    open: bool,
}

pub fn ack_panel() -> AckPanel {
    AckPanel {
        id: None,
        tone: Tone::Info,
        placement: AckPlacement::Right,   // right is the standard side (like drawers)
        icon: None,
        title: None,
        message: None,
        ack_label: "OK".to_string(),
        open: false,
    }
}

impl AckPanel {
    /// Element id — required so JS can call `.open()` / `.close()` on it.
    pub fn id(mut self, s: impl Into<String>) -> Self { self.id = Some(s.into()); self }

    pub fn tone(mut self, t: Tone)                 -> Self { self.tone = t; self }
    pub fn placement(mut self, p: AckPlacement)    -> Self { self.placement = p; self }
    pub fn icon(mut self, i: IconName)             -> Self { self.icon = Some(i); self }
    pub fn title(mut self, s: impl Into<String>)   -> Self { self.title = Some(s.into()); self }
    pub fn message(mut self, s: impl Into<String>) -> Self { self.message = Some(s.into()); self }

    /// Label on the acknowledgment button. Default: "OK". Convention:
    /// use "I understand" / "Acknowledge" for warnings; "OK" for info.
    pub fn ack_label(mut self, s: impl Into<String>) -> Self { self.ack_label = s.into(); self }

    /// Render the panel already open (for page-blocking notices that show
    /// on load). Otherwise trigger from JS.
    pub fn open(mut self) -> Self { self.open = true; self }
}

fn tone_str(t: Tone) -> &'static str {
    match t {
        Tone::Neutral => "neutral",
        Tone::Brand   => "brand",
        Tone::Success => "success",
        Tone::Warning => "warning",
        Tone::Danger  => "danger",
        Tone::Info    => "info",
    }
}
fn placement_str(p: AckPlacement) -> &'static str {
    match p { AckPlacement::Left => "left", AckPlacement::Right => "right" }
}

impl Component for AckPanel {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("tone",       tone_str(self.tone)),
            Attr::kv("placement",  placement_str(self.placement)),
            Attr::kv("ack-label",  self.ack_label.as_str()),
        ];
        if let Some(ref v) = self.id      { attrs.push(Attr::kv("id",      v.as_str())); }
        if let Some(ref v) = self.title   { attrs.push(Attr::kv("title",   v.as_str())); }
        if let Some(ref v) = self.message { attrs.push(Attr::kv("message", v.as_str())); }
        if let Some(i)     = self.icon    { attrs.push(Attr::kv("icon",    i.as_str())); }
        if self.open { attrs.push(Attr::flag("open")); }
        wrap("ui-ack-panel", &attrs, "")
    }
}
