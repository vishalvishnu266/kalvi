use crate::util::html_util::escape_html;

pub fn alert_error(message: &str) -> String {
    alert(message, true)
}

pub fn alert_success(message: &str) -> String {
    alert(message, false)
}

pub fn alert(message: &str, is_error: bool) -> String {
    let alert_class = if is_error { "alert-danger" } else { "alert-success" };

    format!(
        //language=HTML
        r#"<div class="alert {} d-flex align-items-start gap-3 shadow-sm border-0 py-3 px-4 mb-4" role="alert">
            <svg class="bi flex-shrink-0 mt-1" width="20" height="20" fill="currentColor" viewBox="0 0 16 16">
                <path d="M8 16A8 8 0 1 0 8 0a8 8 0 0 0 0 16zm.93-9.412-1 4.705c-.07.34.029.533.304.533.194 0 .487-.07.686-.246l-.088.416c-.287.346-.92.598-1.465.598-.703 0-1.002-.422-.808-1.319l.738-3.468c.064-.293.006-.399-.287-.47l-.451-.081.082-.381 2.29-.287zM8 5.5a1 1 0 1 1 0-2 1 1 0 0 1 0 2z"/>
            </svg>
            <div class="small fw-semibold">{}</div>
        </div>"#,
        alert_class, escape_html(message)
    )
}

pub fn input(label: &str, name: &str, input_type: &str, placeholder: &str, required: bool, error: Option<&str>) -> String {
    let req_attr = if required { "required" } else { "" };
    let (input_class, error_html) = match error {
        Some(msg) => (
            "form-control is-invalid",
            format!(r#"<div class="invalid-feedback fw-bold">{}</div>"#, escape_html(msg))
        ),
        None => ("form-control", "".to_string()),
    };

    format!(
        //language=HTML
        r#"<div class="mb-4">
            <label class="form-label small text-uppercase fw-bold text-secondary ps-1">{}</label>
            <input type="{}" name="{}" placeholder="{}" {} class="{}">
            {}
        </div>"#,
        escape_html(label), input_type, escape_html(name), escape_html(placeholder), req_attr, input_class, error_html
    )
}

pub fn button_primary(label: &str, is_submit: bool) -> String {
    let btn_type = if is_submit { "submit" } else { "button" };
    format!(
        //language=HTML
        r#"<button type="{}" class="btn btn-primary w-100 py-3 shadow-sm">
            {}
        </button>"#,
        btn_type, label
    )
}

pub fn card(content: String) -> String {
    format!(
        //language=HTML
        r#"<div class="card shadow-sm border-0 p-4 p-md-5">
            <div class="card-body p-0">
                {}
            </div>
        </div>"#,
        content
    )
}

pub fn nav_link(href: &str, label: &str, icon_svg: &str) -> String {
    format!(
        //language=HTML
        r#"<a href="{}" class="nav-link d-flex align-items-center gap-3 px-3 py-2 rounded-3 text-secondary">
            <div class="shrink-0">{}</div>
            <span class="fw-semibold small">{}</span>
        </a>"#,
        href, icon_svg, label
    )
}
