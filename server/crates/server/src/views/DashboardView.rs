use crate::web_utils::HtmlHelper::{Html, e};
use crate::views::LayoutView::base_with_theme;
use crate::web_utils::StyleConstants::*;
use crate::middleware::TenantMiddleware::TenantContext;

pub fn render_dashboard(ctx: &TenantContext, username: &str) -> Html {
    // language=html
    let template = r#"
        <nav class="bg-[var(--primary-color)] shadow-md">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center">
                        <i data-lucide="graduation-cap" class="h-8 w-8 text-white mr-3"></i>
                        <span class="text-white font-bold">{{tenant_name}}</span>
                    </div>
                    <div class="flex items-center space-x-4">
                        <div class="flex items-center text-white text-sm font-medium">
                            <i data-lucide="user-circle" class="h-5 w-5 mr-2"></i> {{username}}
                        </div>
                        <a href="/t/{{tenant_slug}}/settings" class="{{NAV_LINK}} mr-2">
                            <i data-lucide="settings" class="h-4 w-4 mr-2"></i> Settings
                        </a>
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
            <div class="px-4 py-6 sm:px-0 text-gray-900 dark:text-white transition-colors duration-200">
                <h1 class="text-3xl font-bold">Welcome back, {{username}}!</h1>
                <p class="mt-2 text-gray-600 dark:text-gray-400">Manage your school activities here.</p>

                <div class="mt-8 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                    <div class="{{CARD}} p-5">
                        <div class="flex items-center">
                            <div class="bg-blue-500 rounded-md p-3">
                                <i data-lucide="users" class="h-6 w-6 text-white"></i>
                            </div>
                            <div class="ml-5">
                                <div class="text-sm font-medium text-gray-500 dark:text-gray-400">Students</div>
                                <div class="text-lg font-medium">Manage Records</div>
                            </div>
                        </div>
                        <div class="mt-4 text-sm text-indigo-700 dark:text-indigo-400 font-medium">Coming soon</div>
                    </div>
                </div>
            </div>
        </main>
    "#;

    let content = Html::new(template.to_string())
        .replace("tenant_slug", &e(&ctx.slug))
        .replace("tenant_name", &e(&ctx.name))
        .replace("username", &e(username))
        .replace("NAV_LINK", NAV_LINK)
        .replace("CARD", CARD);

    base_with_theme(
        &format!("Dashboard - {}", ctx.slug),
        Html::new("".into()),
        content,
        &ctx.primary_color,
        ctx.dark_mode,
    )
}
