use crate::views::components;
use crate::controllers::dashboard_controller::Student;

pub fn render(tenant_id: &str, students: Vec<Student>) -> String {
    let sidebar_items = vec![
        ("Dashboard", "house-chimney", true, &format!("/web/{}/dashboard", tenant_id)),
        ("Students", "user-graduate", false, &format!("/web/{}/students", tenant_id)),
        ("Attendance", "calendar-check", false, "#"),
        ("Fees", "file-invoice-dollar", false, "#"),
        ("Exams", "pen-to-square", false, "#"),
        ("Settings", "sliders", false, "#"),
    ];
    
    let stats = format!(
        //language=HTML
        r##"<div class="row g-3 mb-4">
            <div class="col-sm-6 col-lg-3">{s1}</div>
            <div class="col-sm-6 col-lg-3">{s2}</div>
            <div class="col-sm-6 col-lg-3">{s3}</div>
            <div class="col-sm-6 col-lg-3">{s4}</div>
        </div>"##,
        s1 = components::stat_card("Total Students", &format!("{}", students.len()), "+12%", "user-graduate", true),
        s2 = components::stat_card("Attendance", "94.2%", "+2.1%", "calendar-check", true),
        s3 = components::stat_card("Fee Collected", "$42.5k", "-1.2%", "circle-dollar-to-slot", false),
        s4 = components::stat_card("Active Exams", "3", "0", "file-lines", true)
    );

    let headers = vec!["Student", "Grade", "Section", "Status", "Attendance", ""];
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
            format!(r##"<button class="btn btn-soft btn-sm"><i class="fa-solid fa-chevron-right"></i></button>"##)
        ]
    }).collect();

    let content = format!(
        //language=HTML
        r##"
        {header}
        {stats}
        
        <div class="card border-0 shadow-sm mb-4">
          <div class="card-header bg-transparent border-0 d-flex align-items-center justify-content-between p-4 pb-0">
            <h5 class="fw-bold mb-0">Recent Activity</h5>
            <a href="/web/{tid}/students" class="btn btn-soft btn-sm fw-bold">View All</a>
          </div>
          <div class="card-body p-4">
            {table}
          </div>
        </div>
        "##,
        header = components::page_header("Dashboard Overview", "Welcome back to your administration portal.", vec![("Home", "/"), ("Dashboard", "#")]),
        stats = stats,
        table = components::table(headers, rows),
        tid = tenant_id
    );

    crate::views::layout::app_layout("Dashboard", sidebar_items, &content)
}
