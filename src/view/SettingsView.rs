use crate::view::LayoutView::{render_layout, LayoutContext};
use crate::model::Tenant::Tenant;

pub fn render_settings(tenant: &Tenant, success: Option<String>) -> String {
    let success_alert = match success {
        Some(msg) => format!(
            /* html */
            r#"<div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">{msg}</div>"#,
            msg = msg
        ),
        None => "".to_string(),
    };

    let content = format!(
        /* html */
        r#"<nav class="bg-white dark:bg-slate-800 border-b dark:border-slate-700 px-6 py-4 flex justify-between items-center">
            <div class="flex items-center gap-4">
                <a href="/t/{slug}/dashboard" class="text-slate-500 hover:text-primary transition-colors">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold">Settings</h1>
            </div>
        </nav>

        <div class="max-w-2xl mx-auto p-8">
            {success_alert}
            <div class="bg-white dark:bg-slate-800 rounded-xl shadow-sm border dark:border-slate-700 p-8">
                <h2 class="text-2xl font-bold mb-6">Personalization</h2>
                <form action="/t/{slug}/settings" method="POST" class="space-y-6">
                    <div>
                        <label class="block text-sm font-semibold mb-2">Primary Color</label>
                        <div class="flex items-center gap-4">
                            <input type="color" name="primary_color" value="{primary_color}" class="w-12 h-12 rounded cursor-pointer">
                            <input type="text" name="primary_color_hex" value="{primary_color}" readonly class="bg-slate-50 dark:bg-slate-700 px-3 py-2 rounded text-sm font-mono">
                        </div>
                    </div>
                    
                    <div class="flex items-center justify-between py-4 border-t dark:border-slate-700">
                        <div>
                            <p class="font-semibold">Dark Mode</p>
                            <p class="text-sm text-slate-500">Switch between light and dark themes</p>
                        </div>
                        <label class="relative inline-flex items-center cursor-pointer">
                            <input type="checkbox" name="dark_mode" value="true" {dark_mode_checked} class="sr-only peer">
                            <div class="w-11 h-6 bg-slate-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary/20 rounded-full peer dark:bg-slate-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-slate-600 peer-checked:bg-primary"></div>
                        </label>
                    </div>

                    <button type="submit" class="w-full bg-primary text-white font-bold py-3 rounded-lg shadow-md hover:opacity-90 transition-all">
                        Save Preferences
                    </button>
                </form>
            </div>
        </div>
        <script>
            const colorInput = document.querySelector('input[name="primary_color"]');
            const hexInput = document.querySelector('input[name="primary_color_hex"]');
            colorInput.addEventListener('input', (e) => {{
                hexInput.value = e.target.value;
            }});
        </script>"#,
        slug = tenant.slug,
        primary_color = tenant.primary_color,
        dark_mode_checked = if tenant.dark_mode { "checked" } else { "" },
        success_alert = success_alert
    );

    let ctx = LayoutContext {
        title: format!("Settings - {}", tenant.name),
        primary_color: tenant.primary_color.clone(),
        dark_mode: tenant.dark_mode,
        tenant_slug: Some(tenant.slug.clone()),
    };

    render_layout(ctx, content)
}
