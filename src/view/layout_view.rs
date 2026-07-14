pub fn layout(title: &str, content: String) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} | Kalvi ERP</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script src="https://unpkg.com/@hotwired/turbo@8.0.4/dist/turbo.es2017-umd.js"></script>
    <script>
        (function() {{
            const theme = localStorage.getItem('kalvi_theme') || 'light';
            const color = localStorage.getItem('kalvi_primary') || '#3b82f6';
            if (theme === 'dark') document.documentElement.classList.add('dark');
            document.documentElement.style.setProperty('--primary-color', color);
        }})();
    </script>
    <style>
        :root {{ --primary-color: #3b82f6; }}
        .bg-primary {{ background-color: var(--primary-color); }}
        .text-primary {{ color: var(--primary-color); }}
        .border-primary {{ border-color: var(--primary-color); }}
    </style>
</head>
<body class="bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100 min-h-screen">
    <nav class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 p-4">
        <div class="max-w-7xl mx-auto flex justify-between items-center">
            <a href="/" class="text-2xl font-bold text-primary">Kalvi</a>
            <div id="nav-links">
                <!-- Navigation items -->
            </div>
        </div>
    </nav>
    <main class="max-w-7xl mx-auto p-6">
        {}
    </main>
</body>
</html>"#,
        title, content
    )
}
