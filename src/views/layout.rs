pub fn base_layout(title: &str, content: &str) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{title}</title>
            <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
            <style>
                :root {{
                    --bs-primary: #4f46e5;
                    --bs-primary-rgb: 79, 70, 229;
                }}
                [data-bs-theme="light"] body {{
                    background-color: #f8fafc;
                }}
                [data-bs-theme="dark"] body {{
                    background-color: #020617;
                }}
                .bg-mesh {{
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    height: 400px;
                    background: radial-gradient(circle at 50% -20%, var(--bs-primary), transparent 70%);
                    opacity: 0.15;
                    pointer-events: none;
                    z-index: 0;
                }}
                .glass-card {{
                    background: rgba(var(--bs-tertiary-bg-rgb), 0.7) !important;
                    backdrop-filter: blur(12px);
                    border: 1px solid rgba(255, 255, 255, 0.1) !important;
                    border-radius: 1.25rem !important;
                    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05), 0 2px 4px -1px rgba(0, 0, 0, 0.03);
                }}
                .btn-primary {{
                    --bs-btn-bg: var(--bs-primary);
                    --bs-btn-border-color: var(--bs-primary);
                    --bs-btn-hover-bg: color-mix(in srgb, var(--bs-primary), black 10%);
                }}
                #theme-controls {{
                    position: fixed;
                    top: 1rem;
                    right: 1rem;
                    z-index: 1050;
                    background: rgba(var(--bs-tertiary-bg-rgb), 0.5);
                    backdrop-filter: blur(10px);
                    padding: 0.5rem;
                    border-radius: 50rem;
                    border: 1px solid rgba(255, 255, 255, 0.1);
                    display: flex;
                    gap: 0.5rem;
                }}
            </style>
            <script src="https://unpkg.com/@hotwired/turbo@8.0.0/dist/turbo.es2017-umd.js"></script>
        </head>
        <body class="min-vh-100">
            <div class="bg-mesh"></div>
            
            <!-- Mobile Header -->
            <nav class="navbar d-lg-none glass-card sticky-top m-3 shadow-sm">
                <div class="container-fluid">
                    <button class="navbar-toggler border-0 p-2 shadow-none" type="button" data-bs-toggle="offcanvas" data-bs-target="#mobileSidebar">
                        <svg style="width: 1.5rem; height: 1.5rem;" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                    </button>
                    <span class="navbar-brand fw-black tracking-tighter small mb-0">KALVI <span class="text-primary">ERP</span></span>
                    <div id="mobile-theme-controls" class="d-flex gap-2 bg-white/10 p-1 rounded-pill">
                        <!-- Theme controls moved here for mobile -->
                    </div>
                </div>
            </nav>

            <!-- Desktop Theme Controls -->
            <div id="theme-controls" class="d-none d-lg-flex">
                <input type="color" id="primaryColorPicker" value="#4f46e5" class="form-control-color border-0 bg-transparent rounded-circle" style="width: 2rem; height: 2rem; padding: 0;">
                <button id="themeToggle" class="btn btn-sm rounded-circle d-flex align-items-center justify-content-center" style="width: 2rem; height: 2rem;">
                    <span id="themeIcon">🌙</span>
                </button>
            </div>
            
            <script>
                // Duplicate theme controls to mobile nav
                document.addEventListener('DOMContentLoaded', () => {{
                    const controls = document.getElementById('theme-controls');
                    const mobileContainer = document.getElementById('mobile-theme-controls');
                    if (window.innerWidth < 992) {{
                        mobileContainer.appendChild(document.getElementById('primaryColorPicker'));
                        mobileContainer.appendChild(document.getElementById('themeToggle'));
                    }}
                }});
            </script>
            
            <script>
                const root = document.documentElement;
                const colorPicker = document.getElementById('primaryColorPicker');
                const themeToggle = document.getElementById('themeToggle');
                const themeIcon = document.getElementById('themeIcon');

                function hexToRgb(hex) {{
                    const r = parseInt(hex.slice(1, 3), 16);
                    const g = parseInt(hex.slice(3, 5), 16);
                    const b = parseInt(hex.slice(5, 7), 16);
                    return `${{r}}, ${{g}}, ${{b}}`;
                }}

                function applyColor(hex) {{
                    root.style.setProperty('--bs-primary', hex);
                    root.style.setProperty('--bs-primary-rgb', hexToRgb(hex));
                    localStorage.setItem('primary-color', hex);
                }}

                // Load preferences
                const savedColor = localStorage.getItem('primary-color') || '#4f46e5';
                const savedTheme = localStorage.getItem('theme') || 'light';
                
                applyColor(savedColor);
                colorPicker.value = savedColor;
                root.setAttribute('data-bs-theme', savedTheme);
                themeIcon.textContent = savedTheme === 'dark' ? '☀️' : '🌙';

                colorPicker.addEventListener('input', (e) => applyColor(e.target.value));

                themeToggle.addEventListener('click', () => {{
                    const currentTheme = root.getAttribute('data-bs-theme');
                    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
                    root.setAttribute('data-bs-theme', newTheme);
                    themeIcon.textContent = newTheme === 'dark' ? '☀️' : '🌙';
                    localStorage.setItem('theme', newTheme);
                }});
            </script>

            <main class="container py-5 position-relative z-1">
                {content}
            </main>
            <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
        </body>
        </html>
        "#,
        title = title,
        content = content
    )
}
