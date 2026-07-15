use crate::views::components;
use crate::controllers::dashboard_controller::Student;

pub fn render(tenant_id: &str, students: Vec<Student>) -> String {
    let dashboard_link = format!("/web/{}/dashboard", tenant_id);
    let students_link = format!("/web/{}/students", tenant_id);
    
    let sidebar_items = vec![
        ("Dashboard", "speedometer2", false, dashboard_link.as_str()),
        ("Students", "people", true, students_link.as_str()),
        ("Attendance", "calendar-check", false, "#"),
        ("Fees", "cash-stack", false, "#"),
        ("Exams", "journal-bookmark", false, "#"),
        ("Settings", "gear", false, "#"),
    ];

    let headers = vec!["ID", "Student Name", "Grade", "Section", "Status", "Attendance", "Actions"];
    let rows: Vec<Vec<String>> = students.into_iter().map(|s| {
        let status_color = match s.status.as_str() {
            "Present" => "green",
            "Absent" => "red",
            "Late" => "blue",
            _ => "gray"
        };
        vec![
            format!("<span class='text-secondary small fw-bold'>#{}</span>", s.id),
            format!("<span class='fw-bold'>{}</span>", s.name),
            s.grade,
            s.section,
            components::badge(&s.status, status_color),
            format!("<div class='d-flex align-items-center gap-2'>
                        <div class='progress w-100' style='height: 6px;'>
                            <div class='progress-bar' role='progressbar' style='width: {att}%' aria-valuenow='{att}' aria-valuemin='0' aria-valuemax='100'></div>
                        </div>
                        <span class='small fw-bold'>{att}%</span>
                    </div>", att = s.attendance_pct),
            format!("<div class='d-flex gap-2'>
                        <a href='/web/{tid}/students/{sid}/edit' class='btn btn-sm btn-light border'><i class='bi bi-pencil'></i></a>
                        <form action='/web/{tid}/students/{sid}/delete' method='POST' onsubmit='return confirm(\"Are you sure?\")'>
                            <button type='submit' class='btn btn-sm btn-light border text-danger'><i class='bi bi-trash'></i></button>
                        </form>
                    </div>", tid = tenant_id, sid = s.id)
        ]
    }).collect();

    let create_link = format!("/web/{}/students/new", tenant_id);
    let btn_add_html = format!(r##"<a href="{}" class="text-decoration-none">{}</a>"##, create_link, components::button("Add Student", "primary", Some("plus-lg")));

    let content = format!(
        //language=HTML
        r##"
        <div class="d-flex align-items-center justify-content-between mb-4">
            {header}
            <div class="d-flex gap-2">
                {btn_export}
                {btn_add}
            </div>
        </div>

        <div class="card glass-card border-0 shadow-sm overflow-hidden p-0">
            <div class="p-4 border-bottom bg-light-subtle d-flex align-items-center justify-content-between">
                <div class="input-group style='max-width: 300px;'">
                    <span class="input-group-text bg-transparent border-end-0 text-secondary">
                        <i class="bi bi-search"></i>
                    </span>
                    <input type="text" class="form-control border-start-0 ps-0" placeholder="Search students...">
                </div>
                <div class="d-flex gap-3 align-items-center">
                    <select class="form-select form-select-sm bg-transparent border-0 fw-bold" style="width: auto;">
                        <option>All Grades</option>
                        <option>10th Grade</option>
                        <option>11th Grade</option>
                    </select>
                    <span class="text-secondary small">Showing {count} students</span>
                </div>
            </div>
            {table}
            <div class="p-3 border-top bg-light-subtle d-flex justify-content-center">
                <nav aria-label="Page navigation">
                    <ul class="pagination pagination-sm mb-0">
                        <li class="page-item disabled"><a class="page-link" href="#">Previous</a></li>
                        <li class="page-item active"><a class="page-link" href="#">1</a></li>
                        <li class="page-item"><a class="page-link" href="#">2</a></li>
                        <li class="page-item"><a class="page-link" href="#">Next</a></li>
                    </ul>
                </nav>
            </div>
        </div>
        "##,
        header = components::page_header("Student", "Management", "View and manage student records across the institution."),
        btn_add = btn_add_html,
        btn_export = components::button("Export", "secondary", Some("download")),
        table = components::table(headers, rows),
        count = students.len()
    );

    crate::views::layout::app_layout("Student Management | Kalvi ERP", sidebar_items, &content)
}
