pub fn form_input(label: &str, name: &str, error: Option<&str>) -> String {
    format!(
        r#"
        <div class="mb-4">
            <label for="{name}" class="block text-sm font-medium text-gray-700">{label}</label>
            <input type="text" name="{name}" id="{name}" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
            {error_html}
        </div>
        "#,
        name = name,
        label = label,
        error_html = error.map(|e| format!(r#"<p class="mt-2 text-sm text-red-600">{}</p>"#, e)).unwrap_or_default()
    )
}
