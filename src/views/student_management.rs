use crate::views::components;
use crate::controllers::dashboard_controller::Student;

pub fn render(tenant_id: &str, students: Vec<Student>) -> String {
    let sidebar_items = vec![
        ("Dashboard", "house-chimney", false, &format!("/web/{}/dashboard", tenant_id)),
        ("Students", "user-graduate", true, &format!("/web/{}/students", tenant_id)),
        ("Attendance", "calendar-check", false, "#"),
        ("Fees", "file-invoice-dollar", false, "#"),
        ("Exams", "pen-to-square", false, "#"),
        ("Settings", "sliders", false, "#"),
    ];

    let headers = vec!["Student Name", "Grade", "Section", "Status", "Attendance", "Actions"];
    let rows: Vec<Vec<String>> = students.into_iter().map(|s| {
        let variant = match s.status.as_str() {
            "Present" => "success",
            "Absent" => "danger",
            "Late" => "warning",
            _ => "info"
        };
        vec![
            format!(r##"
                <div class="d-flex align-items-center gap-2">
                    <img src="https://ui-avatars.com/api/?name={}&background=random" class="rounded-circle" width="32">
                    <span class="fw-medium">{}</span>
                </div>
            "##, s.name, s.name),
            s.grade,
            s.section,
            components::badge(&s.status, variant),
            format!(r##"
                <div class="d-flex align-items-center gap-2" style="width: 100px;">
                    <div class="progress w-100" style="height: 6px;">
                        <div class="progress-bar bg-accent" style="width: {}%"></div>
                    </div>
                    <span class="smaller fw-bold">{}%</span>
                </div>
            "##, s.attendance_pct, s.attendance_pct),
            format!(r##"
                <div class="d-flex gap-2">
                    <a href="/web/{tid}/students/{sid}/edit" class="btn btn-soft btn-sm"><i class="fa-solid fa-pen"></i></a>
                    <form action="/web/{tid}/students/{sid}/delete" method="POST" onsubmit="return confirm('Delete student?')">
                        {csrf}
                        <button type="submit" class="btn btn-soft btn-sm text-danger"><i class="fa-solid fa-trash"></i></button>
                    </form>
                </div>
            "##, tid = tenant_id, sid = s.id, csrf = components::csrf_input())
        ]
    }).collect();

    let content = format!(
        //language=HTML
        r##"
        {header}

        <div class="card border-0 shadow-sm overflow-hidden p-0">
          <div class="card-header bg-transparent border-0 d-flex align-items-center justify-content-between p-4 pb-2">
            <div class="d-flex gap-2">
                <a href="/web/{tid}/students/new" class="btn btn-accent fw-bold px-4">
                    <i class="fa-solid fa-plus me-2"></i>Add Student
                </a>
                <button class="btn btn-soft fw-bold"><i class="fa-solid fa-download me-2"></i>Export</button>
            </div>
            <div class="d-flex gap-2 align-items-center">
                <span class="text-muted-custom small">Total: {count}</span>
            </div>
          </div>
          <div class="card-body p-0">
            {table}
          </div>
          <div class="card-footer bg-transparent border-0 p-4 pt-0">
            <nav aria-label="Page navigation">
                <ul class="pagination pagination-sm justify-content-center mb-0">
                    <li class="page-item active"><a class="page-link" href="#">1</a></li>
                    <li class="page-item"><a class="page-link" href="#">2</a></li>
                </ul>
            </nav>
          </div>
        </div>
        "##,
        header = components::page_header("Student Management", "Manage your institution's student records.", vec![("Home", "/"), ("Students", "#")]),
        tid = tenant_id,
        count = rows.len(),
        table = components::table(headers, rows)
    );

    crate::views::layout::app_layout("Students", sidebar_items, &content)
}
