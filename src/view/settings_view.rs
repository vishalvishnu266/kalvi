use crate::view::{render_layout, LayoutContext};
use crate::model::Tenant;

pub struct SettingsView;

impl SettingsView {
    pub fn render_settings(tenant: &Tenant) -> String {
        use crate::view::components;
        
        let form_content = format!(
            //language=HTML
            r#"<div class="d-flex flex-column gap-4">
                <p class="small text-secondary fst-italic mb-2">Note: These personalization settings are stored locally in your browser and only affect your current device.</p>
                
                <div>
                    <label class="form-label small fw-bold text-uppercase text-secondary mb-3">My Accent Color</label>
                    <div class="d-flex align-items-center gap-3 bg-body-tertiary p-3 rounded-4 border">
                        <input type="color" name="user_primary_color" class="form-control form-control-color border-0 bg-transparent" style="width: 48px; height: 48px;">
                        <button onclick="localStorage.removeItem('kalvi_primary_color'); location.reload();" class="btn btn-link text-decoration-none small text-secondary hover-primary">Reset to Default</button>
                    </div>
                </div>
                
                <div class="d-flex align-items-center justify-content-between p-3 bg-body-tertiary rounded-4 border">
                    <div>
                        <p class="fw-bold mb-0">Dark Mode</p>
                        <p class="small text-secondary mb-0">Switch between light and dark themes</p>
                    </div>
                    <button id="theme-toggle" class="btn btn-outline-secondary fw-bold px-4 py-2 small shadow-sm hover-primary-btn">
                        Toggle Mode
                    </button>
                </div>

                <div class="pt-4 border-top mt-2">
                    <button onclick="localStorage.removeItem('kalvi_theme'); localStorage.removeItem('kalvi_primary_color'); location.reload();" class="btn btn-link w-100 text-secondary small text-uppercase fw-bold tracking-wider text-decoration-none hover-danger">
                        Clear all personalization
                    </button>
                </div>
            </div>
            <style>
                .hover-primary:hover {{ color: var(--bs-primary) !important; }}
                .hover-danger:hover {{ color: var(--bs-danger) !important; }}
                .hover-primary-btn:hover {{ background-color: var(--bs-primary) !important; color: white !important; border-color: var(--bs-primary) !important; }}
            </style>"#
        );

        let content = format!(
            //language=HTML
            r#"<nav class="navbar border-bottom bg-body sticky-top z-3 py-3">
                <div class="container-xl d-flex align-items-center">
                    <a href="/web/{slug}/dashboard" class="btn btn-link text-secondary p-2 me-3 hover-primary">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                        </svg>
                    </a>
                    <h1 class="h5 fw-bold mb-0">Settings</h1>
                </div>
            </nav>

            <main class="container py-4 py-md-5" style="max-width: 800px;">
                <h2 class="h3 fw-bold mb-4 px-1">Personalization</h2>
                {card}
            </main>"#,
            slug = tenant.slug,
            card = components::card(form_content)
        );

        render_layout(LayoutContext::for_tenant(tenant, "Settings"), content)
    }
}
