use shared::web::html::{Html, e};
use shared::web::layout::base;
use shared::web::styles::*;

pub fn dashboard_page(tenant_slug: &str, username: &str) -> Html {
    // language=html
    let template = r#"
        <nav class="bg-indigo-600">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center">
                        <i data-lucide="graduation-cap" class="h-8 w-8 text-white mr-3"></i>
                        <span class="text-white font-bold">{{tenant_slug}}</span>
                    </div>
                    <div class="flex items-center space-x-4">
                        <div class="flex items-center text-white text-sm font-medium">
                            <i data-lucide="user-circle" class="h-5 w-5 mr-2"></i> {{username}}
                        </div>
                        <form action="/t/{{tenant_slug}}/logout" method="POST">
                            <button type="submit" class="{{NAV_LINK}}">
                                <i data-lucide="log-out" class="h-4 w-4 mr-2"></i> Logout
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </nav>

        <main class="max-w-7xl mx-auto py-10 sm:px-6 lg:px-8">
            <div class="px-4 py-6 sm:px-0">
                <h1 class="text-3xl font-bold text-gray-900">Welcome back, {{username}}!</h1>
                <p class="mt-2 text-gray-600">Manage your school activities here.</p>

                <div class="mt-8 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                    <div class="bg-white overflow-hidden shadow rounded-lg border border-gray-200 p-5">
                        <div class="flex items-center">
                            <div class="bg-blue-500 rounded-md p-3">
                                <i data-lucide="users" class="h-6 w-6 text-white"></i>
                            </div>
                            <div class="ml-5">
                                <div class="text-sm font-medium text-gray-500">Students</div>
                                <div class="text-lg font-medium text-gray-900">Manage Records</div>
                            </div>
                        </div>
                        <div class="mt-4 text-sm text-indigo-700 font-medium">Coming soon</div>
                    </div>
                    
                    <div class="bg-white overflow-hidden shadow rounded-lg border border-gray-200 p-5">
                        <div class="flex items-center">
                            <div class="bg-green-500 rounded-md p-3">
                                <i data-lucide="settings" class="h-6 w-6 text-white"></i>
                            </div>
                            <div class="ml-5">
                                <div class="text-sm font-medium text-gray-500">Settings</div>
                                <div class="text-lg font-medium text-gray-900">Configuration</div>
                            </div>
                        </div>
                        <div class="mt-4 text-sm text-indigo-700 font-medium">Coming soon</div>
                    </div>
                </div>
            </div>
        </main>
    "#;

    Html(template.to_string())
        .replace("tenant_slug", &e(tenant_slug))
        .replace("username", &e(username))
        .replace("NAV_LINK", NAV_LINK)
}
