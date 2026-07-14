use crate::views::layout::base_layout;

pub fn error_page(message: &str, request_id: &str) -> String {
    let content = format!(
        r#"
        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
            <strong class="font-bold">System Error!</strong>
            <span class="block sm:inline"> {message}</span>
            <p class="mt-2 text-sm">Request ID: {request_id}</p>
        </div>
        "#,
        message = message,
        request_id = request_id
    );
    base_layout("Error", &content)
}
