pub fn sidebar(items: Vec<(&str, &str, bool, &str)>) -> String {
    let items_html: String = items.into_iter().map(|(label, icon, active, link)| {
        let active_class = if active { "active" } else { "" };
        format!(
            //language=HTML
            r##"
            <li>
                <a href="{link}" class="nav-link {active_class}">
                    <i class="fa-solid fa-{icon}"></i>
                    <span>{label}</span>
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
        <aside class="sidebar">
          <a href="/" class="sidebar-brand">
            <div class="brand-icon">
              <i class="fa-solid fa-graduation-cap"></i>
            </div>
            <div class="brand-name">
              <span>KALVI</span>
              <small>ERP SYSTEM</small>
            </div>
          </a>

          <ul class="sidebar-nav">
            <li class="nav-label">Main Menu</li>
            {items_html}
          </ul>

          <div class="sidebar-footer">
            <div class="d-flex align-items-center gap-3 p-3 bg-light-subtle rounded-3">
              <img src="https://ui-avatars.com/api/?name=Admin+User&background=6366f1&color=fff" class="rounded-circle" width="38">
              <div class="overflow-hidden">
                <div class="fw-bold text-truncate small">Admin User</div>
                <div class="text-muted-custom smaller text-truncate">Administrator</div>
              </div>
            </div>
          </div>
        </aside>
        "##,
        items_html = items_html
    )
}

pub fn navbar() -> String {
    format!(
        //language=HTML
        r##"
        <nav class="main-navbar">
          <button class="sidebar-toggler me-3">
            <i class="fa-solid fa-bars-staggered"></i>
          </button>

          <div class="navbar-search d-none d-md-block">
            <i class="fa-solid fa-magnifying-glass"></i>
            <input type="text" placeholder="Search anything...">
          </div>

          <div class="navbar-actions">
            <button class="btn-icon" id="theme-toggle" title="Toggle Theme">
              <i class="fa-solid fa-sun" id="theme-icon-sun"></i>
              <i class="fa-solid fa-moon" id="theme-icon-moon" style="display: none;"></i>
            </button>
            
            <button class="btn-icon" title="Notifications">
              <i class="fa-solid fa-bell"></i>
              <span class="notif-dot"></span>
            </button>

            <div class="vr mx-2 opacity-10"></div>

            <div class="dropdown">
              <a href="#" class="navbar-avatar" data-bs-toggle="dropdown">
                <img src="https://ui-avatars.com/api/?name=Admin+User&background=6366f1&color=fff">
              </a>
              <ul class="dropdown-menu dropdown-menu-end shadow-sm border-0 mt-3">
                <li><a class="dropdown-item" href="#"><i class="fa-solid fa-user me-2 opacity-50"></i> Profile</a></li>
                <li><a class="dropdown-item" href="#"><i class="fa-solid fa-gear me-2 opacity-50"></i> Settings</a></li>
                <li><hr class="dropdown-divider"></li>
                <li><a class="dropdown-item text-danger" href="#"><i class="fa-solid fa-arrow-right-from-bracket me-2 opacity-50"></i> Logout</a></li>
              </ul>
            </div>
          </div>
        </nav>
        "##
    )
}

pub fn page_header(title: &str, subtitle: &str, breadcrumbs: Vec<(&str, &str)>) -> String {
    let breadcrumb_html: String = breadcrumbs.into_iter().map(|(label, link)| {
        format!(r##"<li class="breadcrumb-item"><a href="{}">{}</a></li>"##, link, label)
    }).collect();

    format!(
        //language=HTML
        r##"
        <div class="page-header d-flex flex-wrap align-items-center justify-content-between gap-2 mb-4">
          <div>
            <nav aria-label="breadcrumb">
              <ol class="breadcrumb mb-1">
                {breadcrumb_html}
              </ol>
            </nav>
            <h1 class="h3 fw-bold mb-0">{title}</h1>
            <p class="text-muted-custom mb-0 small">{subtitle}</p>
          </div>
        </div>
        "##,
        breadcrumb_html = breadcrumb_html,
        title = title,
        subtitle = subtitle
    )
}

pub fn stat_card(label: &str, value: &str, change: &str, icon: &str, trend_up: bool) -> String {
    let trend_class = if trend_up { "text-success" } else { "text-danger" };
    let trend_icon = if trend_up { "fa-arrow-trend-up" } else { "fa-arrow-trend-down" };

    format!(
        //language=HTML
        r##"
        <div class="card stat-card h-100">
          <div class="card-body">
            <div class="d-flex align-items-center justify-content-between mb-3">
              <div class="stat-icon">
                <i class="fa-solid fa-{icon}"></i>
              </div>
              <div class="stat-change {trend_class}">
                <i class="fa-solid {trend_icon} me-1"></i>
                <span>{change}</span>
              </div>
            </div>
            <div class="stat-value">{value}</div>
            <div class="stat-label">{label}</div>
          </div>
        </div>
        "##,
        label = label,
        value = value,
        change = change,
        icon = icon,
        trend_class = trend_class,
        trend_icon = trend_icon
    )
}

pub fn table(headers: Vec<&str>, rows: Vec<Vec<String>>) -> String {
    let header_html: String = headers.into_iter().map(|h| {
        format!(r##"<th>{h}</th>"##, h = h)
    }).collect();

    let rows_html: String = rows.into_iter().map(|row| {
        let cells: String = row.into_iter().map(|cell| {
            format!(r##"<td>{cell}</td>"##, cell = cell)
        }).collect();
            format!(r##"<tr>{cells}</tr>"##, cells = cells)
    }).collect();

    format!(
        //language=HTML
        r##"
        <div class="table-responsive">
          <table class="table align-middle">
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

pub fn badge(label: &str, variant: &str) -> String {
    format!(
        //language=HTML
        r##"<span class="badge badge-{variant}-soft">{label}</span>"##,
        label = label,
        variant = variant
    )
}

pub fn button(label: &str, variant: &str, icon: Option<&str>) -> String {
    let icon_html = icon.map(|i| format!(r##"<i class="fa-solid fa-{} me-2"></i>"##, i)).unwrap_or_default();
    format!(
        //language=HTML
        r##"<button class="btn btn-{variant} d-inline-flex align-items-center">{icon_html}<span>{label}</span></button>"##,
        variant = variant,
        icon_html = icon_html,
        label = label
    )
}

pub fn form_input(label: &str, name: &str, input_type: &str, placeholder: &str, value: &str, error: Option<&str>) -> String {
    let is_invalid = if error.is_some() { "is-invalid" } else { "" };
    let error_html = error.map(|e| format!(r##"<div class="invalid-feedback">{e}</div>"##)).unwrap_or_default();

    format!(
        //language=HTML
        r##"
        <div class="mb-3">
          <label for="{name}" class="form-label">{label}</label>
          <input type="{input_type}" name="{name}" id="{name}" class="form-control {is_invalid}" placeholder="{placeholder}" value="{value}">
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
        format!(r##"<option value="{val}" {selected}>{lab}</option>"##, val = val, lab = lab, selected = selected)
    }).collect();

    format!(
        //language=HTML
        r##"
        <div class="mb-3">
          <label for="{name}" class="form-label">{label}</label>
          <select name="{name}" id="{name}" class="form-select">
            {options_html}
          </select>
        </div>
        "##,
        name = name,
        label = label,
        options_html = options_html
    )
}

pub fn csrf_input() -> String {
    format!(
        r##"<input type="hidden" name="csrf_token" value="static_csrf_token_for_dev_12345">"##
    )
}
