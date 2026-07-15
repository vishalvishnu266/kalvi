use crate::views::layout::base_layout;

pub fn error_page(message: &str, request_id: &str) -> String {
    let content = format!(
        //language=HTML
        r#"
        <div class="bg-red-100 border border-red-400 text-red-700 px-4 sm:px-6 py-4 rounded-2xl relative break-words overflow-hidden" role="alert">
            <div class="flex flex-col gap-2">
                <strong class="font-bold text-lg">System Error!</strong>
                <span class="block text-sm sm:text-base"> {message}</span>
                <p class="mt-2 text-xs opacity-75 font-medium tracking-wider uppercase">Request ID: {request_id}</p>
            </div>
        </div>
        "#,
        message = message,
        request_id = request_id
    );
    base_layout("Error", &content)
}
