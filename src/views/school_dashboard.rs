use crate::views::components;

pub fn render() -> String {
    let sidebar_items = vec![
        ("Dashboard", //language=HTML
        r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path></svg>"#, true, "/dashboard"),
        ("Students", //language=HTML
        r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>"#, false, "#"),
        ("Attendance", //language=HTML
        r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012-2"></path></svg>"#, false, "#"),
        ("Fees", //language=HTML
        r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 1.343-3 3s1.343 3 3 3 3 1.343 3 3-1.343 3-3 3m0-18c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm0 5h.01M12 12h.01M12 15h.01M12 18h.01"></path></svg>"#, false, "#"),
        ("Exams", //language=HTML
        r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path></svg>"#, false, "#"),
        ("Settings", //language=HTML
        r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>"#, false, "#"),
    ];

    let stats = format!(
        //language=HTML
        r##"
            {s1}
            {s2}
            {s3}
            {s4}
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
        
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
            <div class="lg:col-span-2">
                <div class="flex items-center justify-between mb-8">
                    <h2 class="text-2xl font-bold text-slate-800 dark:text-white">Recent Student Activity</h2>
                    {button}
                </div>
                {table}
            </div>
            <div class="space-y-8">
                {card1}
                {card2}
            </div>
        </div>
        "##,
        header = components::page_header("School", "Dashboard", "Overview of student performance, attendance, and institution metrics."),
        stats = stats,
        table = components::table(headers, rows),
        button = components::button("Add Student", "primary", Some(r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path></svg>"#)),
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
