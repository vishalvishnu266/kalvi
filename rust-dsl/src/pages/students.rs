//! ERP "Students" page — full page returned as a [`Page`] the caller can
//! render or embed.
//!
//! Uses the standard page presets from `layout.rs` (`page_of`,
//! `page_shell`, `toolbar`, `two_col_with`) and typed `Icons::*` so every
//! ERP page shares the same shell and visual rhythm.

use crate::prelude::*;

/// A single student row for the demo. Real code would pull this from your DB.
pub struct Student<'a> {
    pub name: &'a str,
    pub grade: u32,
    pub roll: u32,
    pub status: &'a str,   // "present" | "late" | "absent"
}

/// Small mock dataset — 8 students across grades 5-8.
pub fn mock_students() -> Vec<Student<'static>> {
    vec![
        Student { name: "Aarav Kumar",   grade: 5, roll: 12, status: "present" },
        Student { name: "Meera Sharma",  grade: 5, roll: 13, status: "late" },
        Student { name: "Rohan Patel",   grade: 5, roll: 14, status: "absent" },
        Student { name: "Diya Verma",    grade: 6, roll: 21, status: "present" },
        Student { name: "Ishaan Thakur", grade: 6, roll: 22, status: "present" },
        Student { name: "Kavya Nair",    grade: 7, roll: 33, status: "late" },
        Student { name: "Priya Mehta",   grade: 7, roll: 34, status: "present" },
        Student { name: "Aryan Iyer",    grade: 8, roll: 41, status: "absent" },
    ]
}

pub fn build(students: &[Student]) -> Page {
    // ── KPIs ──
    let present = students.iter().filter(|s| s.status == "present").count();
    let late    = students.iter().filter(|s| s.status == "late").count();
    let absent  = students.iter().filter(|s| s.status == "absent").count();

    let kpis = grid().cols_min(MinCol::W200)
        .add(stat("Total students", students.len().to_string()).icon(Icons::USERS).trend(Trend::Up).delta("+2.1%"))
        .add(stat("Present today",  present.to_string()).icon(Icons::CHECK).trend(Trend::Up).delta("+3.2%"))
        .add(stat("Late arrivals",  late.to_string()).icon(Icons::BELL).trend(Trend::Up).delta("+0.8%"))
        .add(stat("Absent",         absent.to_string()).icon(Icons::X).trend(Trend::Down).delta("-1.4%"));

    // ── Students table ──
    let mut table = data_table("students-table")
        .searchable().selectable().per_page(5)
        .col("name",   "Name",   ColOpts::text().sortable())
        .col("grade",  "Grade",  ColOpts::text().sortable().center())
        .col("roll",   "Roll",   ColOpts::text().sortable().right())
        .col("status", "Status",
             ColOpts::render(
                 "(v)=>`<ui-badge tone=\"${v==='present'?'success':v==='late'?'warning':'danger'}\" dot>${v}</ui-badge>`"
             ).sortable());
    for s in students {
        table = table.row(vec![
            ("name",   s.name.to_string()),
            ("grade",  s.grade.to_string()),
            ("roll",   s.roll.to_string()),
            ("status", s.status.to_string()),
        ]);
    }

    // ── Add-student form (uses standard save_cancel() footer) ──
    let add_form = form().action("/students").method("post")
        .add(input().label("Full name").name("fullName").placeholder("e.g. Aarav Kumar").required())
        .add(input().label("Guardian email").name("email").kind(InputType::Email).required())
        .add(select().label("Grade").name("grade").placeholder("Choose…").required()
             .option(SelectOption::new("4", "Grade 4"))
             .option(SelectOption::new("5", "Grade 5"))
             .option(SelectOption::new("6", "Grade 6"))
             .option(SelectOption::new("7", "Grade 7"))
             .option(SelectOption::new("8", "Grade 8")))
        .add(radio_group("gender").value("F").horizontal()
             .option(radio("M", "Male"))
             .option(radio("F", "Female"))
             .option(radio("X", "Prefer not to say")))
        .add(checkbox("I consent to the school's data policy").name("consent").required())
        .add(switch("Send onboarding email").name("notify").checked())
        .save_cancel("Save");

    // ── Compose the page ──
    page_of("Students · ERP demo",
        page_shell()
            .add(toolbar()
                .add(breadcrumb()
                    .item(Crumb::link("Home", "#/"))
                    .item(Crumb::link("Students", "#/students"))
                    .item(Crumb::current("Grade 5")))
                .add(spacer())
                .add(button().label("Import CSV").variant(Variant::Secondary).icon(Icons::UPLOAD))
                .add(button().label("Add student").variant(Variant::Primary).icon(Icons::PLUS)))
            .add(kpis)
            .add(two_col_with(3, 1,
                section().title("All students").subtitle("Grade 5")
                    .action(button().label("Filters").variant(Variant::Ghost).size(Size::Sm).icon(Icons::FILTER))
                    .add(card().add(table)),
                section().title("Quick add").subtitle("New student")
                    .add(card().add(add_form))))
    )
}
