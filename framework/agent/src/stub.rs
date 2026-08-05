//! A rule-based [`Agent`] that maps a fixed set of phrases to
//! deterministic responses. Zero LLM, zero external calls — the whole
//! point is to exercise the copilot pipeline end-to-end (composer →
//! POST /agent → SSE → fragment applier → visible UI change) before we
//! swap in a real model.
//!
//! ## Grammar (case-insensitive)
//!
//! | phrase                              | effect                                          |
//! |-------------------------------------|-------------------------------------------------|
//! | `go to dashboard` / `open admin`    | Navigate the `main` island to the given page    |
//! | `show users` / `list users`         | Replace `main` with a users table               |
//! | `add user`                          | Replace `main` with a create-user form          |
//! | `toggle theme` / `dark mode`        | Client side-effect switching light↔dark         |
//! | `hello` / `hi`                      | Small chat bubble in the copilot pane           |
//! | *anything else*                     | Falls back to a "didn't understand" bubble      |
//!
//! Every response is delivered as a stream of [`AgentEvent`]s. The
//! server pipes them straight through [`ui_shell::fragments_sse`].
//!
//! ## Adding a new command
//!
//! 1. Add a match arm in [`handle_one`] returning `Vec<AgentEvent>`.
//! 2. If it needs to build page content, add a helper (see [`users_table`]).
//! 3. Done — no wiring changes needed. The whole surface is behind the
//!    [`Agent`] trait, so a future `LlmAgent` slots in without touching
//!    the route or the copilot component.

use async_trait::async_trait;
use futures_core::Stream;
use serde_json::json;
use std::pin::Pin;

use lit_ui::core::Node;
use lit_ui::prelude::*;
use ui_shell::{Fragment, Target};

use crate::{Agent, AgentEvent, EventStream, Turn};

/// The rule-based agent. Stateless — a single instance can be shared
/// across the whole server.
#[derive(Debug, Default, Clone, Copy)]
pub struct StubAgent;

impl StubAgent {
    /// Convenience constructor so call sites read nicely:
    /// `let agent = StubAgent::new();`.
    pub fn new() -> Self { Self }
}

#[async_trait]
impl Agent for StubAgent {
    async fn handle(&self, turn: Turn) -> EventStream {
        let events = handle_one(&turn.utterance);
        // Convert the Vec into a Stream so the trait signature holds.
        // A future LLM agent would use `async_stream::stream!` to yield
        // incrementally as tokens arrive.
        let stream = futures_util::stream::iter(events);
        Box::pin(stream) as Pin<Box<dyn Stream<Item = AgentEvent> + Send>>
    }
}

// -----------------------------------------------------------------------------
// The command table.
//
// A tiny hand-rolled parser is fine here — the grammar is fixed and
// short, and using anything heavier obscures what's going on. Every arm
// returns a `Vec<AgentEvent>` because most commands produce more than
// one event (a chat bubble AND a fragment, or a tool-call trace).
// -----------------------------------------------------------------------------
fn handle_one(utterance: &str) -> Vec<AgentEvent> {
    let text = utterance.trim().to_ascii_lowercase();

    // 1) Navigation — "go to X" / "open X" / just "dashboard".
    if let Some(path) = navigation_target(&text) {
        return vec![
            chat_bubble(format!("Navigating to {path}")),
            AgentEvent::Fragment(Fragment::replace(Target::Main, navigate_placeholder(&path))),
            // Nudge the client to also update the URL bar so back/forward works.
            AgentEvent::SideEffect {
                kind: "navigate".into(),
                payload: json!({ "url": path }),
            },
        ];
    }

    // 2) Show users — a small table in `main`.
    if text.contains("show users") || text.contains("list users") || text == "users" {
        return vec![
            chat_bubble("Here are the users.".to_string()),
            AgentEvent::Fragment(Fragment::replace(Target::Main, users_table())),
            AgentEvent::SideEffect {
                kind: "navigate".into(),
                payload: json!({ "url": "/users" }),
            },
        ];
    }

    // 3) Add user — a form in `main`. Kept simple; the form POSTs to
    //    /users which returns a fragment on success (server work).
    if text.contains("add user") || text.contains("new user") || text.contains("create user") {
        return vec![
            chat_bubble("Opening the new-user form.".to_string()),
            AgentEvent::Fragment(Fragment::replace(Target::Main, add_user_form())),
        ];
    }

    // 4) Theme toggle — pure client side effect, no fragment.
    if text.contains("dark mode") || text.contains("dark theme") {
        return vec![
            chat_bubble("Switched to dark mode.".to_string()),
            AgentEvent::SideEffect { kind: "theme".into(), payload: json!({ "theme": "dark" }) },
        ];
    }
    if text.contains("light mode") || text.contains("light theme") {
        return vec![
            chat_bubble("Switched to light mode.".to_string()),
            AgentEvent::SideEffect { kind: "theme".into(), payload: json!({ "theme": "light" }) },
        ];
    }
    if text.contains("toggle theme") {
        // We don't know the current theme from here — send a `toggle`
        // side effect and let the client flip it. Registered in
        // ui-app-shell.js alongside the other side-effect handlers.
        return vec![
            chat_bubble("Toggling theme.".to_string()),
            AgentEvent::SideEffect { kind: "theme-toggle".into(), payload: json!({}) },
        ];
    }

    // 5) Greetings — the smallest possible reply. Useful smoke test.
    if text == "hi" || text == "hello" || text == "hey" {
        return vec![chat_bubble("Hi 👋 — try: `go to dashboard`, `show users`, `add user`, `dark mode`.".to_string())];
    }

    // 6) Fallback — polite refusal that also documents the grammar so
    //    a new user can figure out what to type.
    vec![chat_bubble(format!(
        "I didn't understand `{utterance}`. Try one of: `go to dashboard`, `open admin`, `show users`, `add user`, `dark mode`, `light mode`."
    ))]
}

/// Parse "go to X" / "open X" / bare page names into a URL.
///
/// Returns `Some("/dashboard")` etc., or `None` if the utterance is not
/// a navigation command. Central lookup table so adding a new route is
/// a one-line change.
fn navigation_target(text: &str) -> Option<String> {
    // Strip common prefixes so both "go to dashboard" and "dashboard" match.
    let stripped = text
        .strip_prefix("go to ")
        .or_else(|| text.strip_prefix("goto "))
        .or_else(|| text.strip_prefix("open "))
        .or_else(|| text.strip_prefix("navigate to "))
        .or_else(|| text.strip_prefix("show "))
        .unwrap_or(text)
        .trim();

    let path = match stripped {
        "home" | "landing" | "/"        => "/",
        "dashboard" | "dash"            => "/dashboard",
        "admin"                         => "/admin",
        "users"                         => "/users",
        _ => return None,
    };
    Some(path.to_string())
}

// -----------------------------------------------------------------------------
// Content builders (all DSL). Kept small — real pages live in
// `server/src/pages/`. These are just the fragments the agent needs to
// synthesise when it doesn't have a full page handler at hand.
// -----------------------------------------------------------------------------

/// Placeholder shown when the agent "navigates" — the client should
/// follow the accompanying `navigate` side-effect to fetch the real
/// page, but this gives us instant visible feedback in the copilot
/// pipeline in the meantime.
fn navigate_placeholder(path: &str) -> impl Component {
    card()
        .title(format!("Navigating to {path}…"))
        .add(Node::text("The copilot asked the shell to open this page. If you don't see it in a moment, click the link in the sidebar."))
}

/// A tiny users list. The real page (see `server/src/pages/users.rs`)
/// is richer; this one is just for the agent's inline `show users`.
fn users_table() -> impl Component {
    let list = column()
        .gap(Gap::Sm)
        .add(list_item("Aarav Kumar").subtitle("aarav@example.com"))
        .add(list_item("Bilal Ahmed").subtitle("bilal@example.com"))
        .add(list_item("Chitra Rao").subtitle("chitra@example.com"));

    card().title("Users").add(list)
}

/// A minimal create-user form. Posts to `/users` (which we don't
/// implement yet — this is here to show the shape).
fn add_user_form() -> impl Component {
    card()
        .title("Add user")
        .add(form()
            .action("/users")
            .method("post")
            .add(input().label("Full name").name("full_name").required())
            .add(input().label("Email").name("email").kind(InputType::Email).required())
            .add(row_actions()
                .add(button().label("Create").variant(Variant::Primary))
                .add(button().label("Cancel").variant(Variant::Secondary))))
}

/// Convenience — an [`AgentEvent`] that appends a chat bubble to the
/// copilot conversation. The wire format is a fragment targeting the
/// special `copilot-chat` island (a slot inside `<ui-copilot>`).
fn chat_bubble(text: String) -> AgentEvent {
    // The copilot component owns a scrollable chat area with slot name
    // "copilot-chat" so we can `append` bubbles without touching the
    // rest of the copilot chrome (composer, header, etc.).
    let bubble = Node::raw(format!(
        r#"<div class="ui-copilot-bubble" style="padding:10px 12px;margin:4px 0;background:var(--color-surface-2,#f5f5f7);border-radius:10px;font-size:13px;line-height:1.4;">{}</div>"#,
        html_escape(&text),
    ));
    AgentEvent::Fragment(Fragment::from_html(
        ui_shell::Target::Custom("copilot-chat".into()),
        ui_shell::Action::Append,
        bubble.render(),
    ))
}

/// Minimal HTML escaper — only what's needed for chat text. Keeps this
/// crate dependency-free (no `html-escape` etc.).
fn html_escape(s: &str) -> String {
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
