use crate::views::components;

pub fn render() -> String {
    let stats = format!(
        r#"<div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
            {}
            {}
            {}
        </div>"#,
        components::stats_card("Total Revenue", "$128,430", "+12.5%", true),
        components::stats_card("Active Users", "2,420", "+3.2%", true),
        components::stats_card("Churn Rate", "0.8%", "-1.1%", false)
    );

    // Sidebar setup
    let sidebar_items = vec![
        ("Dashboard", r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path></svg>"#, true),
        ("Invoices", r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>"#, false),
        ("Customers", r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>"#, false),
        ("Settings", r#"<svg fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 00-1.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><path d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>"#, false),
    ];

    let sidebar_html = components::sidebar(sidebar_items);

    // Table setup
    let headers = vec!["Client", "Amount", "Status", "Date"];
    let rows = vec![
        vec!["Acme Corp".to_string(), "$4,500.00".to_string(), components::badge("Paid", "green"), "2024-03-12".to_string()],
        vec!["Global Tech".to_string(), "$1,200.00".to_string(), components::badge("Pending", "blue"), "2024-03-14".to_string()],
        vec!["Nexus Ltd".to_string(), "$850.00".to_string(), components::badge("Overdue", "red"), "2024-03-01".to_string()],
    ];
    let table_html = components::table(headers, rows);

    // Modal setup
    let modal_id = "demo-modal";
    let modal_html = components::modal(
        modal_id,
        "Create New Entry",
        &format!(
            r#"<div class="space-y-4">
                <p>Fill out the details below to add a new record to the system.</p>
                {}
                {}
            </div>"#,
            components::form_input("Record Name", "rec_name", "text", "Invoice #1234"),
            components::form_input("Category", "cat", "text", "Billing")
        ),
        &format!(
            r#"{} {}"#,
            components::button("Discard", "secondary"),
            components::button("Confirm & Save", "primary")
        )
    );

    let controls = format!(
        r#"<div class="flex gap-4 mb-8">
            <button onclick="document.getElementById('{}').classList.remove('hidden')" 
                class="px-6 py-3 bg-white/20 hover:bg-white/30 text-white rounded-xl backdrop-blur-md border border-white/20 transition-all font-semibold">
                Open Demo Modal
            </button>
        </div>"#,
        modal_id
    );

    format!(
        r#"
        <div class="flex min-h-[80vh] rounded-3xl overflow-hidden shadow-2xl border border-white/10">
            {sidebar}
            <div class="flex-1 p-8 lg:p-12 overflow-y-auto bg-white/5 dark:bg-slate-900/40 backdrop-blur-sm">
                <header class="mb-12">
                    <h1 class="text-4xl font-extrabold text-white mb-2 tracking-tight">Enterprise Storybook</h1>
                    <p class="text-white/60 text-lg">Extended UI library for Kalvi ERP system.</p>
                </header>
                
                {controls}
                {stats}
                
                <div class="mt-12">
                    <h2 class="text-xl font-bold text-white mb-6">Recent Transactions</h2>
                    {table}
                </div>
            </div>
        </div>
        {modal}
        "#,
        sidebar = sidebar_html,
        stats = stats,
        table = table_html,
        modal = modal_html,
        controls = controls
    )
}
