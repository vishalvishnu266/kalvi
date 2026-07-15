use crate::views::components;

pub fn render() -> String {
    let sidebar_items = vec![
        ("Dashboard", "speedometer2", true, "/dashboard"),
        ("Students", "people", false, "#"),
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
        s1 = components::stats_card("Total Students", "1,248", "+12", true),
        s2 = components::stats_card("Average Attendance", "94.2%", "+2.1%", true),
        s3 = components::stats_card("Fee Collection", "$42.5k", "-1.2%", false),
        s4 = components::stats_card("Upcoming Exams", "3", "Next week", true)
    );

    let headers = vec!["Student Name", "Grade", "Section", "Status", "Attendance"];
    let rows = vec![
        vec!["Alice Johnson".to_string(), "10th".to_string(), "A".to_string(), components::badge("Present", "green"), "98%".to_string()],
        vec!["Bob Smith".to_string(), "11th".to_string(), "B".to_string(), components::badge("Absent", "red"), "85%".to_string()],
        vec!["Charlie Brown".to_string(), "9th".to_string(), "C".to_string(), components::badge("Late", "blue"), "92%".to_string()],
        vec!["David Wilson".to_string(), "12th".to_string(), "A".to_string(), components::badge("Present", "green"), "96%".to_string()],
    ];

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
        header = components::page_header("School", "Dashboard", "Overview of student performance, attendance, and institution metrics."),
        stats = stats,
        table = components::table(headers, rows),
        button = components::button("Add Student", "primary", Some("plus-lg")),
        card1 = components::notice_list("Notices", vec![
            ("Parent-Teacher meeting on Friday.", true),
            ("Winter break starts from Dec 20th.", false),
        ]),
        card2 = components::quick_actions(vec![
            "Take Attendance", "Generate Report", "Collect Fees", "Send SMS"
        ])
    );

    crate::views::layout::app_layout("School ERP Dashboard", sidebar_items, &content)
}
