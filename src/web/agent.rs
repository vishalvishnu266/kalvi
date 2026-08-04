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
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

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
        .route("/agent/invoke",   post(invoke))
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A suggestion/autocomplete/follow-up item — shown as a chip or a row.
#[derive(Serialize)]
struct Item {
    /// Short user-facing label ("Mark today's attendance").
    title: String,
    /// Optional emoji or icon.
    icon: Option<String>,
    /// Optional secondary line (e.g. "12 pending").
    subtitle: Option<String>,
    /// Machine-readable tool name to invoke.
    tool: String,
    /// Pre-filled arguments (context-derived, page-derived, or curated).
    prefill: Value,
    /// Optional keyboard shortcut string ("⌘K").
    shortcut: Option<String>,
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
        }
    }
    fn sub(mut self, s: &str) -> Self { self.subtitle = Some(s.into()); self }
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

    // Always append the "Ask the assistant" fallback so nothing is a dead end.
    out.push(Item {
        title: format!("Ask the assistant about \"{}\"", cq.q),
        icon: Some("💬".into()),
        subtitle: Some("Uses AI — takes a couple of seconds".into()),
        tool: "ask".into(),
        prefill: json!({"text": cq.q}),
        shortcut: None,
    });
    Json(out)
}

/// Tiny fuzzy scorer — substring hits + subsequence hits. Enough for the demo.
fn fuzzy_score(q: &str, hay: &str) -> i32 {
    if q.is_empty() { return 0; }
    let h = hay.to_lowercase();
    let mut score = 0;
    for word in q.split_whitespace() {
        if word.len() < 2 { continue; }
        if h.contains(word) {
            score += 40 + if h.starts_with(word) { 20 } else { 0 };
        } else {
            // subsequence check — each hit adds a little
            let mut qi = 0;
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
}

async fn schema(Query(q): Query<SchemaQ>) -> Json<SchemaOut> {
    let s = match q.tool.as_str() {
        "attendance.mark" => SchemaOut {
            required: vec!["class_id".into(), "date".into()],
            mutating: true,
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
            fields: vec![Field {
                name: "query".into(), label: "Search by name, grade, or ID".into(),
                kind: "text".into(), required: true,
                placeholder: Some("e.g. Aarav, or 5-B, or S-42".into()),
                help: None, enum_options: None,
            }],
        },
        _ => SchemaOut { required: vec![], fields: vec![], mutating: false },
    };
    Json(s)
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
        "core.navigate" => json!({
            "kind": "text",
            "text": format!("🧭 Navigating to {}", a.get("href").and_then(|v| v.as_str()).unwrap_or("?")),
        }),

        "core.set_theme" => json!({
            "kind": "text",
            "text": format!("🎨 Theme set to {}",
                            a.get("theme").and_then(|v| v.as_str()).unwrap_or("default")),
        }),

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

        _ => json!({
            "kind": "text",
            "text": format!("Tool \"{}\" is not wired in the mock backend yet.", req.tool),
        }),
    };

    Json(out)
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
