//! ERP "Dashboard" landing page — KPI stats, admissions kanban, activity
//! timeline, and a quick actions bar.
//!
//! Built on the standard page presets from `layout.rs`:
//! * `page_of` + `page_shell` — outer container and vertical rhythm.
//! * `toolbar`  — top row (breadcrumb + actions), auto-stacks on mobile.
//! * `two_col`  — responsive two-column body (main + side).
//! * `text_body(title, sub)` — the "strong + muted line" mini-layout.
//! * `Icons::*` — typed icon constants, no magic strings.

use crate::prelude::*;

/// Small private helper for the cross-page nav links every DSL demo shares.
/// Uses `.lu-*` layout classes only — zero inline CSS.
fn demo_nav() -> Node {
    Node::raw(r##"
        <nav class="lu-demo-nav">
          <strong>DSL demos:</strong>
          <a href="/dsl/dashboard">Dashboard</a>
          <a href="/dsl/students">Students</a>
          <a href="/dsl/fees">Fees</a>
          <a href="/dsl/attendance">Attendance</a>
          <a href="/dsl/icons">Icons</a>
          <a href="/dsl/layouts">Layouts</a>
          <a href="/dsl/components">Components</a>
          <a href="/dsl/errors">Errors</a>
          <a href="/dsl" class="lu-demo-nav-end">Index</a>
        </nav>
    "##)
}

pub fn build() -> Page {
    // ── KPI stats ──
    let kpis = grid().cols_min(MinCol::W220)
        .add(stat("Total students",     "1,248").icon(Icons::USERS).trend(Trend::Up).delta("+2.1%"))
        .add(stat("Present today",      "1,102").icon(Icons::CHECK).trend(Trend::Up).delta("+3.2%"))
        .add(stat("Fee collection",     "82%"  ).icon(Icons::CARD ).trend(Trend::Up).delta("+5.6%"))
        .add(stat("Pending admissions", "17"   ).icon(Icons::CLIPBOARD).trend(Trend::Down).delta("-3"));

    // ── Admissions kanban (mock) ──
    let kanban_board = kanban()
        .column(kanban_column("Applied")
            .add(kanban_card().id("k1").text("Aarav Kumar · Grade 5"))
            .add(kanban_card().id("k2").text("Meera Sharma · Grade 6"))
            .add(kanban_card().id("k3").text("Rohan Patel · Grade 4")))
        .column(kanban_column("Interview")
            .add(kanban_card().id("k4").text("Diya Verma · Aug 8, 10:30 AM")))
        .column(kanban_column("Offer")
            .add(kanban_card().id("k5").text("Ishaan Thakur · Awaiting response")))
        .column(kanban_column("Enrolled")
            .add(kanban_card().id("k6").text("Kavya Nair · Joined Jul 15")));

    // ── Activity timeline — text_body() replaces four inline-styled Node::raw ──
    let activity = timeline()
        .item(timeline_item().icon(Icons::CHECK).tone(TimelineTone::Success).time("10:24 AM")
              .add(text_body("Fees paid", "Invoice #INV-1042 · ₹4,500")))
        .item(timeline_item().icon(Icons::EDIT).tone(TimelineTone::Info).time("Yesterday")
              .add(text_body("Profile updated", "Guardian phone changed")))
        .item(timeline_item().icon(Icons::BELL).tone(TimelineTone::Warning).time("Aug 10")
              .add(text_body("Late arrival flagged", "Third late day this month")))
        .item(timeline_item().icon(Icons::PLUS).tone(TimelineTone::Muted).time("Jul 15")
              .add(text_body("Admitted", "Grade 5-B")));

    // ── Progress cards ──
    let progress_cards = grid().cols_min(MinCol::W240)
        .add(card().add(progress(72).label("Syllabus (Grade 5)").tone(ProgTone::Success).show_value()))
        .add(card().add(progress(45).label("Warning zone").tone(ProgTone::Warning).show_value()))
        .add(card().add(progress(90).label("Attendance").tone(ProgTone::Info).show_value()));

    // ── Compose the page ──
    page_of("Dashboard · ERP demo",
        page_shell()
            .add(demo_nav())

            // Header
            .add(toolbar()
                .add(breadcrumb().item(Crumb::current("Dashboard")))
                .add(spacer())
                .add(dropdown_menu().align(MenuAlign::End)
                    .trigger(button().label("Actions").variant(Variant::Secondary).icon(Icons::MORE))
                    .item(MenuItem::action("New announcement"))
                    .item(MenuItem::link("Add student", "/dsl/students"))
                    .item(MenuItem::link("New invoice", "/dsl/fees"))
                    .divider()
                    .item(MenuItem::action("Export dashboard PDF")))
                .add(button().label("New announcement").variant(Variant::Primary).icon(Icons::BELL)))

            // KPIs
            .add(kpis)

            // Progress row
            .add(section().title("This term").subtitle("Aug 2026")
                .add(progress_cards))

            // Kanban + timeline — standard responsive two-column pattern
            .add(two_col_with(3, 2,
                section().title("Admissions pipeline")
                    .subtitle("Drag cards to move between stages")
                    .action(button().label("See all").variant(Variant::Ghost).size(Size::Sm).icon(Icons::CHEVRON_RIGHT))
                    .add(card().add(kanban_board)),
                section().title("Recent activity")
                    .add(card().add(activity))))

            // Toast host so client-side JS can fire notifications
            .add(toast_host())
    )
}
