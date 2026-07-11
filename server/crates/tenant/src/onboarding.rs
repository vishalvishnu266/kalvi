// Feature: Tenant Onboarding
// This file handles the complete tenant onboarding flow:
// - Onboarding form UI
// - Form validation
// - Tenant creation
// - Success/error pages

use axum::{
    Router,
    extract::{Extension, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form,
};
use maud::{DOCTYPE, Markup, html};
use sqlx::SqlitePool;
use validator::Validate;

use super::shared::{Tenant, TenantOnboardingForm, db};
use ::shared::middleware::AppState;

// ============================================================================
// Business Logic Layer
// ============================================================================

async fn create_new_tenant(
    master_pool: &SqlitePool,
    form: TenantOnboardingForm,
) -> Result<Tenant, String> {
    // Validate form
    form.validate().map_err(|e| format!("Validation error: {}", e))?;
    
    // Check if slug already exists
    let exists = db::slug_exists(master_pool, &form.slug)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    if exists {
        return Err(format!("Slug '{}' is already taken. Please choose a different one.", form.slug));
    }
    
    // Create tenant record in master database
    let tenant = db::create_tenant(master_pool, &form)
        .await
        .map_err(|e| format!("Failed to create tenant: {}", e))?;
    
    // Initialize tenant's own database
    initialize_tenant_database(&tenant.database_name)
        .await
        .map_err(|e| format!("Failed to initialize tenant database: {}", e))?;
    
    Ok(tenant)
}

async fn initialize_tenant_database(database_name: &str) -> Result<(), sqlx::Error> {
    use sqlx::sqlite::{SqlitePool, SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
    
    let db_url = format!("{}.db", database_name);
    let options = SqliteConnectOptions::new()
        .filename(&db_url)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);
    
    let pool = SqlitePool::connect_with(options).await?;
    
    // Create initial schema for tenant
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS students (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT,
            phone TEXT,
            date_of_birth TEXT,
            enrollment_date TEXT NOT NULL DEFAULT (date('now')),
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(&pool)
    .await?;
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL CHECK(role IN ('admin', 'teacher', 'staff')),
            is_active BOOLEAN NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(&pool)
    .await?;
    
    // Create sessions table
    auth::init_session_tables(&pool).await?;
    
    pool.close().await;
    
    Ok(())
}

// ============================================================================
// Templates (Maud HTML)
// ============================================================================

fn render_onboarding_form(error: Option<&str>) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "School ERP - Tenant Onboarding" }
                
                link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"
                     rel="stylesheet"
                     integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN"
                     crossorigin="anonymous";
                
                style {
                    r#"
                    .hero-section {
                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                        color: white;
                        padding: 3rem 0;
                        margin-bottom: 2rem;
                    }
                    .form-container {
                        max-width: 700px;
                        margin: 0 auto;
                    }
                    "#
                }
            }
            body {
                div class="hero-section" {
                    div class="container text-center" {
                        h1 class="display-4" { "🎓 School ERP System" }
                        p class="lead" { "Welcome! Let's get your institution onboarded" }
                    }
                }
                
                div class="container mb-5" {
                    div class="form-container" {
                        @if let Some(err) = error {
                            div class="alert alert-danger alert-dismissible fade show" role="alert" {
                                strong { "Error: " }
                                (err)
                                button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close" {}
                            }
                        }
                        
                        div class="card shadow" {
                            div class="card-header bg-primary text-white" {
                                h3 class="mb-0" { "Tenant Onboarding Form" }
                            }
                            div class="card-body p-4" {
                                form method="post" action="/onboard" {
                                    div class="mb-4" {
                                        label for="slug" class="form-label" {
                                            strong { "Institution Slug *" }
                                        }
                                        input type="text" 
                                              class="form-control" 
                                              id="slug" 
                                              name="slug" 
                                              placeholder="e.g., greenwood-school"
                                              pattern="[a-z0-9]+(?:-[a-z0-9]+)*"
                                              required;
                                        div class="form-text" {
                                            "Lowercase letters, numbers, and hyphens only. This will be part of your URL: "
                                            code { "/t/your-slug/..." }
                                        }
                                    }
                                    
                                    div class="mb-4" {
                                        label for="name" class="form-label" {
                                            strong { "Institution Name *" }
                                        }
                                        input type="text" 
                                              class="form-control" 
                                              id="name" 
                                              name="name" 
                                              placeholder="e.g., Greenwood International School"
                                              required;
                                    }
                                    
                                    div class="mb-4" {
                                        label for="contact_email" class="form-label" {
                                            strong { "Contact Email *" }
                                        }
                                        input type="email" 
                                              class="form-control" 
                                              id="contact_email" 
                                              name="contact_email" 
                                              placeholder="admin@yourschool.edu"
                                              required;
                                    }
                                    
                                    div class="mb-4" {
                                        label for="contact_phone" class="form-label" {
                                            strong { "Contact Phone *" }
                                        }
                                        input type="tel" 
                                              class="form-control" 
                                              id="contact_phone" 
                                              name="contact_phone" 
                                              placeholder="+1234567890"
                                              required;
                                    }
                                    
                                    div class="mb-4" {
                                        label for="address" class="form-label" {
                                            strong { "Institution Address *" }
                                        }
                                        textarea class="form-control" 
                                                 id="address" 
                                                 name="address" 
                                                 rows="3" 
                                                 placeholder="Street, City, State, ZIP"
                                                 required {}
                                    }
                                    
                                    div class="d-grid gap-2" {
                                        button type="submit" class="btn btn-primary btn-lg" {
                                            "🚀 Create Tenant"
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

fn render_success_page(tenant: &Tenant) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Onboarding Success" }
                
                link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"
                     rel="stylesheet"
                     integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN"
                     crossorigin="anonymous";
                
                style {
                    r#"
                    .success-container {
                        max-width: 700px;
                        margin: 3rem auto;
                    }
                    .celebration {
                        font-size: 4rem;
                        animation: bounce 1s ease infinite;
                    }
                    @keyframes bounce {
                        0%, 100% { transform: translateY(0); }
                        50% { transform: translateY(-20px); }
                    }
                    "#
                }
            }
            body {
                div class="container" {
                    div class="success-container" {
                        div class="card shadow-lg" {
                            div class="card-body text-center p-5" {
                                div class="celebration mb-4" { "🎉" }
                                h1 class="text-success mb-3" { "Success!" }
                                h3 class="mb-4" { (tenant.name) }
                                p class="lead text-muted" { "Your institution has been successfully onboarded." }
                                
                                div class="alert alert-info mt-4" role="alert" {
                                    h5 class="alert-heading" { "Your Access Details" }
                                    hr;
                                    p class="mb-2" {
                                        strong { "Tenant Slug: " }
                                        code class="fs-5" { (tenant.slug) }
                                    }
                                    p class="mb-2" {
                                        strong { "Access URL: " }
                                        code class="fs-6" { "/t/" (tenant.slug) }
                                    }
                                    p class="mb-0 mt-3" {
                                        small class="text-muted" {
                                            "Use this URL prefix to access your tenant-specific pages"
                                        }
                                    }
                                }
                                
                                div class="mt-5" {
                                    a class="btn btn-primary btn-lg" href={"/t/" (tenant.slug) "/student/1"} {
                                        "Go to Dashboard →"
                                    }
                                    " "
                                    a class="btn btn-outline-secondary btn-lg" href="/" {
                                        "← Back to Home"
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

async fn show_onboarding_form_handler() -> impl IntoResponse {
    Html(render_onboarding_form(None).into_string())
}

async fn submit_onboarding_form_handler(
    Extension(master_pool): Extension<SqlitePool>,
    Form(form): Form<TenantOnboardingForm>,
) -> impl IntoResponse {
    match create_new_tenant(&master_pool, form.clone()).await {
        Ok(tenant) => Html(render_success_page(&tenant).into_string()).into_response(),
        Err(error) => Html(render_onboarding_form(Some(&error)).into_string()).into_response(),
    }
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/onboard", get(show_onboarding_form_handler))
        .route("/onboard", post(submit_onboarding_form_handler))
}
