use crate::view::LayoutView::{render_layout, LayoutContext};

pub fn render_form(error: Option<String>) -> String {
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
        r#"<div class="max-w-md mx-auto my-12 p-8 bg-white dark:bg-slate-800 rounded-xl shadow-2xl">
            <h2 class="text-3xl font-bold mb-6 text-center text-slate-900 dark:text-white">Institutional Onboarding</h2>
            {error_alert}
            <form action="/onboard" method="POST" class="space-y-4">
                <div>
                    <label class="block text-sm font-medium mb-1">Institution Name</label>
                    <input type="text" name="name" required class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <div>
                    <label class="block text-sm font-medium mb-1">Slug (URL friendly name)</label>
                    <input type="text" name="slug" required placeholder="demo-school" class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <hr class="my-6 border-slate-200 dark:border-slate-700">
                <h3 class="text-lg font-semibold mb-2">Admin Account</h3>
                <div>
                    <label class="block text-sm font-medium mb-1">Admin Username</label>
                    <input type="text" name="admin_username" required class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <div>
                    <label class="block text-sm font-medium mb-1">Admin Password</label>
                    <input type="password" name="admin_password" required class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <button type="submit" class="w-full bg-primary hover:opacity-90 text-white font-bold py-3 px-4 rounded-lg transition-all mt-6 shadow-md">
                    Initialize Institution
                </button>
            </form>
        </div>"#,
        error_alert = error_alert
    );

    render_layout(LayoutContext::default(), content)
}

pub fn render_success(slug: &str, name: &str) -> String {
    let content = format!(
        /* html */
        r#"<div class="max-w-md mx-auto my-12 p-8 bg-white dark:bg-slate-800 rounded-xl shadow-2xl text-center">
            <div class="w-20 h-20 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-6">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
            </div>
            <h2 class="text-3xl font-bold mb-4 text-slate-900 dark:text-white">Institution Ready!</h2>
            <p class="text-slate-600 dark:text-slate-400 mb-8">
                <strong>{name}</strong> has been successfully initialized. You can now log in to your tenant dashboard.
            </p>
            <a href="/t/{slug}/login" class="inline-block bg-primary hover:opacity-90 text-white font-bold py-3 px-8 rounded-lg shadow-md transition-all">
                Go to Login
            </a>
        </div>"#,
        slug = slug,
        name = name
    );

    render_layout(LayoutContext::default(), content)
}
