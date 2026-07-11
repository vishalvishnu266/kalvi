// Feature: User Dashboard
// Modern, responsive dashboard after login

use axum::{
    Router,
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
};
use maud::{DOCTYPE, Markup, html};

use crate::middleware::RequireAuth;
use ::shared::middleware::AppState;

// ============================================================================
// Templates
// ============================================================================

fn render_dashboard(user: &crate::shared::User, tenant_slug: &str) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Dashboard - School ERP" }
                
                // Bootstrap CSS
                link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"
                     rel="stylesheet"
                     integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN"
                     crossorigin="anonymous";
                
                // Bootstrap Icons
                link href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.1/font/bootstrap-icons.css"
                     rel="stylesheet";
                
                style {
                    (PreEscaped(r#"
                    :root {
                        --primary-gradient: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    }
                    body {
                        background: #f8f9fa;
                        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
                    }
                    .navbar {
                        background: white;
                        box-shadow: 0 2px 4px rgba(0,0,0,0.08);
                    }
                    .navbar-brand {
                        font-weight: 700;
                        background: var(--primary-gradient);
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                    }
                    .user-badge {
                        background: var(--primary-gradient);
                        color: white;
                        padding: 0.5rem 1rem;
                        border-radius: 50px;
                        font-size: 0.875rem;
                        font-weight: 600;
                    }
                    .stats-card {
                        border: none;
                        border-radius: 16px;
                        transition: transform 0.2s, box-shadow 0.2s;
                        height: 100%;
                    }
                    .stats-card:hover {
                        transform: translateY(-4px);
                        box-shadow: 0 12px 24px rgba(0,0,0,0.1);
                    }
                    .stats-card .card-body {
                        padding: 1.5rem;
                    }
                    .stats-icon {
                        width: 56px;
                        height: 56px;
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 1.5rem;
                        margin-bottom: 1rem;
                    }
                    .icon-blue {
                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                        color: white;
                    }
                    .icon-green {
                        background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
                        color: white;
                    }
                    .icon-orange {
                        background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
                        color: white;
                    }
                    .icon-purple {
                        background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
                        color: white;
                    }
                    .quick-action {
                        border: 2px solid #e9ecef;
                        border-radius: 12px;
                        padding: 1.25rem;
                        text-decoration: none;
                        color: inherit;
                        display: block;
                        transition: all 0.2s;
                    }
                    .quick-action:hover {
                        border-color: #667eea;
                        background: #f8f9ff;
                        transform: translateX(4px);
                    }
                    .welcome-card {
                        background: var(--primary-gradient);
                        color: white;
                        border: none;
                        border-radius: 16px;
                        padding: 2rem;
                        margin-bottom: 2rem;
                    }
                    .role-badge {
                        display: inline-block;
                        padding: 0.35rem 0.75rem;
                        background: rgba(255, 255, 255, 0.2);
                        border-radius: 50px;
                        font-size: 0.875rem;
                        font-weight: 600;
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    }
                    "#))
                }
            }
            body {
                // Navbar
                nav class="navbar navbar-expand-lg navbar-light sticky-top" {
                    div class="container-fluid px-4" {
                        a class="navbar-brand" href="/dashboard" {
                            i class="bi bi-mortarboard-fill me-2" {}
                            "School ERP"
                        }
                        
                        div class="d-flex align-items-center" {
                            div class="user-badge me-3" {
                                i class="bi bi-person-circle me-2" {}
                                (user.username)
                            }
                            a href=(format!("/t/{}/logout", tenant_slug)) class="btn btn-outline-danger btn-sm logout-btn" {
                                i class="bi bi-box-arrow-right me-1" {}
                                "Logout"
                            }
                        }
                    }
                }
                
                // Main Content
                div class="container-fluid px-4 py-4" {
                    // Welcome Section
                    div class="welcome-card" {
                        div class="row align-items-center" {
                            div class="col-md-8" {
                                h1 class="mb-2" {
                                    "Welcome back, " (user.username) "! 👋"
                                }
                                p class="mb-2 opacity-90" {
                                    i class="bi bi-envelope me-2" {}
                                    (user.email)
                                }
                                span class="role-badge" {
                                    i class="bi bi-shield-check me-1" {}
                                    (user.role.as_str())
                                }
                            }
                            div class="col-md-4 text-md-end mt-3 mt-md-0" {
                                p class="mb-0 opacity-75" {
                                    small {
                                        i class="bi bi-calendar-event me-1" {}
                                        "Member since " (user.created_at.split('T').next().unwrap_or(&user.created_at))
                                    }
                                }
                            }
                        }
                    }
                    
                    // Stats Cards
                    h5 class="mb-3 text-muted" {
                        i class="bi bi-graph-up me-2" {}
                        "Quick Stats"
                    }
                    div class="row g-3 mb-4" {
                        div class="col-md-3" {
                            div class="card stats-card shadow-sm" {
                                div class="card-body" {
                                    div class="stats-icon icon-blue" {
                                        i class="bi bi-people-fill" {}
                                    }
                                    h3 class="mb-0" { "0" }
                                    p class="text-muted mb-0" { "Total Students" }
                                }
                            }
                        }
                        div class="col-md-3" {
                            div class="card stats-card shadow-sm" {
                                div class="card-body" {
                                    div class="stats-icon icon-green" {
                                        i class="bi bi-person-check-fill" {}
                                    }
                                    h3 class="mb-0" { "0" }
                                    p class="text-muted mb-0" { "Active Users" }
                                }
                            }
                        }
                        div class="col-md-3" {
                            div class="card stats-card shadow-sm" {
                                div class="card-body" {
                                    div class="stats-icon icon-orange" {
                                        i class="bi bi-book-fill" {}
                                    }
                                    h3 class="mb-0" { "0" }
                                    p class="text-muted mb-0" { "Classes" }
                                }
                            }
                        }
                        div class="col-md-3" {
                            div class="card stats-card shadow-sm" {
                                div class="card-body" {
                                    div class="stats-icon icon-purple" {
                                        i class="bi bi-calendar-check-fill" {}
                                    }
                                    h3 class="mb-0" { "0%" }
                                    p class="text-muted mb-0" { "Attendance Today" }
                                }
                            }
                        }
                    }
                    
                    // Quick Actions
                    div class="row" {
                        div class="col-md-8" {
                            h5 class="mb-3 text-muted" {
                                i class="bi bi-lightning-fill me-2" {}
                                "Quick Actions"
                            }
                            div class="card shadow-sm" style="border-radius: 16px; border: none;" {
                                div class="card-body p-0" {
                                    div class="list-group list-group-flush" {
                                        a href="#" class="quick-action" {
                                            div class="d-flex align-items-center" {
                                                div class="flex-shrink-0" {
                                                    div class="stats-icon icon-blue" style="width: 48px; height: 48px; font-size: 1.25rem;" {
                                                        i class="bi bi-person-plus-fill" {}
                                                    }
                                                }
                                                div class="flex-grow-1 ms-3" {
                                                    h6 class="mb-1" { "Add New Student" }
                                                    p class="mb-0 text-muted small" { "Register a new student in the system" }
                                                }
                                                i class="bi bi-chevron-right text-muted" {}
                                            }
                                        }
                                        
                                        a href="#" class="quick-action" {
                                            div class="d-flex align-items-center" {
                                                div class="flex-shrink-0" {
                                                    div class="stats-icon icon-green" style="width: 48px; height: 48px; font-size: 1.25rem;" {
                                                        i class="bi bi-list-check" {}
                                                    }
                                                }
                                                div class="flex-grow-1 ms-3" {
                                                    h6 class="mb-1" { "Mark Attendance" }
                                                    p class="mb-0 text-muted small" { "Record student attendance for today" }
                                                }
                                                i class="bi bi-chevron-right text-muted" {}
                                            }
                                        }
                                        
                                        a href="#" class="quick-action" {
                                            div class="d-flex align-items-center" {
                                                div class="flex-shrink-0" {
                                                    div class="stats-icon icon-orange" style="width: 48px; height: 48px; font-size: 1.25rem;" {
                                                        i class="bi bi-file-earmark-text-fill" {}
                                                    }
                                                }
                                                div class="flex-grow-1 ms-3" {
                                                    h6 class="mb-1" { "Generate Report" }
                                                    p class="mb-0 text-muted small" { "Create attendance and academic reports" }
                                                }
                                                i class="bi bi-chevron-right text-muted" {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        div class="col-md-4 mt-4 mt-md-0" {
                            h5 class="mb-3 text-muted" {
                                i class="bi bi-info-circle-fill me-2" {}
                                "System Info"
                            }
                            div class="card shadow-sm" style="border-radius: 16px; border: none;" {
                                div class="card-body" {
                                    div class="d-flex align-items-start mb-3" {
                                        i class="bi bi-check-circle-fill text-success me-2 mt-1" {}
                                        div {
                                            small class="text-muted" { "System Status" }
                                            div class="fw-bold" { "All Systems Operational" }
                                        }
                                    }
                                    div class="d-flex align-items-start mb-3" {
                                        i class="bi bi-gear-fill text-primary me-2 mt-1" {}
                                        div {
                                            small class="text-muted" { "Your Role" }
                                            div class="fw-bold text-capitalize" { (user.role.as_str()) }
                                        }
                                    }
                                    div class="d-flex align-items-start" {
                                        i class="bi bi-calendar-fill text-info me-2 mt-1" {}
                                        div {
                                            small class="text-muted" { "Account Status" }
                                            div class="fw-bold text-success" { "Active" }
                                        }
                                    }
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

async fn dashboard_handler(
    RequireAuth(user): RequireAuth,
    Extension(tenant_context): Extension<::shared::middleware::TenantContext>,
) -> impl IntoResponse {
    Html(render_dashboard(&user, &tenant_context.slug).into_string())
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/t/{tenant_slug}/dashboard", get(dashboard_handler))
}
