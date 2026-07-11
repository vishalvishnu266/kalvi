use shared::web::html::{Html, IntoHtml, e};
use shared::html;
use shared::web::layout::base;

pub fn dashboard_page(tenant_slug: &str, username: &str) -> Html {
    let navbar = html!(
        // language=html
        r#"<nav class="bg-indigo-600">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center">
                        <div class="flex-shrink-0">
                            <i data-lucide="graduation-cap" class="h-8 w-8 text-white"></i>
                        </div>
                        <div class="hidden md:block">
                            <div class="ml-10 flex items-baseline space-x-4">
                                <span class="text-white font-bold">"#, e(tenant_slug), r#"</span>
                            </div>
                        </div>
                    </div>
                    <div class="flex items-center space-x-4">
                        <div class="flex items-center text-white text-sm font-medium">
                            <i data-lucide="user-circle" class="h-5 w-5 mr-2"></i>"#,
                            e(username),
                        r#"</div>
                        <form action="/t/"#, e(tenant_slug), r#"/logout" method="POST">
                            <button type="submit" class="bg-indigo-700 text-white px-3 py-2 rounded-md text-sm font-medium hover:bg-indigo-800 flex items-center">
                                <i data-lucide="log-out" class="h-4 w-4 mr-2"></i> Logout
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </nav>"#
    );

    let content = html!(
        navbar,
        // language=html
        r#"<main class="max-w-7xl mx-auto py-10 sm:px-6 lg:px-8">
            <div class="px-4 py-6 sm:px-0">
                <h1 class="text-3xl font-bold text-gray-900">Welcome back, "#, e(username), r#"!</h1>
                <p class="mt-2 text-gray-600">Manage your school activities here.</p>

                <div class="mt-8 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                    <!-- Card 1 -->
                    <div class="bg-white overflow-hidden shadow rounded-lg border border-gray-200">
                        <div class="p-5">
                            <div class="flex items-center">
                                <div class="flex-shrink-0 bg-blue-500 rounded-md p-3">
                                    <i data-lucide="users" class="h-6 w-6 text-white"></i>
                                </div>
                                <div class="ml-5 w-0 flex-1">
                                    <dl>
                                        <dt class="text-sm font-medium text-gray-500 truncate">Students</dt>
                                        <dd class="text-lg font-medium text-gray-900">Manage Records</dd>
                                    </dl>
                                </div>
                            </div>
                        </div>
                        <div class="bg-gray-50 px-5 py-3">
                            <div class="text-sm text-indigo-700 font-medium">Coming soon</div>
                        </div>
                    </div>
                    
                    <!-- Card 2 -->
                    <div class="bg-white overflow-hidden shadow rounded-lg border border-gray-200">
                        <div class="p-5">
                            <div class="flex items-center">
                                <div class="flex-shrink-0 bg-green-500 rounded-md p-3">
                                    <i data-lucide="settings" class="h-6 w-6 text-white"></i>
                                </div>
                                <div class="ml-5 w-0 flex-1">
                                    <dl>
                                        <dt class="text-sm font-medium text-gray-500 truncate">Settings</dt>
                                        <dd class="text-lg font-medium text-gray-900">Configuration</dd>
                                    </dl>
                                </div>
                            </div>
                        </div>
                        <div class="bg-gray-50 px-5 py-3">
                            <div class="text-sm text-indigo-700 font-medium">Coming soon</div>
                        </div>
                    </div>
                </div>
            </div>
        </main>"#
    );

    base(&format!("Dashboard - {}", tenant_slug), Html("".to_string()), content)
}
