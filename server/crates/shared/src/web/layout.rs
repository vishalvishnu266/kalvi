use crate::web::html::{Html, IntoHtml, e};

pub fn base<H, B>(title: &str, head_extra: H, body: B) -> Html
where
    H: IntoHtml,
    B: IntoHtml,
{
    // language=html
    let template = r#"
        <!DOCTYPE html>
        <html lang="en" class="h-full bg-gray-50">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>{{title}}</title>
            <!-- Tailwind CSS CDN -->
            <script src="https://cdn.tailwindcss.com"></script>
            <!-- Lucide Icons -->
            <script src="https://unpkg.com/lucide@latest"></script>
            <!-- Hotwire Turbo -->
            <script type="module">
                import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';
                window.Turbo = Turbo;
            </script>
            {{head_extra}}
        </head>
        <body class="h-full">
            {{body}}
            <!-- Initialize icons -->
            <script>lucide.createIcons();</script>
        </body>
        </html>
    "#;

    Html(template.to_string())
        .replace("title", &e(title))
        .replace("head_extra", &head_extra.into_html().0)
        .replace("body", &body.into_html().0)
}
