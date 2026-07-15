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

    let sidebar_items = vec![
        ("Dashboard", "house-chimney", false, &format!("/web/{}/dashboard", tenant_id)),
        ("Students", "user-graduate", true, &format!("/web/{}/students", tenant_id)),
        ("Attendance", "calendar-check", false, "#"),
        ("Fees", "file-invoice-dollar", false, "#"),
        ("Exams", "pen-to-square", false, "#"),
        ("Settings", "sliders", false, "#"),
    ];

    let s = student.unwrap_or(Student {
        id: "".to_string(),
        name: "".to_string(),
        grade: "10th".to_string(),
        section: "A".to_string(),
        status: "Present".to_string(),
        attendance_pct: 100.0,
    });

    let content = format!(
        //language=HTML
        r##"
        {header}

        <div class="row">
          <div class="col-lg-8">
            <div class="card border-0 shadow-sm p-4">
              <form action="{action_url}" method="POST">
                {csrf}
                <div class="row g-3">
                  <div class="col-12">
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
                
                <hr class="my-4 opacity-5">
                
                <div class="d-flex justify-content-end gap-2">
                  <a href="/web/{tid}/students" class="btn btn-soft px-4 fw-bold">Cancel</a>
                  <button type="submit" class="btn btn-accent px-5 fw-bold">{submit_label}</button>
                </div>
              </form>
            </div>
          </div>
          
          <div class="col-lg-4">
            <div class="card border-0 shadow-sm p-4 bg-accent text-white mb-4">
              <h5 class="fw-bold mb-3"><i class="fa-solid fa-circle-info me-2"></i>Quick Guide</h5>
              <p class="small opacity-90 mb-0">
                Registering a student automatically creates their profile and assigns them to their respective class and section. 
              </p>
            </div>
          </div>
        </div>
        "##,
        header = components::page_header(title, "Fill in the required information below.", vec![("Home", "/"), ("Students", &format!("/web/{}/students", tenant_id)), (title, "#")]),
        action_url = action_url,
        csrf = components::csrf_input(),
        submit_label = if is_edit { "Update Changes" } else { "Save Student" },
        tid = tenant_id,
        name_input = components::form_input("Full Name", "name", "text", "Enter student's full name", &s.name, None),
        grade_select = components::form_select("Grade Level", "grade", vec![
            ("9th", "9th Grade"), ("10th", "10th Grade"), ("11th", "11th Grade"), ("12th", "12th Grade")
        ], &s.grade),
        section_input = components::form_input("Section", "section", "text", "e.g. A, B", &s.section, None),
        status_select = components::form_select("Current Status", "status", vec![
            ("Present", "Present"), ("Absent", "Absent"), ("Late", "Late")
        ], &s.status),
        attendance_input = components::form_input("Initial Attendance %", "attendance_pct", "number", "100", &s.attendance_pct.to_string(), None)
    );

    crate::views::layout::app_layout("Student Form", sidebar_items, &content)
}
