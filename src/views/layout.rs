pub fn base_layout(title: &str, content: &str) -> String {
    format!(
        //language=HTML
        r#"
        <!DOCTYPE html>
        <html lang="en" class="light">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{title}</title>
            <script src="https://cdn.tailwindcss.com"></script>
            <script>
                tailwind.config = {{
                    darkMode: 'class',
                    theme: {{
                        extend: {{
                            colors: {{
                                primary: 'var(--primary-color, #4f46e5)',
                            }},
                            boxShadow: {{
                                'subtle': '0 4px 6px -1px rgba(0, 0, 0, 0.05), 0 2px 4px -1px rgba(0, 0, 0, 0.03)',
                            }}
                        }}
                    }}
                }}
            </script>
            <style>
                :root {{
                    --primary-color: #4f46e5;
                }}
                body {{
                    background-color: #f8fafc;
                    min-height: 100vh;
                }}
                .dark body {{
                    background-color: #020617;
                }}
                .bg-mesh {{
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    height: 400px;
                    background: radial-gradient(circle at 50% -20%, var(--primary-color), transparent 70%);
                    opacity: 0.15;
                    pointer-events: none;
                    z-index: 0;
                }}
                .glass-card {{
                    background: rgba(255, 255, 255, 0.7);
                    backdrop-filter: blur(12px);
                    border: 1px solid rgba(255, 255, 255, 0.3);
                }}
                .dark .glass-card {{
                    background: rgba(15, 23, 42, 0.6);
                    border: 1px solid rgba(255, 255, 255, 0.05);
                }}
            </style>
            <script src="https://unpkg.com/@hotwired/turbo@8.0.0/dist/turbo.es2017-umd.js"></script>
        </head>
        <body class="antialiased transition-colors duration-300 text-slate-900 dark:text-slate-100">
            <div class="bg-mesh"></div>
            <div id="theme-controls" class="fixed top-4 right-4 z-50 flex gap-2 items-center bg-white/50 p-2 rounded-full backdrop-blur-md border border-white/20 shadow-lg">
                <input type="color" id="primaryColorPicker" value="#4f46e5" class="w-8 h-8 rounded-full border-none cursor-pointer" title="Primary Color">
                <button id="themeToggle" class="p-2 rounded-full bg-slate-800 text-white dark:bg-white dark:text-slate-800" title="Toggle Theme">
                    <svg id="sunIcon" class="hidden w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>
                    <svg id="moonIcon" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path></svg>
                </button>
            </div>
            
            <script>
                const root = document.documentElement;
                const colorPicker = document.getElementById('primaryColorPicker');
                const themeToggle = document.getElementById('themeToggle');
                const sunIcon = document.getElementById('sunIcon');
                const moonIcon = document.getElementById('moonIcon');

                // Load preferences
                const savedColor = localStorage.getItem('primary-color') || '#4f46e5';
                const savedTheme = localStorage.getItem('theme') || 'light';
                
                root.style.setProperty('--primary-color', savedColor);
                colorPicker.value = savedColor;
                
                if (savedTheme === 'dark') {{
                    root.classList.add('dark');
                    sunIcon.classList.remove('hidden');
                    moonIcon.classList.add('hidden');
                }}

                colorPicker.addEventListener('input', (e) => {{
                    const color = e.target.value;
                    root.style.setProperty('--primary-color', color);
                    localStorage.setItem('primary-color', color);
                }});

                themeToggle.addEventListener('click', () => {{
                    root.classList.toggle('dark');
                    const isDark = root.classList.contains('dark');
                    sunIcon.classList.toggle('hidden', !isDark);
                    moonIcon.classList.toggle('hidden', isDark);
                    localStorage.setItem('theme', isDark ? 'dark' : 'light');
                }});
            </script>

            <main class="container mx-auto py-6 sm:py-12 px-4 max-w-full overflow-x-hidden">
                {content}
            </main>
        </body>
        </html>
        "#,
        title = title,
        content = content
    )
}

pub fn app_layout(title: &str, sidebar_items: Vec<(&str, &str, bool, &str)>, content: &str) -> String {
    let sidebar = crate::views::components::sidebar(sidebar_items);
    let layout_content = format!(
        //language=HTML
        r#"
        <div class="flex flex-col lg:flex-row min-h-screen lg:min-h-[90vh] max-w-[1600px] mx-auto lg:rounded-[2rem] overflow-hidden shadow-2xl border border-white/20 relative z-10">
            <div class="lg:hidden p-4 bg-white/50 dark:bg-slate-900/50 backdrop-blur-md border-b border-white/20 flex items-center justify-between">
                <span class="text-xl font-black tracking-tighter text-slate-800 dark:text-white">KALVI <span class="text-primary">ERP</span></span>
                <button onclick="document.getElementById('mobile-sidebar').classList.toggle('hidden')" class="p-2 bg-primary/10 text-primary rounded-lg hover:scale-105 active:scale-95 transition-all">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"></path></svg>
                </button>
            </div>
            <div id="mobile-sidebar" class="hidden lg:block shrink-0">
                {sidebar}
            </div>
            <div class="flex-1 p-4 sm:p-8 lg:p-16 overflow-y-auto bg-white/40 dark:bg-slate-900/40 backdrop-blur-md">
                {content}
            </div>
        </div>
        "#,
        sidebar = sidebar,
        content = content
    );
    base_layout(title, &layout_content)
}
