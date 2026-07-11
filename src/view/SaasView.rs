use crate::view::LayoutView::{render_layout, LayoutContext};

pub fn render_onboard(error: Option<String>) -> String {
    let error_alert = match error {
        Some(err) => format!(
            /* html */
            r#"<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">{err}</div>"#,
            err = err
        ),
        None => "".to_string(),
    };

    let content = format!(
        /* html */
        r#"<div class="max-w-md mx-auto my-12 p-8 bg-white dark:bg-slate-800 rounded-xl shadow-2xl">
            <h2 class="text-3xl font-bold mb-6 text-center">SaaS Owner Setup</h2>
            {error_alert}
            <form action="/saas/onboard" method="POST" class="space-y-4">
                <div>
                    <label class="block text-sm font-medium mb-1">Username</label>
                    <input type="text" name="username" required class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <div>
                    <label class="block text-sm font-medium mb-1">Full Name</label>
                    <input type="text" name="full_name" required class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <div>
                    <label class="block text-sm font-medium mb-1">Password</label>
                    <input type="password" name="password" required class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <button type="submit" class="w-full bg-primary text-white font-bold py-3 rounded-lg mt-6 shadow-md hover:opacity-90">
                    Create SaaS Admin
                </button>
            </form>
        </div>"#,
        error_alert = error_alert
    );

    render_layout(LayoutContext::default(), content)
}

pub fn render_login(error: Option<String>) -> String {
    let error_alert = match error {
        Some(err) => format!(
            /* html */
            r#"<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">{err}</div>"#,
            err = err
        ),
        None => "".to_string(),
    };

    let content = format!(
        /* html */
        r#"<div class="max-w-md mx-auto my-24 p-8 bg-white dark:bg-slate-800 rounded-xl shadow-2xl">
            <h2 class="text-3xl font-bold mb-6 text-center">SaaS Owner Login</h2>
            {error_alert}
            <form action="/saas/login" method="POST" class="space-y-4">
                <div>
                    <label class="block text-sm font-medium mb-1">Username</label>
                    <input type="text" name="username" required class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <div>
                    <label class="block text-sm font-medium mb-1">Password</label>
                    <input type="password" name="password" required class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <button type="submit" class="w-full bg-primary text-white font-bold py-3 rounded-lg mt-6 shadow-md hover:opacity-90">
                    Sign In
                </button>
            </form>
        </div>"#,
        error_alert = error_alert
    );

    render_layout(LayoutContext::default(), content)
}
