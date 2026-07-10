use maud::{DOCTYPE, Markup, PreEscaped, html};

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
            // Hotwire Turbo - Using jsdelivr CDN (same as working Node.js example)
            script type="module" {
                (PreEscaped("import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';"))
            }

            // Stimulus
            script type="module" {
                (PreEscaped(r#"
                    import { Application } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
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
                nav.navbar.navbar-expand-lg.navbar-dark.bg-primary {
                    div.container-fluid {
                        a.navbar-brand href="/" { "ERP System" }
                        button.navbar-toggler type="button"
                               data-bs-toggle="collapse"
                               data-bs-target="#navbarNav" {
                            span.navbar-toggler-icon {}
                        }
                        div.collapse.navbar-collapse id="navbarNav" {
                            ul.navbar-nav {
                                li.nav-item {
                                    a.nav-link href="/students" { "Students" }
                                }
                                li.nav-item {
                                    a.nav-link href="/finance" { "Finance" }
                                }
                                li.nav-item {
                                    a.nav-link href="/hr" { "HR" }
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
                footer.bg-light.text-center.py-3.mt-5 {
                    div.container {
                        p.text-muted.mb-0 { "© 2026 ERP System" }
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
        div.container.mt-5 {
            div.row {
                div.col-md-8.offset-md-2 {
                    div.card {
                        div.card-header.bg-primary.text-white {
                            h2.mb-0 { (title) }
                        }
                        div.card-body {
                            (content)
                        }
                        @if let Some(footer_content) = footer {
                            div.card-footer {
                                (footer_content)
                            }
                        }
                    }
                }
            }
        }
    }
}
