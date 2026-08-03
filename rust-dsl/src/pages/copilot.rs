//! `/dsl/copilot` — dedicated Copilot demo page.
//!
//! Shows the floating FAB (open by default), lists every mock prompt the
//! Axum backend (`src/web/copilot.rs`) recognises, and drops a sample
//! `<ui-form>` on the page so the `fill_form` UI-action has something to
//! populate.
//!
//! On mobile (< 720 px viewport) the drawer switches to a bottom-sheet —
//! use Chrome DevTools "Toggle device toolbar" (Ctrl/Cmd+Shift+M) to see it.

use crate::prelude::*;

fn prompt_chip(text: &str) -> Node {
    // `data-copilot-suggest` is picked up by <ui-copilot>'s messages click
    // delegate → sends the prompt as if the user typed it. But that only
    // fires inside the panel; here we render a plain link that opens the
    // panel and pre-fills the composer via a small inline script.
    Node::raw(format!(
        r##"<button class="copilot-try" data-prompt="{p}">{p}</button>"##,
        p = crate::core::escape_html(text),
    ))
}

fn scenario_list() -> Node {
    Node::raw(r##"
        <style>
          .copilot-scenarios { display: grid; gap: 12px; }
          .copilot-scenarios .row {
            display: grid; gap: 10px;
            grid-template-columns: minmax(240px, 1fr) 2fr;
            align-items: center;
            padding: 12px 14px;
            border-radius: 12px;
            background: var(--color-surface);
            border: 1px solid var(--color-border);
          }
          @media (max-width: 640px) {
            .copilot-scenarios .row { grid-template-columns: 1fr; }
          }
          .copilot-try {
            appearance: none; cursor: pointer;
            font: inherit; text-align: left;
            padding: 8px 12px; border-radius: 10px;
            background: var(--color-primary-soft);
            color: var(--color-text);
            border: 1px solid var(--color-primary-ring);
          }
          .copilot-try:hover {
            background: var(--color-primary);
            color: var(--color-primary-contrast);
          }
          .copilot-desc { color: var(--color-text-muted); font-size: var(--fs-sm); }
        </style>
        <script>
          // Click a chip → open Copilot and send its prompt.
          document.addEventListener('click', (e) => {
            const btn = e.target.closest('.copilot-try');
            if (!btn) return;
            const cp = document.querySelector('ui-copilot');
            if (!cp) return;
            cp.openPanel?.();
            // Give the panel a tick to render, then invoke _send via public API.
            setTimeout(() => {
              const ta = cp.renderRoot.querySelector('textarea');
              if (ta) ta.value = btn.dataset.prompt;
              cp._send?.(btn.dataset.prompt);
            }, 50);
          });
        </script>
    "##)
}

fn scenario_row(prompt: &str, desc: &str) -> Card {
    let mut c = card();
    c = c.add(Node::raw(r#"<div class="row">"#));
    c = c.add(prompt_chip(prompt));
    c = c.add(Node::raw(format!(r#"<div class="copilot-desc">{}</div>"#,
        crate::core::escape_html(desc))));
    c = c.add(Node::raw("</div>"));
    c
}

pub fn build() -> Page {
    let intro = card()
        .add(text_body(
            "Try the mock agent",
            "Click any prompt below (or press Ctrl/Cmd + J anywhere on the page). \
             The panel is a bottom-sheet on mobile and a right-hand drawer on desktop.",
        ));

    let scenarios = section()
        .title("Scenarios")
        .subtitle("Each button sends a prompt and shows a different capability.")
        .add(Node::raw(r#"<div class="copilot-scenarios">"#))
        .add(scenario_row("Open the dashboard",
            "ui_action navigate → /dsl/dashboard"))
        .add(scenario_row("Show students",
            "ui_action navigate → /dsl/students"))
        .add(scenario_row("Fees",
            "ui_action navigate → /dsl/fees"))
        .add(scenario_row("Attendance",
            "ui_action navigate → /dsl/attendance"))
        .add(scenario_row("Enrol a new student",
            "Multi-step: 3 tool_calls, animated step bar 1/3 → 3/3, success toast"))
        .add(scenario_row("Fill the form",
            "ui_action fill_form populates the sample form below"))
        .add(scenario_row("Highlight",
            "Pulses + focuses the first ui-input on the page"))
        .add(scenario_row("Enable dark mode",
            "ui_action set_theme → data-theme=\"dark\" on <html>"))
        .add(scenario_row("Enable light mode",
            "ui_action set_theme → data-theme=\"light\""))
        .add(scenario_row("Help",
            "Renders clickable suggestion chips inside the assistant bubble"))
        .add(scenario_row("Hello there!",
            "Fallback: word-by-word streamed echo"))
        .add(Node::raw("</div>"))
        .add(scenario_list());

    // A sample form so `fill_form` has a target on this page.
    let sample_form = section()
        .title("Sample form (target for `fill the form`)")
        .subtitle("The mock backend fills these fields when you ask it to.")
        .add(card().add(
            form()
                .add(input().label("Full name").name("fullName").placeholder("e.g. Aarav Kumar"))
                .add(input().label("Primary email").name("primaryEmail").kind(InputType::Email))
                .add(input().label("Backup email").name("backupEmail").kind(InputType::Email))
                .add(input().label("Grade").name("grade").placeholder("e.g. 5"))
                .add(row_actions()
                    .add(button().label("Reset").variant(Variant::Ghost).reset())
                    .add(button().label("Save").variant(Variant::Primary).submit()))
        ));

    // Note: page_of() automatically injects <ui-copilot> — no extra work.
    // We also flip it to `open` so the panel is visible on first paint.
    page_of("Copilot demo · ERP",
        page_shell()
            .add(toolbar()
                .add(breadcrumb()
                    .item(Crumb::link("DSL", "/dsl"))
                    .item(Crumb::current("Copilot")))
                .add(spacer())
                .add(button().label("Open panel").variant(Variant::Primary).icon(Icons::MESSAGE)
                    .add(Node::raw(r#" <script>
                      // Wire the "Open panel" button to the copilot element.
                      document.currentScript.parentElement.addEventListener('click', () => {
                        document.querySelector('ui-copilot')?.openPanel?.();
                      });
                    </script>"#))))
            .add(intro)
            .add(scenarios)
            .add(sample_form)
            .add(toast_host())
    )
}
