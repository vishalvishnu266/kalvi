pub fn input(label: &str, name: &str, input_type: &str, value: &str, error: Option<&String>) -> String {
    let border_class = if error.is_some() {
        "border-red-500 focus:ring-red-500"
    } else {
        "border-gray-300 dark:border-gray-600 focus:ring-primary"
    };

    let error_message = match error {
        Some(msg) => format!(r#"<p class="mt-1 text-xs text-red-500">{}</p>"#, msg),
        None => "".to_string(),
    };

    format!(
        r#"<div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{}</label>
            <input type="{}" name="{}" value="{}" 
                class="w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-1 bg-white dark:bg-gray-800 {}" />
            {}
        </div>"#,
        label, input_type, name, value, border_class, error_message
    )
}

pub fn button(label: &str, variant: &str) -> String {
    let classes = match variant {
        "primary" => "bg-primary text-white hover:opacity-90",
        "secondary" => "bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 hover:bg-gray-300",
        _ => "bg-blue-500 text-white",
    };

    format!(
        r#"<button class="px-4 py-2 rounded-md font-medium transition {}">{}</button>"#,
        classes, label
    )
}
