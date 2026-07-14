use crate::view::{render_layout, LayoutContext};
use crate::model::{Tenant, User};

pub struct DashboardView;

impl DashboardView {
    pub fn render_dashboard(tenant: &Tenant, user: &User) -> String {
        let content = format!(
            //language=HTML
            r###"{nav}
    
            <main class="container-xl py-4 py-md-5">
                <div class="row g-3 g-md-4 mb-5">
                    <div class="col-sm-6 col-lg-4">
                        <div class="card shadow-sm border-0 h-100 transition-all hover-border-primary">
                            <div class="card-body p-4">
                                <p class="small text-secondary fw-bold text-uppercase mb-1">Total Students</p>
                                <h3 class="display-6 fw-bold mb-0">0</h3>
                            </div>
                        </div>
                    </div>
                    <div class="col-sm-6 col-lg-4">
                        <div class="card shadow-sm border-0 h-100 transition-all hover-border-primary">
                            <div class="card-body p-4">
                                <p class="small text-secondary fw-bold text-uppercase mb-1">Active Classes</p>
                                <h3 class="display-6 fw-bold mb-0">0</h3>
                            </div>
                        </div>
                    </div>
                    <div class="col-sm-12 col-lg-4">
                        <div class="card shadow-sm border-0 h-100 transition-all hover-border-primary">
                            <div class="card-body p-4">
                                <p class="small text-secondary fw-bold text-uppercase mb-1">Pending Tasks</p>
                                <h3 class="display-6 fw-bold mb-0">0</h3>
                            </div>
                        </div>
                    </div>
                </div>
    
                <div class="card shadow-sm border-0">
                    <div class="card-body p-4 p-md-5">
                        <h2 class="h4 fw-bold mb-4">Quick Actions</h2>
                        <div class="row row-cols-2 row-cols-sm-3 row-cols-md-4 row-cols-lg-6 g-3">
                            <div class="col">
                                <a href="/web/{slug}/students/add" class="btn btn-light w-100 py-4 d-flex flex-column align-items-center gap-3 border-0 bg-body-tertiary transition-all hover-primary-subtle group text-decoration-none">
                                    <div class="bg-body shadow-sm rounded-3 d-flex align-items-center justify-content-center p-3 transition-all group-hover-scale">
                                         <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                                        </svg>
                                    </div>
                                    <span class="small fw-bold text-secondary">Add Student</span>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </main>
            <style>
                .hover-border-primary:hover {{ border: 1px solid var(--bs-primary) !important; }}
                .hover-primary-subtle:hover {{ background-color: var(--bs-primary-bg-subtle) !important; color: var(--bs-primary) !important; }}
                .group:hover .group-hover-scale {{ transform: scale(1.1); background-color: var(--bs-primary) !important; color: white !important; }}
            </style>"###,
            nav = Self::render_tenant_nav(tenant, user),
            slug = tenant.slug
        );
    
        render_layout(LayoutContext::for_tenant(tenant, "Dashboard"), content)
    }
    
    pub fn render_tenant_nav(tenant: &Tenant, user: &User) -> String {
        format!(
            //language=HTML
            r###"<nav class="navbar navbar-expand border-bottom bg-body sticky-top z-3 py-3">
                <div class="container-xl d-flex justify-content-between align-items-center">
                    <div class="d-flex align-items-center gap-3">
                        <div class="bg-primary rounded-3 d-flex align-items-center justify-content-center text-white fw-bold shadow-sm" style="width: 40px; height: 40px; font-size: 1.25rem;">
                            {logo_char}
                        </div>
                        <h1 class="h5 fw-bold mb-0 d-none d-sm-block">{tenant_name}</h1>
                    </div>
                    
                    <div class="d-flex align-items-center gap-3">
                        <div class="d-flex align-items-center gap-2 pe-3 border-end">
                            <button id="theme-toggle" class="btn btn-link text-secondary p-2 rounded-3 hover-bg-light" title="Toggle Theme">
                                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="theme-sun-icon d-none-dark">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                </svg>
                                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="theme-moon-icon d-none-light text-warning">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 9H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                </svg>
                            </button>
                        </div>
    
                        <div class="d-none d-md-flex align-items-center gap-2 px-2">
                            <span class="small fw-semibold text-secondary">{username}</span>
                            <div class="bg-primary-subtle text-primary rounded-circle d-flex align-items-center justify-content-center fw-bold text-uppercase" style="width: 32px; height: 32px; font-size: 0.75rem;">
                                {user_char}
                            </div>
                        </div>
                        
                        <div class="d-flex align-items-center gap-1">
                            <a href="/web/{slug}/settings" class="btn btn-link text-secondary p-2 hover-bg-light" title="Settings">
                                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                </svg>
                            </a>
                            <form action="/web/{slug}/logout" method="POST" class="m-0" data-turbo="false">
                                <button type="submit" class="btn btn-link text-secondary p-2 hover-text-danger" title="Logout">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                                    </svg>
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            </nav>
            <style>
                .hover-bg-light:hover {{ background-color: var(--bs-tertiary-bg); color: var(--bs-primary) !important; }}
                .hover-text-danger:hover {{ color: var(--bs-danger) !important; }}
                .theme-sun-icon {{ display: block; }}
                .theme-moon-icon {{ display: none; }}
                [data-bs-theme="dark"] .theme-sun-icon {{ display: none; }}
                [data-bs-theme="dark"] .theme-moon-icon {{ display: block; }}
            </style>"###,
            logo_char = crate::util::html_util::escape_html(&tenant.name).chars().next().unwrap_or('K'),
            tenant_name = crate::util::html_util::escape_html(&tenant.name),
            username = crate::util::html_util::escape_html(user.full_name.as_deref().unwrap_or(&user.username)),
            user_char = crate::util::html_util::escape_html(&user.username).chars().next().unwrap_or('U').to_uppercase(),
            slug = tenant.slug
        )
    }
}
