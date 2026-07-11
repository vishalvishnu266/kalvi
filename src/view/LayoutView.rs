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
    let dark_class = if ctx.dark_mode { "dark" } else { "" };
    
    // Using simple multi-line string with comment for IDE injection
    format!(
        /* html */
        r#"<!DOCTYPE html>
<html lang="en" class="{dark_class}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script type="module">
        import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';
        window.Turbo = Turbo;
    </script>
    <script>
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
            --primary-color: {primary_color};
        }}
        /* Fallbacks for non-tailwind elements if any */
        .bg-primary {{ background-color: var(--primary-color); }}
        .text-primary {{ color: var(--primary-color); }}
        .border-primary {{ border-color: var(--primary-color); }}
    </style>
    <script>
        // Synchronize theme and colors when Turbo loads a new body
        document.addEventListener('turbo:load', () => {{
            const body = document.body;
            const isDark = body.dataset.darkMode === 'true';
            const primaryColor = body.dataset.primaryColor;

            if (isDark) {{
                document.documentElement.classList.add('dark');
            }} else {{
                document.documentElement.classList.remove('dark');
            }}

            if (primaryColor) {{
                document.documentElement.style.setProperty('--primary-color', primaryColor);
            }}
        }});
    </script>
</head>
<body 
    class="bg-slate-50 dark:bg-slate-900 text-slate-900 dark:text-slate-100 min-h-screen transition-colors duration-200"
    data-dark-mode="{dark_mode}"
    data-primary-color="{primary_color}"
>
    <div id="app-container">
        {content}
    </div>
</body>
</html>"#,
        dark_class = dark_class,
        dark_mode = ctx.dark_mode,
        title = ctx.title,
        primary_color = ctx.primary_color,
        content = content
    )
}
