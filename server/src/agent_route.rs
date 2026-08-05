//! `POST /agent` — the copilot's turn endpoint.
//!
//! The `<ui-copilot>` composer POSTs the user's utterance here as JSON.
//! We wrap the currently-registered [`Agent`] and stream its output
//! through [`ui_shell::fragments_sse`] so the browser sees a series of
//! `<ui-fragment>` envelopes (chat bubbles, main-island swaps, side
//! effects) as the agent produces them.
//!
//! The route is deliberately thin — swap `StubAgent` for a real LLM
//! agent later and this file doesn't change.

use axum::{
    extract::State,
    response::{sse::Event, Sse},
    Json,
};
use futures_core::Stream;
use futures_util::StreamExt;
use std::{convert::Infallible, sync::Arc};

use agent::{Agent, AgentEvent, Turn};
use ui_shell::{Fragment, Render, Target};

/// State injected into the router — a shared, boxed agent so we can
/// swap implementations without rewriting the handler signature.
#[derive(Clone)]
pub struct AgentState {
    pub agent: Arc<dyn Agent + Send + Sync>,
}

/// The handler. Returns an SSE response whose events carry
/// `<ui-fragment>` envelopes. The client's `<ui-copilot>` component
/// pipes each `data` payload through `window.ui.applyAll(...)`.
pub async fn handler(
    State(state): State<AgentState>,
    Json(turn): Json<Turn>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Ask the agent for its event stream, then map each event to an SSE
    // event whose data is the wire-format fragment envelope.
    let events = state.agent.handle(turn).await;
    let mapped = events
        .map(|ev| Ok(agent_event_to_sse(ev)))
        .chain(futures_util::stream::once(async {
            Ok(Event::default().event("done").data(""))
        }));

    Sse::new(mapped).keep_alive(
        axum::response::sse::KeepAlive::new().interval(std::time::Duration::from_secs(15)),
    )
}

/// Convert an [`AgentEvent`] to an SSE [`Event`] carrying a
/// `<ui-fragment>` envelope. Every variant is rendered through the same
/// wire format so the browser applier only needs one code path.
fn agent_event_to_sse(ev: AgentEvent) -> Event {
    let envelope = match ev {
        AgentEvent::Fragment(f) => f.render_string(),

        // Tool call + result → simple system bubbles for now. When the
        // real copilot lands we'll swap these for dedicated tool cards.
        AgentEvent::ToolCall(tc) => Fragment::from_html(
            Target::Custom("copilot-chat".into()),
            ui_shell::Action::Append,
            format!(
                r#"<div class="ui-copilot-tool" style="font-size:11px;opacity:.65;margin:6px 0;">→ tool: {}</div>"#,
                escape(&tc.name),
            ),
        )
        .render_string(),
        AgentEvent::ToolResult(tr) => Fragment::from_html(
            Target::Custom("copilot-chat".into()),
            ui_shell::Action::Append,
            format!(
                r#"<div class="ui-copilot-tool" style="font-size:11px;opacity:.65;margin:6px 0;">← result ({}): {}</div>"#,
                if tr.ok { "ok" } else { "err" },
                escape(&tr.data.to_string()),
            ),
        )
        .render_string(),

        // Side effects use a dedicated envelope targeting the special
        // `__side_effect__` island in the client runtime.
        AgentEvent::SideEffect { kind, payload } => Fragment::from_html(
            Target::Custom("__side_effect__".into()),
            ui_shell::Action::Replace,
            // The runtime reads `kind` from the attribute and JSON-parses
            // the text content as `payload`. Encoded as an attribute on
            // the envelope so the client can dispatch without parsing.
            format!(
                r#"<ui-side-effect kind="{}">{}</ui-side-effect>"#,
                escape(&kind),
                escape(&payload.to_string()),
            ),
        )
        .render_string(),
    };

    Event::default().event("fragment").data(envelope)
}

// Tiny attribute/text escaper — no external dep.
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}
