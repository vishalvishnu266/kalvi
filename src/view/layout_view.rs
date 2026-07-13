pub struct LayoutContext {
    pub title: String,
    pub primary_color: String,
    pub dark_mode: bool,
    pub tenant_slug: Option<String>,
}

impl Default for LayoutContext {
    fn default() -> Self {
        Self {
            title: "Kalvi ERP".to_string(),
            primary_color: "#3b82f6".to_string(), // blue-500
            dark_mode: false,
            tenant_slug: None,
        }
    }
}

pub fn render_layout(ctx: LayoutContext, content: String) -> String {
    // Using simple multi-line string with comment for IDE injection
    format!(
        //language=HTML
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>{title}</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script type="module">
        import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';
        window.Turbo = Turbo;
    </script>
    <script>
        // language=javascript
        tailwind.config = {{
            darkMode: 'class',
            theme: {{
                extend: {{
                    colors: {{
                        slate: {{
                            950: '#020617', // Deeper black
                            900: '#0f172a',
                        }},
                        primary: {{
                            DEFAULT: 'var(--primary-color)',
                            50: 'color-mix(in srgb, var(--primary-color), white 90%)',
                            100: 'color-mix(in srgb, var(--primary-color), white 80%)',
                            200: 'color-mix(in srgb, var(--primary-color), white 60%)',
                            300: 'color-mix(in srgb, var(--primary-color), white 40%)',
                            400: 'color-mix(in srgb, var(--primary-color), white 20%)',
                            500: 'var(--primary-color)',
                            600: 'color-mix(in srgb, var(--primary-color), black 20%)',
                            700: 'color-mix(in srgb, var(--primary-color), black 40%)',
                            800: 'color-mix(in srgb, var(--primary-color), black 60%)',
                            900: 'color-mix(in srgb, var(--primary-color), black 80%)',
                        }},
                    }}
                }}
            }}
        }}
    </script>
    <style>
        /* language=css */
        :root {{
            --primary-color: {primary_color};
            --primary-color-rgb: 59, 130, 246; /* Default blue, updated by JS */
        }}
        
        .dark {{
            color-scheme: dark;
        }}

        body {{
            background: radial-gradient(circle at top left, rgba(var(--primary-color-rgb), 0.05), transparent 40%),
                        radial-gradient(circle at bottom right, rgba(var(--primary-color-rgb), 0.05), transparent 40%);
        }}

        .dark body {{
            background: radial-gradient(circle at top left, rgba(var(--primary-color-rgb), 0.1), transparent 40%),
                        linear-gradient(to bottom, #0f172a, #020617);
        }}

        /* Custom scrollbar for a polished look */
        ::-webkit-scrollbar {{ width: 8px; }}
        ::-webkit-scrollbar-track {{ background: transparent; }}
        ::-webkit-scrollbar-thumb {{ background: #cbd5e1; border-radius: 10px; }}
        .dark ::-webkit-scrollbar-thumb {{ background: #334155; }}

        .bg-primary {{ background-color: var(--primary-color); }}
        .text-primary {{ color: var(--primary-color); }}
        .border-primary {{ border-color: var(--primary-color); }}
    </style>
    <script>
        // language=javascript
        (function() {{
            // 1. Resolve Priorities: LocalStorage > Server Default > System Default
            const serverDefaultDark = {dark_mode};
            const serverDefaultColor = '{primary_color}';
            
            const savedTheme = localStorage.getItem('kalvi_theme');
            const savedColor = localStorage.getItem('kalvi_primary_color');
            const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            
            // Apply Theme Mode
            const isDark = savedTheme ? (savedTheme === 'dark') : (serverDefaultDark || systemDark);
            if (isDark) document.documentElement.classList.add('dark');
            
            // Apply Primary Color
            const activeColor = savedColor || serverDefaultColor;
            
            function updateCSSVariables(hex) {{
                document.documentElement.style.setProperty('--primary-color', hex);
                // Convert hex to RGB for gradients
                const r = parseInt(hex.slice(1, 3), 16);
                const g = parseInt(hex.slice(3, 5), 16);
                const b = parseInt(hex.slice(5, 7), 16);
                document.documentElement.style.setProperty('--primary-color-rgb', `${{r}}, ${{g}}, ${{b}}`);
            }}
            
            updateCSSVariables(activeColor);
        }})();

        document.addEventListener('turbo:load', () => {{
            const themeToggle = document.getElementById('theme-toggle');
            if (themeToggle) {{
                themeToggle.addEventListener('click', () => {{
                    const isDark = document.documentElement.classList.toggle('dark');
                    localStorage.setItem('kalvi_theme', isDark ? 'dark' : 'light');
                }});
            }}

            // Global color picker logic (for settings page)
            const colorInput = document.querySelector('input[name="user_primary_color"]');
            if (colorInput) {{
                colorInput.value = getComputedStyle(document.documentElement).getPropertyValue('--primary-color').trim();
                colorInput.addEventListener('input', (e) => {{
                    const hex = e.target.value;
                    document.documentElement.style.setProperty('--primary-color', hex);
                    localStorage.setItem('kalvi_primary_color', hex);
                }});
            }}
        }});
    </script>
</head>
<body class="bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100 min-h-screen transition-colors duration-300">
    <div id="app-container" class="relative min-h-screen flex flex-col">
        <div class="flex-grow">
            {content}
        </div>
        
        <footer class="py-8 px-6 border-t dark:border-slate-900 bg-white dark:bg-slate-950/50 backdrop-blur-sm">
            <div class="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center gap-4 text-slate-400 text-xs font-medium">
                <p>&copy; 2026 Kalvi ERP. All rights reserved.</p>
                <div class="flex items-center gap-6">
                    <a href="/contact" class="hover:text-primary transition-colors">Support</a>
                    <a href="#" class="hover:text-primary transition-colors">Privacy</a>
                    <span class="px-2 py-1 bg-slate-100 dark:bg-slate-900 rounded-md border dark:border-slate-800">System v0.1.0</span>
                </div>
            </div>
        </footer>
    </div>
</body>
</html>"#,
        title = ctx.title,
        primary_color = ctx.primary_color,
        dark_mode = ctx.dark_mode,
        content = content
    )
}
