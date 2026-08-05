//! Global command registry — every command the launcher and copilot
//! can execute, defined ONCE here.
//!
//! Adding a command is a one-liner. Grep `Command::new(` to find them all.
//!
//! Commands are grouped into three buckets by naming convention:
//!
//! * `nav.<app>`   — navigation shortcuts (auto-generated from `apps::all()`)
//! * `theme.*`     — appearance
//! * everything else — feature commands
//!
//! The registry powers:
//!   • `POST /agent` (copilot)   — via `StubAgent::from_registry`
//!   • `GET  /launcher/search`   — via the launcher endpoint

use agent::{Command, CommandRegistry};
use serde_json::json;
use ui_shell::{Action, Fragment, Target};

use crate::apps;

/// Build the app's command registry. Called once at startup.
pub fn build() -> CommandRegistry {
    let mut reg = CommandRegistry::new();

    // Auto-register a navigate command for each registered app. This
    // means every entry in `apps::all()` gets a copilot phrase like
    // "go to dashboard" AND a launcher hit — no separate wiring.
    for app in apps::all() {
        let route  = app.route.to_string();
        let label  = format!("Go to {}", app.label);
        let icon   = app.icon.to_string();
        let hint   = route.clone();
        let route2 = route.clone();

        reg = reg.add(
            Command::new(format!("nav.{}", app.id), label)
                .hint(hint)
                .icon(icon)
                .keywords(&[
                    &app.label.to_lowercase(),
                    &format!("go to {}", app.label.to_lowercase()),
                    &format!("open {}", app.label.to_lowercase()),
                    app.id,
                ])
                .handler(move |_| {
                    // A navigate side-effect makes the client fetch the
                    // route as a normal fragment nav (updates URL bar +
                    // main island in one go).
                    vec![agent::AgentEvent::SideEffect {
                        kind: "navigate".into(),
                        payload: json!({ "url": route2 }),
                    }]
                }),
        );
    }

    // ─── Theme commands ──────────────────────────────────────────────
    reg = reg
        .add(Command::new("theme.dark", "Switch to dark mode")
            .hint("Persists across reloads")
            .icon("moon")
            .keywords(&["dark", "night", "theme"])
            .handler(|_| vec![agent::AgentEvent::SideEffect {
                kind: "theme".into(),
                payload: json!({ "theme": "dark" }),
            }]))
        .add(Command::new("theme.light", "Switch to light mode")
            .hint("Persists across reloads")
            .icon("sun")
            .keywords(&["light", "day", "theme"])
            .handler(|_| vec![agent::AgentEvent::SideEffect {
                kind: "theme".into(),
                payload: json!({ "theme": "light" }),
            }]))
        .add(Command::new("theme.toggle", "Toggle theme")
            .icon("sun")
            .keywords(&["toggle", "flip", "theme"])
            .handler(|_| vec![agent::AgentEvent::SideEffect {
                kind: "theme-toggle".into(),
                payload: json!({}),
            }]));

    // ─── Feature commands ────────────────────────────────────────────
    // Convention: pushing a card into `main` is a common pattern, so we
    // build it once as a helper below and reuse.
    reg = reg
        .add(Command::new("users.add", "Add user")
            .hint("Open the new-user form")
            .icon("plus")
            .keywords(&["add", "new", "create", "user"])
            .handler(|_| vec![agent::AgentEvent::Fragment(
                Fragment::from_html(
                    Target::Main, Action::Replace,
                    r#"<div style="padding:24px;">
                         <h2 style="margin:0 0 12px;">Add user</h2>
                         <form action="/users" method="post" style="display:flex;flex-direction:column;gap:10px;max-width:360px;">
                           <input name="full_name" placeholder="Full name" required style="padding:8px 10px;border:1px solid #e5e7eb;border-radius:8px;">
                           <input name="email" type="email" placeholder="Email" required style="padding:8px 10px;border:1px solid #e5e7eb;border-radius:8px;">
                           <button type="submit" style="padding:8px 14px;border:0;border-radius:8px;background:#4f46e5;color:#fff;cursor:pointer;">Create</button>
                         </form>
                       </div>"#,
                )
            )]));

    reg
}
