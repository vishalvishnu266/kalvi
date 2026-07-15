pub fn card(title: &str, content: &str) -> String {
    format!(
        r#"
        <div class="card glass-card p-4 mb-4 h-100">
            <div class="card-body">
                <h5 class="card-title fw-bold mb-4">{title}</h5>
                <div class="card-text">
                    {content}
                </div>
            </div>
        </div>
        "#,
        title = title,
        content = content
    )
}

pub fn button(label: &str, variant: &str) -> String {
    let bootstrap_variant = match variant {
        "primary" => "btn-primary",
        "secondary" => "btn-secondary",
        "danger" => "btn-danger",
        _ => "btn-primary"
    };
    
    format!(
        r#"<button class="btn {bootstrap_variant} px-4 py-2 rounded-pill fw-semibold shadow-sm transition-all">{label}</button>"#,
        label = label,
        bootstrap_variant = bootstrap_variant
    )
}

pub fn form_input(label: &str, name: &str, input_type: &str, placeholder: &str, error: Option<&str>) -> String {
    let is_invalid = if error.is_some() { "is-invalid" } else { "" };
    let error_html = error.map(|e| format!(r#"<div class="invalid-feedback fw-medium">{}</div>"#, e)).unwrap_or_default();

    format!(
        r#"
        <div class="mb-4">
            <label for="{name}" class="form-label fw-bold small text-muted text-uppercase tracking-wider">{label}</label>
            <input type="{input_type}" name="{name}" id="{name}" placeholder="{placeholder}" 
                class="form-control form-control-lg rounded-4 bg-light-subtle {is_invalid}">
            {error_html}
        </div>
        "#,
        name = name,
        label = label,
        input_type = input_type,
        placeholder = placeholder,
        is_invalid = is_invalid,
        error_html = error_html
    )
}

pub fn form_select(label: &str, name: &str, options: Vec<(&str, &str)>) -> String {
    let options_html: String = options.into_iter().map(|(val, lab)| {
        format!(r#"<option value="{}">{}</option>"#, val, lab)
    }).collect();

    format!(
        r#"
        <div class="mb-4">
            <label for="{name}" class="form-label fw-bold small text-muted text-uppercase tracking-wider">{label}</label>
            <select name="{name}" id="{name}" class="form-select form-select-lg rounded-4 bg-light-subtle">
                {options_html}
            </select>
        </div>
        "#,
        name = name,
        label = label,
        options_html = options_html
    )
}

pub fn form_checkbox(label: &str, name: &str, description: &str) -> String {
    format!(
        r#"
        <div class="form-check mb-4">
            <input class="form-check-input" type="checkbox" id="{name}" name="{name}">
            <label class="form-check-label ms-2" for="{name}">
                <div class="fw-bold">{label}</div>
                <div class="small text-muted">{description}</div>
            </label>
        </div>
        "#,
        name = name,
        label = label,
        description = description
    )
}

pub fn form_toggle(label: &str, name: &str) -> String {
    format!(
        r#"
        <div class="form-check form-switch d-flex justify-content-between align-items-center ps-0 mb-4">
            <label class="form-check-label fw-bold" for="{name}">{label}</label>
            <input class="form-check-input ms-0" type="checkbox" role="switch" id="{name}" style="width: 2.5rem; height: 1.25rem;">
        </div>
        "#,
        label = label,
        name = name
    )
}

pub fn badge(label: &str, color: &str) -> String {
    let bootstrap_color = match color {
        "green" => "success",
        "red" => "danger",
        "blue" => "primary",
        _ => "secondary"
    };

    format!(
        r#"<span class="badge rounded-pill bg-{bootstrap_color}-subtle text-{bootstrap_color} px-3 py-2">{label}</span>"#,
        label = label,
        bootstrap_color = bootstrap_color
    )
}

pub fn stats_card(label: &str, value: &str, trend: &str, is_up: bool) -> String {
    let trend_color = if is_up { "text-success" } else { "text-danger" };
    let trend_icon = if is_up { "↑" } else { "↓" };

    format!(
        r#"
        <div class="card glass-card p-4 border-0 shadow-sm">
            <div class="small fw-bold text-muted text-uppercase tracking-wider mb-1">{label}</div>
            <div class="d-flex justify-content-between align-items-end">
                <h3 class="fw-bold mb-0">{value}</h3>
                <div class="{trend_color} fw-bold small">
                    {trend_icon} {trend}
                </div>
            </div>
        </div>
        "#,
        label = label,
        value = value,
        trend = trend,
        trend_color = trend_color,
        trend_icon = trend_icon
    )
}

pub fn table(headers: Vec<&str>, rows: Vec<Vec<String>>) -> String {
    let header_html: String = headers.into_iter().map(|h| {
        format!(r#"<th class="border-0 small fw-bold text-muted text-uppercase tracking-widest px-4 py-3">{}</th>"#, h)
    }).collect();

    let rows_html: String = rows.into_iter().map(|row| {
        let cells: String = row.into_iter().map(|cell| {
            format!(r#"<td class="px-4 py-3 align-middle fw-medium">{}</td>"#, cell)
        }).collect();
        format!(r#"<tr>{}</tr>"#, cells)
    }).collect();

    format!(
        r#"
        <div class="card glass-card border-0 shadow-sm overflow-hidden">
            <div class="table-responsive">
                <table class="table table-hover mb-0">
                    <thead class="table-light-subtle">
                        <tr>{header_html}</tr>
                    </thead>
                    <tbody class="border-top-0">
                        {rows_html}
                    </tbody>
                </table>
            </div>
        </div>
        "#,
        header_html = header_html,
        rows_html = rows_html
    )
}

pub fn sidebar(items: Vec<(&str, &str, bool, &str)>) -> String {
    let items_html: String = items.iter().map(|(label, icon, active, link)| {
        let active_class = if *active { "bg-primary text-white shadow" } else { "text-muted" };
        format!(
            r#"
            <a href="{link}" class="nav-link d-flex align-items-center gap-3 px-4 py-3 rounded-4 transition-all mb-1 {active_class}">
                <span style="width: 1.25rem;">{icon}</span>
                <span class="fw-bold small">{label}</span>
            </a>
            "#,
            label = label,
            icon = icon,
            active_class = active_class,
            link = link
        )
    }).collect();

    let sidebar_content = format!(
        r#"
        <div class="d-flex flex-column h-100">
            <div class="d-flex align-items-center gap-3 mb-5 px-2">
                <div class="bg-primary rounded-3 p-2 text-white shadow">
                    <svg style="width: 1.5rem; height: 1.5rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                </div>
                <span class="fs-4 fw-black tracking-tighter">KALVI <span class="text-primary">ERP</span></span>
            </div>
            <nav class="nav flex-column flex-grow-1">
                {items_html}
            </nav>
            <div class="mt-auto pt-4 border-top">
                <div class="d-flex align-items-center gap-3 px-2">
                    <div class="rounded-circle bg-secondary overflow-hidden" style="width: 2.5rem; height: 2.5rem;">
                        <img src="https://ui-avatars.com/api/?name=Admin+User&background=random" alt="Avatar" class="w-100">
                    </div>
                    <div>
                        <div class="small fw-bold">Admin User</div>
                        <div class="x-small text-muted">Super Admin</div>
                    </div>
                </div>
            </div>
        </div>
        "#,
        items_html = items_html
    );

    format!(
        r#"
        <!-- Desktop Sidebar -->
        <aside class="glass-card border-end d-none d-lg-block p-4 me-4" style="width: 300px; min-height: 80vh;">
            {content}
        </aside>

        <!-- Mobile/Tablet Offcanvas -->
        <div class="offcanvas offcanvas-start glass-card" tabindex="-1" id="mobileSidebar" aria-labelledby="mobileSidebarLabel">
            <div class="offcanvas-header">
                <button type="button" class="btn-close text-reset ms-auto" data-bs-dismiss="offcanvas" aria-label="Close"></button>
            </div>
            <div class="offcanvas-body p-4">
                {content}
            </div>
        </div>
        "#,
        content = sidebar_content
    )
}

pub fn modal(id: &str, title: &str, content: &str, footer: &str) -> String {
    format!(
        r#"
        <div class="modal fade" id="{id}" tabindex="-1" aria-hidden="true">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content glass-card border-0 shadow-lg">
                    <div class="modal-header border-0 px-4 pt-4">
                        <h5 class="modal-title fw-bold">{title}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body px-4 py-3 text-muted">
                        {content}
                    </div>
                    <div class="modal-footer border-0 px-4 pb-4 pt-0">
                        {footer}
                    </div>
                </div>
            </div>
        </div>
        "#,
        id = id,
        title = title,
        content = content,
        footer = footer
    )
}
