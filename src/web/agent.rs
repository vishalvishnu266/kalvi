//! `/agent/*` — Mock backend for the second-generation Copilot (`<ui-copilot-v2>`).
//!
//! Endpoints
//! ---------
//! * `GET  /agent/suggest?page=…`    → proactive suggestion chips (JSON list)
//! * `GET  /agent/complete?q=…`      → live autocomplete matches (JSON list)
//! * `GET  /agent/schema?tool=…`     → tool argument schema (JSON)
//! * `POST /agent/invoke  {tool,args}` → executes the tool, returns a message
//!
//! No LLM. No DB. Everything is hand-coded fixtures — the whole point is to
//! let you *feel* the UX end to end without any external dependencies. The
//! shapes here match what a real backend would emit, so swapping the mock
//! for real data later is a small change.

use axum::{
    extract::Query,
    response::sse::{Event, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt as _;

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/agent/suggest",  get(suggest))
        .route("/agent/complete", get(complete))
        .route("/agent/schema",   get(schema))
        .route("/agent/entities", get(entities))
        .route("/agent/invoke",   post(invoke))
        .route("/agent/stream",   post(stream_tool))
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A suggestion/autocomplete/follow-up item — shown as a chip or a row.
///
/// `cost` is a hint to the UI so it can show a badge — "Instant" (deterministic
/// tool, ~10ms), "LLM" (falls through to the AI, ~2s + tokens). Power users
/// learn to prefer the Instant paths.
#[derive(Serialize)]
struct Item {
    title: String,
    icon: Option<String>,
    subtitle: Option<String>,
    tool: String,
    prefill: Value,
    shortcut: Option<String>,
    /// "instant" | "llm" | "confirm" — UI renders a small badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    cost: Option<String>,
}

impl Item {
    fn new(title: &str, icon: &str, tool: &str, prefill: Value) -> Self {
        Self {
            title: title.into(),
            icon: Some(icon.into()),
            subtitle: None,
            tool: tool.into(),
            prefill,
            shortcut: None,
            cost: Some("instant".into()),
        }
    }
    fn sub(mut self, s: &str) -> Self { self.subtitle = Some(s.into()); self }
    #[allow(dead_code)]
    fn cost(mut self, c: &str) -> Self { self.cost = Some(c.into()); self }
}

// ---------------------------------------------------------------------------
// GET /agent/suggest  → proactive chips shown above the input
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct SuggestQ { page: Option<String> }

async fn suggest(Query(q): Query<SuggestQ>) -> Json<Vec<Item>> {
    // Personalise by current page — the biggest single UX win.
    let page = q.page.unwrap_or_default();
    let items = if page.contains("attendance") {
        vec![
            Item::new("Mark today's attendance", "✅", "attendance.mark",
                json!({"date": "today"})).sub("Grade 5-B pending"),
            Item::new("Yesterday's absentees",  "📋", "attendance.absentees",
                json!({"date": "yesterday"})),
            Item::new("Attendance report — March", "📊", "attendance.report",
                json!({"month": "march"})),
        ]
    } else if page.contains("fees") {
        vec![
            Item::new("Overdue fees",            "💰", "fees.overdue", json!({})),
            Item::new("Charge March music fee",  "🎵", "fees.charge",
                json!({"category": "music", "term": "march"})),
            Item::new("Fee collection summary",  "📈", "fees.summary", json!({})),
            Item::new("Send payment reminders",  "📩", "fees.remind",   json!({})),
        ]
    } else if page.contains("student") {
        vec![
            Item::new("Add a new student",   "👤", "students.enrol",   json!({})),
            Item::new("Find a student",      "🔎", "students.find",    json!({})),
            Item::new("Grade 5 roster",      "📋", "students.list",    json!({"grade": "5"})),
        ]
    } else {
        // Dashboard / any other page — the "top of mind" for a school admin.
        vec![
            Item::new("Mark today's attendance", "✅", "attendance.mark", json!({"date":"today"})),
            Item::new("Overdue fees",            "💰", "fees.overdue",    json!({})),
            Item::new("Add a new student",       "👤", "students.enrol",  json!({})),
            Item::new("Today's timetable",       "🗓️", "timetable.today", json!({})),
            Item::new("Open Dashboard",          "🏠", "core.navigate",
                json!({"href":"/dsl/dashboard"})),
            Item::new("Dark mode",               "🌙", "core.set_theme",
                json!({"theme":"dark"})),
        ]
    };
    Json(items)
}

// ---------------------------------------------------------------------------
// GET /agent/complete?q=…  → live matches as the user types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct CompleteQ { q: String }

async fn complete(Query(cq): Query<CompleteQ>) -> Json<Vec<Item>> {
    // In a real system: fuzzy-match against tool registry + entity index.
    // Here: hand-rolled patterns to demonstrate the UX faithfully.
    let q = cq.q.to_lowercase();
    let mut out: Vec<Item> = Vec::new();

    // Rule-based intents (would come from your ToolSpec.keywords in real code)
    let candidates: Vec<(&str, Item)> = vec![
        ("attendance mark absent present class today",
            Item::new("Mark today's attendance", "✅", "attendance.mark",
                json!({"date":"today"})).sub("Choose a class")),
        ("attendance report month percent",
            Item::new("Attendance report", "📊", "attendance.report", json!({}))
                .sub("For a month or class")),
        ("absentee absent yesterday who missing",
            Item::new("Who was absent yesterday?", "📋", "attendance.absentees",
                json!({"date":"yesterday"}))),
        ("fees fee bill invoice due overdue pending",
            Item::new("Overdue fees", "💰", "fees.overdue", json!({}))
                .sub("Students with pending dues")),
        ("charge collect fee march music sports transport",
            Item::new("Charge a fee category", "🧾", "fees.charge", json!({}))
                .sub("Choose category, term, and class")),
        ("remind reminder sms email payment fee",
            Item::new("Send fee reminders", "📩", "fees.remind", json!({}))
                .sub("SMS/email to parents with overdue balance")),
        ("student add new enrol enroll admit admission",
            Item::new("Add a new student", "👤", "students.enrol", json!({}))
                .sub("Enrol into a class")),
        ("student find search lookup profile",
            Item::new("Find a student", "🔎", "students.find", json!({}))
                .sub("By name, grade, or ID")),
        ("dashboard home summary overview today",
            Item::new("Open dashboard", "🏠", "core.navigate",
                json!({"href":"/dsl/dashboard"}))),
        ("timetable schedule today class period",
            Item::new("Today's timetable", "🗓️", "timetable.today", json!({}))),
        ("dark theme night mode",
            Item::new("Enable dark mode", "🌙", "core.set_theme",
                json!({"theme":"dark"}))),
        ("light theme day mode",
            Item::new("Enable light mode", "☀️", "core.set_theme",
                json!({"theme":"light"}))),
        ("help commands what can you do",
            Item::new("What can I do here?", "❓", "meta.help", json!({}))),
    ];

    // Score each candidate; keep those with a positive score.
    let mut scored: Vec<(i32, Item)> = candidates
        .into_iter()
        .filter_map(|(keywords, item)| {
            let s = fuzzy_score(&q, keywords) + fuzzy_score(&q, &item.title);
            if s > 0 { Some((s, item)) } else { None }
        })
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    out.extend(scored.into_iter().take(6).map(|(_, i)| i));

    // Did-you-mean: if we had few or no direct tool matches, run a cheap
    // fuzzy search over student names/IDs so typing "aar" surfaces
    // "Open Aarav Kumar's profile" even before any tool matches.
    if out.len() < 3 && cq.q.trim().len() >= 2 {
        let hits = fuzzy_students(&cq.q);
        for (label, id) in hits.into_iter().take(3) {
            out.push(Item {
                title: format!("Open {}'s profile", label),
                icon: Some("👤".into()),
                subtitle: Some(format!("Student {}", id)),
                tool: "students.open".into(),
                prefill: json!({"student_id": id, "name": label}),
                shortcut: None,
                cost: Some("instant".into()),
            });
        }
    }

    // Always append the "Ask the assistant" fallback so nothing is a dead end.
    out.push(Item {
        title: format!("Ask the assistant about \"{}\"", cq.q),
        icon: Some("💬".into()),
        subtitle: Some("Uses AI — takes a couple of seconds".into()),
        tool: "ask".into(),
        prefill: json!({"text": cq.q}),
        shortcut: None,
        cost: Some("llm".into()),
    });
    Json(out)
}

/// Cheap fuzzy student search — mock analog of a real DB search.
fn fuzzy_students(q: &str) -> Vec<(String, String)> {
    let students: &[(&str, &str)] = &[
        ("Aarav Kumar",   "S-42"),
        ("Priya Shah",    "S-51"),
        ("Rohan Mehta",   "S-89"),
        ("Ishaan Rao",    "S-93"),
        ("Ananya Patel",  "S-15"),
        ("Vivaan Singh",  "S-80"),
        ("Kabir Nair",    "S-101"),
    ];
    let ql = q.to_lowercase();
    let mut scored: Vec<(i32, (String, String))> = students.iter()
        .filter_map(|(name, id)| {
            let s = fuzzy_score(&ql, name) + fuzzy_score(&ql, id);
            if s > 0 { Some((s, (name.to_string(), id.to_string()))) } else { None }
        })
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, x)| x).collect()
}

/// Tiny fuzzy scorer — substring hits + subsequence hits. Enough for the demo.
fn fuzzy_score(q: &str, hay: &str) -> i32 {
    if q.is_empty() { return 0; }
    let h = hay.to_lowercase();
    // Explicit type annotation — needed so `saturating_sub` below has a
    // concrete integer type to dispatch on (was E0689 otherwise).
    let mut score: i32 = 0;
    for word in q.split_whitespace() {
        if word.len() < 2 { continue; }
        if h.contains(word) {
            score += 40 + if h.starts_with(word) { 20 } else { 0 };
        } else {
            // subsequence check — each hit adds a little
            let mut qi = 0usize;
            let wb = word.as_bytes();
            for c in h.bytes() {
                if qi < wb.len() && c == wb[qi] { qi += 1; score += 1; }
            }
            if qi < wb.len() { score = score.saturating_sub(3); }
        }
    }
    score
}

// ---------------------------------------------------------------------------
// GET /agent/schema?tool=…  → tool argument schema for inline forms
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct SchemaQ { tool: String }

/// Human-friendly field descriptor used by the inline form renderer.
#[derive(Serialize)]
struct Field {
    name: String,
    label: String,
    #[serde(rename = "type")]
    kind: String,                       // "text" | "number" | "date" ...
    required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help: Option<String>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    enum_options: Option<Vec<EnumOpt>>,
}

#[derive(Serialize)]
struct EnumOpt { value: String, label: String }

#[derive(Serialize)]
struct SchemaOut {
    /// List of required field names (client uses to short-circuit).
    required: Vec<String>,
    /// Full field list (order matters — this is the form order).
    fields: Vec<Field>,
    /// If true, the client will show a confirmation card before invoking.
    mutating: bool,
    /// If true, the client will route this tool through the SSE `/agent/stream`
    /// endpoint instead of the one-shot `/agent/invoke`, and render progress
    /// (step bar, tool cards, streamed tokens) as it arrives.
    streaming: bool,
}

async fn schema(Query(q): Query<SchemaQ>) -> Json<SchemaOut> {
    let s = match q.tool.as_str() {
        "attendance.mark" => SchemaOut {
            required: vec!["class_id".into(), "date".into()],
            mutating: true,
            streaming: false,
            fields: vec![
                Field {
                    name: "class_id".into(), label: "For which class?".into(),
                    kind: "select".into(), required: true, placeholder: None, help: None,
                    enum_options: Some(vec![
                        EnumOpt { value: "5A".into(), label: "Grade 5-A (32 students)".into() },
                        EnumOpt { value: "5B".into(), label: "Grade 5-B (30 students)".into() },
                        EnumOpt { value: "6A".into(), label: "Grade 6-A (28 students)".into() },
                        EnumOpt { value: "6B".into(), label: "Grade 6-B (29 students)".into() },
                    ]),
                },
                Field {
                    name: "date".into(), label: "Date".into(),
                    kind: "date".into(), required: true,
                    placeholder: Some("today".into()),
                    help: Some("Defaults to today.".into()),
                    enum_options: None,
                },
            ],
        },
        "attendance.report" => SchemaOut {
            required: vec!["month".into()],
            mutating: false,
            streaming: false,
            fields: vec![
                Field {
                    name: "month".into(), label: "Month".into(),
                    kind: "select".into(), required: true, placeholder: None, help: None,
                    enum_options: Some(month_options()),
                },
                Field {
                    name: "grade".into(), label: "Grade (optional)".into(),
                    kind: "select".into(), required: false, placeholder: None,
                    help: Some("Leave blank for all grades.".into()),
                    enum_options: Some(vec![
                        EnumOpt { value: "".into(),  label: "All grades".into() },
                        EnumOpt { value: "5".into(), label: "Grade 5".into() },
                        EnumOpt { value: "6".into(), label: "Grade 6".into() },
                    ]),
                },
            ],
        },
        "fees.charge" => SchemaOut {
            required: vec!["category".into(), "term".into(), "grade".into()],
            mutating: true,
            // Streaming: charging a grade takes multiple visible steps
            // (issue invoices → generate PDFs → schedule reminders), so we
            // route through /agent/stream and animate the progress bar.
            streaming: true,
            fields: vec![
                Field {
                    name: "category".into(), label: "Fee category".into(),
                    kind: "select".into(), required: true, placeholder: None, help: None,
                    enum_options: Some(vec![
                        EnumOpt { value: "tuition".into(),   label: "Tuition".into() },
                        EnumOpt { value: "music".into(),     label: "Music class".into() },
                        EnumOpt { value: "sports".into(),    label: "Sports".into() },
                        EnumOpt { value: "transport".into(), label: "Transport".into() },
                    ]),
                },
                Field {
                    name: "term".into(), label: "Term".into(),
                    kind: "select".into(), required: true, placeholder: None, help: None,
                    enum_options: Some(month_options()),
                },
                Field {
                    name: "grade".into(), label: "For which grade?".into(),
                    kind: "select".into(), required: true, placeholder: None, help: None,
                    enum_options: Some(vec![
                        EnumOpt { value: "5".into(), label: "Grade 5 (62 students)".into() },
                        EnumOpt { value: "6".into(), label: "Grade 6 (57 students)".into() },
                        EnumOpt { value: "all".into(), label: "All grades (250 students)".into() },
                    ]),
                },
            ],
        },
        "students.enrol" => SchemaOut {
            required: vec!["full_name".into(), "grade".into(), "guardian_phone".into()],
            mutating: true,
            // Multi-step: create profile → assign roll no → notify guardian
            // → generate ID card. Perfect fit for streaming UX.
            streaming: true,
            fields: vec![
                Field {
                    name: "full_name".into(), label: "Student full name".into(),
                    kind: "text".into(), required: true,
                    placeholder: Some("e.g. Aarav Kumar".into()), help: None,
                    enum_options: None,
                },
                Field {
                    name: "grade".into(), label: "Enrol into grade".into(),
                    kind: "select".into(), required: true, placeholder: None, help: None,
                    enum_options: Some(vec![
                        EnumOpt { value: "5".into(), label: "Grade 5".into() },
                        EnumOpt { value: "6".into(), label: "Grade 6".into() },
                        EnumOpt { value: "7".into(), label: "Grade 7".into() },
                    ]),
                },
                Field {
                    name: "guardian_phone".into(), label: "Guardian phone".into(),
                    kind: "text".into(), required: true,
                    placeholder: Some("+91 98xxx xxxxx".into()),
                    help: Some("We'll send the welcome SMS here.".into()),
                    enum_options: None,
                },
            ],
        },
        "students.find" => SchemaOut {
            required: vec!["query".into()],
            mutating: false,
            streaming: false,
            fields: vec![Field {
                name: "query".into(), label: "Search by name, grade, or ID".into(),
                kind: "text".into(), required: true,
                placeholder: Some("e.g. Aarav, or 5-B, or S-42".into()),
                help: None, enum_options: None,
            }],
        },
        // A tool that takes a resolved @mention and does something with it.
        // Demonstrates the mention-driven flow (see /agent/entities).
        "student.remind_fees" => SchemaOut {
            required: vec!["student_id".into()],
            mutating: true,
            streaming: false,
            fields: vec![Field {
                name: "student_id".into(), label: "Student".into(),
                kind: "mention".into(), required: true,
                placeholder: Some("Type @ to search…".into()),
                help: Some("The parent will receive an SMS + email.".into()),
                enum_options: None,
            }],
        },

        "demo.bulk_import" => SchemaOut {
            required: vec![],
            mutating: true,
            streaming: true,
            fields: vec![],
        },

        _ => SchemaOut { required: vec![], fields: vec![], mutating: false, streaming: false },
    };
    Json(s)
}

// ---------------------------------------------------------------------------
// GET /agent/entities?type=…&q=…  → typeahead for @-mentions
// ---------------------------------------------------------------------------
//
// This is the mock equivalent of hitting your database's entity index.
// Filter by `type` (student | staff | class), narrow by `q` (fuzzy).
// In production this would be an SQL query or a small FTS/index lookup.

#[derive(Deserialize)]
struct EntitiesQ {
    /// Entity type: "student" | "staff" | "class". Empty = all types.
    #[serde(default, rename = "type")]
    kind: String,
    /// Free-text query. Empty = "top N recent".
    #[serde(default)]
    q: String,
}

#[derive(Serialize)]
struct Entity {
    /// Stable ID the tool will receive as the arg value (e.g. "S-42").
    id: String,
    /// User-facing label shown in the popover and rendered as a chip.
    label: String,
    /// One-line context ("Grade 5-B · Guardian: Rakesh Kumar").
    subtitle: String,
    /// Entity type for filtering + icon.
    kind: String,
    /// Optional emoji rendered in the popover.
    icon: String,
}

async fn entities(Query(q): Query<EntitiesQ>) -> Json<Vec<Entity>> {
    // Hand-rolled fixture — mirror your real domain so the demo feels alive.
    let all: Vec<Entity> = vec![
        // Students
        Entity { id: "S-42".into(),  label: "Aarav Kumar".into(),
                 subtitle: "Grade 5-B · Guardian: Rakesh Kumar".into(),
                 kind: "student".into(), icon: "👦".into() },
        Entity { id: "S-51".into(),  label: "Priya Shah".into(),
                 subtitle: "Grade 5-B · Guardian: Meena Shah".into(),
                 kind: "student".into(), icon: "👧".into() },
        Entity { id: "S-89".into(),  label: "Rohan Mehta".into(),
                 subtitle: "Grade 6-A · Guardian: Anil Mehta".into(),
                 kind: "student".into(), icon: "👦".into() },
        Entity { id: "S-93".into(),  label: "Ishaan Rao".into(),
                 subtitle: "Grade 6-B · ₹12,500 overdue".into(),
                 kind: "student".into(), icon: "👦".into() },
        Entity { id: "S-15".into(),  label: "Ananya Patel".into(),
                 subtitle: "Grade 5-A · Perfect attendance".into(),
                 kind: "student".into(), icon: "👧".into() },
        Entity { id: "S-80".into(),  label: "Vivaan Singh".into(),
                 subtitle: "Grade 6-A".into(),
                 kind: "student".into(), icon: "👦".into() },
        Entity { id: "S-101".into(), label: "Kabir Nair".into(),
                 subtitle: "Grade 7-A · Attendance 73%".into(),
                 kind: "student".into(), icon: "👦".into() },
        // Staff
        Entity { id: "T-4".into(),  label: "Ms. Rao".into(),
                 subtitle: "Maths · Class teacher 5-B".into(),
                 kind: "staff".into(), icon: "👩‍🏫".into() },
        Entity { id: "T-7".into(),  label: "Mr. Sharma".into(),
                 subtitle: "English · Class teacher 6-A".into(),
                 kind: "staff".into(), icon: "👨‍🏫".into() },
        Entity { id: "T-11".into(), label: "Mr. D'Souza".into(),
                 subtitle: "Music".into(),
                 kind: "staff".into(), icon: "👨‍🏫".into() },
        // Classes
        Entity { id: "C-5A".into(), label: "Grade 5-A".into(),
                 subtitle: "32 students · Teacher: Ms. Iyer".into(),
                 kind: "class".into(), icon: "🏫".into() },
        Entity { id: "C-5B".into(), label: "Grade 5-B".into(),
                 subtitle: "30 students · Teacher: Ms. Rao".into(),
                 kind: "class".into(), icon: "🏫".into() },
        Entity { id: "C-6A".into(), label: "Grade 6-A".into(),
                 subtitle: "28 students · Teacher: Mr. Sharma".into(),
                 kind: "class".into(), icon: "🏫".into() },
    ];

    let ql = q.q.to_lowercase();
    let kf = q.kind.to_lowercase();
    let mut scored: Vec<(i32, Entity)> = all
        .into_iter()
        .filter(|e| kf.is_empty() || e.kind == kf)
        .filter_map(|e| {
            let s = if ql.is_empty() { 1 }
                    else { fuzzy_score(&ql, &e.label) + fuzzy_score(&ql, &e.subtitle) / 2 };
            if s > 0 { Some((s, e)) } else { None }
        })
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    Json(scored.into_iter().take(8).map(|(_, e)| e).collect())
}

fn month_options() -> Vec<EnumOpt> {
    ["January","February","March","April","May","June",
     "July","August","September","October","November","December"]
        .iter()
        .map(|m| EnumOpt { value: m.to_lowercase(), label: (*m).into() })
        .collect()
}

// ---------------------------------------------------------------------------
// POST /agent/invoke  → execute the tool
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct InvokeReq {
    tool: String,
    #[serde(default)]
    args: Value,
}

async fn invoke(Json(req): Json<InvokeReq>) -> Json<Value> {
    let a = req.args;
    let out = match req.tool.as_str() {

        // ── Attendance ─────────────────────────────────────────────
        "attendance.mark" => {
            let cls = a.get("class_id").and_then(|v| v.as_str()).unwrap_or("?");
            let date = a.get("date").and_then(|v| v.as_str()).unwrap_or("today");
            json!({
                "kind": "text",
                "text": format!("✅ Attendance sheet ready for Grade {cls} on {date}. \
                                 32 present, 4 absent (mock)."),
                "followups": [
                    { "title": "Send absence SMS to parents", "icon": "📩",
                      "tool": "attendance.notify_absent",
                      "prefill": {"class_id": cls, "date": date} },
                    { "title": "See yesterday's attendance", "icon": "📋",
                      "tool": "attendance.absentees",
                      "prefill": {"date": "yesterday"} },
                ]
            })
        }

        "attendance.absentees" => json!({
            "kind": "table",
            "title": "Absentees — yesterday",
            "columns": [
                {"key":"student", "label":"Student"},
                {"key":"grade",   "label":"Grade"},
                {"key":"reason",  "label":"Reason"},
                {"key":"status",  "label":"Status", "badge": true},
            ],
            "rows": [
                {"student":"Aarav Kumar", "grade":"5-B", "reason":"Sick",         "status":"info"},
                {"student":"Priya Shah",  "grade":"5-B", "reason":"—",            "status":"due"},
                {"student":"Rohan Mehta", "grade":"6-A", "reason":"Family event", "status":"info"},
                {"student":"Ishaan Rao",  "grade":"6-B", "reason":"—",            "status":"due"},
            ],
            "followups": [
                { "title": "Notify parents of unexcused absences", "icon": "📩",
                  "tool": "attendance.notify_absent", "prefill": {} },
            ]
        }),

        "attendance.report" => {
            let month = a.get("month").and_then(|v| v.as_str()).unwrap_or("this month");
            json!({
                "kind": "stat",
                "title": format!("Attendance — {}", month),
                "stats": [
                    {"label":"Overall %","value":"94.2%","hint":"↑ 1.4 vs last month"},
                    {"label":"Perfect attendance","value":"38 students"},
                    {"label":"Below 75%","value":"6 students","hint":"needs follow-up"},
                ],
                "followups": [
                    { "title": "See students below 75%", "icon": "⚠️",
                      "tool": "attendance.at_risk", "prefill": {"threshold": 75} },
                    { "title": "Compare with February", "icon": "📊",
                      "tool": "attendance.compare",
                      "prefill": {"a": month, "b": "february"} },
                ]
            })
        }

        "attendance.at_risk" => json!({
            "kind": "table",
            "title": "Students below 75% attendance",
            "columns": [
                {"key":"name", "label":"Student"},
                {"key":"grade", "label":"Grade"},
                {"key":"pct", "label":"%"},
            ],
            "rows": [
                {"name":"Ishaan Rao",  "grade":"6-B", "pct":"68%"},
                {"name":"Priya Shah",  "grade":"5-B", "pct":"71%"},
                {"name":"Kabir Nair",  "grade":"7-A", "pct":"73%"},
            ],
        }),

        "attendance.notify_absent" => json!({
            "kind": "text",
            "text": "📩 Sent 4 SMS + 4 email notifications to guardians (mock).",
            "side_effects": [
                { "kind": "toast", "tone": "success", "message": "Guardians notified — 4 SMS + 4 emails" },
            ],
        }),

        "attendance.compare" => json!({
            "kind": "stat",
            "stats": [
                {"label":"March %", "value":"94.2%"},
                {"label":"February %", "value":"92.8%"},
                {"label":"Change", "value":"↑ 1.4"},
            ],
        }),

        // ── Fees ──────────────────────────────────────────────────
        "fees.overdue" => json!({
            "kind": "table",
            "title": "Overdue fees (top 5 by amount)",
            "columns": [
                {"key":"student", "label":"Student"},
                {"key":"grade",   "label":"Grade"},
                {"key":"amount",  "label":"Amount"},
                {"key":"days",    "label":"Days overdue"},
                {"key":"status",  "label":"Status", "badge": true},
            ],
            "rows": [
                {"student":"Ishaan Rao",  "grade":"6-B", "amount":"₹12,500", "days":45, "status":"due"},
                {"student":"Priya Shah",  "grade":"5-B", "amount":"₹8,200",  "days":32, "status":"due"},
                {"student":"Kabir Nair",  "grade":"7-A", "amount":"₹6,750",  "days":18, "status":"due"},
                {"student":"Ananya Patel","grade":"5-A", "amount":"₹4,500",  "days":10, "status":"info"},
                {"student":"Vivaan Singh","grade":"6-A", "amount":"₹3,200",  "days":6,  "status":"info"},
            ],
            "followups": [
                { "title": "Send reminders to all", "icon": "📩",
                  "tool": "fees.remind",  "prefill": {"scope":"overdue"} },
                { "title": "Waive Ishaan's fee",    "icon": "🎗",
                  "tool": "fees.waive",   "prefill": {"student":"Ishaan Rao","amount":12500} },
            ]
        }),

        "fees.charge" => {
            let cat  = a.get("category").and_then(|v| v.as_str()).unwrap_or("tuition");
            let term = a.get("term").and_then(|v| v.as_str()).unwrap_or("this term");
            let grade = a.get("grade").and_then(|v| v.as_str()).unwrap_or("all");
            let n = if grade == "all" { 250 } else if grade == "5" { 62 } else { 57 };
            json!({
                "kind": "text",
                "text": format!("🧾 Issued {n} invoices for {cat} — {term} — Grade {grade}. \
                                 Total billed: ₹{}. Reminders will go out tomorrow.",
                                fmt_money(n * 4500)),
                "followups": [
                    { "title": "Download invoice PDFs", "icon": "📎",
                      "tool": "fees.export", "prefill": {"format":"pdf"} },
                    { "title": "Send now instead of tomorrow", "icon": "📩",
                      "tool": "fees.remind", "prefill": {"scope":"just_charged"} },
                ]
            })
        }

        "fees.remind" => json!({
            "kind": "text",
            "text": "📩 Sent 47 SMS + 47 email reminders. Delivery report will be in the inbox in ~5 min.",
            "side_effects": [
                { "kind": "toast", "tone": "success", "message": "📩 47 SMS + 47 emails queued" },
            ],
        }),

        "fees.summary" => json!({
            "kind": "stat",
            "title": "Fee collection — this term",
            "stats": [
                {"label":"Collected", "value":"₹18.4L", "hint":"of ₹22.1L billed"},
                {"label":"% collected", "value":"83%",   "hint":"↑ 3 vs last term"},
                {"label":"Overdue", "value":"₹3.7L",     "hint":"47 students"},
            ],
            "followups": [
                { "title": "See overdue students", "icon": "💰",
                  "tool": "fees.overdue", "prefill": {} },
            ]
        }),

        "fees.waive" => {
            let student = a.get("student").and_then(|v| v.as_str()).unwrap_or("student");
            json!({ "kind": "text",
                    "text": format!("🎗 Fee waived for {student}. Logged to audit trail.") })
        }

        "fees.export" => json!({
            "kind": "text",
            "text": "📎 Preparing 250 PDFs — you'll get a download link in ~30 seconds (mock).",
            "side_effects": [
                { "kind": "toast", "tone": "info", "message": "📎 PDF export started (250 files)" },
            ],
        }),

        // ── Students ──────────────────────────────────────────────
        "students.enrol" => {
            let name = a.get("full_name").and_then(|v| v.as_str()).unwrap_or("student");
            let grade = a.get("grade").and_then(|v| v.as_str()).unwrap_or("?");
            json!({
                "kind": "text",
                "text": format!("🎉 {name} enrolled into Grade {grade}. \
                                 Student ID assigned: S-{}. Welcome SMS sent to guardian.",
                                fake_id(name)),
                "followups": [
                    { "title": "Apply term fees now", "icon": "💰",
                      "tool": "fees.charge",
                      "prefill": {"category":"tuition","term":"march","grade": grade} },
                    { "title": "Open the new student's profile", "icon": "👤",
                      "tool": "core.navigate",
                      "prefill": {"href":"/dsl/students"} },
                ]
            })
        }

        // Direct open-a-profile shortcut, from the did-you-mean fallback.
        "students.open" => {
            let id = a.get("student_id").and_then(|v| v.as_str()).unwrap_or("?");
            let name = a.get("name").and_then(|v| v.as_str()).unwrap_or("Student");
            json!({
                "kind": "text",
                "text": format!("Opening {name}'s profile ({id})."),
                "ui_action": { "action": "navigate", "href": "/dsl/students" },
            })
        }

        "students.find" => {
            let q = a.get("query").and_then(|v| v.as_str()).unwrap_or("");
            let q_low = q.to_lowercase();
            let all = vec![
                ("Aarav Kumar",   "5-B", "S-42"),
                ("Priya Shah",    "5-B", "S-51"),
                ("Rohan Mehta",   "6-A", "S-89"),
                ("Ishaan Rao",    "6-B", "S-93"),
                ("Ananya Patel",  "5-A", "S-15"),
                ("Vivaan Singh",  "6-A", "S-80"),
                ("Kabir Nair",    "7-A", "S-101"),
            ];
            let matches: Vec<_> = all
                .into_iter()
                .filter(|(n, g, id)|
                    n.to_lowercase().contains(&q_low) ||
                    g.to_lowercase().contains(&q_low) ||
                    id.to_lowercase().contains(&q_low)
                )
                .collect();
            if matches.is_empty() {
                return Json(json!({
                    "kind": "text",
                    "text": format!("No students matched \"{}\". Try a shorter name or a grade like \"5-B\".", q),
                }));
            }
            json!({
                "kind": "table",
                "title": format!("Students matching \"{}\"", q),
                "columns": [
                    {"key":"name","label":"Name"},
                    {"key":"grade","label":"Grade"},
                    {"key":"id","label":"ID"},
                ],
                "rows": matches.into_iter().map(|(n,g,i)|
                    json!({"name":n, "grade":g, "id":i})
                ).collect::<Vec<_>>(),
            })
        }

        "students.list" => json!({
            "kind": "table",
            "title": "Grade 5 roster",
            "columns": [
                {"key":"name","label":"Name"},
                {"key":"section","label":"Section"},
                {"key":"guardian","label":"Guardian"},
            ],
            "rows": [
                {"name":"Aarav Kumar",   "section":"B", "guardian":"Rakesh Kumar"},
                {"name":"Priya Shah",    "section":"B", "guardian":"Meena Shah"},
                {"name":"Ananya Patel",  "section":"A", "guardian":"Suresh Patel"},
            ],
        }),

        // ── Meta / core ───────────────────────────────────────────
        // Client-driving tools: return `ui_action` so the browser executes it.
        // The `text` bubble is still shown so the user gets feedback.
        "core.navigate" => {
            let href = a.get("href").and_then(|v| v.as_str()).unwrap_or("/");
            json!({
                "kind": "text",
                "text": format!("🧭 Taking you to {}", href),
                "ui_action": { "action": "navigate", "href": href },
            })
        }

        "core.set_theme" => {
            let theme = a.get("theme").and_then(|v| v.as_str()).unwrap_or("light");
            json!({
                "kind": "text",
                "text": format!("🎨 Switched to {} mode", theme),
                "ui_action": { "action": "set_theme", "theme": theme },
            })
        }

        "core.toast" => {
            let msg = a.get("message").and_then(|v| v.as_str()).unwrap_or("Hello");
            json!({
                "kind": "text",
                "text": format!("🔔 {}", msg),
                "ui_action": { "action": "toast", "message": msg, "tone": "info" },
            })
        }

        "timetable.today" => json!({
            "kind": "table",
            "title": "Today's timetable — Grade 5-B",
            "columns": [
                {"key":"period","label":"Period"},
                {"key":"subject","label":"Subject"},
                {"key":"teacher","label":"Teacher"},
            ],
            "rows": [
                {"period":"1 (08:30)","subject":"Maths",   "teacher":"Ms. Rao"},
                {"period":"2 (09:20)","subject":"English", "teacher":"Mr. Sharma"},
                {"period":"3 (10:10)","subject":"Science", "teacher":"Ms. Iyer"},
                {"period":"4 (11:20)","subject":"History", "teacher":"Ms. Nair"},
                {"period":"5 (12:10)","subject":"Music",   "teacher":"Mr. D'Souza"},
            ],
        }),

        "meta.help" => json!({
            "kind": "text",
            "text": "You can ask me things like: \"mark today's attendance\", \
                     \"show overdue fees\", \"add a new student\", \"who was absent yesterday\", \
                     or tap any suggestion above. On desktop, press Ctrl/Cmd+J anywhere to open this panel.",
            "followups": [
                { "title": "Mark today's attendance", "icon": "✅",
                  "tool": "attendance.mark", "prefill": {"date":"today"} },
                { "title": "Overdue fees", "icon": "💰",
                  "tool": "fees.overdue", "prefill": {} },
            ],
        }),

        // ── Freeform fallback (would be the LLM path in real code) ──
        "ask" => {
            let text = a.get("text").and_then(|v| v.as_str()).unwrap_or("");
            json!({
                "kind": "text",
                "text": format!("🤖 I heard: \"{}\". In a real deployment this would go to the AI. \
                                 For the demo, try one of the shortcuts on the right.", text),
                "followups": [
                    { "title": "What can I do here?", "icon": "❓",
                      "tool": "meta.help", "prefill": {} },
                ]
            })
        }

        // Mention-driven tool — receives a resolved entity ID.
        "student.remind_fees" => {
            let id = a.get("student_id").and_then(|v| v.as_str()).unwrap_or("?");
            json!({
                "kind": "text",
                "text": format!("📩 Fee reminder sent to guardian of {id} (SMS + email)."),
                "side_effects": [
                    { "kind": "toast", "tone": "success",
                      "message": format!("Reminder sent to guardian of {}", id) },
                ],
            })
        }

        _ => json!({
            "kind": "text",
            "text": format!("Tool \"{}\" is not wired in the mock backend yet.", req.tool),
        }),
    };

    Json(out)
}

// ---------------------------------------------------------------------------
// POST /agent/stream — SSE: multi-step tools that render progress live
// ---------------------------------------------------------------------------
//
// UX-wise this is what the user experiences when a tool takes >200 ms and
// has visible sub-steps. The client renders a running step bar + tool
// cards + streamed final text, then a "message_end" marks the turn done
// so the connection closes (no long-lived sockets).
//
// Event vocabulary (mirrors what /copilot/message emits, so the client
// can share code):
//   step         { "n":1, "of":3, "label":"…" }
//   tool_call    <html card fragment>
//   tool_result  <html card fragment>
//   token        { "id":"m-…", "text":"…" }
//   message_end  { "id":"m-…" }
//   error        { "message":"…" }

async fn stream_tool(
    Json(req): Json<InvokeReq>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::channel::<Event>(32);
    let tool = req.tool.clone();
    let args = req.args.clone();

    tokio::spawn(async move {
        let em = StreamEmitter { tx };
        let msg_id = format!("m-{}", uuid::Uuid::new_v4().simple());

        match tool.as_str() {
            "fees.charge" => {
                let cat = args.get("category").and_then(|v| v.as_str()).unwrap_or("tuition");
                let term = args.get("term").and_then(|v| v.as_str()).unwrap_or("this term");
                let grade = args.get("grade").and_then(|v| v.as_str()).unwrap_or("all");
                let n = if grade == "all" { 250i64 } else if grade == "5" { 62 } else { 57 };

                em.step(1, 4, &format!("Preparing {n} invoices for {cat} — {term}")).await;
                em.tool_call("build_invoices",
                    &format!("category={cat}, term={term}, grade={grade}")).await;
                em.sleep(700).await;
                em.tool_result(true, &format!("{n} invoices prepared")).await;

                em.step(2, 4, "Generating PDFs").await;
                em.tool_call("render_pdfs", &format!("count={n}")).await;
                em.sleep(900).await;
                em.tool_result(true, &format!("{n} PDF files ready")).await;

                em.step(3, 4, "Scheduling reminders").await;
                em.tool_call("schedule_notifications", "channels=sms,email").await;
                em.sleep(500).await;
                em.tool_result(true, "Reminders queued for 9:00 AM tomorrow").await;

                em.step(4, 4, "Wrapping up").await;
                em.message_start(&msg_id).await;
                em.stream_words(&msg_id, &format!(
                    "🧾 Done. {n} invoices for {cat} ({term}, Grade {grade}) totaling ₹{}. \
                     Reminders go out at 9 AM.", fmt_money(n * 4500)), 25).await;
                em.end(&msg_id).await;
            }

            "students.enrol" => {
                let name  = args.get("full_name").and_then(|v| v.as_str()).unwrap_or("student");
                let grade = args.get("grade").and_then(|v| v.as_str()).unwrap_or("?");
                let phone = args.get("guardian_phone").and_then(|v| v.as_str()).unwrap_or("");
                let sid = format!("S-{}", fake_id(name));

                em.step(1, 4, "Creating student profile").await;
                em.tool_call("db_insert", &format!("name={name}, grade={grade}")).await;
                em.sleep(600).await;
                em.tool_result(true, &format!("Profile created — ID {sid}")).await;

                em.step(2, 4, "Assigning roll number").await;
                em.tool_call("assign_roll_no", &format!("grade={grade}")).await;
                em.sleep(450).await;
                em.tool_result(true, "Roll no. 33").await;

                em.step(3, 4, "Notifying guardian").await;
                em.tool_call("send_sms", &format!("to={phone}, template=welcome")).await;
                em.sleep(700).await;
                em.tool_result(true, "SMS delivered").await;

                em.step(4, 4, "Generating ID card").await;
                em.tool_call("render_id_card", &format!("student_id={sid}")).await;
                em.sleep(550).await;
                em.tool_result(true, "ID card ready to print").await;

                em.message_start(&msg_id).await;
                em.stream_words(&msg_id, &format!(
                    "🎉 {name} is enrolled in Grade {grade}. Welcome SMS sent to guardian at {phone}. \
                     Would you like to charge this term's fees now?", ), 25).await;
                em.end(&msg_id).await;
            }

            // A deliberate failure scenario — shows the UX when a step fails
            // mid-stream. Trigger from the "Bulk import (fails at step 3)"
            // demo button. Streams progress until step 3, then emits an
            // `error` event and stops.
            "demo.bulk_import" => {
                em.step(1, 4, "Reading uploaded CSV").await;
                em.tool_call("csv_parse", "file=roster.csv, rows=138").await;
                em.sleep(650).await;
                em.tool_result(true, "138 rows parsed").await;

                em.step(2, 4, "Validating fields").await;
                em.tool_call("validate_rows", "columns=name,dob,grade").await;
                em.sleep(700).await;
                em.tool_result(true, "132 valid rows, 6 warnings").await;

                em.step(3, 4, "Writing to database").await;
                em.tool_call("db_bulk_insert", "table=students, count=132").await;
                em.sleep(900).await;
                em.tool_result(false, "Row 47 rejected: duplicate guardian phone").await;
                em.send(
                    Event::default().event("error")
                        .data(json!({
                            "message": "Bulk import aborted at step 3 — duplicate guardian phone in row 47. \
                                        First 46 rows were inserted (rolled back). Fix the CSV and retry."
                        }).to_string())
                ).await;
            }

            _ => {
                em.message_start(&msg_id).await;
                em.stream_words(&msg_id,
                    &format!("Streaming isn't wired for tool \"{tool}\" yet."), 30).await;
                em.end(&msg_id).await;
            }
        }
    });

    Sse::new(ReceiverStream::new(rx).map(Ok))
}

// ── SSE emitter helper (shared between /copilot and /agent/stream ideally
// ── but kept local here to keep the mock file self-contained) ──────────
struct StreamEmitter { tx: mpsc::Sender<Event> }

impl StreamEmitter {
    async fn send(&self, ev: Event) { let _ = self.tx.send(ev).await; }
    async fn sleep(&self, ms: u64) { tokio::time::sleep(Duration::from_millis(ms)).await; }

    async fn step(&self, n: u32, of: u32, label: &str) {
        self.send(
            Event::default().event("step")
                .data(json!({"n": n, "of": of, "label": label}).to_string())
        ).await;
    }

    async fn tool_call(&self, name: &str, arg: &str) {
        let html = format!(
            r#"<div class="tool-card">🔧 <strong>{}</strong> <code>{}</code> <span class="thinking"><span></span><span></span><span></span></span></div>"#,
            html_escape(name), html_escape(arg)
        );
        self.send(Event::default().event("tool_call").data(html)).await;
    }

    async fn tool_result(&self, ok: bool, summary: &str) {
        let icon = if ok { "✅" } else { "⚠️" };
        let html = format!(r#"<div class="tool-card">{icon} {}</div>"#, html_escape(summary));
        self.send(Event::default().event("tool_result").data(html)).await;
    }

    async fn message_start(&self, id: &str) {
        let html = format!(
            r#"<div class="msg-bot"><div class="bubble" id="{id}"></div></div>"#
        );
        self.send(Event::default().event("message_start").data(html)).await;
    }

    async fn stream_words(&self, id: &str, text: &str, ms: u64) {
        for word in split_keep_spaces(text) {
            self.send(
                Event::default().event("token")
                    .data(json!({"id": id, "text": word}).to_string())
            ).await;
            self.sleep(ms).await;
        }
    }

    async fn end(&self, id: &str) {
        self.send(
            Event::default().event("message_end")
                .data(json!({"id": id}).to_string())
        ).await;
    }
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

fn split_keep_spaces(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_ws = false;
    for c in s.chars() {
        let ws = c.is_whitespace();
        if ws != in_ws && !cur.is_empty() { out.push(std::mem::take(&mut cur)); }
        cur.push(c);
        in_ws = ws;
    }
    if !cur.is_empty() { out.push(cur); }
    out
}

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

fn fmt_money(n: i64) -> String {
    // Simple thousands separator for the demo — real code would use `num-format`.
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { out.push(','); }
        out.push(c);
    }
    out.chars().rev().collect()
}

fn fake_id(seed: &str) -> u64 {
    // Deterministic pseudo-ID from the name for the demo.
    let mut h: u64 = 5381;
    for b in seed.bytes() { h = h.wrapping_mul(33) ^ (b as u64); }
    100 + (h % 900)
}

/// Convenience for other modules that may want to plug in a real
/// entity index later. Currently unused; kept so the map from mock →
/// production is a one-line change.
#[allow(dead_code)]
fn _entity_stub() -> HashMap<&'static str, Vec<&'static str>> { HashMap::new() }
