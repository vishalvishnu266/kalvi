pub struct LayoutContext {
    pub title: String,
    pub tenant_slug: Option<String>,
}

impl Default for LayoutContext {
    fn default() -> Self {
        Self {
            title: "Kalvi ERP".to_string(),
            tenant_slug: None,
        }
    }
}

impl LayoutContext {
    pub fn for_tenant(tenant: &crate::model::Tenant, title: &str) -> Self {
        Self {
            title: format!("{} - {}", title, tenant.name),
            tenant_slug: Some(tenant.slug.clone()),
        }
    }

    pub fn render(self, content: String) -> String {
        render_layout(self, content)
    }
}

pub fn render_layout(ctx: LayoutContext, content: String) -> String {
    // language=HTML
    const TEMPLATE: &str = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>{title}</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
    <script type="module">
        import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';
        window.Turbo = Turbo;
    </script>
    <style>
        /* language=css */
        :root {
            --primary-color: #3b82f6;
            --primary-color-rgb: 59, 130, 246;
            
            /* Map to Bootstrap variables */
            --bs-primary: var(--primary-color);
            --bs-primary-rgb: var(--primary-color-rgb);
            --bs-link-color: var(--primary-color);
            --bs-link-hover-color: color-mix(in srgb, var(--primary-color), black 20%);
        }

        [data-bs-theme="dark"] {
            color-scheme: dark;
        }

        body {
            background: radial-gradient(circle at top left, rgba(var(--primary-color-rgb), 0.05), transparent 40%),
                        radial-gradient(circle at bottom right, rgba(var(--primary-color-rgb), 0.05), transparent 40%);
            min-height: 100vh;
        }

        [data-bs-theme="dark"] body {
            background: radial-gradient(circle at top left, rgba(var(--primary-color-rgb), 0.1), transparent 40%),
                        linear-gradient(to bottom, #0f172a, #020617);
        }

        /* Modern UI Polish */
        .card { border-radius: 1.5rem; border: 1px solid rgba(0,0,0,0.05); }
        [data-bs-theme="dark"] .card { border: 1px solid rgba(255,255,255,0.05); background-color: #0f172a; }
        .btn { border-radius: 1rem; padding: 0.75rem 1.5rem; font-weight: 600; }
        .form-control, .form-select { border-radius: 1rem; padding: 0.75rem 1rem; border-color: rgba(0,0,0,0.1); }
        [data-bs-theme="dark"] .form-control { background-color: #020617; border-color: rgba(255,255,255,0.1); color: white; }
        
        /* Custom scrollbar */
        ::-webkit-scrollbar { width: 8px; }
        ::-webkit-scrollbar-track { background: transparent; }
        ::-webkit-scrollbar-thumb { background: #cbd5e1; border-radius: 10px; }
        [data-bs-theme="dark"] ::-webkit-scrollbar-thumb { background: #334155; }
    </style>
    <script>
        // language=javascript
        (function() {
            const savedTheme = localStorage.getItem('kalvi_theme');
            const savedColor = localStorage.getItem('kalvi_primary_color');
            const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            
            const isDark = savedTheme ? (savedTheme === 'dark') : systemDark;
            document.documentElement.setAttribute('data-bs-theme', isDark ? 'dark' : 'light');
            
            const activeColor = savedColor || '#3b82f6';
            
            function updateCSSVariables(hex) {
                document.documentElement.style.setProperty('--primary-color', hex);
                const r = parseInt(hex.slice(1, 3), 16);
                const g = parseInt(hex.slice(3, 5), 16);
                const b = parseInt(hex.slice(5, 7), 16);
                document.documentElement.style.setProperty('--primary-color-rgb', `${r}, ${g}, ${b}`);
            }
            
            updateCSSVariables(activeColor);
        })();

        document.addEventListener('turbo:load', () => {
            function updateThemeUI() {
                const themeToggle = document.getElementById('theme-toggle');
                if (themeToggle) {
                    const isDark = document.documentElement.getAttribute('data-bs-theme') === 'dark';
                    themeToggle.textContent = isDark ? 'Switch to Light Mode' : 'Switch to Dark Mode';
                }
            }

            const themeToggle = document.getElementById('theme-toggle');
            if (themeToggle) {
                themeToggle.addEventListener('click', () => {
                    const currentTheme = document.documentElement.getAttribute('data-bs-theme');
                    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
                    document.documentElement.setAttribute('data-bs-theme', newTheme);
                    localStorage.setItem('kalvi_theme', newTheme);
                    updateThemeUI();
                });
            }
            updateThemeUI();

            const colorInput = document.querySelector('input[name="user_primary_color"]');
            if (colorInput) {
                const currentColor = getComputedStyle(document.documentElement).getPropertyValue('--primary-color').trim();
                colorInput.value = currentColor;
                colorInput.addEventListener('input', (e) => {
                    const hex = e.target.value;
                    document.documentElement.style.setProperty('--primary-color', hex);
                    localStorage.setItem('kalvi_primary_color', hex);
                    const r = parseInt(hex.slice(1, 3), 16);
                    const g = parseInt(hex.slice(3, 5), 16);
                    const b = parseInt(hex.slice(5, 7), 16);
                    document.documentElement.style.setProperty('--primary-color-rgb', `${r}, ${g}, ${b}`);
                });
            }
        });
    </script>
</head>
<body>
    <div id="app-container" class="d-flex flex-column min-vh-100">
        <div class="flex-grow-1">
            {content}
        </div>
        
        <footer class="py-5 px-4 border-top">
            <div class="container-xl d-flex flex-column flex-md-row justify-content-between align-items-center gap-3 text-secondary small font-medium">
                <p class="mb-0">&copy; 2026 Kalvi ERP. All rights reserved.</p>
                <div class="d-flex align-items-center gap-4">
                    <a href="/contact" class="text-decoration-none text-secondary">Support</a>
                    <a href="#" class="text-decoration-none text-secondary">Privacy</a>
                    <span class="badge rounded-pill bg-body-tertiary text-secondary border">System v0.1.0</span>
                </div>
            </div>
        </footer>
    </div>
</body>
</html>"###;
    TEMPLATE
        .replace("{title}", &ctx.title)
        .replace("{content}", &content)
}
