//! ERP "Attendance" page — daily register with P/L/A segmented control per
//! student, plus KPI summary at the top and quick date-navigation controls.
//!
//! Uses the standard page presets from `layout.rs`:
//! * `page_of` + `page_shell` — outer container and vertical rhythm.
//! * `toolbar`  — top row (breadcrumb + actions), auto-stacks on mobile.
//! * `two_col_with` — responsive two-column body (roll call + notes).
//! * `Icons::*` — typed icon constants.

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

    // ── KPIs — auto-fit grid, reflows down to 1 column on mobile ──
    let kpis = grid().cols_min(MinCol::W200)
        .add(stat("Roll strength", total.to_string()).icon(Icons::USERS))
        .add(stat("Present",       present.to_string()).icon(Icons::CHECK).trend(Trend::Up).delta("+2"))
        .add(stat("Late",          late.to_string()).icon(Icons::BELL))
        .add(stat("Absent",        absent.to_string()).icon(Icons::X).trend(Trend::Down).delta("-1"));

    // ── Roll list with P/L/A segmented control per row ──
    let mut roll_list = card();
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

    // ── Notes form (uses standard save_cancel() footer) ──
    let notes_form = form().action("/attendance/notes").method("post")
        .add(input().label("Reason").name("reason").placeholder("e.g. Medical leave"))
        .add(input().label("Details").name("details").kind(InputType::Textarea).hint("Optional"))
        .save_cancel("Save");

    // ── Compose the page ──
    page_of(format!("Attendance · {} · {}", class_name, date_iso),
        page_shell()
            .add(toolbar()
                .add(breadcrumb()
                    .item(Crumb::link("Home", "#/"))
                    .item(Crumb::link("Attendance", "#/attendance"))
                    .item(Crumb::current(class_name.to_string())))
                .add(spacer())
                .add(datepicker().value(date_iso.to_string()).label(""))
                .add(button().label("Export").variant(Variant::Secondary).icon(Icons::UPLOAD))
                .add(button().label("Save changes").variant(Variant::Primary).icon(Icons::CHECK)))

            .add(section().title(format!("Class {}", class_name))
                .subtitle(format!("Register for {}", date_iso))
                .add(kpis))

            .add(two_col_with(2, 1,
                section().title("Roll call")
                    .subtitle("Tap P / L / A to mark each student")
                    .action(button().label("Mark all present").variant(Variant::Ghost).size(Size::Sm).icon(Icons::CHECK))
                    .add(roll_list),
                section().title("Notes")
                    .add(card().add(notes_form))))
    )
}
