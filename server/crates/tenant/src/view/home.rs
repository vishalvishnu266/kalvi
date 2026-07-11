use shared::web::html::{Html, IntoHtml};
use shared::html;
use shared::web::layout::base;

pub fn home_page() -> Html {
    let content = html!(
        "<div class=\"min-h-full flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8 bg-gray-50\">",
            "<div class=\"max-w-md w-full space-y-8\">",
                "<div>",
                    "<div class=\"flex justify-center\">",
                        "<i data-lucide=\"graduation-cap\" class=\"h-16 w-16 text-indigo-600\"></i>",
                    "</div>",
                    "<h2 class=\"mt-6 text-center text-4xl font-extrabold text-gray-900\">School ERP</h2>",
                    "<p class=\"mt-2 text-center text-sm text-gray-600\">",
                        "Multi-tenant Education Management System",
                    "</p>",
                "</div>",

                "<div class=\"mt-10\">",
                    "<div class=\"bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10 text-center\">",
                        "<div class=\"space-y-6\">",
                            "<a href=\"/onboard\" ",
                                "class=\"w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm text-lg font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500\">",
                                "<i data-lucide=\"plus-circle\" class=\"h-6 w-6 mr-2\"></i> Onboard New Institution",
                            "</a>",
                            
                            "<div class=\"relative\">",
                                "<div class=\"absolute inset-0 flex items-center\">",
                                    "<div class=\"w-full border-t border-gray-300\"></div>",
                                "</div>",
                                "<div class=\"relative flex justify-center text-sm\">",
                                    "<span class=\"px-2 bg-white text-gray-500\">Existing Users</span>",
                                "</div>",
                            "</div>",

                            "<p class=\"text-sm text-gray-600\">",
                                "Already registered? Visit your institution's custom URL:",
                                "<br>",
                                "<code class=\"bg-gray-100 px-2 py-1 rounded text-indigo-700 font-mono text-xs mt-2 inline-block\">",
                                    "/t/&lt;your-slug&gt;/login",
                                "</code>",
                            "</p>",
                        "</div>",
                    "</div>",
                "</div>",
            "</div>",
        "</div>"
    );

    base("School ERP - Home", Html("".to_string()), content)
}
