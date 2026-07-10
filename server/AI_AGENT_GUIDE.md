# AI Agent Development Guide

**Complete guide for AI agents working with this Rust + Axum + Hotwire ERP system.**

---

## 🎯 Quick Reference

### Project Structure
```
crates/
├── server/          # Main binary - runs the server
├── shared/          # Cross-domain utilities (DB, layout, middleware)
└── student/         # Student domain (example domain)
    ├── lib.rs                    # Domain entry point
    ├── shared.rs                 # Domain-specific shared code
    ├── view_student.rs           # One feature = one file
    └── edit_student_turbo.rs     # Another feature
```

### Tech Stack
- **Web Framework**: Axum (async Rust)
- **Database**: SQLite + SQLx
- **Templates**: Maud (type-safe HTML in Rust)
- **Frontend**: Hotwire Turbo + Stimulus
- **CSS**: Bootstrap 5

---

## 🚀 Adding a New Feature

### Step 1: Create Feature File
```rust
// crates/student/src/create_student.rs

use axum::{Router, extract::Extension, response::{Html, IntoResponse}, routing::{get, post}, Form};
use maud::{html, Markup};
use sqlx::SqlitePool;
use serde::Deserialize;
use super::AppState;

// ============================================================================
// Models
// ============================================================================

#[derive(Deserialize)]
pub struct CreateStudentForm {
    pub name: String,
}

// ============================================================================
// Database Layer
// ============================================================================

async fn insert_student(pool: &SqlitePool, name: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO students (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}

// ============================================================================
// Templates
// ============================================================================

fn render_create_form() -> Markup {
    html! {
        div.container.mt-5 {
            div.card {
                div.card-header.bg-primary.text-white {
                    h2 { "Create Student" }
                }
                div.card-body {
                    form method="post" action="/students" {
                        div.mb-3 {
                            label.form-label for="name" { "Name" }
                            input#name.form-control type="text" name="name" required;
                        }
                        button.btn.btn-primary type="submit" { "Create" }
                    }
                }
            }
        }
    }
}

// ============================================================================
// Handlers
// ============================================================================

async fn show_form_handler() -> Html<String> {
    Html(render_create_form().into_string())
}

async fn submit_handler(
    Extension(pool): Extension<SqlitePool>,
    Form(form): Form<CreateStudentForm>,
) -> impl IntoResponse {
    match insert_student(&pool, &form.name).await {
        Ok(id) => axum::response::Redirect::to(&format!("/student/{}", id)),
        Err(_) => axum::response::Redirect::to("/students/new"),
    }
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/students/new", get(show_form_handler))
        .route("/students", post(submit_handler))
}
```

### Step 2: Register in Domain
```rust
// crates/student/src/lib.rs
mod create_student;  // Add this

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(view_student::routes())
        .merge(create_student::routes())  // Add this
}
```

**That's it!** ✅ Feature is live.

---

## 🎨 Hotwire Turbo Integration

### Key Concepts

1. **Turbo Drive**: Automatic AJAX navigation (no full page reloads)
2. **Turbo Frames**: Independent page sections
3. **Turbo Streams**: Multi-element updates

### Include Turbo in Layout

```rust
use shared::layout::render_layout;

let html = render_layout("Page Title", content, true); // true = include Hotwire
```

This adds:
```html
<script type="module">
  import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';
</script>
```

### Turbo Frame Example

```rust
// View mode
fn render_profile(student: &Student) -> Markup {
    html! {
        turbo-frame id="student-profile" {
            div.card {
                div.card-body {
                    p { "Name: " (student.name) }
                    a.btn.btn-primary 
                      href={"/student/" (student.id) "/edit"}
                      data-turbo-frame="student-profile" {
                        "Edit"
                    }
                }
            }
        }
    }
}

// Edit mode
fn render_edit_form(student: &Student) -> Markup {
    html! {
        turbo-frame id="student-profile" {
            form method="post" action={"/student/" (student.id)} {
                input type="text" name="name" value=(student.name);
                button type="submit" { "Save" }
            }
        }
    }
}
```

**Result**: Clicking "Edit" replaces only the frame content, no page reload!

### Turbo Stream Example

```rust
use axum::response::{Response, IntoResponse};

async fn update_student(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateForm>,
) -> Response {
    let student = /* update database */;
    
    // Return Turbo Stream response
    let html = html! {
        // Update multiple targets at once!
        turbo-stream action="replace" target="student-profile" {
            template {
                (render_profile(&student))
            }
        }
        turbo-stream action="update" target="last-modified" {
            template {
                span { "Updated just now" }
            }
        }
    };
    
    (
        [("Content-Type", "text/vnd.turbo-stream.html")],
        Html(html.into_string())
    ).into_response()
}
```

### Critical CDN URLs

**Always use these exact URLs** (tested and working):

```rust
// Turbo
import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';

// Stimulus  
import { Application } from 'https://cdn.jsdelivr.net/npm/@hotwired/stimulus@3.2.2/+esm';
```

❌ **Don't use**: `cdn.skypack.dev` (has compatibility issues)

---

## 🗄️ Database Patterns

### Shared Queries (in `shared/src/db.rs`)

```rust
pub async fn get_student_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Student>, sqlx::Error> {
    sqlx::query_as!(Student, "SELECT * FROM students WHERE id = ?", id)
        .fetch_optional(pool)
        .await
}
```

### Domain-Specific Queries (in domain feature file)

```rust
async fn search_students(pool: &SqlitePool, query: &str) -> Result<Vec<Student>, sqlx::Error> {
    sqlx::query_as!(
        Student,
        "SELECT * FROM students WHERE name LIKE ?",
        format!("%{}%", query)
    )
    .fetch_all(pool)
    .await
}
```

---

## 🎨 Layout & Templates

### Using Shared Layout

```rust
use shared::layout::{render_layout, render_card};

// Full page with navbar
let content = render_card("Student Profile", student_html, None);
let page = render_layout("Student Profile", content, true);
Html(page.into_string())
```

### Custom Layout

```rust
use maud::{DOCTYPE, html, Markup};

fn custom_page(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "My Page" }
                // Include Turbo
                script type="module" {
                    (PreEscaped("import * as Turbo from 'https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm';"))
                }
            }
            body {
                (content)
            }
        }
    }
}
```

---

## 🛤️ Routing Patterns

### Path Parameters

```rust
// Route definition - use curly braces {}
Router::new()
    .route("/student/{id}", get(handler))
    .route("/student/{id}/edit", get(edit_handler))

// Handler - extract with Path
async fn handler(Path(id): Path<i64>) -> Html<String> {
    // Use id here
}

// Multiple params
async fn handler(Path((id, section)): Path<(i64, String)>) -> Html<String> {
    // Use id and section
}
```

### Query Parameters

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_handler(Query(params): Query<Pagination>) -> Html<String> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    // ...
}
```

### Form Data

```rust
use axum::Form;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login_handler(Form(form): Form<LoginForm>) -> impl IntoResponse {
    // Use form.username and form.password
}
```

---

## 🏗️ Adding a New Domain

### 1. Create Crate Structure

```bash
mkdir -p crates/finance/src
```

### 2. Create Cargo.toml

```toml
[package]
name = "finance"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
maud = "0.26"
serde = { version = "1.0", features = ["derive"] }
shared = { path = "../shared" }
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite"] }
tokio = { version = "1", features = ["full"] }
```

### 3. Create lib.rs

```rust
// crates/finance/src/lib.rs
mod shared;
mod view_invoice;

use axum::Router;
use ::shared::middleware::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(view_invoice::routes())
}
```

### 4. Create shared.rs

```rust
// crates/finance/src/shared.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Invoice {
    pub id: i64,
    pub amount: f64,
}

pub mod db {
    use super::*;
    use sqlx::SqlitePool;

    pub async fn get_invoice_by_id(pool: &SqlitePool, id: i64) 
        -> Result<Option<Invoice>, sqlx::Error> 
    {
        sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE id = ?", id)
            .fetch_optional(pool)
            .await
    }
}
```

### 5. Add to server

```rust
// crates/server/src/main.rs
let app = Router::new()
    .merge(student::routes())
    .merge(finance::routes())  // Add this
    // ...
```

### 6. Update workspace Cargo.toml

```toml
[workspace]
members = ["crates/server", "crates/shared", "crates/student", "crates/finance"]
```

---

## 🧪 Common Patterns

### HTML + JSON Endpoints

```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/students", get(list_html))
        .route("/api/students", get(list_json))
}

async fn list_html(Extension(pool): Extension<SqlitePool>) -> Html<String> {
    let students = get_all_students(&pool).await.unwrap();
    Html(render_student_list(&students).into_string())
}

async fn list_json(Extension(pool): Extension<SqlitePool>) -> Json<Vec<Student>> {
    let students = get_all_students(&pool).await.unwrap();
    Json(students)
}
```

### Error Handling

```rust
use axum::http::StatusCode;

async fn handler(Extension(pool): Extension<SqlitePool>, Path(id): Path<i64>) 
    -> Result<Html<String>, StatusCode> 
{
    match get_student(&pool, id).await {
        Ok(Some(student)) => Ok(Html(render(&student).into_string())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
```

### Redirects

```rust
use axum::response::Redirect;

async fn after_create() -> Redirect {
    Redirect::to("/students")
}
```

---

## 🎯 Best Practices

### ✅ DO

1. **Keep features in one file** (unless >500 lines)
2. **Use Turbo Frame IDs** that match feature names
3. **Return Turbo Streams** for multi-element updates
4. **Use shared layout helpers** from `shared::layout`
5. **Put domain-wide queries** in `domain/shared.rs`
6. **Use correct CDN URLs** (jsdelivr, not skypack)

### ❌ DON'T

1. **Don't split features** across multiple files unnecessarily
2. **Don't use old route syntax** (`:id` instead of `{id}`)
3. **Don't mix Turbo versions** or CDNs
4. **Don't forget Content-Type** for Turbo Streams
5. **Don't put domain logic** in `shared/`

---

## 📊 Architecture Decision Record

### Why Feature-Based Organization?

**Traditional** (hard for AI):
```
controllers/student_controller.rs
models/student.rs
views/student/show.html
services/student_service.rs
```

**Our approach** (AI-friendly):
```
student/src/view_student.rs  ← Everything in one file
```

**Benefits:**
- AI sees complete context in one place
- No jumping between files
- Clear feature boundaries
- Easy to add/modify features

### Why Maud Over Templates?

1. **Type safety**: Catch errors at compile time
2. **Colocation**: Templates next to logic
3. **No separate template language**: Just Rust
4. **AI-friendly**: Clear syntax, easy to generate

### Why Multi-Tenancy by Default?

- **Header-based**: `X-Tenant-ID` header
- **Database per tenant**: Isolated data
- **Zero code changes**: Middleware handles it
- **Optional**: Works without header (uses default DB)

---

## 🔧 Development Workflow

### Run Server
```bash
cargo run -p server
# Server runs on http://localhost:3000
```

### Test Endpoints
```bash
# View student
curl http://localhost:3000/student/1

# JSON API
curl http://localhost:3000/api/student/1

# With tenant header
curl -H "X-Tenant-ID: company1" http://localhost:3000/student/1
```

### Add Migration
```rust
// In main.rs or migration file
sqlx::query("CREATE TABLE IF NOT EXISTS invoices (id INTEGER PRIMARY KEY, amount REAL)")
    .execute(&pool)
    .await?;
```

---

## 🎓 Working Example

See `crates/student/src/edit_student_turbo.rs` for a complete Turbo Frame + Stream example:

- ✅ Turbo Frame switching between view/edit
- ✅ Turbo Stream updating multiple elements
- ✅ Proper Content-Type headers
- ✅ Working CDN imports

**Test it:**
```
http://localhost:3000/turbo-demo/student/1
```

---

## 📚 Quick Links

- **Axum Docs**: https://docs.rs/axum
- **Maud Docs**: https://maud.lambda.xyz
- **Turbo Docs**: https://turbo.hotwired.dev
- **SQLx Docs**: https://docs.rs/sqlx

---

## 🆘 Troubleshooting

### Turbo Not Working?

1. Check CDN URL (use jsdelivr, not skypack)
2. Check import syntax (`import * as Turbo`, not default import)
3. Check Content-Type for Turbo Streams: `text/vnd.turbo-stream.html`
4. Check Frame IDs match between view and edit modes

### Database Errors?

1. Check table exists
2. Check column names match struct fields
3. Use `sqlx::query_as!` for compile-time checked queries

### Routes Not Working?

1. Use curly braces: `/student/{id}` not `/student/:id`
2. Check route is registered in `lib.rs`
3. Check domain routes are merged in `main.rs`

---

**This is your single source of truth for AI agent development. Refer to this guide for all patterns and conventions.**
