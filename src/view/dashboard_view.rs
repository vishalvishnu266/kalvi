use crate::view::{render_layout, LayoutContext};
use crate::model::Tenant;
use crate::model::User;

pub struct DashboardView;

impl DashboardView {
    pub fn render_dashboard(tenant: &Tenant, user: &User, student_count: i64) -> String {
        let nav = Self::render_nav(tenant, user);
        
        let content = format!(
            r###"{nav}
            <main class="flex-grow container mx-auto px-4 py-8">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                    <div class="bg-white dark:bg-slate-900 p-6 rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm">
                        <p class="text-sm font-bold text-slate-500 uppercase mb-1">Total Students</p>
                        <h3 class="text-3xl font-black">{}</h3>
                    </div>
                </div>

                <div class="bg-white dark:bg-slate-900 p-8 rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm">
                    <h2 class="text-xl font-bold mb-6">Quick Actions</h2>
                    <div class="flex flex-wrap gap-4">
                        <a href="/web/{}/students/add" class="bg-primary/10 text-primary hover:bg-primary hover:text-white px-6 py-4 rounded-xl font-bold transition-all flex flex-col items-center gap-2 group">
                            <div class="w-10 h-10 rounded-full bg-primary/20 flex items-center justify-center group-hover:bg-white/20">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                                </svg>
                            </div>
                            <span>Enroll Student</span>
                        </a>
                    </div>
                </div>
            </main>"###,
            student_count, tenant.slug
        );

        render_layout(LayoutContext {
            title: "Dashboard".to_string(),
            tenant_name: Some(tenant.name.clone()),
        }, content)
    }

    fn render_nav(tenant: &Tenant, user: &User) -> String {
        format!(
            r###"<nav class="bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 px-6 py-4 flex justify-between items-center sticky top-0 z-50">
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 bg-primary rounded-xl flex items-center justify-center text-white font-black text-xl shadow-lg shadow-primary/20">
                        {}
                    </div>
                    <span class="font-bold text-lg hidden md:inline">{}</span>
                </div>

                <div class="flex items-center gap-4">
                    <button id="theme-toggle" class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                        </svg>
                    </button>
                    
                    <div class="h-8 w-[1px] bg-slate-200 dark:border-slate-800"></div>

                    <div class="flex items-center gap-2">
                        <div class="text-right hidden sm:block">
                            <p class="text-sm font-bold leading-none">{}</p>
                            <p class="text-xs text-slate-500 capitalize">{}</p>
                        </div>
                        <div class="w-10 h-10 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center font-bold text-slate-500">
                            {}
                        </div>
                    </div>
                </div>
            </nav>"###,
            tenant.name.chars().next().unwrap_or('K'),
            tenant.name,
            user.full_name.as_deref().unwrap_or(&user.username),
            user.role,
            user.username.chars().next().unwrap_or('U').to_uppercase()
        )
    }
}
