//! `GET /launcher/search?q=…` — the OS-style launcher backend.
//!
//! Returns a **single HTML fragment** — the pre-formatted list of
//! grouped results. The `<ui-launcher>` client just swaps it into its
//! results panel; no JSON, no client-side templating. Fully consistent
//! with the rest of the framework's fragment protocol.
//!
//! Result groups (in display order):
//!   1. Apps       — matches against [`crate::apps::all`]
//!   2. Commands   — matches against the shared [`CommandRegistry`]
//!   3. Recent     — last N routes (kept client-side; not implemented
//!                   here yet — the client passes them via a query arg
//!                   in a future revision, or we return an empty group
//!                   the client fills in)
//!   4. Ask copilot — always the last row; Enter with no match sends
//!                    the text to `/agent` as a normal turn.

use axum::{
    extract::{Query, State},
    response::Html,
};
use serde::Deserialize;
use std::sync::Arc;

use agent::CommandRegistry;

use crate::apps;

/// Query string: `?q=whatever the user typed`.
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    #[serde(default)]
    pub q: String,
}

/// Router state: the shared command registry. Cloning is cheap
/// (`Arc<Vec<Command>>` under the hood via `CommandRegistry`'s Clone).
#[derive(Clone)]
pub struct LauncherState {
    pub registry: Arc<CommandRegistry>,
}

/// The handler. Returns raw HTML — the launcher component sets its
/// results panel's innerHTML to whatever we return.
pub async fn handler(
    State(state): State<LauncherState>,
    Query(query): Query<SearchQuery>,
) -> Html<String> {
    let q = query.q.trim();
    let mut out = String::with_capacity(1024);

    // ─── Apps ─────────────────────────────────────────────────────────
    let apps_hits: Vec<_> = if q.is_empty() {
        apps::all().iter().collect()
    } else {
        let ql = q.to_ascii_lowercase();
        apps::all()
            .iter()
            .filter(|a| {
                a.label.to_ascii_lowercase().contains(&ql)
                    || a.id.contains(&ql)
                    || a.route.contains(&ql)
            })
            .collect()
    };
    if !apps_hits.is_empty() {
        section(&mut out, "APPS");
        for app in apps_hits.iter().take(5) {
            row(&mut out, "nav", app.icon, app.label, app.route, app.route);
        }
    }

    // ─── Commands ────────────────────────────────────────────────────
    // Skip nav.* commands here — they duplicate the Apps section above.
    let cmd_hits: Vec<_> = state
        .registry
        .search(q)
        .into_iter()
        .filter(|(_, c)| !c.id.starts_with("nav."))
        .take(6)
        .collect();
    if !cmd_hits.is_empty() {
        section(&mut out, "COMMANDS");
        for (_, c) in &cmd_hits {
            row(&mut out, "cmd", &c.icon, &c.label, &c.hint, &c.id);
        }
    }

    // ─── Ask copilot fallback (always last row) ──────────────────────
    let ask_label = if q.is_empty() {
        "Ask copilot…".to_string()
    } else {
        format!("Ask copilot: “{}”", escape(q))
    };
    section(&mut out, "");
    row(&mut out, "ask", "message", &ask_label, "Enter to send", q);

    // Wrap so the launcher can select its own results container easily.
    Html(format!(r#"<div class="ui-launcher-groups">{out}</div>"#))
}

/// Emit a section header. Empty title = a divider (used above the
/// "Ask copilot" fallback so it visually separates from real matches).
fn section(out: &mut String, title: &str) {
    if title.is_empty() {
        out.push_str(r#"<div class="ui-launcher-divider"></div>"#);
    } else {
        out.push_str(&format!(
            r#"<div class="ui-launcher-section">{}</div>"#,
            escape(title)
        ));
    }
}

/// Emit one result row. `kind` is `"nav" | "cmd" | "ask"`; the client
/// dispatches by it on Enter (navigate, run command, send to copilot).
/// `payload` carries whatever the client needs to act on the row
/// (route for nav, command id for cmd, raw text for ask).
fn row(
    out: &mut String,
    kind: &str,
    icon: &str,
    label: &str,
    hint: &str,
    payload: &str,
) {
    out.push_str(&format!(
        r#"<button type="button" class="ui-launcher-row" role="option"
             data-kind="{kind}" data-payload="{payload}">
             <ui-icon name="{icon}" size="18"></ui-icon>
             <span class="ui-launcher-label">{label}</span>
             <span class="ui-launcher-hint">{hint}</span>
           </button>"#,
        kind    = escape(kind),
        payload = escape(payload),
        icon    = escape(icon),
        label   = escape(label),
        hint    = escape(hint),
    ));
}

/// Tiny attribute/text escaper.
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&'  => out.push_str("&amp;"),
            '<'  => out.push_str("&lt;"),
            '>'  => out.push_str("&gt;"),
            '"'  => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _    => out.push(ch),
        }
    }
    out
}
