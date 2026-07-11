use crate::view::LayoutView::{render_layout, LayoutContext};
use crate::model::Tenant::Tenant;
use crate::model::User::User;

pub fn render_dashboard(tenant: &Tenant, user: &User) -> String {
    let content = format!(
        /* html */
        r#"<nav class="bg-white dark:bg-slate-800 border-b dark:border-slate-700 px-6 py-4 flex justify-between items-center shadow-sm">
            <div class="flex items-center space-y-0 gap-4">
                <div class="w-10 h-10 bg-primary rounded-lg flex items-center justify-center text-white font-bold text-xl">
                    {logo_char}
                </div>
                <h1 class="text-xl font-bold text-slate-900 dark:text-white">{tenant_name}</h1>
            </div>
            <div class="flex items-center gap-6">
                <div class="flex items-center gap-2">
                    <span class="text-sm font-medium text-slate-500">Welcome, {username}</span>
                    <div class="h-8 w-8 bg-slate-200 dark:bg-slate-700 rounded-full flex items-center justify-center">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd" />
                        </svg>
                    </div>
                </div>
                <div class="flex items-center gap-3 border-l dark:border-slate-700 pl-6">
                    <a href="/t/{slug}/settings" class="text-slate-500 hover:text-primary transition-colors" title="Settings">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                    </a>
                    <form action="/t/{slug}/logout" method="POST">
                        <button type="submit" class="text-slate-500 hover:text-red-500 transition-colors" title="Logout">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                            </svg>
                        </button>
                    </form>
                </div>
            </div>
        </nav>

        <div class="p-8 max-w-7xl mx-auto">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                <div class="bg-white dark:bg-slate-800 p-6 rounded-xl shadow-sm border dark:border-slate-700">
                    <p class="text-sm text-slate-500 font-medium mb-1">Total Students</p>
                    <h3 class="text-3xl font-bold">0</h3>
                </div>
                <div class="bg-white dark:bg-slate-800 p-6 rounded-xl shadow-sm border dark:border-slate-700">
                    <p class="text-sm text-slate-500 font-medium mb-1">Active Classes</p>
                    <h3 class="text-3xl font-bold">0</h3>
                </div>
                <div class="bg-white dark:bg-slate-800 p-6 rounded-xl shadow-sm border dark:border-slate-700">
                    <p class="text-sm text-slate-500 font-medium mb-1">Pending Tasks</p>
                    <h3 class="text-3xl font-bold">0</h3>
                </div>
            </div>

            <div class="bg-white dark:bg-slate-800 p-8 rounded-xl shadow-sm border dark:border-slate-700">
                <h2 class="text-2xl font-bold mb-4">Quick Actions</h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <button class="flex flex-col items-center justify-center p-4 rounded-lg bg-slate-50 dark:bg-slate-700/50 hover:bg-primary/10 transition-colors group">
                        <div class="w-12 h-12 bg-white dark:bg-slate-600 rounded-full flex items-center justify-center mb-3 shadow-sm group-hover:bg-primary group-hover:text-white transition-all">
                             <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                            </svg>
                        </div>
                        <span class="text-sm font-semibold">Add Student</span>
                    </button>
                    <!-- More buttons can be added here -->
                </div>
            </div>
        </div>"#,
        logo_char = tenant.name.chars().next().unwrap_or('K'),
        tenant_name = tenant.name,
        username = user.full_name.as_deref().unwrap_or(&user.username)
    );

    let ctx = LayoutContext {
        title: format!("Dashboard - {}", tenant.name),
        primary_color: tenant.primary_color.clone(),
        dark_mode: tenant.dark_mode,
        tenant_slug: Some(tenant.slug.clone()),
    };

    render_layout(ctx, content)
}
