use shared::web::html::{Html, IntoHtml, escape};
use shared::html;
use shared::web::layout::base;

pub fn login_page(tenant_slug: &str, error: Option<String>) -> Html {
    let error_alert = match error {
        Some(msg) => html!(
            "<div class=\"bg-red-50 border-l-4 border-red-400 p-4 mb-6\">",
                "<div class=\"flex\">",
                    "<div class=\"flex-shrink-0\">",
                        "<i data-lucide=\"alert-circle\" class=\"h-5 w-5 text-red-400\"></i>",
                    "</div>",
                    "<div class=\"ml-3\">",
                        "<p class=\"text-sm text-red-700\">", msg, "</p>",
                    "</div>",
                "</div>",
            "</div>"
        ),
        None => Html("".to_string()),
    };

    let content = html!(
        "<div class=\"min-h-full flex flex-col justify-center py-12 sm:px-6 lg:px-8\">",
            "<div class=\"sm:mx-auto sm:w-full sm:max-w-md\">",
                "<h2 class=\"mt-6 text-center text-3xl font-extrabold text-gray-900\">Sign in to your account</h2>",
                "<p class=\"mt-2 text-center text-sm text-gray-600\">",
                    "Institution: <span class=\"font-medium text-indigo-600\">", tenant_slug, "</span>",
                "</p>",
            "</div>",

            "<div class=\"mt-8 sm:mx-auto sm:w-full sm:max-w-md\">",
                "<div class=\"bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10\">",
                    error_alert,
                    "<form class=\"space-y-6\" action=\"/t/", tenant_slug, "/login\" method=\"POST\">",
                        "<div>",
                            "<label for=\"username\" class=\"block text-sm font-medium text-gray-700\">Username</label>",
                            "<div class=\"mt-1\">",
                                "<input id=\"username\" name=\"username\" type=\"text\" required autofocus ",
                                    "class=\"appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm\">",
                            "</div>",
                        "</div>",

                        "<div>",
                            "<label for=\"password\" class=\"block text-sm font-medium text-gray-700\">Password</label>",
                            "<div class=\"mt-1\">",
                                "<input id=\"password\" name=\"password\" type=\"password\" required ",
                                    "class=\"appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm\">",
                            "</div>",
                        "</div>",

                        "<div>",
                            "<button type=\"submit\" ",
                                "class=\"w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500\">",
                                "Sign in",
                            "</button>",
                        "</div>",
                    "</form>",
                "</div>",
                "<p class=\"mt-6 text-center\">",
                    "<a href=\"/\" class=\"text-sm font-medium text-indigo-600 hover:text-indigo-500\">",
                        "&larr; Back to home",
                    "</a>",
                "</p>",
            "</div>",
        "</div>"
    );

    base(&format!("Login - {}", tenant_slug), Html("".to_string()), content)
}
