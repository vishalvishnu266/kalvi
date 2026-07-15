use crate::views::components;
use crate::controllers::dashboard_controller::Student;

pub fn render(tenant_id: &str, student: Option<Student>) -> String {
    let is_edit = student.is_some();
    let title = if is_edit { "Edit Student" } else { "Add New Student" };
    let action_url = if let Some(ref s) = student {
        format!("/web/{}/students/{}/update", tenant_id, s.id)
    } else {
        format!("/web/{}/students/create", tenant_id)
    };

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

    let s = student.unwrap_or(Student {
        id: "".to_string(),
        name: "".to_string(),
        grade: "".to_string(),
        section: "".to_string(),
        status: "Present".to_string(),
        attendance_pct: 100.0,
    });

    let content = format!(
        //language=HTML
        r##"
        <div class="mb-4">
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    <li class="breadcrumb-item"><a href="{students_link}" class="text-decoration-none">Students</a></li>
                    <li class="breadcrumb-item active">{title}</li>
                </ol>
            </nav>
            {header}
        </div>

        <div class="row">
            <div class="col-lg-8">
                <div class="card glass-card border-0 shadow-sm p-4">
                    <form action="{action_url}" method="POST">
                        <div class="row">
                            <div class="col-md-12">
                                {name_input}
                            </div>
                            <div class="col-md-6">
                                {grade_select}
                            </div>
                            <div class="col-md-6">
                                {section_input}
                            </div>
                            <div class="col-md-6">
                                {status_select}
                            </div>
                            <div class="col-md-6">
                                {attendance_input}
                            </div>
                        </div>
                        
                        <hr class="my-4 opacity-50">
                        
                        <div class="d-flex justify-content-end gap-2">
                            <a href="{students_link}" class="btn btn-light border px-4">Cancel</a>
                            <button type="submit" class="btn btn-primary px-5 fw-bold">{submit_label}</button>
                        </div>
                    </form>
                </div>
            </div>
            
            <div class="col-lg-4">
                <div class="card bg-primary-subtle border-0 p-4 mb-4">
                    <h5 class="fw-bold text-primary mb-3"><i class="bi bi-info-circle-fill me-2"></i>Quick Tips</h5>
                    <p class="small text-primary-emphasis mb-0">
                        Ensure the student's name matches their official identification. Grade and section are required for proper classroom assignment.
                    </p>
                </div>
            </div>
        </div>
        "##,
        students_link = students_link,
        title = title,
        header = components::page_header("Institution", title, "Fill in the details below to manage student records."),
        action_url = action_url,
        submit_label = if is_edit { "Update Student" } else { "Register Student" },
        name_input = components::form_input("Full Name", "name", "text", "e.g. John Doe", &s.name, None),
        grade_select = components::form_select("Grade / Class", "grade", vec![
            ("9th", "9th Grade"), ("10th", "10th Grade"), ("11th", "11th Grade"), ("12th", "12th Grade")
        ], &s.grade),
        section_input = components::form_input("Section", "section", "text", "e.g. A, B or C", &s.section, None),
        status_select = components::form_select("Status", "status", vec![
            ("Present", "Present"), ("Absent", "Absent"), ("Late", "Late")
        ], &s.status),
        attendance_input = components::form_input("Attendance %", "attendance_pct", "number", "0-100", &s.attendance_pct.to_string(), None)
    );

    crate::views::layout::app_layout("Student Management | Kalvi ERP", sidebar_items, &content)
}
