pub fn base_layout(title: &str, content: &str) -> String {
    format!(
        //language=HTML
        r##"
        <!DOCTYPE html>
        <html lang="en" data-bs-theme="light">
        <head>
          <meta charset="UTF-8">
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
          <title>{title} - SchoolDesk ERP</title>
          <meta name="csrf-token" content="static_csrf_token_for_dev_12345">
          <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
          <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css" rel="stylesheet">
          <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap" rel="stylesheet">
          <link href="/public/css/theme.css" rel="stylesheet">
          <script src="https://unpkg.com/@hotwired/turbo@8.0.0/dist/turbo.es2017-umd.js"></script>
        </head>
        <body>
          <div class="sidebar-overlay"></div>
          {content}
          <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
          <script src="/public/js/theme.js"></script>
        </body>
        </html>
        "##,
        title = title,
        content = content
    )
}

pub fn app_layout(title: &str, sidebar_items: Vec<(&str, &str, bool, &str)>, content: &str) -> String {
    let sidebar = crate::views::components::sidebar(sidebar_items);
    let navbar = crate::views::components::navbar();
    
    let layout_content = format!(
        //language=HTML
        r##"
        {sidebar}
        {navbar}
        <main class="main-content">
            {content}
        </main>
        "##,
        sidebar = sidebar,
        navbar = navbar,
        content = content
    );
    
    base_layout(title, &layout_content)
}
