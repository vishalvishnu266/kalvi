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
        r#"<div class="max-w-md mx-auto my-24 p-8 bg-white dark:bg-slate-800 rounded-xl shadow-2xl">
            <h2 class="text-3xl font-bold mb-2 text-center text-slate-900 dark:text-white">{tenant_name}</h2>
            <p class="text-center text-slate-500 mb-8 font-medium">Please sign in to continue</p>
            {error_alert}
            <form action="/t/{slug}/login" method="POST" class="space-y-5">
                <div>
                    <label class="block text-sm font-semibold mb-1">Username</label>
                    <input type="text" name="username" required autofocus class="w-full px-4 py-2.5 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none transition-all">
                </div>
                <div>
                    <label class="block text-sm font-semibold mb-1">Password</label>
                    <input type="password" name="password" required class="w-full px-4 py-2.5 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none transition-all">
                </div>
                <button type="submit" class="w-full bg-primary hover:opacity-90 text-white font-bold py-3 px-4 rounded-lg transition-all mt-6 shadow-lg transform hover:-translate-y-0.5 active:translate-y-0">
                    Sign In
                </button>
            </form>
        </div>"#,
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
