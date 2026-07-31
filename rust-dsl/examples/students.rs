//! A realistic ERP "Students" page composed entirely with `lit-ui`.
//!
//! Demonstrates:
//!   * `page()` + `container()` for width control (fluid vs centered)
//!   * `row()` / `column()` / `grid()` for responsive layout
//!   * KPI stats row that auto-wraps on narrow screens
//!   * `breadcrumb` + `card` header
//!   * `data_table` populated from typed Rust rows (with badge cell renderer)
//!   * `form` with `input`, `select`, `checkbox`, `switch` + submit button
//!
//! Run — this writes the file directly (no shell redirection):
//!
//! ```sh
//! cargo run -p lit-ui --example students
//! # -> ../lit-components/dsl-students.html
//! # open http://localhost:3000/lit-components/dsl-students.html
//! ```

use lit_ui::prelude::*;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // ------------------------- KPI stats -------------------------
    let kpis = grid()
        .cols_min("220px")
        .gap(Gap::Md)
        .add(stat("Total students", "1,248").icon("users").trend(Trend::Up).delta("+2.1%"))
        .add(stat("Present today", "1,102").icon("check").trend(Trend::Up).delta("+3.2%"))
        .add(stat("Late arrivals", "47").icon("bell").trend(Trend::Up).delta("+0.8%"))
        .add(stat("Absent",       "99").icon("x").trend(Trend::Down).delta("-1.4%"));

    // ------------------------- Students table -------------------------
    // Fake dataset — in a real handler this would come from your DB.
    let rows: Vec<(&str, u32, u32, &str)> = vec![
        ("Aarav Kumar",  5, 12, "present"),
        ("Meera Sharma", 5, 13, "late"),
        ("Rohan Patel",  5, 14, "absent"),
        ("Diya Verma",   6, 21, "present"),
        ("Ishaan Thakur",6, 22, "present"),
        ("Kavya Nair",   7, 33, "late"),
        ("Priya Mehta",  7, 34, "present"),
        ("Aryan Iyer",   8, 41, "absent"),
    ];

    let mut students = data_table("students-table")
        .searchable()
        .selectable()
        .per_page(5)
        .col("name",  "Name",  ColOpts::text().sortable())
        .col("grade", "Grade", ColOpts::text().sortable().center())
        .col("roll",  "Roll",  ColOpts::text().sortable().right())
        .col("status","Status",
             ColOpts::render(
                 "(v)=>`<ui-badge tone=\"${v==='present'?'success':v==='late'?'warning':'danger'}\" dot>${v}</ui-badge>`"
             ).sortable());
    for (name, grade, roll, status) in rows {
        students = students.row(vec![
            ("name",   name.to_string()),
            ("grade",  grade.to_string()),
            ("roll",   roll.to_string()),
            ("status", status.to_string()),
        ]);
    }

    // ------------------------- Add-student form -------------------------
    let add_form = form()
        .action("/students")
        .method("post")
        .add(input().label("Full name").name("fullName")
                    .placeholder("e.g. Aarav Kumar").required())
        .add(input().label("Guardian email").name("email")
                    .kind(InputType::Email).required())
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
        .action_btn(button().label("Reset").variant(Variant::Secondary))
        .action_btn(button().label("Save").variant(Variant::Primary).icon("check"));

    // ------------------------- Layout -------------------------
    // Body is fluid; container() centers and caps width at 1200px.
    let html = page()
        .title("Students · ERP demo")
        .add(container()
            .max_width("1200px")
            .add(column().gap(Gap::Lg)

                // Header row: breadcrumb + primary CTA
                .add(row().align(Align::Center).gap(Gap::Md)
                    .add(breadcrumb()
                        .item(Crumb::link("Home", "#/"))
                        .item(Crumb::link("Students", "#/students"))
                        .item(Crumb::current("Grade 5")))
                    .add(spacer())
                    .add(button().label("Import CSV").variant(Variant::Secondary).icon("upload"))
                    .add(button().label("Add student").variant(Variant::Primary).icon("plus")))

                // KPIs
                .add(kpis)

                // Main two-column area: table + side card
                .add(row().gap(Gap::Lg).align(Align::Start)
                    .add(column().flex(3).min_width("300px")
                        .add(card().title("All students").subtitle("Grade 5").padded()
                            .action(button().label("Filters").variant(Variant::Ghost).size(Size::Sm).icon("filter"))
                            .add(students)))
                    .add(column().flex(1).min_width("280px")
                        .add(card().title("Quick add").subtitle("New student").padded()
                            .add(add_form)))))
        )
        .render();

    // Write next to the Lit components so the local static server finds it at
    // `/lit-components/dsl-students.html` immediately.
    let out = output_path("dsl-students.html");
    fs::create_dir_all(out.parent().unwrap())?;
    fs::write(&out, &html)?;

    eprintln!("wrote {} bytes → {}", html.len(), out.display());
    eprintln!("open  → http://localhost:3000/lit-components/dsl-students.html");
    Ok(())
}

/// Resolves `<repo-root>/lit-components/<file>` relative to this source file.
fn output_path(filename: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.parent().unwrap().join("lit-components").join(filename)
}
