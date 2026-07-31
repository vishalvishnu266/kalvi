//! ERP "Attendance" page — daily register with P/L/A segmented control per
//! student, plus KPI summary at the top and quick date-navigation controls.
//!
//! This page is the **reference** for the standardised responsive layout
//! system. Notice:
//!
//! * The KPI row uses `grid().cols_min(MinCol::W200)` — auto-fit, no
//!   inline CSS, reflows fluidly.
//! * The two-column body uses `.mobile_stack()` so it becomes a single
//!   column on mobile (≤ 640 px). The Notes card slides underneath the
//!   Roll call, both full width.
//! * Column widths use typed `.min_w(MinW::W300)` — not a raw px string.
//!
//! All classes are defined once in `lit-components/assets/layout.css` and
//! loaded automatically by `page()`.

use crate::prelude::*;

pub struct AttendanceRow<'a> {
    pub name: &'a str,
    pub roll: u32,
    pub status: &'a str,   // "present" | "late" | "absent"
}

pub fn mock_class() -> Vec<AttendanceRow<'static>> {
    vec![
        AttendanceRow { name: "Aarav Kumar",   roll: 12, status: "present" },
        AttendanceRow { name: "Meera Sharma",  roll: 13, status: "late" },
        AttendanceRow { name: "Rohan Patel",   roll: 14, status: "absent" },
        AttendanceRow { name: "Diya Verma",    roll: 15, status: "present" },
        AttendanceRow { name: "Ishaan Thakur", roll: 16, status: "present" },
        AttendanceRow { name: "Kavya Nair",    roll: 17, status: "present" },
        AttendanceRow { name: "Priya Mehta",   roll: 18, status: "late" },
        AttendanceRow { name: "Aryan Iyer",    roll: 19, status: "present" },
        AttendanceRow { name: "Anaya Rao",     roll: 20, status: "absent" },
        AttendanceRow { name: "Vihaan Khan",   roll: 21, status: "present" },
    ]
}

pub fn build(class_name: &str, date_iso: &str, rows: &[AttendanceRow]) -> Page {
    let present = rows.iter().filter(|r| r.status == "present").count();
    let late    = rows.iter().filter(|r| r.status == "late").count();
    let absent  = rows.iter().filter(|r| r.status == "absent").count();
    let total   = rows.len();

    // KPI cards — auto-fit grid, reflows down to 1 column on mobile
    let kpis = grid().cols_min(MinCol::W200).gap(Gap::Md)
        .add(stat("Roll strength", total.to_string()).icon("users"))
        .add(stat("Present",       present.to_string()).icon("check").trend(Trend::Up).delta("+2"))
        .add(stat("Late",          late.to_string()).icon("bell"))
        .add(stat("Absent",        absent.to_string()).icon("x").trend(Trend::Down).delta("-1"));

    // Roll list with P/L/A segmented control per row
    let mut roll_list = card().padded();
    for r in rows {
        let seg = segmented().value(r.status)
            .segment(Segment::new("present", "P"))
            .segment(Segment::new("late",    "L"))
            .segment(Segment::new("absent",  "A"));
        roll_list = roll_list.add(
            list_item(r.name.to_string())
                .subtitle(format!("Roll {}", r.roll))
                .leading(avatar(r.name.to_string()).size(AvatarSize::Sm))
                .trailing(seg)
        );
    }

    // Notes form
    let notes_form = form().action("/attendance/notes").method("post")
        .add(input().label("Reason").name("reason").placeholder("e.g. Medical leave"))
        .add(input().label("Details").name("details").kind(InputType::Textarea).hint("Optional"))
        .action_btn(button().label("Cancel").variant(Variant::Secondary))
        .action_btn(button().label("Save").variant(Variant::Primary));

    page().title(format!("Attendance · {} · {}", class_name, date_iso)).add(
        container().size(ContainerSize::Lg).add(column().gap(Gap::Lg)

            // Header row — also stacks on mobile so the datepicker gets its own line
            .add(row().align(Align::Center).gap(Gap::Md).mobile_stack()
                .add(breadcrumb()
                    .item(Crumb::link("Home", "#/"))
                    .item(Crumb::link("Attendance", "#/attendance"))
                    .item(Crumb::current(class_name.to_string())))
                .add(spacer())
                .add(datepicker().value(date_iso.to_string()).label(""))
                .add(button().label("Export").variant(Variant::Secondary).icon("upload"))
                .add(button().label("Save changes").variant(Variant::Primary).icon("check")))

            // KPIs
            .add(section().title(format!("Class {}", class_name))
                .subtitle(format!("Register for {}", date_iso))
                .add(kpis))

            // ── TWO-COLUMN BODY — the reference responsive pattern ──
            // On desktop:  Roll call (flex 2) | Notes (flex 1)
            // On mobile:   Roll call stacked above Notes, both full width.
            .add(row().gap(Gap::Lg).align(Align::Start).mobile_stack()
                .add(column().flex(2).min_w(MinW::W320)
                    .add(section().title("Roll call")
                        .subtitle("Tap P / L / A to mark each student")
                        .action(button().label("Mark all present").variant(Variant::Ghost).size(Size::Sm).icon("check"))
                        .add(roll_list)))
                .add(column().flex(1).min_w(MinW::W260)
                    .add(section().title("Notes")
                        .add(card().padded().add(notes_form)))))
        )
    )
}
