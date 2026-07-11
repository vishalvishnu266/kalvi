// Feature: Landing/Home Page
// Simple landing page to guide users to onboarding

use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use maud::{DOCTYPE, Markup, html};

use ::shared::middleware::AppState;

// ============================================================================
// Templates
// ============================================================================

fn render_home_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "School ERP System" }
                
                link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"
                     rel="stylesheet"
                     integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN"
                     crossorigin="anonymous";
                
                style {
                    r#"
                    body {
                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                        min-height: 100vh;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                    }
                    .hero-card {
                        background: white;
                        border-radius: 20px;
                        padding: 3rem;
                        box-shadow: 0 20px 60px rgba(0,0,0,0.3);
                        max-width: 600px;
                    }
                    "#
                }
            }
            body {
                div class="container" {
                    div class="hero-card text-center" {
                        h1 class="display-3 mb-4" { "🎓" }
                        h2 class="mb-3" { "School ERP System" }
                        p class="lead text-muted mb-5" {
                            "Comprehensive Education Management Platform"
                        }
                        
                        div class="d-grid gap-3" {
                            a class="btn btn-primary btn-lg" href="/onboard" {
                                "🚀 Onboard New Institution"
                            }
                        }
                        
                        div class="mt-5 pt-4 border-top" {
                            h5 class="text-muted mb-3" { "Features" }
                            div class="row text-start" {
                                div class="col-md-6 mb-2" {
                                    "✓ Multi-tenant Architecture"
                                }
                                div class="col-md-6 mb-2" {
                                    "✓ Student Management"
                                }
                                div class="col-md-6 mb-2" {
                                    "✓ User Management"
                                }
                                div class="col-md-6 mb-2" {
                                    "✓ Secure & Scalable"
                                }
                            }
                        }
                    }
                }
                
                script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js"
                       integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL"
                       crossorigin="anonymous" {}
            }
        }
    }
}

// ============================================================================
// HTTP Handlers
// ============================================================================

async fn home_handler() -> impl IntoResponse {
    Html(render_home_page().into_string())
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home_handler))
}
