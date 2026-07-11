use crate::web::html::{Html, IntoHtml, e};

pub fn base<H, B>(title: &str, head_extra: H, body: B) -> Html
where
    H: IntoHtml,
    B: IntoHtml,
{
    // Try to get theme settings from request extensions (set by tenant middleware)
    // We can't easily access extensions here without passing them in, so we'll look for them 
    // in the body or pass them as parameters. For now, let's assume default and allow 
    // the views to provide a more specific base if needed.
    
    // Better: let's update the signature to accept optional theme settings
    base_with_theme(title, head_extra, body, "#4f46e5", false)
}

pub fn base_with_theme<H, B>(title: &str, head_extra: H, body: B, primary_color: &str, dark_mode: bool) -> Html
where
    H: IntoHtml,
    B: IntoHtml,
{
    let dark_class = if dark_mode { "dark" } else { "" };
    
    // language=html
    let template = r#"
        <!DOCTYPE html>
        <html lang="en" class="h-full {{dark_class}}">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>{{title}}</title>
            <!-- Tailwind CSS CDN -->
            <script src="https://cdn.tailwindcss.com"></script>
            <script>
                tailwind.config = {
                    darkMode: 'class',
                    theme: {
                        extend: {
                            colors: {
                                primary: '{{primary_color}}',
                            }
                        }
                    }
                }
            </script>
            <style>
                :root {
                    --primary-color: {{primary_color}};
                }
                .dark body { background-color: #111827; color: white; }
                body { background-color: #f9fafb; color: #111827; }
            </style>
            <!-- Lucide Icons -->
            <script src="https://unpkg.com/lucide@latest"></script>
            <!-- Hotwire Turbo -->
            <script type="module">
                import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';
                window.Turbo = Turbo;
            </script>
            {{head_extra}}
        </head>
        <body class="h-full transition-colors duration-200">
            {{body}}
            <!-- Initialize icons -->
            <script>lucide.createIcons();</script>
        </body>
        </html>
    "#;

    Html(template.to_string())
        .replace("title", &e(title))
        .replace("dark_class", dark_class)
        .replace("primary_color", primary_color)
        .replace("head_extra", &head_extra.into_html().0)
        .replace("body", &body.into_html().0)
}
