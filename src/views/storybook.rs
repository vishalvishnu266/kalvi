use crate::views::components;

pub fn render(page: u32) -> String {
    let sidebar_items = vec![
        ("Dashboard", //language=HTML
        r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path></svg>"#, page == 1, "/storybook/1"),
        ("Forms & Inputs", //language=HTML
        r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>"#, page == 2, "/storybook/2"),
        ("UI Elements", //language=HTML
        r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path></svg>"#, false, "#"),
    ];

    let content = match page {
        2 => render_forms_page(),
        _ => render_dashboard_page(),
    };

    crate::views::layout::app_layout("Kalvi ERP Storybook", sidebar_items, &content)
}

fn render_dashboard_page() -> String {
    let stats = format!(
        //language=HTML
        r##"<div class="row g-4 mb-5">
            <div class="col-md-4">{s1}</div>
            <div class="col-md-4">{s2}</div>
            <div class="col-md-4">{s3}</div>
        </div>"##,
        s1 = components::stats_card("Q3 Revenue", "$248,500", "+18.2%", true),
        s2 = components::stats_card("Active Projects", "42", "+4", true),
        s3 = components::stats_card("Risk Factor", "2.4%", "-0.5%", false)
    );

    let headers = vec!["Project Name", "Lead", "Budget", "Status", "Deadline"];
    let rows = vec![
        vec!["Aurora ERP".to_string(), "Sarah Chen".to_string(), "$120k".to_string(), components::badge("In Progress", "blue"), "Oct 24".to_string()],
        vec!["Nebula Cloud".to_string(), "Mike Ross".to_string(), "$85k".to_string(), components::badge("Completed", "green"), "Sep 12".to_string()],
        vec!["Titan Infrastructure".to_string(), "Alex Vance".to_string(), "$340k".to_string(), components::badge("Critical", "red"), "Aug 30".to_string()],
    ];

    format!(
        //language=HTML
        r##"
        {header}
        
        {stats}
        
        <div class="mt-5">
            <div class="d-flex align-items-center justify-content-between mb-4">
                <h3 class="fw-bold text-body">Active Engagements</h3>
                {button}
            </div>
            <div class="card glass-card border-0 shadow-sm overflow-hidden p-0">
                {table}
            </div>
        </div>
        "##,
        header = components::page_header("Executive", "Overview", "Real-time performance metrics and high-level project statuses for your enterprise."),
        stats = stats,
        table = components::table(headers, rows),
        button = components::button("Export Report", "secondary", Some("download"))
    )
}

fn render_forms_page() -> String {
    let form_fields = format!(
        //language=HTML
        r##"
        <div class="row g-5">
            <div class="col-md-6">
                <h5 class="fw-bold mb-4 d-flex align-items-center gap-2">
                    <i class="bi bi-person-badge text-primary"></i>
                    Identity Information
                </h5>
                {f1}
                {f2}
                {f3}
            </div>
            <div class="col-md-6">
                <h5 class="fw-bold mb-4 d-flex align-items-center gap-2">
                    <i class="bi bi-sliders text-primary"></i>
                    System Preferences
                </h5>
                {f4}
                {f5}
                {f6}
            </div>
        </div>
        <div class="mt-5 pt-4 border-top d-flex flex-column flex-sm-row justify-content-end gap-3">
            {b1}
            {b2}
        </div>
        "##,
        f1 = components::form_input("Full Display Name", "name", "text", "e.g. Jonathan Smith", None),
        f2 = components::form_input("Recovery Email", "email", "email", "jonathan@acme.com", Some("Email is already registered in our system")),
        f3 = components::form_select("Primary Role", "role", vec![("admin", "Administrator"), ("editor", "Editor"), ("viewer", "Viewer")]),
        f4 = components::form_toggle("Enable Multi-Factor Authentication", "mfa"),
        f5 = components::form_toggle("Beta Feature Access", "beta"),
        f6 = components::form_checkbox("Usage Analytics", "analytics", "Share anonymous usage data to help us improve your experience."),
        b1 = components::button("Cancel Changes", "secondary", Some("x-circle")),
        b2 = components::button("Save Configuration", "primary", Some("check-circle"))
    );

    format!(
        //language=HTML
        r##"
        {header}

        <div class="card glass-card border-0 shadow-sm p-4 p-md-5">
            {fields}
        </div>
        "##,
        header = components::page_header("Interface", "Elements", "A comprehensive set of professional form controls with validation states and dark mode support."),
        fields = form_fields
    )
}
