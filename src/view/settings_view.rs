use crate::view::{render_layout, LayoutContext};
use crate::model::Tenant;

pub struct SettingsView;

impl SettingsView {
    pub fn render_settings(tenant: &Tenant, success: Option<String>) -> String {
        use crate::view::components;
        let success_alert = success.map(|msg| components::alert(&msg, false)).unwrap_or_default();

        let form_content = format!(
            //language=HTML
            r#"{success_alert}
            <div class="space-y-6">
                <p class="text-sm text-slate-500 italic mb-4">Note: These personalization settings are stored locally in your browser and only affect your current device.</p>
                
                <div>
                    <label class="block text-sm font-semibold mb-4 ml-1">My Accent Color</label>
                    <div class="flex items-center gap-4 bg-slate-50 dark:bg-slate-950 p-4 rounded-2xl border dark:border-slate-800">
                        <input type="color" name="user_primary_color" class="w-12 h-12 rounded-xl cursor-pointer border-none bg-transparent">
                        <button onclick="localStorage.removeItem('kalvi_primary_color'); location.reload();" class="text-xs text-slate-400 hover:text-primary underline">Reset to Brand Default</button>
                    </div>
                </div>
                
                <div class="flex items-center justify-between p-4 bg-slate-50 dark:bg-slate-950 rounded-2xl border dark:border-slate-800">
                    <div>
                        <p class="font-bold">Dark Mode</p>
                        <p class="text-sm text-slate-500">Switch between light and dark themes</p>
                    </div>
                    <button id="theme-toggle" class="px-6 py-2 bg-white dark:bg-slate-800 border dark:border-slate-700 rounded-xl text-sm font-bold hover:bg-primary hover:text-white transition-all shadow-sm">
                        Toggle Mode
                    </button>
                </div>

                <div class="pt-6 border-t dark:border-slate-800">
                    <button onclick="localStorage.removeItem('kalvi_theme'); localStorage.removeItem('kalvi_primary_color'); location.reload();" class="w-full py-4 text-slate-400 text-xs uppercase font-bold tracking-widest hover:text-red-400">
                        Clear all local overrides
                    </button>
                </div>
            </div>"#,
            success_alert = success_alert
        );

        let content = format!(
            //language=HTML
            r#"<nav class="bg-white dark:bg-slate-900/80 backdrop-blur-md border-b dark:border-slate-800 px-4 md:px-6 py-4 flex items-center sticky top-0 z-50">
                <a href="/web/{slug}/dashboard" class="p-2 mr-4 text-slate-500 hover:text-primary transition-colors">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-slate-900 dark:text-white">Settings</h1>
            </nav>

            <main class="max-w-2xl mx-auto p-4 md:p-8">
                <h2 class="text-2xl font-bold mb-8">Personalization</h2>
                {card}
            </main>"#,
            slug = tenant.slug,
            card = components::card(form_content)
        );

        let ctx = LayoutContext {
            title: format!("Settings - {}", tenant.name),
            primary_color: tenant.primary_color.clone(),
            dark_mode: tenant.dark_mode,
            tenant_slug: Some(tenant.slug.clone()),
        };

        render_layout(ctx, content)
    }
}
