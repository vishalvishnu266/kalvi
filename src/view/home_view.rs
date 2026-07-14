use crate::view::{render_layout, LayoutContext};

pub struct HomeView;

impl HomeView {
    pub fn render() -> String {
        let content = format!(
            //language=HTML
            r#"<div class="container-xl d-flex flex-column align-items-center justify-content-center min-vh-100 p-4 text-center position-relative overflow-hidden">
                <!-- Background Gradient Blurs -->
                <div class="position-absolute top-0 start-0 translate-middle rounded-circle bg-primary opacity-10 blur-3xl" style="width: 300px; height: 300px; filter: blur(80px);"></div>
                <div class="position-absolute bottom-0 end-0 translate-middle-x rounded-circle bg-primary opacity-10 blur-3xl" style="width: 400px; height: 400px; filter: blur(100px);"></div>

                <div class="position-relative z-1 py-5" style="max-width: 900px;">
                    <div class="d-inline-flex align-items-center gap-2 px-3 py-1 rounded-pill bg-primary-subtle text-primary small fw-bold mb-4 border border-primary-subtle">
                        <span class="d-flex position-relative h-2 w-2" style="width: 8px; height: 8px;">
                          <span class="position-absolute top-0 start-0 w-100 h-100 rounded-circle bg-primary opacity-75 animate-ping"></span>
                          <span class="position-relative d-inline-block rounded-circle bg-primary" style="width: 8px; height: 8px;"></span>
                        </span>
                        MULTI-TENANT ERP SOLUTION
                    </div>
                    
                    <h1 class="display-3 fw-bolder mb-4 text-body-emphasis lh-sm">
                        Empowering Schools with <br class="d-none d-md-block"/>
                        <span class="text-primary">Smart Technology</span>
                    </h1>
                    
                    <p class="lead mb-5 mx-auto text-secondary" style="max-width: 650px;">
                        Kalvi ERP is a modern, fast, and secure education management platform built to scale with your institution.
                    </p>
                    
                    <div class="d-flex flex-column flex-sm-row gap-3 justify-content-center">
                        <a href="/registration" class="btn btn-primary btn-lg px-5 py-3 shadow">
                            Get Started
                        </a>
                        <a href="/login" class="btn btn-outline-secondary btn-lg px-5 py-3 bg-body">
                            Login to Portal
                        </a>
                    </div>

                    <div class="mt-5 pt-5 row g-4 opacity-50 grayscale hover-grayscale-0 transition-all">
                        <div class="col-6 col-md-3 fs-4 fw-bold text-secondary">InstitutionA</div>
                        <div class="col-6 col-md-3 fs-4 fw-bold text-secondary">InstitutionB</div>
                        <div class="col-6 col-md-3 fs-4 fw-bold text-secondary">InstitutionC</div>
                        <div class="col-6 col-md-3 fs-4 fw-bold text-secondary">InstitutionD</div>
                    </div>
                </div>
            </div>
            <style>
                .animate-ping {
                    animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
                }
                @keyframes ping {
                    75%, 100% { transform: scale(2); opacity: 0; }
                }
                .blur-3xl { filter: blur(64px); }
                .grayscale { filter: grayscale(1); }
                .hover-grayscale-0:hover { filter: grayscale(0); }
            </style>"#
        );

        render_layout(LayoutContext::default(), content)
    }
}
