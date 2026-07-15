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
        r#"<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8 mb-12">
            {s1}
            {s2}
            {s3}
        </div>"#,
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
        r#"
        {header}
        
        {stats}
        
        <div class="mt-16">
            <div class="flex items-center justify-between mb-8">
                <h2 class="text-2xl font-bold text-slate-800 dark:text-white">Active Engagements</h2>
                {button}
            </div>
            {table}
        </div>
        "#,
        header = components::page_header("Executive", "Overview", "Real-time performance metrics and high-level project statuses for your enterprise."),
        stats = stats,
        table = components::table(headers, rows),
        button = components::button("Export Report", "secondary", Some(r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>"#))
    )
}

fn render_forms_page() -> String {
    let form_fields = format!(
        //language=HTML
        r#"
        <div class="grid grid-cols-1 md:grid-cols-2 gap-10">
            <div class="space-y-6">
                <h3 class="text-lg font-bold mb-4 flex items-center gap-2">
                    <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path></svg>
                    Identity Information
                </h3>
                {f1}
                {f2}
                {f3}
            </div>
            <div class="space-y-6">
                <h3 class="text-lg font-bold mb-4 flex items-center gap-2">
                    <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"></path></svg>
                    System Preferences
                </h3>
                {f4}
                {f5}
                {f6}
            </div>
        </div>
        <div class="mt-12 pt-8 border-t border-slate-100 dark:border-slate-800/50 flex justify-end gap-4">
            {b1}
            {b2}
        </div>
        "#,
        f1 = components::form_input("Full Display Name", "name", "text", "e.g. Jonathan Smith", None),
        f2 = components::form_input("Recovery Email", "email", "email", "jonathan@acme.com", Some("Email is already registered in our system")),
        f3 = components::form_select("Primary Role", "role", vec![("admin", "Administrator"), ("editor", "Editor"), ("viewer", "Viewer")]),
        f4 = components::form_toggle("Enable Multi-Factor Authentication", "mfa"),
        f5 = components::form_toggle("Beta Feature Access", "beta"),
        f6 = components::form_checkbox("Usage Analytics", "analytics", "Share anonymous usage data to help us improve your experience."),
        b1 = components::button("Cancel Changes", "secondary", Some(r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>"#)),
        b2 = components::button("Save Configuration", "primary", Some(r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4"></path></svg>"#))
    );

    format!(
        //language=HTML
        r#"
        {header}

        <div class="glass-card p-6 sm:p-10">
            {fields}
        </div>
        "#,
        header = components::page_header("Interface", "Elements", "A comprehensive set of professional form controls with validation states and dark mode support."),
        fields = form_fields
    )
}
