//! `/copilot/*` — mock agentic Copilot backend.
//!
//! There is **no LLM and no database** wired up yet — every scenario here is
//! a hand-rolled mock so the front-end (`<ui-copilot>`) can be exercised end
//! to end.
//!
//! Endpoints
//! ---------
//! * `POST /copilot/session` → `{"session_id": "…"}`
//! * `GET  /copilot/history/:id` → `text/html` of prior turns (in-memory).
//! * `POST /copilot/message` → `text/event-stream` streaming a chosen scenario.
//!
//! Scenarios are picked from the user prompt (case-insensitive substring
//! match against `SCENARIOS` — see below). Anything unrecognised falls back
//! to a generic "streaming echo + suggestion chips" reply so the demo always
//! shows something interesting.
//!
//! Design intent
//! -------------
//! Emit **named SSE events** so the client dispatcher is dumb:
//! * `message_start`  — server-rendered assistant bubble skeleton.
//! * `token`          — JSON `{id,text}` to append into that bubble.
//! * `tool_call`      — server-rendered "tool card" HTML.
//! * `tool_result`    — server-rendered result HTML.
//! * `step`           — JSON `{n,of,label}` for the progress bar.
//! * `ui_action`      — JSON telling the browser to `navigate`, `focus`,
//!                       `fill_form`, `highlight`, `toast`, etc.
//! * `message_end`    — JSON `{id}` — finalises the assistant turn.
//! * `error`          — JSON `{message}`.
//!
//! When we later plug in a real LLM/orchestrator, the only thing that changes
//! is the *body of the `run_scenario` future*: the event schema stays the same.

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive, Sse},
        Html, IntoResponse, Json, Response,
    },
    routing::{get, post},
    Router,
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt as _;

// ---------------------------------------------------------------------------
// State — in-memory "session store" so history hydrates across page reloads.
// ---------------------------------------------------------------------------

/// A single stored turn (either user or assistant), already rendered as HTML.
#[derive(Clone)]
struct StoredMsg {
    html: String,
}

#[derive(Clone, Default)]
pub struct CopilotState {
    inner: Arc<Mutex<HashMap<String, Vec<StoredMsg>>>>,
}

impl CopilotState {
    pub fn new() -> Self { Self::default() }

    fn append(&self, session: &str, html: String) {
        let mut g = self.inner.lock().unwrap();
        g.entry(session.to_string()).or_default().push(StoredMsg { html });
    }

    fn render_history(&self, session: &str) -> String {
        let g = self.inner.lock().unwrap();
        match g.get(session) {
            None => String::new(),
            Some(list) => list.iter().map(|m| m.html.as_str()).collect::<Vec<_>>().join(""),
        }
    }
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

/// Build the `/copilot/*` sub-router. Wire this into `build_router` with
/// `.merge(copilot::router(state))`.
pub fn router(state: CopilotState) -> Router {
    Router::new()
        .route("/copilot/session",       post(session_new))
        .route("/copilot/history/{sid}", get(history))
        .route("/copilot/message",       post(message))
        .with_state(state)
}

// ---------------------------------------------------------------------------
// POST /copilot/session
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct SessionResp { session_id: String }

async fn session_new() -> Json<SessionResp> {
    Json(SessionResp {
        session_id: format!("cs-{}", uuid::Uuid::new_v4().simple()),
    })
}

// ---------------------------------------------------------------------------
// GET /copilot/history/:sid
// ---------------------------------------------------------------------------

async fn history(State(st): State<CopilotState>, Path(sid): Path<String>) -> Html<String> {
    Html(st.render_history(&sid))
}

// ---------------------------------------------------------------------------
// POST /copilot/message  → text/event-stream
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct SendMsg {
    session_id: String,
    prompt: String,
}

async fn message(
    State(st): State<CopilotState>,
    Json(body): Json<SendMsg>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, Response> {
    if body.prompt.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "empty prompt").into_response());
    }

    // Persist the user turn so history hydrates on reload.
    st.append(
        &body.session_id,
        format!(
            r#"<div class="msg msg-user"><div class="bubble">{}</div></div>"#,
            html_escape(&body.prompt)
        ),
    );

    // 32 events buffer is plenty for the mock scenarios below.
    let (tx, rx) = mpsc::channel::<Event>(32);
    let state = st.clone();
    let sid = body.session_id.clone();
    let prompt = body.prompt.clone();

    tokio::spawn(async move {
        let em = EventEmitter { tx, state, sid };
        if let Err(e) = run_scenario(&em, &prompt).await {
            let _ = em.tx.send(sse_json("error", &serde_json::json!({"message": e}))).await;
        }
    });

    let stream = ReceiverStream::new(rx).map(Ok);
    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

// ---------------------------------------------------------------------------
// Event emitter — small helper wrapping the channel + persistence.
// ---------------------------------------------------------------------------

struct EventEmitter {
    tx: mpsc::Sender<Event>,
    state: CopilotState,
    sid: String,
}

impl EventEmitter {
    async fn send_event(&self, ev: Event) {
        let _ = self.tx.send(ev).await;
    }

    async fn sleep(&self, ms: u64) {
        tokio::time::sleep(Duration::from_millis(ms)).await;
    }

    /// Emit `message_start` with a fresh assistant bubble skeleton, storing
    /// the HTML in history so reloads can replay it. Returns the bubble id.
    async fn message_start(&self, id: &str) {
        let html = format!(
            r#"<div class="msg msg-assistant" id="{id}"><div class="bubble"></div></div>"#
        );
        self.state.append(&self.sid, html.clone());
        self.send_event(sse_raw("message_start", &html)).await;
    }

    /// Emit a token to append into the given bubble.
    async fn token(&self, id: &str, text: &str) {
        // Also append to the last stored message so history matches.
        self.state.append_token(&self.sid, id, text);
        self.send_event(sse_json("token", &serde_json::json!({"id": id, "text": text}))).await;
    }

    /// Stream a full sentence, one word at a time, into the given bubble.
    async fn stream_text(&self, id: &str, text: &str, wpm_ms: u64) {
        for word in split_keep_spaces(text) {
            self.token(id, &word).await;
            self.sleep(wpm_ms).await;
        }
    }

    async fn tool_call(&self, id: &str, name: &str, arg: &str) {
        let html = format!(
            r#"<div class="tool-card" id="{id}">🔧 <strong>{}</strong> <code>{}</code> <span class="thinking"><span></span><span></span><span></span></span></div>"#,
            html_escape(name), html_escape(arg)
        );
        self.state.append(&self.sid, html.clone());
        self.send_event(sse_raw("tool_call", &html)).await;
    }

    async fn tool_result(&self, tool_id: &str, ok: bool, summary: &str) {
        let icon = if ok { "✅" } else { "⚠️" };
        let html = format!(
            r#"<div class="tool-card" data-for="{tool_id}">{icon} {}</div>"#,
            html_escape(summary)
        );
        self.state.append(&self.sid, html.clone());
        self.send_event(sse_raw("tool_result", &html)).await;
    }

    async fn step(&self, n: u32, of: u32, label: &str) {
        self.send_event(sse_json("step",
            &serde_json::json!({"n": n, "of": of, "label": label}))).await;
    }

    async fn ui_action(&self, payload: serde_json::Value) {
        self.send_event(sse_json("ui_action", &payload)).await;
    }

    async fn end(&self, id: &str) {
        self.send_event(sse_json("message_end", &serde_json::json!({"id": id}))).await;
    }
}

// Extend CopilotState with a helper to append streaming tokens into the
// last stored assistant bubble so `/history` renders the fully-typed reply.
impl CopilotState {
    fn append_token(&self, session: &str, bubble_id: &str, text: &str) {
        let mut g = self.inner.lock().unwrap();
        if let Some(list) = g.get_mut(session) {
            // Find the bubble by id and insert text before `</div></div>`.
            for m in list.iter_mut().rev() {
                if m.html.contains(&format!(r#"id="{bubble_id}""#)) {
                    let escaped = html_escape(text);
                    if let Some(pos) = m.html.rfind("</div></div>") {
                        m.html.insert_str(pos, &escaped);
                    } else {
                        m.html.push_str(&escaped);
                    }
                    return;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Scenarios — mock agent behaviour.
// ---------------------------------------------------------------------------

async fn run_scenario(em: &EventEmitter, prompt: &str) -> Result<(), String> {
    let p = prompt.to_lowercase();
    let bubble_id = format!("m-{}", uuid::Uuid::new_v4().simple());

    // 1) Navigation intent
    if p.contains("dashboard") || p.contains("go to dashboard") || p.contains("open dashboard") {
        em.message_start(&bubble_id).await;
        em.stream_text(&bubble_id, "Sure — taking you to the Dashboard now.", 40).await;
        em.sleep(300).await;
        em.ui_action(serde_json::json!({"action":"navigate","href":"/dsl/dashboard"})).await;
        em.end(&bubble_id).await;
        return Ok(());
    }
    if p.contains("student") && (p.contains("list") || p.contains("show") || p.contains("open")) {
        em.message_start(&bubble_id).await;
        em.stream_text(&bubble_id, "Opening the Students page.", 40).await;
        em.ui_action(serde_json::json!({"action":"navigate","href":"/dsl/students"})).await;
        em.end(&bubble_id).await;
        return Ok(());
    }
    if p.contains("fees") || p.contains("invoice") || p.contains("payment") {
        em.message_start(&bubble_id).await;
        em.stream_text(&bubble_id, "Here are the fees.", 40).await;
        em.ui_action(serde_json::json!({"action":"navigate","href":"/dsl/fees"})).await;
        em.end(&bubble_id).await;
        return Ok(());
    }
    if p.contains("attendance") {
        em.message_start(&bubble_id).await;
        em.stream_text(&bubble_id, "Opening today's attendance.", 40).await;
        em.ui_action(serde_json::json!({"action":"navigate","href":"/dsl/attendance"})).await;
        em.end(&bubble_id).await;
        return Ok(());
    }

    // 2) Theme toggle
    if p.contains("dark mode") || p.contains("dark theme") {
        em.message_start(&bubble_id).await;
        em.stream_text(&bubble_id, "Switching to dark mode.", 35).await;
        em.ui_action(serde_json::json!({"action":"set_theme","theme":"dark"})).await;
        em.end(&bubble_id).await;
        return Ok(());
    }
    if p.contains("light mode") || p.contains("light theme") {
        em.message_start(&bubble_id).await;
        em.stream_text(&bubble_id, "Switching to light mode.", 35).await;
        em.ui_action(serde_json::json!({"action":"set_theme","theme":"light"})).await;
        em.end(&bubble_id).await;
        return Ok(());
    }

    // 3) Multi-step agent task — "enrol / add student"
    if p.contains("enrol") || p.contains("enroll") || (p.contains("add") && p.contains("student")) {
        em.message_start(&bubble_id).await;
        em.stream_text(
            &bubble_id,
            "On it — I'll enrol the student, apply this term's fees, and notify the parent.",
            25,
        ).await;

        em.step(1, 3, "Creating enrolment").await;
        let t1 = format!("t-{}", uuid::Uuid::new_v4().simple());
        em.tool_call(&t1, "create_enrolment", "grade=5, section=B").await;
        em.sleep(900).await;
        em.tool_result(&t1, true, "Enrolment #E-1042 created").await;

        em.step(2, 3, "Applying March fees").await;
        let t2 = format!("t-{}", uuid::Uuid::new_v4().simple());
        em.tool_call(&t2, "apply_fees", "term=March, amount=₹4,500").await;
        em.sleep(700).await;
        em.tool_result(&t2, true, "Invoice #INV-2201 issued").await;

        em.step(3, 3, "Notifying parent").await;
        let t3 = format!("t-{}", uuid::Uuid::new_v4().simple());
        em.tool_call(&t3, "send_sms", "to=guardian, template=welcome").await;
        em.sleep(500).await;
        em.tool_result(&t3, true, "SMS delivered").await;

        em.sleep(200).await;
        em.stream_text(
            &bubble_id,
            "\n\nAll done. Want me to open the new student's profile?",
            25,
        ).await;
        em.ui_action(serde_json::json!({"action":"toast","message":"Student enrolled","tone":"success"})).await;
        em.end(&bubble_id).await;
        return Ok(());
    }

    // 4) Form-fill / autofill demo
    if p.contains("fill") && p.contains("form") {
        em.message_start(&bubble_id).await;
        em.stream_text(&bubble_id, "Filling the new-student form with sample values.", 30).await;
        em.ui_action(serde_json::json!({
            "action": "fill_form",
            "selector": "ui-form, form",
            "values": {
                "fullName":     "Aarav Kumar",
                "primaryEmail": "aarav@example.com",
                "backupEmail":  "guardian@example.com",
                "grade":        "5",
            }
        })).await;
        em.ui_action(serde_json::json!({
            "action": "highlight", "selector": "ui-form, form"
        })).await;
        em.end(&bubble_id).await;
        return Ok(());
    }

    // 5) Highlight / focus a field
    if p.contains("highlight") || p.contains("focus") {
        em.message_start(&bubble_id).await;
        em.stream_text(&bubble_id, "Highlighting the first input on the page.", 30).await;
        em.ui_action(serde_json::json!({"action":"highlight","selector":"ui-input, input"})).await;
        em.ui_action(serde_json::json!({"action":"focus","selector":"ui-input, input"})).await;
        em.end(&bubble_id).await;
        return Ok(());
    }

    // 6) Help / suggestions
    if p.contains("help") || p.contains("what can you do") {
        em.message_start(&bubble_id).await;
        em.stream_text(
            &bubble_id,
            "I can navigate pages, fill forms, run multi-step actions, and stream progress. Try:",
            25,
        ).await;
        // Render clickable suggestion chips inside the assistant bubble.
        em.send_event(sse_raw("tool_result",
            r##"<div class="tool-card"><strong>Try one:</strong>
                 <a href="#" data-copilot-suggest="Open the dashboard">Open the dashboard</a> ·
                 <a href="#" data-copilot-suggest="Add a new student">Add a new student</a> ·
                 <a href="#" data-copilot-suggest="Fill the form">Fill the form</a> ·
                 <a href="#" data-copilot-suggest="Enable dark mode">Dark mode</a>
               </div>"##)).await;
        em.end(&bubble_id).await;
        return Ok(());
    }

    // 7) Default — streaming echo so the user still sees "AI-like" output.
    em.message_start(&bubble_id).await;
    let reply = format!(
        "You said: “{}”. I'm a mock agent right now — try things like: \
         “open the dashboard”, “add a student”, “fill the form”, or “dark mode”.",
        prompt
    );
    em.stream_text(&bubble_id, &reply, 20).await;
    em.end(&bubble_id).await;
    Ok(())
}

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

fn sse_raw(event: &str, data: &str) -> Event {
    Event::default().event(event).data(data)
}

fn sse_json(event: &str, v: &serde_json::Value) -> Event {
    Event::default().event(event).data(v.to_string())
}

fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&'  => out.push_str("&amp;"),
            '<'  => out.push_str("&lt;"),
            '>'  => out.push_str("&gt;"),
            '"'  => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _    => out.push(c),
        }
    }
    out
}

/// Word-by-word splitter that preserves the whitespace between words so the
/// streamed text renders with normal spacing.
fn split_keep_spaces(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_ws = false;
    for c in s.chars() {
        let ws = c.is_whitespace();
        if ws != in_ws && !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
        cur.push(c);
        in_ws = ws;
    }
    if !cur.is_empty() { out.push(cur); }
    out
}
