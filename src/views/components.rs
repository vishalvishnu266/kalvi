pub fn page_header(title: &str, highlight: &str, description: &str) -> String {
    format!(
        //language=HTML
        r##"
        <header class="mb-4 mb-md-5">
            <h1 class="display-4 fw-black text-body mb-3 tracking-tighter">{title} <span class="text-primary">{highlight}</span></h1>
            <p class="lead text-secondary max-w-2xl">{description}</p>
        </header>
        "##,
        title = title,
        highlight = highlight,
        description = description
    )
}

pub fn card(title: &str, content: &str) -> String {
    format!(
        //language=HTML
        r##"
        <div class="card glass-card shadow-sm border-0 p-3 p-md-4 transition-all duration-300">
            <div class="card-body">
                <h5 class="card-title fw-bold text-body mb-4">{title}</h5>
                <div class="card-text text-secondary">
                    {content}
                </div>
            </div>
        </div>
        "##,
        title = title,
        content = content
    )
}

pub fn button(label: &str, variant: &str, icon: Option<&str>) -> String {
    let bootstrap_variant = match variant {
        "primary" => "btn-primary shadow-primary-sm",
        "secondary" => "btn-light border text-body",
        "danger" => "btn-danger",
        _ => "btn-primary"
    };
    
    let icon_html = icon.map(|i| format!(
        //language=HTML
        r##"<i class="bi bi-{i} me-2"></i>"##, i = i
    )).unwrap_or_default();

    format!(
        //language=HTML
        r##"<button class="btn {bootstrap_variant} px-4 py-2 fw-medium d-inline-flex align-items-center justify-content-center">{icon_html}<span>{label}</span></button>"##,
        bootstrap_variant = bootstrap_variant,
        icon_html = icon_html,
        label = label
    )
}

pub fn form_input(label: &str, name: &str, input_type: &str, placeholder: &str, error: Option<&str>) -> String {
    let is_invalid = if error.is_some() { "is-invalid" } else { "" };
    let error_html = error.map(|e| format!(
        //language=HTML
        r##"<div class="invalid-feedback fw-medium">{e}</div>"##, e = e)).unwrap_or_default();

    format!(
        //language=HTML
        r##"
        <div class="mb-4">
            <label for="{name}" class="form-label small fw-bold text-secondary mb-2">{label}</label>
            <input type="{input_type}" name="{name}" id="{name}" placeholder="{placeholder}" 
                class="form-control bg-body-tertiary border-0 {is_invalid}">
            {error_html}
        </div>
        "##,
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
        format!(
            //language=HTML
            r##"<option value="{val}">{lab}</option>"##, val = val, lab = lab)
    }).collect();

    format!(
        //language=HTML
        r##"
        <div class="mb-4">
            <label for="{name}" class="form-label small fw-bold text-secondary mb-2">{label}</label>
            <select name="{name}" id="{name}" class="form-select bg-body-tertiary border-0 cursor-pointer">
                {options_html}
            </select>
        </div>
        "##,
        name = name,
        label = label,
        options_html = options_html
    )
}

pub fn form_checkbox(label: &str, name: &str, description: &str) -> String {
    format!(
        //language=HTML
        r##"
        <div class="form-check mb-4">
            <input id="{name}" name="{name}" type="checkbox" class="form-check-input">
            <label for="{name}" class="form-check-label ms-2">
                <span class="d-block fw-bold small text-body">{label}</span>
                <span class="text-secondary small">{description}</span>
            </label>
        </div>
        "##,
        name = name,
        label = label,
        description = description
    )
}

pub fn form_toggle(label: &str, name: &str) -> String {
    format!(
        //language=HTML
        r##"
        <div class="form-check form-switch d-flex justify-content-between align-items-center mb-4 ps-0">
            <label class="form-check-label fw-bold small text-body" for="{name}">{label}</label>
            <input class="form-check-input ms-0" type="checkbox" role="switch" id="{name}" name="{name}">
        </div>
        "##,
        label = label,
        name = name
    )
}

pub fn badge(label: &str, color: &str) -> String {
    let color_class = match color {
        "green" => "bg-success-subtle text-success border-success-subtle",
        "red" => "bg-danger-subtle text-danger border-danger-subtle",
        "blue" => "bg-primary-subtle text-primary border-primary-subtle",
        _ => "bg-secondary-subtle text-secondary border-secondary-subtle"
    };

    format!(
        //language=HTML
        r##"<span class="badge {color_class} border rounded-pill fw-bold" style="font-size: 0.7rem;">{label}</span>"##,
        label = label,
        color_class = color_class
    )
}

pub fn stats_card(label: &str, value: &str, trend: &str, is_up: bool) -> String {
    let trend_color = if is_up { "text-success" } else { "text-danger" };
    let trend_icon = if is_up { "bi-graph-up-arrow" } else { "bi-graph-down-arrow" };

    format!(
        //language=HTML
        r##"
        <div class="card glass-card shadow-sm border-0 h-100 transition-all duration-300">
            <div class="card-body p-4">
                <p class="small fw-bold text-secondary text-uppercase mb-2 tracking-wider">{label}</p>
                <div class="d-flex align-items-end justify-content-between">
                    <h3 class="fw-black text-body mb-0">{value}</h3>
                    <div class="small fw-bold {trend_color}">
                        <i class="bi {trend_icon} me-1"></i>
                        {trend}
                    </div>
                </div>
            </div>
        </div>
        "##,
        label = label,
        value = value,
        trend = trend,
        trend_color = trend_color,
        trend_icon = trend_icon
    )
}

pub fn table(headers: Vec<&str>, rows: Vec<Vec<String>>) -> String {
    let header_html: String = headers.into_iter().map(|h| {
        format!(
            //language=HTML
            r##"<th class="border-0 small fw-bold text-secondary text-uppercase py-3">{h}</th>"##, h = h)
    }).collect();

    let rows_html: String = rows.into_iter().map(|row| {
        let cells: String = row.into_iter().map(|cell| {
            format!(
                //language=HTML
                r##"<td class="py-3 text-body align-middle">{cell}</td>"##, cell = cell)
        }).collect();
            format!(
                //language=HTML
                r##"<tr class="border-bottom border-light-subtle">{cells}</tr>"##, cells = cells)
    }).collect();

    format!(
        //language=HTML
        r##"
        <div class="table-responsive">
            <table class="table table-hover mb-0">
                <thead>
                    <tr>{header_html}</tr>
                </thead>
                <tbody class="border-0">
                    {rows_html}
                </tbody>
            </table>
        </div>
        "##,
        header_html = header_html,
        rows_html = rows_html
    )
}

pub fn sidebar(items: Vec<(&str, &str, bool, &str)>) -> String {
    let items_html: String = items.into_iter().map(|(label, icon, active, link)| {
        let active_class = if active { "active bg-primary-subtle text-primary fw-bold" } else { "text-secondary" };
        format!(
            //language=HTML
            r##"
            <a href="{link}" class="nav-link p-3 rounded-3 d-flex align-items-center gap-3 transition-all mb-1 {active_class}">
                <i class="bi bi-{icon} fs-5"></i>
                <span>{label}</span>
            </a>
            "##,
            label = label,
            icon = icon,
            active_class = active_class,
            link = link
        )
    }).collect();

    format!(
        //language=HTML
        r##"
        <div class="d-flex flex-column h-100 p-4 glass-card border-0 rounded-0 border-end">
            <div class="d-flex align-items-center gap-3 mb-5 px-2">
                <div class="bg-primary rounded-3 d-flex align-items-center justify-content-center text-white shadow" style="width: 40px; height: 40px;">
                    <i class="bi bi-lightning-fill fs-4"></i>
                </div>
                <span class="h4 mb-0 fw-black tracking-tighter">KALVI <span class="text-primary">ERP</span></span>
            </div>
            
            <nav class="nav flex-column nav-pills flex-grow-1">
                {items_html}
            </nav>
            
            <div class="mt-auto pt-4 border-top border-light-subtle">
                <div class="d-flex align-items-center gap-3 px-2">
                    <img src="https://ui-avatars.com/api/?name=Admin+User&background=random" class="rounded-circle shadow-sm" width="40" height="40" alt="Avatar">
                    <div class="overflow-hidden">
                        <p class="small fw-bold text-body mb-0 text-truncate">Admin User</p>
                        <p class="extra-small text-secondary mb-0">Super Admin</p>
                    </div>
                </div>
            </div>
        </div>
        "##,
        items_html = items_html
    )
}

pub fn modal(id: &str, title: &str, content: &str, footer: &str) -> String {
    format!(
        //language=HTML
        r##"
        <div class="modal fade" id="{id}" tabindex="-1" aria-hidden="true">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content glass-card border-0 shadow">
                    <div class="modal-header border-bottom border-light-subtle px-4 py-3">
                        <h5 class="modal-title fw-bold">{title}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body p-4">
                        {content}
                    </div>
                    <div class="modal-footer border-top border-light-subtle bg-body-tertiary rounded-bottom-4 px-4 py-3">
                        {footer}
                    </div>
                </div>
            </div>
        </div>
        "##,
        id = id,
        title = title,
        content = content,
        footer = footer
    )
}

pub fn alert(title: &str, message: &str, variant: &str, footer: Option<&str>) -> String {
    let bootstrap_variant = match variant {
        "danger" => "alert-danger",
        "success" => "alert-success",
        "warning" => "alert-warning",
        _ => "alert-info"
    };

    let footer_html = footer.map(|f| format!(
        //language=HTML
        r##"<p class="mt-2 mb-0 small opacity-75 fw-bold text-uppercase tracking-wider">{f}</p>"##, f = f
    )).unwrap_or_default();

    format!(
        //language=HTML
        r##"
        <div class="alert {bootstrap_variant} border-0 shadow-sm rounded-4 p-4 mb-0" role="alert">
            <div class="d-flex flex-col gap-1">
                <strong class="h5 fw-bold mb-1">{title}</strong>
                <span class="text-body">{message}</span>
                {footer_html}
            </div>
        </div>
        "##,
        bootstrap_variant = bootstrap_variant,
        title = title,
        message = message,
        footer_html = footer_html
    )
}

pub fn notice_list(title: &str, notices: Vec<(&str, bool)>) -> String {
    let items_html: String = notices.into_iter().map(|(text, important)| {
        let border_class = if important { "border-primary shadow-sm" } else { "border-light-subtle" };
        format!(
            //language=HTML
            r##"<li class="list-group-item bg-body-tertiary rounded-3 mb-2 border {border_class} small">{text}</li>"##,
            border_class = border_class,
            text = text
        )
    }).collect();

    card(title, &format!(
        //language=HTML
        r##"<ul class="list-group list-group-flush bg-transparent">{items_html}</ul>"##,
        items_html = items_html
    ))
}

pub fn quick_actions(actions: Vec<&str>) -> String {
    let buttons_html: String = actions.into_iter().map(|action| {
        format!(
            //language=HTML
            r##"
            <div class="col-6">
                <button class="btn btn-light border w-100 py-3 small fw-bold text-secondary hover-primary transition-all shadow-sm">
                    {action}
                </button>
            </div>
            "##,
            action = action
        )
    }).collect();

    card("Quick Actions", &format!(
        //language=HTML
        r##"<div class="row g-2">{buttons_html}</div>"##,
        buttons_html = buttons_html
    ))
}
