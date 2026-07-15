pub fn page_header(title: &str, highlight: &str, description: &str) -> String {
    format!(
        //language=HTML
        r##"
        <header class="mb-4">
            <h1 class="fw-black text-body mb-2">{title} <span class="text-primary">{highlight}</span></h1>
            <p class="text-secondary">{description}</p>
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
        <div class="card shadow-sm mb-4">
            <div class="card-header bg-transparent border-0 pt-4 px-4">
                <h5 class="card-title fw-bold mb-0">{title}</h5>
            </div>
            <div class="card-body px-4 pb-4">
                {content}
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
    let trend_icon = if is_up { "bi-arrow-up-right" } else { "bi-arrow-down-right" };

    format!(
        //language=HTML
        r##"
        <div class="card shadow-sm h-100">
            <div class="card-body p-4">
                <div class="text-secondary small fw-bold mb-1">{label}</div>
                <div class="d-flex align-items-center justify-content-between">
                    <h2 class="mb-0 fw-bold">{value}</h2>
                    <span class="{trend_color} small fw-bold">
                        <i class="bi {trend_icon} me-1"></i>{trend}
                    </span>
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
            r##"<th class="bg-light text-secondary small fw-bold border-bottom py-3">{h}</th>"##, h = h)
    }).collect();

    let rows_html: String = rows.into_iter().map(|row| {
        let cells: String = row.into_iter().map(|cell| {
            format!(
                //language=HTML
                r##"<td class="py-3 align-middle">{cell}</td>"##, cell = cell)
        }).collect();
            format!(
                //language=HTML
                r##"<tr>{cells}</tr>"##, cells = cells)
    }).collect();

    format!(
        //language=HTML
        r##"
        <div class="table-responsive">
            <table class="table table-hover border align-middle mb-0">
                <thead>
                    <tr>{header_html}</tr>
                </thead>
                <tbody>
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
        let active_class = if active { "active bg-primary text-white" } else { "link-body-emphasis" };
        format!(
            //language=HTML
            r##"
            <li>
                <a href="{link}" class="nav-link {active_class} d-flex align-items-center gap-3 py-2 px-3 rounded-2 mb-1">
                    <i class="bi bi-{icon} fs-5"></i>
                    {label}
                </a>
            </li>
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
        <div class="d-flex flex-column p-3 h-100 border-end bg-body-tertiary">
            <a href="/" class="d-flex align-items-center mb-4 px-2 text-decoration-none text-body">
                <i class="bi bi-lightning-fill text-primary fs-3 me-2"></i>
                <span class="fs-4 fw-black tracking-tighter">KALVI <span class="text-primary">ERP</span></span>
            </a>
            <hr>
            <ul class="nav nav-pills flex-column mb-auto">
                {items_html}
            </ul>
            <hr>
            <div class="dropdown px-2">
                <a href="#" class="d-flex align-items-center text-decoration-none dropdown-toggle text-body" data-bs-toggle="dropdown">
                    <img src="https://ui-avatars.com/api/?name=Admin+User" alt="" width="32" height="32" class="rounded-circle me-2 shadow-sm">
                    <strong class="small">Admin User</strong>
                </a>
                <ul class="dropdown-menu shadow">
                    <li><a class="dropdown-item small" href="#">Profile</a></li>
                    <li><hr class="dropdown-divider"></li>
                    <li><a class="dropdown-item small" href="#">Sign out</a></li>
                </ul>
            </div>
        </div>
        "##,
        items_html = items_html
    )
}

pub fn notice_list(title: &str, notices: Vec<(&str, bool)>) -> String {
    let items_html: String = notices.into_iter().map(|(text, important)| {
        let border_class = if important { "border-primary fw-bold" } else { "" };
        format!(
            //language=HTML
            r##"<li class="list-group-item py-3 {border_class}">{text}</li>"##,
            border_class = border_class,
            text = text
        )
    }).collect();

    card(title, &format!(
        //language=HTML
        r##"<ul class="list-group list-group-flush border rounded-3">{items_html}</ul>"##,
        items_html = items_html
    ))
}

pub fn quick_actions(actions: Vec<&str>) -> String {
    let buttons_html: String = actions.into_iter().map(|action| {
        format!(
            //language=HTML
            r##"
            <div class="col-6">
                <button class="btn btn-outline-primary w-100 py-3 small fw-bold">
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

pub fn form_input(label: &str, name: &str, input_type: &str, placeholder: &str, value: &str, error: Option<&str>) -> String {
    let is_invalid = if error.is_some() { "is-invalid" } else { "" };
    let error_html = error.map(|e| format!(
        //language=HTML
        r##"<div class="invalid-feedback fw-medium">{e}</div>"##, e = e)).unwrap_or_default();

    format!(
        //language=HTML
        r##"
        <div class="mb-3">
            <label for="{name}" class="form-label small fw-bold text-secondary mb-2">{label}</label>
            <input type="{input_type}" name="{name}" id="{name}" placeholder="{placeholder}" value="{value}"
                class="form-control bg-body-tertiary border-0 {is_invalid}">
            {error_html}
        </div>
        "##,
        name = name,
        label = label,
        input_type = input_type,
        placeholder = placeholder,
        value = value,
        is_invalid = is_invalid,
        error_html = error_html
    )
}

pub fn form_select(label: &str, name: &str, options: Vec<(&str, &str)>, selected_value: &str) -> String {
    let options_html: String = options.into_iter().map(|(val, lab)| {
        let selected = if val == selected_value { "selected" } else { "" };
        format!(
            //language=HTML
            r##"<option value="{val}" {selected}>{lab}</option>"##, val = val, lab = lab, selected = selected)
    }).collect();

    format!(
        //language=HTML
        r##"
        <div class="mb-3">
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

pub fn form_checkbox(label: &str, name: &str, description: &str, checked: bool) -> String {
    let is_checked = if checked { "checked" } else { "" };
    format!(
        //language=HTML
        r##"
        <div class="form-check mb-3">
            <input id="{name}" name="{name}" type="checkbox" class="form-check-input" {is_checked}>
            <label for="{name}" class="form-check-label ms-2">
                <span class="d-block fw-bold small text-body">{label}</span>
                <span class="text-secondary small">{description}</span>
            </label>
        </div>
        "##,
        name = name,
        label = label,
        description = description,
        is_checked = is_checked
    )
}
