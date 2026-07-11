use crate::view::LayoutView::{render_layout, LayoutContext};

pub fn render_common_login(error: Option<String>) -> String {
    let error_alert = match error {
        Some(err) => format!(
            /* html */
            r#"<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4 text-sm">{err}</div>"#,
            err = err
        ),
        None => "".to_string(),
    };

    let content = format!(
        /* html */
        r#"<div class="max-w-md mx-auto my-24 p-8 bg-white dark:bg-slate-800 rounded-xl shadow-2xl border dark:border-slate-700">
            <h2 class="text-3xl font-bold mb-2 text-center text-slate-900 dark:text-white">Portal Login</h2>
            <p class="text-center text-slate-500 mb-8 text-sm">Enter your institution slug and credentials</p>
            {error_alert}
            <form action="/login" method="POST" class="space-y-4">
                <div>
                    <label class="block text-sm font-semibold mb-1">Institution Slug</label>
                    <input type="text" name="slug" required placeholder="e.g. demo-school" class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <div>
                    <label class="block text-sm font-semibold mb-1">Username</label>
                    <input type="text" name="username" required class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <div>
                    <label class="block text-sm font-semibold mb-1">Password</label>
                    <input type="password" name="password" required class="w-full px-4 py-2 rounded-lg border dark:bg-slate-700 dark:border-slate-600 focus:ring-2 focus:ring-primary outline-none">
                </div>
                <button type="submit" class="w-full bg-primary text-white font-bold py-3 rounded-lg mt-6 shadow-lg hover:opacity-90 transition-all">
                    Access Institution
                </button>
            </form>
            <div class="mt-8 pt-6 border-t dark:border-slate-700 text-center text-sm">
                <a href="/saas/login" class="text-slate-500 hover:text-primary transition-colors">SaaS Administration</a>
            </div>
        </div>"#,
        error_alert = error_alert
    );

    render_layout(LayoutContext::default(), content)
}
