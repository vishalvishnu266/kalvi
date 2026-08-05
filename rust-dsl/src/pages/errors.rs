//! `/dsl/errors` — the **error UX handbook**.
//!
//! One page that shows every error surface with the exact Rust code that
//! produced it. Use this when you're building a form / page and need to
//! decide where an error message should live.
//!
//! ## The three surfaces (never mix them)
//!
//! | Error type | Surface | Component |
//! |---|---|---|
//! | Field-level (client-side + server-side validation) | Under the specific field | `input().error(...)` |
//! | Form-level (business rule spanning multiple fields) | Banner at the top of the form | `form_banner()` inside `.banner(...)` |
//! | Page-level (system-wide, transient) | Toast (ephemeral) or `alert()` (persistent) | `toast()` / `alert()` |
//! | Blocking acknowledgment (must be confirmed) | Slide-in panel from left/right | `ack_panel()` |
//!
//! The showcase is grouped exactly along these four lines.

use crate::pages::doc_helpers::*;
use crate::prelude::*;

pub fn build() -> Page {
    let mut body = page_shell()
        .add(toolbar()
            .add(breadcrumb()
                .item(Crumb::link("DSL", "/dsl"))
                .item(Crumb::current("Error UX")))
            .add(spacer())
            .add(button().label("Components").variant(Variant::Secondary).icon(Icons::GRID))
            .add(button().label("Layout guide").variant(Variant::Secondary).icon(Icons::STAR))
            .add(Node::raw(r##"<a href="/dsl/errors/combos" style="text-decoration:none"><ui-button variant="secondary">All combinations →</ui-button></a>"##))
            .add(Node::raw(r##"<a href="/dsl/errors/roundtrip" style="text-decoration:none"><ui-button variant="primary">Live round-trip demo →</ui-button></a>"##)));

    body = body.add(card().add(Node::raw(
        "<p style=\"margin:0\">This page shows every error surface the DSL \
         provides. The rule of thumb: <strong>match the surface to the scope \
         of the error</strong>. Field errors go under the field. Cross-field \
         business rules go in the form banner. System-wide alerts go at the \
         top of the page. Never combine them into a generic \"errors panel\".</p>",
    )));

    // ─────────────────────────────────────────────────────────────────────
    // 1. Field-level errors
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "1. Field-level errors",
        "One error per field. Set by the server (or by client-side JS on blur). \
         The .error(msg) helper sets `invalid` AND replaces the hint so the \
         user sees an actionable message right where they made the mistake.",
        vec![
            example("input().error(msg)",
                    "Red border, red text below the field.",
                    input().label("Guardian email").name("g_email")
                           .kind(InputType::Email).required()
                           .value("not-an-email")
                           .error("Please enter a valid email address"),
                    "input().label(\"Guardian email\").name(\"g_email\")\n    .kind(InputType::Email).required()\n    .error(\"Please enter a valid email address\")"),
            example("input().maybe_error(errors.get(name))",
                    "Same effect, but takes an Option so server code doesn't need a match on every field.",
                    input().label("Full name").name("name")
                           .maybe_error(Some("Full name is required")),
                    "let msg: Option<&str> = errors.get(\"name\").copied();\ninput().label(\"Full name\").name(\"name\").maybe_error(msg)"),
            example("select().error(msg)",
                    "Same treatment on select — including the searchable and allow-new variants.",
                    select().label("Grade").name("grade").required()
                            .option(SelectOption::new("5", "Grade 5"))
                            .option(SelectOption::new("6", "Grade 6"))
                            .error("Please choose a grade"),
                    "select().label(\"Grade\").required()\n    .option(...)\n    .error(\"Please choose a grade\")"),
            example("select().searchable().error(msg)",
                    "Type-ahead pickers get the same red-border + red-hint treatment.",
                    select().label("Student").name("student")
                            .placeholder("Search students…")
                            .searchable()
                            .option(SelectOption::new("aarav", "Aarav Kumar"))
                            .error("Please select a student"),
                    "select().label(\"Student\").placeholder(\"Search students…\")\n    .searchable()\n    .option(SelectOption::new(\"aarav\", \"Aarav Kumar\"))\n    .error(\"Please select a student\")"),
            example("radio_group().error(msg)",
                    "Applies to the whole group (no single \"invalid\" option makes sense).",
                    radio_group("gender").horizontal()
                        .option(radio("M", "Male"))
                        .option(radio("F", "Female"))
                        .error("Please choose one"),
                    "radio_group(\"gender\").horizontal()\n    .option(radio(\"M\", \"Male\"))\n    .option(radio(\"F\", \"Female\"))\n    .error(\"Please choose one\")"),
            example("checkbox().error(msg)",
                    "For required consent / terms & conditions.",
                    checkbox("I consent to the school's data policy")
                        .name("consent").required()
                        .error("You must consent to continue"),
                    "checkbox(\"I consent to the school's data policy\")\n    .required()\n    .error(\"You must consent to continue\")"),
            example("datepicker().error(msg)",
                    "For \"date must be in the future\", \"cannot be a holiday\", etc.",
                    datepicker().label("Due date").name("due").value("2025-01-01")
                        .error("Due date cannot be in the past"),
                    "datepicker().label(\"Due date\").name(\"due\").value(\"2025-01-01\")\n    .error(\"Due date cannot be in the past\")"),
            example("file_upload().error(msg)",
                    "For \"file too large\", \"unsupported format\", server rejections.",
                    file_upload().name("attachment").accept(".pdf")
                        .error("PDF is required — please upload a .pdf file"),
                    "file_upload().name(\"attachment\").accept(\".pdf\")\n    .error(\"PDF is required — please upload a .pdf file\")"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // 2. Form-level errors — form_banner()
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "2. Form-level errors — form_banner()",
        "For cross-field business rules and server-side rejections. Sits at the \
         top of the form, above the first field. Use `form().banner(...)` to \
         attach it, or render standalone above any field group.",
        vec![
            example("form_banner().tone(Tone::Danger).message(...)",
                    "Cross-field business rule.",
                    form_banner().tone(Tone::Danger)
                        .message("Guardian email must be different from the student's email."),
                    "form().action(\"/students\").method(\"post\")\n    .banner(form_banner().tone(Tone::Danger)\n        .message(\"Guardian email must be different from the student's email.\"))\n    .add(input().label(\"Student email\"))\n    .add(input().label(\"Guardian email\"))\n    .save_cancel(\"Save\")"),
            example("form_banner().tone(Tone::Warning)",
                    "Soft rule — user can proceed but should be aware.",
                    form_banner().tone(Tone::Warning)
                        .title("Late fee will apply")
                        .message("This invoice is being created past the term deadline."),
                    "form_banner().tone(Tone::Warning)\n    .title(\"Late fee will apply\")\n    .message(\"This invoice is being created past the term deadline.\")"),
            example("form_banner().tone(Tone::Success).dismissible()",
                    "After a successful save on the same page.",
                    form_banner().tone(Tone::Success).dismissible()
                        .title("Invoice saved")
                        .message("INV-1042 was created and emailed to the guardian."),
                    "form_banner().tone(Tone::Success).dismissible()\n    .title(\"Invoice saved\")\n    .message(\"INV-1042 was created and emailed to the guardian.\")"),
            example("form_banner().tone(Tone::Info)",
                    "Contextual heads-up — not an error.",
                    form_banner().tone(Tone::Info)
                        .message("Auto-save is enabled — changes are saved as you type."),
                    "form_banner().tone(Tone::Info)\n    .message(\"Auto-save is enabled — changes are saved as you type.\")"),
            example("form_banner().errors_summary(...)  ← accessibility win",
                    "When the server rejects N fields, render a summary at the top. \
                     Each item is a link that scrolls to and focuses the offending field.",
                    form_banner().tone(Tone::Danger)
                        .title("Please fix 2 errors")
                        .errors_summary(vec![
                            ("name".into(),  "Full name is required".into()),
                            ("email".into(), "Guardian email is invalid".into()),
                        ]),
                    "form_banner().tone(Tone::Danger)\n    .title(\"Please fix 2 errors\")\n    .errors_summary(vec![\n        (\"name\".into(),  \"Full name is required\".into()),\n        (\"email\".into(), \"Guardian email is invalid\".into()),\n    ])"),
            example("Multi-section — errors AND warnings in one banner",
                    "One inline banner with a Danger section (hard errors) AND a Warning section (soft rules). Both stay visible while the user fixes things — no modality, no ACK button.",
                    form_banner().tone(Tone::Danger)
                        .add_section(banner_section(Tone::Danger)
                            .title("Please fix 2 errors")
                            .errors_summary(vec![
                                FieldError::from(("email", "Email is invalid")),
                                FieldError::from(("phone", "Phone is required")),
                            ]))
                        .add_section(banner_section(Tone::Warning)
                            .title("1 warning")
                            .errors_summary(vec![
                                FieldError::from(("date", "Due date is a public holiday — invoice may be delayed")),
                            ])),
                    "form_banner().tone(Tone::Danger)\n    .add_section(banner_section(Tone::Danger)\n        .title(\"Please fix 2 errors\")\n        .errors_summary(vec![\n            (\"email\", \"Email is invalid\").into(),\n            (\"phone\", \"Phone is required\").into(),\n        ]))\n    .add_section(banner_section(Tone::Warning)\n        .title(\"1 warning\")\n        .errors_summary(vec![\n            (\"date\", \"Due date is a public holiday\").into(),\n        ]))"),
            example("errors_and_warnings_banner(errors, warnings)  ← one-liner",
                    "The canonical helper: pass two lists, get a fully-formed multi-section banner (or None if both empty). Auto-pluralises titles.",
                    errors_and_warnings_banner(
                        vec![
                            ("email", "Email is invalid"),
                            ("phone", "Phone is required"),
                        ],
                        vec![
                            ("date", "Due date is a public holiday"),
                        ],
                    ).unwrap_or_else(form_banner),
                    "// In your Axum handler:\nlet errors:   Vec<FieldError> = validate(&input);\nlet warnings: Vec<FieldError> = business_warnings(&input);\n\nform().action(\"/students\").method(\"post\")\n    .maybe_banner(errors_and_warnings_banner(errors, warnings))\n    .add(...)\n    .save_cancel(\"Save\")"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // 3. Page-level alerts — alert() (persistent) + toast() (transient)
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "3. Page-level notices — alert() and toast()",
        "For system-wide messages not tied to a specific field or form. \
         Use alert() when the message must stay until dismissed; use toast() \
         (fired from JS) for transient feedback like \"Saved.\" that vanishes \
         after a few seconds.",
        vec![
            example("alert().tone(Tone::Warning).icon(Icons::WARNING)",
                    "Persistent — user must dismiss it.",
                    alert().tone(Tone::Warning).icon(Icons::WARNING).dismissible()
                        .title("Read-only mode")
                        .message("Your role doesn't allow editing fees. Contact admin@school for access."),
                    "alert().tone(Tone::Warning).icon(Icons::WARNING).dismissible()\n    .title(\"Read-only mode\")\n    .message(\"Your role doesn't allow editing fees. Contact admin@school for access.\")"),
            example("alert().tone(Tone::Info).icon(Icons::INFO)",
                    "Non-blocking context.",
                    alert().tone(Tone::Info).icon(Icons::INFO)
                        .title("New version available")
                        .message("Reload the page to get the latest features."),
                    "alert().tone(Tone::Info).icon(Icons::INFO)\n    .title(\"New version available\")\n    .message(\"Reload the page to get the latest features.\")"),
            example("alert().tone(Tone::Danger).icon(Icons::X)",
                    "System-level error that isn't tied to a form (e.g. failed background job).",
                    alert().tone(Tone::Danger).icon(Icons::X).dismissible()
                        .title("Nightly attendance sync failed")
                        .message("Yesterday's attendance did not propagate to the district portal. Retry from Settings → Sync."),
                    "alert().tone(Tone::Danger).icon(Icons::X).dismissible()\n    .title(\"Nightly attendance sync failed\")\n    .message(\"Yesterday's attendance did not propagate to the district portal.\")"),
            example("toast_host() + JS-fired toast()",
                    "Ephemeral — vanishes after ~4s. Add toast_host() once per page and fire from JS.",
                    toast_host(),
                    "// Once per page:\ntoast_host()\n\n// From JS after a successful save:\ndocument.querySelector('ui-toast-host').show({\n    tone: 'success',\n    message: 'Invoice saved'\n});"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // 3b. Slide-in acknowledgment panel — ack_panel()
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "3b. Blocking acknowledgment — ack_panel()",
        "A drawer-style panel that slides in from the left or right, presents a \
         message, and REQUIRES the user to click an ACK button to dismiss it. \
         Use for notices that cannot be missed (compliance changes, destructive \
         side-effects, account-level lockouts). NOT a general drawer — for that \
         see components/drawer.",
        vec![
            example("Warning, right side, page-blocking",
                    "Click the button to trigger it. Press the ACK button (or Enter) to dismiss.",
                    Node::raw(r##"
                        <ui-ack-panel id="ex-ack-1"
                            tone="warning" placement="right" icon="warning"
                            title="Fees module is locked"
                            message="Editing is disabled until the term audit completes on Aug 15. Contact admin@school for urgent adjustments."
                            ack-label="I understand"></ui-ack-panel>
                        <button type="button"
                            onclick="document.getElementById('ex-ack-1').openPanel()"
                            style="padding:8px 14px;border-radius:8px;border:1px solid var(--color-border);background:var(--color-surface);cursor:pointer">
                            Open ack panel (right, warning)
                        </button>
                    "##),
                    "ack_panel()\n    .id(\"ex-ack-1\")\n    .tone(Tone::Warning)\n    .placement(AckPlacement::Right)\n    .icon(Icons::WARNING)\n    .title(\"Fees module is locked\")\n    .message(\"Editing is disabled until the term audit completes on Aug 15.\")\n    .ack_label(\"I understand\")\n\n// Open from JS on any event:\n// document.getElementById('ex-ack-1').openPanel();"),
            example("Danger, left side, page-blocking",
                    "Same component, opposite placement + danger tone.",
                    Node::raw(r##"
                        <ui-ack-panel id="ex-ack-2"
                            tone="danger" placement="left" icon="x"
                            title="Session about to expire"
                            message="You will be signed out in 30 seconds due to inactivity. Save any pending work now."
                            ack-label="Acknowledge"></ui-ack-panel>
                        <button type="button"
                            onclick="document.getElementById('ex-ack-2').openPanel()"
                            style="padding:8px 14px;border-radius:8px;border:1px solid var(--color-border);background:var(--color-surface);cursor:pointer">
                            Open ack panel (left, danger)
                        </button>
                    "##),
                    "ack_panel()\n    .id(\"ex-ack-2\")\n    .tone(Tone::Danger)\n    .placement(AckPlacement::Left)\n    .icon(Icons::X)\n    .title(\"Session about to expire\")\n    .message(\"You will be signed out in 30 seconds due to inactivity.\")\n    .ack_label(\"Acknowledge\")"),
            example("Info, opens on page load",
                    ".open() renders the panel already open — no JS needed. Great for compliance notices \
                     that must appear the first time a page is visited.",
                    Node::raw(r##"
                        <button type="button"
                            onclick="const el = document.getElementById('ex-ack-3'); el && (el.open = true, el.openPanel && el.openPanel())"
                            style="padding:8px 14px;border-radius:8px;border:1px solid var(--color-border);background:var(--color-surface);cursor:pointer">
                            Open ack panel (info, right)
                        </button>
                        <ui-ack-panel id="ex-ack-3"
                            tone="info" placement="right" icon="info"
                            title="New privacy notice"
                            message="We've updated how we store guardian contact details. Please review and acknowledge to continue."
                            ack-label="OK, got it"></ui-ack-panel>
                    "##),
                    "ack_panel()\n    .id(\"welcome-notice\")\n    .tone(Tone::Info)\n    .placement(AckPlacement::Right)\n    .icon(Icons::INFO)\n    .title(\"New privacy notice\")\n    .message(\"We've updated how we store guardian contact details.\")\n    .ack_label(\"OK, got it\")\n    .open()   // ← show immediately on page load"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // 4. End-to-end pattern — the full form
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(section().title("4. End-to-end example — the full pattern")
        .subtitle("A form that has every error surface active, so you can see how they compose.")
        .add(card().add(
            form().action("/students").method("post")
                .banner(form_banner().tone(Tone::Danger)
                    .title("Please fix 2 errors")
                    .message("Guardian email must be different from the student's email.")
                    .errors_summary(vec![
                        ("s_email".into(), "Student email is required".into()),
                        ("g_email".into(), "Guardian email is invalid".into()),
                    ]))
                .add(input().label("Full name").name("name")
                     .value("Aarav Kumar").required())
                .add(input().label("Student email").name("s_email")
                     .kind(InputType::Email).required()
                     .error("Student email is required"))
                .add(input().label("Guardian email").name("g_email")
                     .kind(InputType::Email).required()
                     .value("aarav@school.example")
                     .error("Guardian email is invalid — cannot be the same as the student"))
                .add(select().label("Grade").name("grade").required()
                     .option(SelectOption::new("5", "Grade 5"))
                     .option(SelectOption::new("6", "Grade 6")))
                .add(checkbox("I consent to the school's data policy")
                     .name("consent").required())
                .save_cancel("Save"))));

    body = body.add(card().add(Node::raw(
        "<h3 style=\"margin:0 0 8px\">Server-side integration pattern</h3>\
         <pre style=\"margin:0;padding:12px 14px;background:var(--color-surface);\
                     border:1px solid var(--color-border);border-radius:8px;\
                     font-family:ui-monospace,SFMono-Regular,Menlo,monospace;\
                     font-size:12px;line-height:1.55;overflow:auto\"><code>\
// axum handler\npub async fn create_student(Form(input): Form&lt;NewStudent&gt;) -&gt; Response {\
\n    let errors: HashMap&lt;&amp;str, String&gt; = validate(&amp;input);\
\n    if errors.is_empty() {\
\n        save(input).await;\
\n        return Redirect::to(\"/students\").into_response();\
\n    }\
\n    // Re-render the SAME form, this time WITH errors annotated.\
\n    // Returning 422 is the RFC-correct status.\
\n    let page = students::add_form_page(&amp;input, &amp;errors);\
\n    (StatusCode::UNPROCESSABLE_ENTITY, Html(page.render())).into_response()\
\n}\
\n\
\n// in students::add_form_page(input, errors):\
\nform().action(\"/students\").method(\"post\")\
\n    .maybe_banner(errors_summary_banner(errors))    // top-of-form summary\
\n    .add(input().label(\"Full name\").name(\"name\")\
\n         .value(&amp;input.name)\
\n         .maybe_error(errors.get(\"name\").cloned()))\
\n    .add(input().label(\"Guardian email\").name(\"g_email\")\
\n         .value(&amp;input.g_email)\
\n         .maybe_error(errors.get(\"g_email\").cloned()))\
\n    .save_cancel(\"Save\")\
\n</code></pre>\
\n<p style=\"margin:12px 0 0;color:var(--color-text-muted);font-size:var(--fs-sm)\">\
\nThe shell's submit interceptor swaps the target region on a 422 response, so the user \
\nsees the same form with errors highlighted — zero per-page JS required. \
\nWorks identically inside a native WebView (Capacitor/Flutter).</p>",
    )));

    page_of("Error UX · DSL handbook", body)
}
