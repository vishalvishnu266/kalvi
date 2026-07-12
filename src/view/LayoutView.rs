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
            const userDark = {dark_mode};
            const savedTheme = localStorage.getItem('theme');
            const savedColor = localStorage.getItem('primary-color');
            const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            
            let isDark = false;
            if (savedTheme) {{
                isDark = savedTheme === 'dark';
            }} else {{
                isDark = userDark || systemDark;
            }}
            
            console.log('Theme init:', {{savedTheme, userDark, systemDark, isDark}});
            
            if (isDark) {{
                document.documentElement.classList.add('dark');
            }}
            
            function updatePrimaryColor(color) {{
                document.documentElement.style.setProperty('--primary-color', color);
                // Simple hex to rgb conversion
                const r = parseInt(color.slice(1, 3), 16);
                const g = parseInt(color.slice(3, 5), 16);
                const b = parseInt(color.slice(5, 7), 16);
                document.documentElement.style.setProperty('--primary-color-rgb', `${{r}}, ${{g}}, ${{b}}`);
            }}

            if (savedColor) {{
                updatePrimaryColor(savedColor);
            }} else {{
                updatePrimaryColor('{primary_color}');
            }}
        }})();

        document.addEventListener('turbo:load', () => {{
            const updatePrimaryColor = (color) => {{
                document.documentElement.style.setProperty('--primary-color', color);
                const r = parseInt(color.slice(1, 3), 16);
                const g = parseInt(color.slice(3, 5), 16);
                const b = parseInt(color.slice(5, 7), 16);
                document.documentElement.style.setProperty('--primary-color-rgb', `${{r}}, ${{g}}, ${{b}}`);
            }};

            const themeToggle = document.getElementById('theme-toggle');
            if (themeToggle) {{
                themeToggle.addEventListener('click', () => {{
                    const isDark = document.documentElement.classList.toggle('dark');
                    localStorage.setItem('theme', isDark ? 'dark' : 'light');
                }});
            }}

            const colorPickers = document.querySelectorAll('.color-picker');
            colorPickers.forEach(picker => {{
                picker.addEventListener('click', (e) => {{
                    const color = e.target.dataset.color;
                    if (color) {{
                        updatePrimaryColor(color);
                        localStorage.setItem('primary-color', color);
                    }}
                }});
            }});
        }});
    </script>
</head>
<body class="bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100 min-h-screen transition-colors duration-300">
    <div id="app-container" class="relative">
        {content}
    </div>
</body>
</html>"#,
        title = ctx.title,
        primary_color = ctx.primary_color,
        dark_mode = ctx.dark_mode,
        content = content
    )
}
