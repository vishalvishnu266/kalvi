use maud::{html, Markup, PreEscaped, DOCTYPE};

/// Common HTML head with Bootstrap and optional Hotwire
pub fn render_head(title: &str, include_hotwire: bool) -> Markup {
    html! {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        title { (title) }
        
        // Bootstrap CSS
        link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" 
             rel="stylesheet" 
             integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN" 
             crossorigin="anonymous";
        
        // Bootstrap Icons (optional)
        link href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.1/font/bootstrap-icons.css" 
             rel="stylesheet";
        
        @if include_hotwire {
            // Hotwire Turbo
            script type="module" {
                (PreEscaped("import hotwiredTurbo from 'https://cdn.skypack.dev/@hotwired/turbo';"))
            }
            
            // Stimulus
            script type="module" {
                (PreEscaped(r#"
                    import { Application } from 'https://cdn.skypack.dev/@hotwired/stimulus';
                    window.Stimulus = Application.start();
                "#))
            }
        }
    }
}

/// Common Bootstrap footer scripts
pub fn render_footer_scripts() -> Markup {
    html! {
        script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js" 
               integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL" 
               crossorigin="anonymous" {}
    }
}

/// Common page layout with Bootstrap navbar
pub fn render_layout(title: &str, content: Markup, include_hotwire: bool) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                (render_head(title, include_hotwire))
            }
            body {
                // Navigation bar
                nav class="navbar navbar-expand-lg navbar-dark bg-primary" {
                    div class="container-fluid" {
                        a class="navbar-brand" href="/" { "ERP System" }
                        button class="navbar-toggler" type="button" 
                               data-bs-toggle="collapse" 
                               data-bs-target="#navbarNav" {
                            span class="navbar-toggler-icon" {}
                        }
                        div class="collapse navbar-collapse" id="navbarNav" {
                            ul class="navbar-nav" {
                                li class="nav-item" {
                                    a class="nav-link" href="/students" { "Students" }
                                }
                                li class="nav-item" {
                                    a class="nav-link" href="/finance" { "Finance" }
                                }
                                li class="nav-item" {
                                    a class="nav-link" href="/hr" { "HR" }
                                }
                            }
                        }
                    }
                }
                
                // Main content
                main {
                    (content)
                }
                
                // Footer
                footer class="bg-light text-center py-3 mt-5" {
                    div.container {
                        p class="text-muted mb-0" { "© 2026 ERP System" }
                    }
                }
                
                (render_footer_scripts())
            }
        }
    }
}

/// Simple card layout
pub fn render_card(title: &str, content: Markup, footer: Option<Markup>) -> Markup {
    html! {
        div class="container mt-5" {
            div.row {
                div class="col-md-8 offset-md-2" {
                    div.card {
                        div class="card-header bg-primary text-white" {
                            h2 class="mb-0" { (title) }
                        }
                        div class="card-body" {
                            (content)
                        }
                        @if let Some(footer_content) = footer {
                            div class="card-footer" {
                                (footer_content)
                            }
                        }
                    }
                }
            }
        }
    }
}
