use crate::view::{render_layout, LayoutContext};

pub struct HomeView;

impl HomeView {
    pub fn render() -> String {
        let content = format!(
            //language=HTML
            r#"<div class="hero-section position-relative overflow-hidden">
                <div class="hero-bg-accent-1"></div>
                <div class="hero-bg-accent-2"></div>

                <div class="container-xl position-relative z-1 d-flex flex-column align-items-center justify-content-center min-vh-100 text-center py-5">
                    <div class="badge-pill mb-4 animate-fade-in">
                        <span class="pulse-dot"></span>
                        MULTI-TENANT ERP SOLUTION
                    </div>
                    
                    <h1 class="hero-title mb-4 animate-slide-up">
                        Empowering Schools with <br class="d-none d-md-block"/>
                        <span class="text-gradient">Smart Technology</span>
                    </h1>
                    
                    <p class="hero-lead mb-5 mx-auto animate-fade-in-delayed">
                        Kalvi ERP is a modern, fast, and secure education management platform built to scale with your institution.
                    </p>
                    
                    <div class="d-flex flex-column flex-sm-row gap-3 justify-content-center animate-fade-in-delayed">
                        <a href="/registration" class="btn btn-primary btn-lg px-5 py-3 shadow-lg hero-btn-primary">
                            Get Started
                        </a>
                        <a href="/login" class="btn btn-outline-secondary btn-lg px-5 py-3 bg-body hero-btn-secondary">
                            Login to Portal
                        </a>
                    </div>

                    <div class="mt-5 pt-5 row g-4 partner-logos animate-fade-in-delayed">
                        <div class="col-6 col-md-3">InstitutionA</div>
                        <div class="col-6 col-md-3">InstitutionB</div>
                        <div class="col-6 col-md-3">InstitutionC</div>
                        <div class="col-6 col-md-3">InstitutionD</div>
                    </div>
                </div>
            </div>
            <style>
                :root {{
                    --hero-gradient: linear-gradient(135deg, var(--bs-primary) 0%, #2563eb 100%);
                }}

                .hero-section {{
                    background: radial-gradient(circle at 50% 50%, rgba(var(--bs-primary-rgb), 0.02) 0%, transparent 70%);
                }}

                .hero-bg-accent-1 {{
                    position: absolute;
                    top: -10%;
                    left: -10%;
                    width: 40%;
                    height: 60%;
                    background: radial-gradient(circle, rgba(var(--bs-primary-rgb), 0.08) 0%, transparent 70%);
                    filter: blur(80px);
                    z-1: -1;
                }}

                .hero-bg-accent-2 {{
                    position: absolute;
                    bottom: -10%;
                    right: -10%;
                    width: 50%;
                    height: 50%;
                    background: radial-gradient(circle, rgba(var(--bs-primary-rgb), 0.05) 0%, transparent 70%);
                    filter: blur(100px);
                    z-1: -1;
                }}

                .hero-title {{
                    font-size: clamp(2.5rem, 8vw, 4.5rem);
                    font-weight: 800;
                    letter-spacing: -0.02em;
                    line-height: 1.1;
                    color: var(--bs-emphasis-color);
                }}

                .text-gradient {{
                    background: var(--hero-gradient);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                }}

                .hero-lead {{
                    font-size: 1.25rem;
                    color: var(--bs-secondary-color);
                    max-width: 700px;
                    line-height: 1.6;
                }}

                .badge-pill {{
                    display: inline-flex;
                    align-items: center;
                    gap: 0.75rem;
                    padding: 0.5rem 1.25rem;
                    background: rgba(var(--bs-primary-rgb), 0.1);
                    border: 1px solid rgba(var(--bs-primary-rgb), 0.2);
                    border-radius: 100px;
                    color: var(--bs-primary);
                    font-size: 0.75rem;
                    font-weight: 700;
                    letter-spacing: 0.05em;
                }}

                .pulse-dot {{
                    width: 8px;
                    height: 8px;
                    background: var(--bs-primary);
                    border-radius: 50%;
                    position: relative;
                }}

                .pulse-dot::after {{
                    content: '';
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: inherit;
                    border-radius: inherit;
                    animation: pulse 2s cubic-bezier(0, 0, 0.2, 1) infinite;
                }}

                @keyframes pulse {{
                    75%, 100% {{ transform: scale(2.5); opacity: 0; }}
                }}

                .hero-btn-primary {{
                    background: var(--hero-gradient);
                    border: none;
                    transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
                }}

                .hero-btn-primary:hover {{
                    transform: translateY(-3px);
                    box-shadow: 0 10px 20px -5px rgba(var(--bs-primary-rgb), 0.4) !important;
                }}

                .partner-logos {{
                    font-size: 1.5rem;
                    font-weight: 700;
                    color: var(--bs-secondary-color);
                    opacity: 0.4;
                    filter: grayscale(1);
                    transition: opacity 0.3s, filter 0.3s;
                }}

                .partner-logos:hover {{
                    opacity: 0.8;
                    filter: grayscale(0);
                }}

                /* Animations */
                .animate-slide-up {{
                    animation: slideUp 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards;
                }}

                .animate-fade-in {{
                    animation: fadeIn 1s ease-out forwards;
                }}

                .animate-fade-in-delayed {{
                    opacity: 0;
                    animation: fadeIn 1s ease-out 0.3s forwards;
                }}

                @keyframes slideUp {{
                    from {{ transform: translateY(30px); opacity: 0; }}
                    to {{ transform: translateY(0); opacity: 1; }}
                }}

                @keyframes fadeIn {{
                    from {{ opacity: 0; }}
                    to {{ opacity: 1; }}
                }}
            </style>"#
        );

        render_layout(LayoutContext::default(), content)
    }
}
