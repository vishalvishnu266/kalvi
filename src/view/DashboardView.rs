use crate::view::LayoutView::{render_layout, LayoutContext};
use crate::model::Tenant::Tenant;
use crate::model::User::User;

pub fn render_dashboard(tenant: &Tenant, user: &User) -> String {
    let content = format!(
        /* html */
        r###"<nav class="bg-white dark:bg-slate-900/80 backdrop-blur-md border-b dark:border-slate-800 px-4 md:px-6 py-4 flex justify-between items-center sticky top-0 z-50">
            <div class="flex items-center gap-3">
                <div class="w-10 h-10 bg-primary rounded-xl flex items-center justify-center text-white font-bold text-xl shadow-lg shadow-primary/20">
                    {logo_char}
                </div>
                <h1 class="text-lg md:text-xl font-bold text-slate-900 dark:text-white truncate max-w-[120px] md:max-w-none">{tenant_name}</h1>
            </div>
            
            <div class="flex items-center gap-2 md:gap-4">
                <!-- Theme & Color Controls -->
                <div class="flex items-center gap-1 md:gap-2 mr-2 md:mr-4 pr-2 md:pr-4 border-r dark:border-slate-800">
                    <button id="theme-toggle" class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 transition-colors" title="Toggle Theme">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 dark:hidden" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                        </svg>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 hidden dark:block text-yellow-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 9H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                        </svg>
                    </button>
                    
                    <div class="flex gap-1">
                        <button class="color-picker w-5 h-5 rounded-full bg-blue-500 border-2 border-white dark:border-slate-700 cursor-pointer" data-color="#3b82f6"></button>
                        <button class="color-picker w-5 h-5 rounded-full bg-emerald-500 border-2 border-white dark:border-slate-700 cursor-pointer" data-color="#10b981"></button>
                        <button class="color-picker w-5 h-5 rounded-full bg-rose-500 border-2 border-white dark:border-slate-700 cursor-pointer" data-color="#f43f5e"></button>
                    </div>
                </div>

                <div class="hidden md:flex items-center gap-2">
                    <span class="text-sm font-medium text-slate-500">{username}</span>
                    <div class="h-8 w-8 bg-primary/10 text-primary rounded-full flex items-center justify-center font-bold">
                        {user_char}
                    </div>
                </div>
                
                <div class="flex items-center gap-2 md:gap-3">
                    <a href="/{slug}/settings" class="p-2 text-slate-500 hover:text-primary transition-colors" title="Settings">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                    </a>
                    <form action="/{slug}/logout" method="POST" class="inline">
                        <button type="submit" class="p-2 text-slate-500 hover:text-red-500 transition-colors" title="Logout">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                            </svg>
                        </button>
                    </form>
                </div>
            </div>
        </nav>

        <main class="p-4 md:p-8 max-w-7xl mx-auto">
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-6 mb-8">
                <div class="bg-white dark:bg-slate-900 p-6 rounded-2xl shadow-sm border dark:border-slate-800 hover:border-primary/30 transition-all group">
                    <p class="text-sm text-slate-500 font-medium mb-1">Total Students</p>
                    <h3 class="text-3xl font-bold group-hover:text-primary transition-colors">0</h3>
                </div>
                <div class="bg-white dark:bg-slate-900 p-6 rounded-2xl shadow-sm border dark:border-slate-800 hover:border-primary/30 transition-all group">
                    <p class="text-sm text-slate-500 font-medium mb-1">Active Classes</p>
                    <h3 class="text-3xl font-bold group-hover:text-primary transition-colors">0</h3>
                </div>
                <div class="bg-white dark:bg-slate-900 p-6 rounded-2xl shadow-sm border dark:border-slate-800 hover:border-primary/30 transition-all group">
                    <p class="text-sm text-slate-500 font-medium mb-1">Pending Tasks</p>
                    <h3 class="text-3xl font-bold group-hover:text-primary transition-colors">0</h3>
                </div>
            </div>

            <div class="bg-white dark:bg-slate-900 p-6 md:p-8 rounded-2xl shadow-sm border dark:border-slate-800">
                <h2 class="text-xl md:text-2xl font-bold mb-6">Quick Actions</h2>
                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
                    <button class="flex flex-col items-center justify-center p-4 rounded-xl bg-slate-50 dark:bg-slate-800/50 hover:bg-primary/10 transition-all group border border-transparent hover:border-primary/20">
                        <div class="w-12 h-12 bg-white dark:bg-slate-700 rounded-xl flex items-center justify-center mb-3 shadow-sm group-hover:bg-primary group-hover:text-white transition-all transform group-hover:scale-110">
                             <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                            </svg>
                        </div>
                        <span class="text-xs md:text-sm font-semibold text-slate-600 dark:text-slate-300">Add Student</span>
                    </button>
                    <!-- More buttons can be added here -->
                </div>
            </div>
        </main>"###,
        logo_char = tenant.name.chars().next().unwrap_or('K'),
        tenant_name = tenant.name,
        username = user.full_name.as_deref().unwrap_or(&user.username),
        user_char = user.username.chars().next().unwrap_or('U').to_uppercase(),
        slug = tenant.slug
    );

    let ctx = LayoutContext {
        title: format!("Dashboard - {}", tenant.name),
        primary_color: tenant.primary_color.clone(),
        dark_mode: tenant.dark_mode,
        tenant_slug: Some(tenant.slug.clone()),
    };

    render_layout(ctx, content)
}
