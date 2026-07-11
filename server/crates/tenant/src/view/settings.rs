use shared::web::html::{Html, e};
use shared::web::layout::base_with_theme;
use shared::web::styles::*;
use shared::TenantContext;

pub fn settings_page(ctx: &TenantContext, message: Option<String>) -> Html {
    let message_html = match message {
        Some(msg) => format!(
            r#"<div class="bg-green-50 border-l-4 border-green-400 p-4 mb-6">
                <div class="flex">
                    <i data-lucide="check-circle" class="h-5 w-5 text-green-400 mr-3"></i>
                    <p class="text-sm text-green-700">{}</p>
                </div>
            </div>"#,
            e(&msg)
        ),
        None => "".to_string(),
    };

    let dark_mode_checked = if ctx.dark_mode { "checked" } else { "" };

    // language=html
    let template = r#"
        <nav class="bg-[var(--primary-color)]">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center">
                        <a href="/t/{{tenant_slug}}/dashboard" class="flex items-center">
                            <i data-lucide="graduation-cap" class="h-8 w-8 text-white mr-3"></i>
                            <span class="text-white font-bold">{{tenant_slug}}</span>
                        </a>
                    </div>
                </div>
            </div>
        </nav>

        <main class="max-w-4xl mx-auto py-10 px-4 sm:px-6 lg:px-8">
            <div class="px-4 py-6 sm:px-0">
                <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-8">Settings</h1>
                
                {{message}}

                <div class="{{CARD}}">
                    <form action="/t/{{tenant_slug}}/settings" method="POST" class="space-y-8">
                        <div>
                            <h3 class="text-lg font-medium leading-6 text-gray-900 dark:text-white">Theme Customization</h3>
                            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">Personalize your institution's look and feel.</p>
                        </div>

                        <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                            <div class="sm:col-span-4">
                                <label for="primary_color" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Primary Color</label>
                                <div class="mt-1 flex items-center space-x-3">
                                    <input type="color" name="primary_color" id="primary_color" value="{{primary_color}}" 
                                        class="h-10 w-10 border border-gray-300 rounded-md cursor-pointer">
                                    <input type="text" value="{{primary_color}}" readonly 
                                        class="bg-gray-50 dark:bg-gray-700 border border-gray-300 rounded-md px-3 py-2 text-sm w-32 dark:text-white">
                                </div>
                            </div>

                            <div class="sm:col-span-4">
                                <div class="flex items-start">
                                    <div class="flex items-center h-5">
                                        <input id="dark_mode" name="dark_mode" type="checkbox" value="true" {{dark_mode_checked}}
                                            class="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300 rounded">
                                    </div>
                                    <div class="ml-3 text-sm">
                                        <label for="dark_mode" class="font-medium text-gray-700 dark:text-gray-300">Dark Mode</label>
                                        <p class="text-gray-500 dark:text-gray-400">Enable dark theme for all pages.</p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="pt-5 border-t dark:border-gray-700">
                            <div class="flex justify-end">
                                <a href="/t/{{tenant_slug}}/dashboard" 
                                    class="bg-white dark:bg-gray-700 py-2 px-4 border border-gray-300 dark:border-gray-600 rounded-md text-sm font-medium text-gray-700 dark:text-gray-200 mr-3">Cancel</a>
                                <button type="submit" class="{{BTN_PRIMARY}} w-auto">Save Changes</button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        </main>
    "#;

    let content = Html(template.to_string())
        .replace("tenant_slug", &e(&ctx.slug))
        .replace("primary_color", &ctx.primary_color)
        .replace("dark_mode_checked", dark_mode_checked)
        .replace("message", &message_html)
        .replace("CARD", CARD)
        .replace("BTN_PRIMARY", BTN_PRIMARY);

    base_with_theme(
        &format!("Settings - {}", ctx.slug),
        Html("".into()),
        content,
        &ctx.primary_color,
        ctx.dark_mode,
    )
}
