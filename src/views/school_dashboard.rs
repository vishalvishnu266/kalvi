use crate::views::components;
use crate::controllers::dashboard_controller::Student;

pub fn render(tenant_id: &str, students: Vec<Student>) -> String {
    let dashboard_link = format!("/web/{}/dashboard", tenant_id);
    let students_link = format!("/web/{}/students", tenant_id);
    
    let sidebar_items = vec![
        ("Dashboard", "speedometer2", true, dashboard_link.as_str()),
        ("Students", "people", false, students_link.as_str()),
        ("Attendance", "calendar-check", false, "#"),
        ("Fees", "cash-stack", false, "#"),
        ("Exams", "journal-bookmark", false, "#"),
        ("Settings", "gear", false, "#"),
    ];

    let stats = format!(
        //language=HTML
        r##"<div class="row g-4 mb-5">
            <div class="col-sm-6 col-lg-3">{s1}</div>
            <div class="col-sm-6 col-lg-3">{s2}</div>
            <div class="col-sm-6 col-lg-3">{s3}</div>
            <div class="col-sm-6 col-lg-3">{s4}</div>
        </div>"##,
        s1 = components::stats_card("Total Students", &format!("{}", students.len()), "+0", true),
        s2 = components::stats_card("Average Attendance", "94.2%", "+0%", true),
        s3 = components::stats_card("Fee Collection", "$0", "0%", false),
        s4 = components::stats_card("Upcoming Exams", "0", "None", true)
    );

    let headers = vec!["Student Name", "Grade", "Section", "Status", "Attendance"];
    let rows: Vec<Vec<String>> = students.into_iter().map(|s| {
        let status_color = match s.status.as_str() {
            "Present" => "green",
            "Absent" => "red",
            "Late" => "blue",
            _ => "gray"
        };
        vec![
            s.name,
            s.grade,
            s.section,
            components::badge(&s.status, status_color),
            format!("{:.1}%", s.attendance_pct)
        ]
    }).collect();

    let content = format!(
        //language=HTML
        r##"
        {header}
        
        {stats}
        
        <div class="row g-4">
            <div class="col-lg-8">
                <div class="d-flex align-items-center justify-content-between mb-4">
                    <h3 class="fw-bold text-body">Recent Student Activity</h3>
                    {button}
                </div>
                <div class="card glass-card border-0 shadow-sm overflow-hidden p-0">
                    {table}
                </div>
            </div>
            <div class="col-lg-4">
                <div class="d-flex flex-column gap-4">
                    {card1}
                    {card2}
                </div>
            </div>
        </div>
        "##,
        header = components::page_header("Institution", "Overview", &format!("Monitoring metrics for Tenant ID: {}", tenant_id)),
        stats = stats,
        table = components::table(headers, rows),
        button = components::button("Add Student", "primary", Some("plus-lg")),
        card1 = components::notice_list("Recent Notices", vec![
            ("System successfully initialized for this tenant.", true),
        ]),
        card2 = components::quick_actions(vec![
            "Take Attendance", "Generate Report", "Collect Fees", "Send SMS"
        ])
    );

    crate::views::layout::app_layout("School ERP Dashboard", sidebar_items, &content)
}
