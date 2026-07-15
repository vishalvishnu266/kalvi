use crate::views::layout::base_layout;

use crate::views::components;

pub fn error_page(message: &str, request_id: &str) -> String {
    let content = components::alert(
        "System Error!",
        message,
        "danger",
        Some(&format!("Request ID: {request_id}", request_id = request_id))
    );
    base_layout("Error", &content)
}
