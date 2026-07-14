use crate::view::components::{input, button};

pub fn render_form(
    first_name: &str,
    last_name: &str,
    email: &str,
    errors: &std::collections::HashMap<String, String>
) -> String {
    format!(
        r#"<turbo-frame id="student-form">
            <form action="/web/student/add" method="POST" class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
                <h2 class="text-xl font-bold mb-4">Enroll New Student</h2>
                {}
                {}
                {}
                <div class="mt-4">
                    {}
                </div>
            </form>
        </turbo-frame>"#,
        input("First Name", "first_name", "text", first_name, errors.get("first_name")),
        input("Last Name", "last_name", "text", last_name, errors.get("last_name")),
        input("Email Address", "email", "email", email, errors.get("email")),
        button("Enroll Student", "primary")
    )
}
