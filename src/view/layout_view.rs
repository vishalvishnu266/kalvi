pub struct LayoutContext {
    pub title: String,
    pub tenant_name: Option<String>,
}

impl Default for LayoutContext {
    fn default() -> Self {
        Self {
            title: "Kalvi ERP".to_string(),
            tenant_name: None,
        }
    }
}

pub fn render_layout(ctx: LayoutContext, content: String) -> String {
    let page_title = match &ctx.tenant_name {
        Some(name) => format!("{} - {}", ctx.title, name),
        None => ctx.title.clone(),
    };

    format!(
        //language=HTML
        r###"<!DOCTYPE html>
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
        :root {{
            --primary-color: #3b82f6;
            --primary-color-rgb: 59, 130, 246;
        }}
        .dark {{ color-scheme: dark; }}
        body {{ min-height: 100vh; }}
    </style>
    <script>
        // language=javascript
        (function() {{
            const savedTheme = localStorage.getItem('kalvi_theme');
            const savedColor = localStorage.getItem('kalvi_color');
            const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            
            if (savedTheme === 'dark' || (!savedTheme && systemDark)) {{
                document.documentElement.classList.add('dark');
            }}
            
            if (savedColor) {{
                document.documentElement.style.setProperty('--primary-color', savedColor);
            }}
        }})();

        document.addEventListener('turbo:load', () => {{
            // Theme Toggle logic
            const themeBtn = document.getElementById('theme-toggle');
            if (themeBtn) {{
                themeBtn.onclick = () => {{
                    const isDark = document.documentElement.classList.toggle('dark');
                    localStorage.setItem('kalvi_theme', isDark ? 'dark' : 'light');
                }};
            }}
        }});
    </script>
</head>
<body class="bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100 transition-colors duration-300">
    <div id="app-root" class="min-h-screen flex flex-col">
        {content}
    </div>
</body>
</html>"###,
        title = page_title,
        content = content
    )
}
