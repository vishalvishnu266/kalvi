//! `<ui-copilot>` — typed builder for the agentic AI chat window.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! // Simplest form — uses default `/copilot/*` endpoints:
//! let html = copilot().render();
//!
//! // Custom endpoints (e.g. tenant-scoped):
//! let html = copilot()
//!     .session_url("/web/acme/copilot/session")
//!     .stream_url("/web/acme/copilot/message")
//!     .history_url("/web/acme/copilot/history")
//!     .label("Acme Copilot")
//!     .render();
//! ```
//!
//! The rendered custom element is *self-contained* — it draws its own
//! floating action button and drawer/bottom-sheet. Put it once at the end
//! of your page (or let `page().with_copilot()` inject it for you).

use crate::core::{wrap, Attr, Component};

pub struct Copilot {
    session_url: String,
    stream_url:  String,
    history_url: String,
    label:       String,
    open:        bool,
}

/// Start a new `<ui-copilot>` builder with sensible defaults
/// (`/copilot/session`, `/copilot/message`, `/copilot/history`).
pub fn copilot() -> Copilot {
    Copilot {
        session_url: "/copilot/session".into(),
        stream_url:  "/copilot/message".into(),
        history_url: "/copilot/history".into(),
        label:       "ERP Copilot".into(),
        open:        false,
    }
}

impl Copilot {
    /// URL to POST to for creating (or resuming) a chat session.
    /// The server must return `{"session_id": "…"}`.
    pub fn session_url(mut self, s: impl Into<String>) -> Self { self.session_url = s.into(); self }
    /// URL to POST `{session_id, prompt}` to; must respond `text/event-stream`.
    pub fn stream_url (mut self, s: impl Into<String>) -> Self { self.stream_url  = s.into(); self }
    /// URL prefix for GET-ing prior history. Called as `<prefix>/<session_id>`.
    pub fn history_url(mut self, s: impl Into<String>) -> Self { self.history_url = s.into(); self }
    /// Panel title shown in the drawer header.
    pub fn label      (mut self, s: impl Into<String>) -> Self { self.label       = s.into(); self }
    /// Start with the panel open (useful for demos).
    pub fn open       (mut self)                       -> Self { self.open = true; self }
}

impl Component for Copilot {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("session-url", self.session_url.as_str()),
            Attr::kv("stream-url",  self.stream_url.as_str()),
            Attr::kv("history-url", self.history_url.as_str()),
            Attr::kv("label",       self.label.as_str()),
        ];
        if self.open { attrs.push(Attr::flag("open")); }
        wrap("ui-copilot", &attrs, "")
    }
}
