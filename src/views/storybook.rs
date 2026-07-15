use crate::views::components;

pub fn render(page: u32) -> String {
    let sidebar_items = vec![
        ("Dashboard", r#"<svg style="width:1.25rem" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path></svg>"#, page == 1, "/storybook/1"),
        ("Forms & Inputs", r#"<svg style="width:1.25rem" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>"#, page == 2, "/storybook/2"),
        ("UI Elements", r#"<svg style="width:1.25rem" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path></svg>"#, false, "#"),
    ];

    let content = match page {
        2 => render_forms_page(),
        _ => render_dashboard_page(),
    };

    format!(
        r#"
        <div class="d-flex rounded-4 rounded-lg-5 shadow-lg overflow-hidden border border-light-subtle position-relative flex-column flex-lg-row" style="background: rgba(var(--bs-body-bg-rgb), 0.4); backdrop-filter: blur(10px); min-height: 80vh;">
            {sidebar}
            <div class="flex-grow-1 p-3 p-lg-5 overflow-auto" style="max-height: 90vh;">
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
        r#"<div class="row g-4 mb-5">
            <div class="col-md-4">{}</div>
            <div class="col-md-4">{}</div>
            <div class="col-md-4">{}</div>
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
        <header class="mb-5">
            <h1 class="display-4 fw-black text-body mb-2 tracking-tighter">Executive <span class="text-primary">Overview</span></h1>
            <p class="lead text-muted">Real-time performance metrics and high-level project statuses.</p>
        </header>
        
        {stats}
        
        <div class="mt-5">
            <div class="d-flex align-items-center justify-content-between mb-4">
                <h2 class="h4 fw-bold mb-0">Active Engagements</h2>
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
    let modal_id = "bootstrap-demo-modal";
    let modal_html = components::modal(
        modal_id,
        "System Configuration",
        "Adjusting these settings will affect all users within your enterprise tenant. Please proceed with caution.",
        &format!(
            r#"<button class="btn btn-link text-decoration-none text-muted" data-bs-dismiss="modal">Discard</button> {}"#,
            components::button("Save Changes", "primary")
        )
    );

    let form_fields = format!(
        r#"
        <div class="row g-5">
            <div class="col-md-6">
                <h3 class="h5 fw-bold mb-4 d-flex align-items-center gap-2">
                    Identity Information
                </h3>
                {}
                {}
                {}
            </div>
            <div class="col-md-6">
                <h3 class="h5 fw-bold mb-4 d-flex align-items-center gap-2">
                    System Preferences
                </h3>
                {}
                {}
                {}
            </div>
        </div>
        <div class="mt-5 pt-4 border-top d-flex justify-content-end gap-3">
            <button class="btn btn-outline-primary rounded-pill px-4" data-bs-toggle="modal" data-bs-target="#{modal_id}">
                Show Demo Modal
            </button>
            {}
            {}
        </div>
        {modal}
        "#,
        components::form_input("Full Display Name", "name", "text", "e.g. Jonathan Smith", None),
        components::form_input("Recovery Email", "email", "email", "jonathan@acme.com", Some("Email is already registered")),
        components::form_select("Primary Role", "role", vec![("admin", "Administrator"), ("editor", "Editor"), ("viewer", "Viewer")]),
        components::form_toggle("Enable Multi-Factor Authentication", "mfa"),
        components::form_toggle("Beta Feature Access", "beta"),
        components::form_checkbox("Usage Analytics", "analytics", "Share anonymous usage data to help us improve your experience."),
        components::button("Cancel", "secondary"),
        components::button("Save Configuration", "primary"),
        modal = modal_html,
        modal_id = modal_id
    );

    format!(
        r#"
        <header class="mb-5">
            <h1 class="display-4 fw-black text-body mb-2 tracking-tighter">Interface <span class="text-primary">Elements</span></h1>
            <p class="lead text-muted">A comprehensive set of professional Bootstrap 5 components.</p>
        </header>

        <div class="card glass-card border-0 p-4 p-lg-5">
            {fields}
        </div>
        "#,
        fields = form_fields
    )
}
