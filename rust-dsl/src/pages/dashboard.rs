//! ERP "Dashboard" landing page — KPI stats, admissions kanban, activity
//! timeline, and a quick actions bar.

use crate::prelude::*;

pub fn build() -> Page {
    // KPI stats
    let kpis = grid().cols_min("220px")
        .add(stat("Total students",     "1,248").icon("users").trend(Trend::Up).delta("+2.1%"))
        .add(stat("Present today",      "1,102").icon("check").trend(Trend::Up).delta("+3.2%"))
        .add(stat("Fee collection",     "82%").icon("card").trend(Trend::Up).delta("+5.6%"))
        .add(stat("Pending admissions", "17").icon("clipboard").trend(Trend::Down).delta("-3"));

    // Admissions kanban (mock)
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

    // Activity timeline
    let activity = timeline()
        .item(timeline_item().icon("check").tone(TimelineTone::Success).time("10:24 AM")
              .add(Node::raw("<strong>Fees paid</strong><div style=\"color:var(--color-text-muted);font-size:var(--fs-xs);\">Invoice #INV-1042 · ₹4,500</div>")))
        .item(timeline_item().icon("edit").tone(TimelineTone::Info).time("Yesterday")
              .add(Node::raw("<strong>Profile updated</strong><div style=\"color:var(--color-text-muted);font-size:var(--fs-xs);\">Guardian phone changed</div>")))
        .item(timeline_item().icon("bell").tone(TimelineTone::Warning).time("Aug 10")
              .add(Node::raw("<strong>Late arrival flagged</strong><div style=\"color:var(--color-text-muted);font-size:var(--fs-xs);\">Third late day this month</div>")))
        .item(timeline_item().icon("plus").tone(TimelineTone::Muted).time("Jul 15")
              .add(Node::raw("<strong>Admitted</strong><div style=\"color:var(--color-text-muted);font-size:var(--fs-xs);\">Grade 5-B</div>")));

    // Progress cards
    let progress_cards = grid().cols_min("240px")
        .add(card().padded().add(progress(72).label("Syllabus (Grade 5)").tone(ProgTone::Success).show_value()))
        .add(card().padded().add(progress(45).label("Warning zone").tone(ProgTone::Warning).show_value()))
        .add(card().padded().add(progress(90).label("Attendance").tone(ProgTone::Info).show_value()));

    page().title("Dashboard · ERP demo").add(
        container().max_width("1280px").add(column().gap(Gap::Lg)
            // Header
            .add(row().align(Align::Center).gap(Gap::Md)
                .add(breadcrumb().item(Crumb::current("Dashboard")))
                .add(spacer())
                .add(dropdown_menu().align(MenuAlign::End)
                    .trigger(button().label("Actions").variant(Variant::Secondary).icon("more"))
                    .item(MenuItem::action("New announcement"))
                    .item(MenuItem::link("Add student", "/dsl/students"))
                    .item(MenuItem::link("New invoice", "/dsl/fees"))
                    .divider()
                    .item(MenuItem::action("Export dashboard PDF")))
                .add(button().label("New announcement").variant(Variant::Primary).icon("bell")))

            // KPIs
            .add(kpis)

            // Progress row
            .add(section().title("This term").subtitle("Aug 2026")
                .add(progress_cards))

            // Kanban + timeline
            .add(row().gap(Gap::Lg).align(Align::Start)
                .add(column().flex(3).min_width("400px")
                    .add(section().title("Admissions pipeline")
                        .subtitle("Drag cards to move between stages")
                        .action(button().label("See all").variant(Variant::Ghost).size(Size::Sm).icon("chevronRight"))
                        .add(card().padded().add(kanban_board))))
                .add(column().flex(2).min_width("300px")
                    .add(section().title("Recent activity")
                        .add(card().padded().add(activity)))))

            // Toast host so client-side JS can fire notifications
            .add(toast_host())
        )
    )
}
