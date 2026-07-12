use crate::view::LayoutView::{render_layout, LayoutContext};
use crate::model::Tenant::Tenant;

pub fn render_login(tenant: &Tenant, error: Option<String>) -> String {
    let error_alert = match error {
        Some(err) => format!(
            /* html */
            r#"<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4" role="alert">
                <span class="block sm:inline">{err}</span>
            </div>"#,
            err = err
        ),
        None => "".to_string(),
    };

    let content = format!(
        /* html */
        r#"<div class="min-h-screen flex items-center justify-center p-4 md:p-6 relative overflow-hidden">
            <div class="max-w-md w-full p-8 bg-white dark:bg-slate-900 rounded-3xl shadow-2xl border dark:border-slate-800 relative z-10">
                <div class="flex flex-col items-center mb-8">
                    <div class="w-16 h-16 bg-primary/10 text-primary rounded-2xl flex items-center justify-center font-bold text-3xl mb-4 shadow-inner">
                        {logo_char}
                    </div>
                    <h2 class="text-3xl font-bold text-slate-900 dark:text-white">{tenant_name}</h2>
                    <p class="text-slate-500 dark:text-slate-400 font-medium">Please sign in to continue</p>
                </div>

                {error_alert}
                
                <form action="/{slug}/login" method="POST" class="space-y-5">
                    <div>
                        <label class="block text-xs uppercase tracking-wider font-bold text-slate-500 dark:text-slate-400 mb-2">Username</label>
                        <input type="text" name="username" required autofocus class="w-full px-4 py-3.5 rounded-2xl border border-slate-200 dark:border-slate-800 dark:bg-slate-950 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent outline-none transition-all placeholder-slate-400" placeholder="Enter your username">
                    </div>
                    <div>
                        <label class="block text-xs uppercase tracking-wider font-bold text-slate-500 dark:text-slate-400 mb-2">Password</label>
                        <input type="password" name="password" required class="w-full px-4 py-3.5 rounded-2xl border border-slate-200 dark:border-slate-800 dark:bg-slate-950 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent outline-none transition-all placeholder-slate-400" placeholder="••••••••">
                    </div>
                    <button type="submit" class="w-full bg-primary hover:bg-primary-600 text-white font-bold py-4 px-4 rounded-2xl transition-all mt-6 shadow-xl shadow-primary/20 transform hover:-translate-y-1 active:scale-95">
                        Sign In
                    </button>
                </form>

                <div class="mt-8 text-center">
                    <a href="/" class="text-sm text-slate-500 hover:text-primary transition-colors">← Back to home</a>
                </div>
            </div>
        </div>"#,
        logo_char = tenant.name.chars().next().unwrap_or('K'),
        tenant_name = tenant.name,
        slug = tenant.slug,
        error_alert = error_alert
    );

    let ctx = LayoutContext {
        title: format!("Login - {}", tenant.name),
        primary_color: tenant.primary_color.clone(),
        dark_mode: tenant.dark_mode,
        tenant_slug: Some(tenant.slug.clone()),
    };

    render_layout(ctx, content)
}
