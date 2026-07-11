use crate::web::html::{Html, IntoHtml};
use crate::html;

pub fn base<H, B>(title: &str, head_extra: H, body: B) -> Html
where
    H: IntoHtml,
    B: IntoHtml,
{
    html!(
        // language=html
        r#"<!DOCTYPE html>
        <html lang="en" class="h-full bg-gray-50">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>"#, e(title), r#"</title>
            <!-- Tailwind CSS CDN -->
            <script src="https://cdn.tailwindcss.com"></script>
            <!-- Lucide Icons -->
            <script src="https://unpkg.com/lucide@latest"></script>
            <!-- Hotwire Turbo -->
            <script type="module">
                import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';
                window.Turbo = Turbo;
            </script>"#,
            head_extra,
        r#"</head>
        <body class="h-full">"#,
            body,
            r#"<!-- Initialize icons -->
            <script>lucide.createIcons();</script>
        </body>
        </html>"#
    )
}
