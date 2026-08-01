//! `/dsl/errors/combos` — **exhaustive combinations catalogue**.
//!
//! Every meaningful shape of the error UX in one page. Use as a visual
//! regression checklist and as a menu when you're building a new page and
//! trying to decide which pattern applies.
//!
//! Grouped by "surface stack" — i.e. which of the four surfaces are active:
//!
//!   * field only
//!   * banner only
//!   * banner + field
//!   * banner (multi-section) + field
//!   * alert only
//!   * alert + banner + field
//!   * ack panel only (blocking, unrelated to form)
//!   * ack panel + banner + field (rare — separate concerns, both shown)
//!   * happy path (success banner + no errors)
//!
//! Each combo shows the rendered surface AND the exact Rust code that
//! produced it.

use crate::pages::doc_helpers::*;
use crate::prelude::*;

/// Build a small demo form used inside several combos. Field errors passed
/// as an `Option<&str>` per field so we can compose combos with/without.
fn demo_form(
    name_err:  Option<&str>,
    email_err: Option<&str>,
    banner:    Option<FormBanner>,
) -> Form {
    let name_err  = name_err.map(str::to_string);
    let email_err = email_err.map(str::to_string);
    form().action("/demo").method("post")
        .maybe_banner(banner)
        .add(input().label("Full name").name("name").required()
             .value("Aarav Kumar")
             .maybe_error(name_err))
        .add(input().label("Guardian email").name("email")
             .kind(InputType::Email).required()
             .value("aarav@school.example")
             .maybe_error(email_err))
        .add(datepicker().label("Due date").name("due")
             .value("2026-08-15"))
        .save_cancel("Save")
}

pub fn build() -> Page {
    let mut body = page_shell()
        .add(toolbar()
            .add(breadcrumb()
                .item(Crumb::link("DSL", "/dsl"))
                .item(Crumb::link("Error UX", "/dsl/errors"))
                .item(Crumb::current("Combinations")))
            .add(spacer())
            .add(button().label("Error UX handbook").variant(Variant::Secondary).icon(Icons::WARNING)));

    body = body.add(card().add(Node::raw(
        "<p style=\"margin:0\">Every meaningful <strong>combination</strong> of \
         error surfaces in one place. Use this as a visual checklist and a \
         menu of ready-to-copy patterns. Each combo shows a rendered demo + \
         the exact Rust code that produced it. If a pattern you need isn't \
         here, it probably shouldn't exist — combining more surfaces than \
         these usually hurts UX.</p>",
    )));

    // ─────────────────────────────────────────────────────────────────────
    // A. Field only
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "A. Field errors only",
        "Simplest case: server rejects one or two fields, no cross-field rules. \
         Use when the errors are self-explanatory and localised.",
        vec![
            example("One field error",
                    "Under the specific field, red border + red hint.",
                    demo_form(None, Some("Guardian email is invalid"), None),
                    "form().action(\"/demo\").method(\"post\")\n    .add(input().label(\"Full name\").name(\"name\").required())\n    .add(input().label(\"Guardian email\").name(\"email\")\n         .kind(InputType::Email).required()\n         .error(\"Guardian email is invalid\"))\n    .save_cancel(\"Save\")"),
            example("Two field errors, no banner",
                    "For short forms this is enough — the user can see all errors at once without scrolling.",
                    demo_form(Some("Full name is required"),
                              Some("Guardian email is invalid"),
                              None),
                    "input().label(\"Full name\").name(\"name\")\n    .maybe_error(errors.get(\"name\").cloned())\n\ninput().label(\"Guardian email\").name(\"email\")\n    .kind(InputType::Email)\n    .maybe_error(errors.get(\"email\").cloned())"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // B. Banner only (business rule, no field-specific errors)
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "B. Banner only — cross-field business rule",
        "The individual fields are valid, but the combination isn't. Show the \
         rule at the top of the form; don't invent field errors for it.",
        vec![
            example("Danger banner, no field errors",
                    "e.g. \"Guardian email must differ from student email\" when both fields are individually valid.",
                    demo_form(None, None,
                        Some(form_banner().tone(Tone::Danger)
                            .message("Guardian email must be different from the student's email."))),
                    "demo_form(None, None,\n    Some(form_banner().tone(Tone::Danger)\n        .message(\"Guardian email must be different from the student's email.\")))"),
            example("Warning banner (soft rule, user may still proceed)",
                    "e.g. \"Late fee will apply\" — advisory, not blocking.",
                    demo_form(None, None,
                        Some(form_banner().tone(Tone::Warning)
                            .title("Late fee will apply")
                            .message("This invoice is being created past the term deadline."))),
                    "form_banner().tone(Tone::Warning)\n    .title(\"Late fee will apply\")\n    .message(\"This invoice is being created past the term deadline.\")"),
            example("Info banner (contextual, not an error)",
                    "e.g. \"Auto-save is enabled\" — informational.",
                    demo_form(None, None,
                        Some(form_banner().tone(Tone::Info)
                            .message("Auto-save is enabled — changes are saved as you type."))),
                    "form_banner().tone(Tone::Info)\n    .message(\"Auto-save is enabled — changes are saved as you type.\")"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // C. Banner + field errors (standard server-rejection pattern)
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "C. Banner + field errors — the canonical server-rejection pattern",
        "N field errors → summary banner at the top with click-to-focus links, \
         PLUS the individual red hints under each field. This is what \
         `errors_summary_banner()` produces automatically.",
        vec![
            example("errors_summary_banner() — pluralised title, click-to-focus",
                    "The user scans the summary, clicks a link, jumps to the field, fixes it.",
                    demo_form(
                        Some("Full name is required"),
                        Some("Guardian email is invalid"),
                        errors_summary_banner(vec![
                            ("name",  "Full name is required"),
                            ("email", "Guardian email is invalid"),
                        ])),
                    "let errors: HashMap<&str, String> = validate(&input);\n\ndemo_form(\n    errors.get(\"name\").map(String::as_str),\n    errors.get(\"email\").map(String::as_str),\n    errors_summary_banner(errors.iter())   // Option<FormBanner>\n)"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // D. Multi-section banner + field errors
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "D. Multi-section banner — errors AND warnings together",
        "One banner, two coloured sections, still inline, still non-modal. Field \
         errors continue to live under their fields. Use `errors_and_warnings_banner()` \
         when you have both hard errors and soft rules to communicate.",
        vec![
            example("errors_and_warnings_banner()  ← one-liner",
                    "Auto-pluralises titles, hides empty sections, returns None if both empty.",
                    demo_form(
                        Some("Full name is required"),
                        Some("Guardian email is invalid"),
                        errors_and_warnings_banner(
                            vec![
                                ("name",  "Full name is required"),
                                ("email", "Guardian email is invalid"),
                            ] as Vec<(&str, &str)>,
                            vec![
                                ("due", "Due date is a public holiday — invoice may be delayed"),
                            ] as Vec<(&str, &str)>,
                        )),
                    "demo_form(\n    errors.get(\"name\"),\n    errors.get(\"email\"),\n    errors_and_warnings_banner(errors_list, warnings_list),\n)"),
            example("Section .message() — narrative context per section",
                    "Each section can carry its own `.message(...)` for a short explanation before the linked list.",
                    demo_form(None, None,
                        Some(form_banner().tone(Tone::Warning)
                            .add_section(banner_section(Tone::Warning)
                                .title("Late fee applies")
                                .message("This invoice is being created past the term deadline. Continue to attach a late-fee line automatically."))
                            .add_section(banner_section(Tone::Info)
                                .title("Auto-save enabled")
                                .message("Your draft will save every 5 s while you type.")))),
                    "form_banner().tone(Tone::Warning)\n    .add_section(banner_section(Tone::Warning)\n        .title(\"Late fee applies\")\n        .message(\"This invoice is being created past the term deadline. Continue to attach a late-fee line automatically.\"))\n    .add_section(banner_section(Tone::Info)\n        .title(\"Auto-save enabled\")\n        .message(\"Your draft will save every 5 s while you type.\"))"),
            example("Three-section: danger + warning + info",
                    "For rare cases where all three severities apply. Sections render in the order added.",
                    demo_form(
                        Some("Full name is required"),
                        None,
                        Some(form_banner().tone(Tone::Danger)
                            .add_section(banner_section(Tone::Danger)
                                .title("Please fix 1 error")
                                .errors_summary(vec![
                                    ("name", "Full name is required").into(),
                                ]))
                            .add_section(banner_section(Tone::Warning)
                                .title("Late fee applies")
                                .message("Past the term deadline — a late fee will be attached."))
                            .add_section(banner_section(Tone::Info)
                                .message("Auto-save runs every 5 s while you type.")))),
                    "form_banner().tone(Tone::Danger)\n    .add_section(banner_section(Tone::Danger)\n        .title(\"Please fix 1 error\")\n        .errors_summary(vec![(\"name\", \"Full name is required\").into()]))\n    .add_section(banner_section(Tone::Warning)\n        .title(\"Late fee applies\")\n        .message(\"...\"))\n    .add_section(banner_section(Tone::Info)\n        .message(\"Auto-save runs every 5 s while you type.\"))"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // E. Happy path — success banner
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "E. Happy path — success confirmation",
        "After a successful save on the same page (no redirect), show a Success \
         banner so the user knows their action landed. Dismissible so it \
         doesn't linger when they start editing again.",
        vec![
            example("Success banner, dismissible",
                    "The classic \"Saved.\" confirmation for stay-on-page workflows.",
                    demo_form(None, None,
                        Some(form_banner().tone(Tone::Success).dismissible()
                            .title("Invoice saved")
                            .message("INV-1042 was created and emailed to the guardian."))),
                    "form_banner().tone(Tone::Success).dismissible()\n    .title(\"Invoice saved\")\n    .message(\"INV-1042 was created and emailed to the guardian.\")"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // F. Alert (page-level, unrelated to form)
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "F. Page-level alert + form",
        "The `alert()` sits ABOVE the form and communicates something system-\
         level (read-only mode, background job failure). It's independent of \
         the form submit — the form still works normally underneath.",
        vec![
            example("Alert (warning) + form with field error",
                    "Alert flags a system-wide condition; form still shows its own field errors.",
                    column().gap(Gap::Md)
                        .add(alert().tone(Tone::Warning).icon(Icons::WARNING).dismissible()
                            .title("Read-only mode")
                            .message("Your role can save drafts but cannot submit."))
                        .add(demo_form(None, Some("Guardian email is invalid"), None)),
                    "column().gap(Gap::Md)\n    .add(alert().tone(Tone::Warning).icon(Icons::WARNING).dismissible()\n        .title(\"Read-only mode\")\n        .message(\"Your role can save drafts but cannot submit.\"))\n    .add(demo_form(...))"),
            example("Alert (info) — persistent context banner",
                    "Non-dismissible for compliance / policy notices.",
                    alert().tone(Tone::Info).icon(Icons::INFO)
                        .title("New privacy policy")
                        .message("Guardian contact details are now encrypted at rest. See settings for details."),
                    "alert().tone(Tone::Info).icon(Icons::INFO)\n    .title(\"New privacy policy\")\n    .message(\"Guardian contact details are now encrypted at rest.\")"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // G. Ack panel (rare, blocking) + form
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(doc_section(
        "G. Ack panel + form — sequential, not combined",
        "The ack panel appears FIRST (blocking the whole page). Once the user \
         acknowledges, the panel closes and the form is available normally. \
         Use ONLY when the notice is unrelated to the current submit and \
         cannot be missed.",
        vec![
            example("Trigger a warning ack panel above a form",
                    "Click the button to slide the panel in from the right. Press ACK to reveal the form beneath.",
                    Node::raw(r##"
                        <div style="display:flex;flex-direction:column;gap:12px">
                          <button type="button"
                              onclick="document.getElementById('combo-ack').openPanel()"
                              style="padding:8px 14px;border-radius:8px;border:1px solid var(--color-border);background:var(--color-surface);cursor:pointer;align-self:flex-start;">
                              Open ack panel
                          </button>
                          <ui-ack-panel id="combo-ack"
                              tone="warning" placement="right" icon="warning"
                              title="Fees module is locked"
                              message="Editing is disabled until the term audit completes on Aug 15. You can still view invoices."
                              ack-label="I understand"></ui-ack-panel>
                          <em style="color:var(--color-text-muted);font-size:var(--fs-sm)">
                              (After you acknowledge, the form below behaves normally.)
                          </em>
                        </div>
                    "##),
                    "// Both rendered on the same page; ack_panel is triggered by JS/state.\ncolumn().gap(Gap::Md)\n    .add(ack_panel()\n        .id(\"fees-locked\")\n        .tone(Tone::Warning)\n        .placement(AckPlacement::Right)\n        .icon(Icons::WARNING)\n        .title(\"Fees module is locked\")\n        .message(\"Editing is disabled until the term audit completes on Aug 15.\")\n        .ack_label(\"I understand\")\n        .open())     // ← show immediately if condition is true\n    .add(demo_form(...))"),
        ],
    ));

    // ─────────────────────────────────────────────────────────────────────
    // H. Decision guide
    // ─────────────────────────────────────────────────────────────────────
    body = body.add(card().title("Which combination should I use?")
        .add(Node::raw(r##"
<div style="font-size:var(--fs-sm);line-height:1.6">
<p style="margin:0 0 12px"><strong>Answer these questions in order:</strong></p>
<ol style="margin:0;padding-left:20px">
  <li>Is the error tied to <strong>one specific field</strong> the user typed? → Use <code>input().error()</code>. Section <strong>A</strong>.</li>
  <li>Does the error span <strong>multiple fields</strong> (business rule)? → Add <code>form_banner()</code>. Section <strong>B</strong> or <strong>C</strong>.</li>
  <li>Do you have both <strong>hard errors AND soft warnings</strong> after a submit? → Use <code>errors_and_warnings_banner()</code>. Section <strong>D</strong>.</li>
  <li>Is the message about a <strong>system-wide state</strong> (read-only, policy notice)? → Use <code>alert()</code>. Section <strong>F</strong>.</li>
  <li>Must the user <strong>acknowledge before proceeding</strong> AND is it unrelated to the current submit? → Use <code>ack_panel()</code>. Section <strong>G</strong>.</li>
  <li>Did the save just <strong>succeed</strong>? → Use a Success <code>form_banner().dismissible()</code>. Section <strong>E</strong>. For redirects, use a <code>toast()</code> on the next page instead.</li>
</ol>
<p style="margin:12px 0 0;color:var(--color-text-muted)">
  If none of the above matches, you probably don't need an error surface — reconsider whether the situation is actually an error, or whether inline hint text on the field would suffice.
</p>
</div>
        "##)));

    page_of("Error UX combinations · DSL catalogue", body)
}
