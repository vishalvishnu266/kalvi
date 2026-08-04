//! `<ui-copilot>` — typed builder for the unified agentic AI chat surface.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! // Simplest — uses default `/agent/*` endpoints:
//! let html = copilot().render();
//!
//! // Custom base (e.g. per-tenant):
//! let html = copilot()
//!     .agent_base("/web/acme/agent")
//!     .label("Acme Copilot")
//!     .render();
//! ```
//!
//! The rendered custom element is *self-contained* — it draws its own
//! floating action button and drawer/bottom-sheet, and expects the server
//! to expose the standard set of `/agent/*` endpoints:
//!
//! * `GET  <base>/suggest?page=…`   → proactive chips
//! * `GET  <base>/complete?q=…`     → live autocomplete
//! * `GET  <base>/schema?tool=…`    → tool arg schema
//! * `GET  <base>/entities?type=…`  → mention typeahead
//! * `POST <base>/invoke {tool,args}`  → one-shot tool call
//! * `POST <base>/stream {tool,args}`  → SSE multi-step stream
//!
//! Put the element once at the end of your page (or let
//! `page().with_copilot()` inject it for you — done automatically for every
//! page built through `page_of()`).

use crate::core::{wrap, Attr, Component};

pub struct Copilot {
    agent_base: String,
    label:      String,
    open:       bool,
}

/// Start a new `<ui-copilot>` builder with sensible defaults.
/// Base defaults to `/agent`.
pub fn copilot() -> Copilot {
    Copilot {
        agent_base: "/agent".into(),
        label:      "ERP Copilot".into(),
        open:       false,
    }
}

impl Copilot {
    /// Base URL for the agent endpoints (default `/agent`).
    /// For a tenant-scoped instance: `.agent_base("/web/acme/agent")`.
    pub fn agent_base(mut self, s: impl Into<String>) -> Self { self.agent_base = s.into(); self }

    /// Panel title shown in the drawer header.
    pub fn label(mut self, s: impl Into<String>) -> Self { self.label = s.into(); self }

    /// Start with the panel open (useful for demos).
    pub fn open(mut self) -> Self { self.open = true; self }

    // ── Backwards-compat shims (deprecated) ────────────────────────────
    // Older code called `.session_url(...)`, etc., against the v1 backend.
    // v2 collapses those into one `agent_base`. The setters below are kept
    // so the DSL still compiles for callers that hadn't migrated; they
    // simply infer the base from the URL prefix if possible.
    #[deprecated(note = "v1 SSE endpoint; use .agent_base() with the /agent/* backend")]
    pub fn session_url(self, _s: impl Into<String>) -> Self { self }
    #[deprecated(note = "v1 SSE endpoint; use .agent_base() with the /agent/* backend")]
    pub fn stream_url (self, _s: impl Into<String>) -> Self { self }
    #[deprecated(note = "v1 SSE endpoint; use .agent_base() with the /agent/* backend")]
    pub fn history_url(self, _s: impl Into<String>) -> Self { self }
}

impl Component for Copilot {
    fn render(&self) -> String {
        let mut attrs = vec![
            Attr::kv("agent-base", self.agent_base.as_str()),
            Attr::kv("label",      self.label.as_str()),
        ];
        if self.open { attrs.push(Attr::flag("open")); }
        wrap("ui-copilot", &attrs, "")
    }
}
