use crate::views::components;

pub fn render(page: u32) -> String {
    let sidebar_items = vec![
        ("Dashboard", r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path></svg>"#, page == 1, "/storybook/1"),
        ("Forms & Inputs", r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>"#, page == 2, "/storybook/2"),
        ("UI Elements", r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path></svg>"#, false, "#"),
    ];

    let content = match page {
        2 => render_forms_page(),
        _ => render_dashboard_page(),
    };

    format!(
        r#"
        <div class="flex min-h-[90vh] max-w-[1600px] mx-auto rounded-[2rem] overflow-hidden shadow-2xl border border-white/20 relative z-10">
            {sidebar}
            <div class="flex-1 p-8 lg:p-16 overflow-y-auto bg-white/40 dark:bg-slate-900/40 backdrop-blur-md">
                {content}
            </div>
        </div>
        "#,
        sidebar = components::sidebar(sidebar_items),
        content = content
    )
}

fn render_dashboard_page() -> String {
    let stats = format!(
        r#"<div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-12">
            {}
            {}
            {}
        </div>"#,
        components::stats_card("Q3 Revenue", "$248,500", "+18.2%", true),
        components::stats_card("Active Projects", "42", "+4", true),
        components::stats_card("Risk Factor", "2.4%", "-0.5%", false)
    );

    let headers = vec!["Project Name", "Lead", "Budget", "Status", "Deadline"];
    let rows = vec![
        vec!["Aurora ERP".to_string(), "Sarah Chen".to_string(), "$120k".to_string(), components::badge("In Progress", "blue"), "Oct 24".to_string()],
        vec!["Nebula Cloud".to_string(), "Mike Ross".to_string(), "$85k".to_string(), components::badge("Completed", "green"), "Sep 12".to_string()],
        vec!["Titan Infrastructure".to_string(), "Alex Vance".to_string(), "$340k".to_string(), components::badge("Critical", "red"), "Aug 30".to_string()],
    ];

    format!(
        r#"
        <header class="mb-12">
            <h1 class="text-5xl font-black text-slate-800 dark:text-white mb-4 tracking-tighter">Executive <span class="text-primary">Overview</span></h1>
            <p class="text-slate-500 dark:text-slate-400 text-xl max-w-2xl">Real-time performance metrics and high-level project statuses for your enterprise.</p>
        </header>
        
        {stats}
        
        <div class="mt-16">
            <div class="flex items-center justify-between mb-8">
                <h2 class="text-2xl font-bold text-slate-800 dark:text-white">Active Engagements</h2>
                {}
            </div>
            {table}
        </div>
        "#,
        stats = stats,
        table = components::table(headers, rows),
        button = components::button("Export Report", "secondary")
    )
}

fn render_forms_page() -> String {
    let form_fields = format!(
        r#"
        <div class="grid grid-cols-1 md:grid-cols-2 gap-10">
            <div class="space-y-6">
                <h3 class="text-lg font-bold mb-4 flex items-center gap-2">
                    <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path></svg>
                    Identity Information
                </h3>
                {}
                {}
                {}
            </div>
            <div class="space-y-6">
                <h3 class="text-lg font-bold mb-4 flex items-center gap-2">
                    <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"></path></svg>
                    System Preferences
                </h3>
                {}
                {}
                {}
            </div>
        </div>
        <div class="mt-12 pt-8 border-t border-slate-100 dark:border-slate-800/50 flex justify-end gap-4">
            {}
            {}
        </div>
        "#,
        components::form_input("Full Display Name", "name", "text", "e.g. Jonathan Smith", None),
        components::form_input("Recovery Email", "email", "email", "jonathan@acme.com", Some("Email is already registered in our system")),
        components::form_select("Primary Role", "role", vec![("admin", "Administrator"), ("editor", "Editor"), ("viewer", "Viewer")]),
        components::form_toggle("Enable Multi-Factor Authentication", "mfa"),
        components::form_toggle("Beta Feature Access", "beta"),
        components::form_checkbox("Usage Analytics", "analytics", "Share anonymous usage data to help us improve your experience."),
        components::button("Cancel Changes", "secondary"),
        components::button("Save Configuration", "primary")
    );

    format!(
        r#"
        <header class="mb-12">
            <h1 class="text-5xl font-black text-slate-800 dark:text-white mb-4 tracking-tighter">Interface <span class="text-primary">Elements</span></h1>
            <p class="text-slate-500 dark:text-slate-400 text-xl max-w-2xl">A comprehensive set of professional form controls with validation states and dark mode support.</p>
        </header>

        <div class="glass-card p-10">
            {fields}
        </div>
        "#,
        fields = form_fields
    )
}
