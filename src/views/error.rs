use crate::views::layout::base_layout;

use crate::views::{components, layout};

pub fn error_page(message: &str, request_id: &str) -> String {
    let content = format!(
        //language=HTML
        r##"
        <div class="row justify-content-center mt-5">
            <div class="col-md-8 col-lg-6">
                {alert}
                <div class="mt-4 text-center">
                    <a href="/dashboard" class="btn btn-light px-4">
                        <i class="bi bi-house-door me-2"></i>
                        Back to Dashboard
                    </a>
                </div>
            </div>
        </div>
        "##,
        alert = components::alert(
            "System Error!",
            message,
            "danger",
            Some(&format!("Support Code: {request_id}", request_id = request_id))
        )
    );
    layout::base_layout("System Error", &content)
}
