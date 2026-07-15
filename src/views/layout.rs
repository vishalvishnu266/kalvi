pub fn base_layout(title: &str, content: &str) -> String {
    format!(
        //language=HTML
        r##"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{title}</title>
            <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css">
            <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
            <style>
                :root {{
                    --primary-color: #4f46e5;
                    --bs-primary: var(--primary-color);
                    --bs-primary-rgb: 79, 70, 229;
                }}

                body {{
                    min-height: 100vh;
                }}

                .btn-primary {{
                    background-color: var(--primary-color);
                    border-color: var(--primary-color);
                }}

                .btn-primary:hover {{
                    background-color: var(--primary-color);
                    border-color: var(--primary-color);
                    filter: brightness(0.9);
                }}
                
                .btn {{
                    transition: all 0.2s ease;
                }}
                
                .btn:active {{
                    transform: scale(0.95);
                }}

                .fw-black {{ font-weight: 900; }}
            </style>
            <script src="https://unpkg.com/@hotwired/turbo@8.0.0/dist/turbo.es2017-umd.js"></script>
        </head>
        <body>
            <div id="theme-controls" class="fixed-top mt-3 me-3 d-flex justify-content-end align-items-center gap-2 z-3">
                <div class="bg-body-secondary p-2 rounded-pill shadow-sm border d-flex gap-2">
                    <input type="color" id="primaryColorPicker" value="#4f46e5" class="form-control form-control-color border-0 bg-transparent p-0 rounded-circle" style="width: 28px; height: 28px;" title="Primary Color">
                    <button id="themeToggle" class="btn btn-outline-secondary btn-sm rounded-circle p-0 d-flex align-items-center justify-content-center" style="width: 28px; height: 28px;" title="Toggle Theme">
                        <i id="themeIcon" class="bi bi-moon-stars"></i>
                    </button>
                </div>
            </div>
            
            <script>
                const root = document.documentElement;
                const colorPicker = document.getElementById('primaryColorPicker');
                const themeToggle = document.getElementById('themeToggle');
                const themeIcon = document.getElementById('themeIcon');

                function hexToRgb(hex) {{
                    const result = /^#?([a-f\d]{{2}})([a-f\d]{{2}})([a-f\d]{{2}})$/i.exec(hex);
                    return result ? `${{parseInt(result[1], 16)}}, ${{parseInt(result[2], 16)}}, ${{parseInt(result[3], 16)}}` : '79, 70, 229';
                }}

                // Load preferences
                const savedColor = localStorage.getItem('primary-color') || '#4f46e5';
                const savedTheme = localStorage.getItem('theme') || 'light';
                
                root.style.setProperty('--primary-color', savedColor);
                root.style.setProperty('--bs-primary-rgb', hexToRgb(savedColor));
                colorPicker.value = savedColor;
                
                if (savedTheme === 'dark') {{
                    root.setAttribute('data-bs-theme', 'dark');
                    themeIcon.className = 'bi bi-sun-fill';
                }}

                colorPicker.addEventListener('input', (e) => {{
                    const color = e.target.value;
                    root.style.setProperty('--primary-color', color);
                    root.style.setProperty('--bs-primary-rgb', hexToRgb(color));
                    localStorage.setItem('primary-color', color);
                }});

                themeToggle.addEventListener('click', () => {{
                    const currentTheme = root.getAttribute('data-bs-theme');
                    const nextTheme = currentTheme === 'dark' ? 'light' : 'dark';
                    root.setAttribute('data-bs-theme', nextTheme);
                    themeIcon.className = nextTheme === 'dark' ? 'bi bi-sun-fill' : 'bi bi-moon-stars';
                    localStorage.setItem('theme', nextTheme);
                }});
            </script>

            <main class="container py-4 py-md-5 position-relative z-1">
                {content}
            </main>
        </body>
        </html>
        "##,
        title = title,
        content = content
    )
}

pub fn app_layout(title: &str, sidebar_items: Vec<(&str, &str, bool, &str)>, content: &str) -> String {
    let sidebar = crate::views::components::sidebar(sidebar_items);
    let layout_content = format!(
        //language=HTML
        r##"
        <div class="container-fluid p-0">
            <div class="row g-0 min-vh-100">
                <!-- Mobile Nav -->
                <div class="col-12 d-lg-none p-3 border-bottom d-flex align-items-center justify-content-between sticky-top z-2 bg-body">
                    <span class="h4 mb-0 fw-black tracking-tighter">KALVI <span class="text-primary">ERP</span></span>
                    <button class="btn btn-link text-primary p-2" type="button" data-bs-toggle="collapse" data-bs-target="#sidebarCollapse">
                        <i class="bi bi-list fs-3"></i>
                    </button>
                </div>
                
                <!-- Sidebar -->
                <div class="col-lg-auto d-lg-block collapse border-end" id="sidebarCollapse" style="width: 260px;">
                    <div class="h-100 bg-body-tertiary">
                        {sidebar}
                    </div>
                </div>
                
                <!-- Content -->
                <div class="col min-vh-100 bg-body">
                    <div class="p-4 p-md-5">
                        {content}
                    </div>
                </div>
            </div>
        </div>
        "##,
        sidebar = sidebar,
        content = content
    );
    base_layout(title, &layout_content)
}
