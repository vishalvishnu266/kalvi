# Quick Start Guide

## 🎯 TL;DR

This project uses **one file per feature** architecture. Want to add a feature? Create ONE file with everything.

## 📁 Current Structure

```
crates/
├── shared/              # Database, errors (cross-domain)
├── student/             # Student domain
│   ├── shared.rs       # Student model & common queries
│   └── view_student.rs # Complete feature in one file
└── server/             # Main application
```

## ⚡ Add a New Feature (5 Steps)

### Example: Add "Create Student" feature

1. **Create file**: `crates/student/src/create_student.rs`

2. **Write feature** (copy this template):

```rust
use axum::{extract::Extension, response::{Html, IntoResponse}, routing::{get, post}, Router, Form};
use maud::{html, Markup, DOCTYPE};
use sqlx::SqlitePool;
use serde::Deserialize;
use super::{shared::Student, AppState};

// Models
#[derive(Deserialize)]
pub struct CreateStudentRequest {
    pub name: String,
}

// Database
async fn insert_student(pool: &SqlitePool, name: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO students (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}

// Business Logic
pub async fn create_student(pool: &SqlitePool, name: String) -> Result<Student, sqlx::Error> {
    let id = insert_student(pool, &name).await?;
    Ok(Student { id, name })
}

// Templates
fn render_form() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head { title { "Create Student" } }
            body {
                h1 { "Create Student" }
                form method="post" action="/students" {
                    input type="text" name="name" required;
                    button { "Create" }
                }
            }
        }
    }
}

// Handlers
async fn show_form_handler() -> Html<String> {
    Html(render_form().into_string())
}

async fn submit_handler(
    Extension(pool): Extension<SqlitePool>,
    Form(req): Form<CreateStudentRequest>,
) -> impl IntoResponse {
    match create_student(&pool, req.name).await {
        Ok(student) => Html(format!("<h1>Created: {}</h1>", student.name)),
        Err(_) => Html("<h1>Error</h1>".to_string()),
    }
}

// Routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/students/new", get(show_form_handler))
        .route("/students", post(submit_handler))
}
```

3. **Add to lib.rs**: `crates/student/src/lib.rs`

```rust
mod shared;
mod view_student;
mod create_student;  // ADD THIS LINE

// ... existing code ...

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(view_student::routes())
        .merge(create_student::routes())  // ADD THIS LINE
}
```

4. **Build**: `cargo build`

5. **Run**: `cargo run -p server`

Done! ✅

## 🎨 Template Pattern

Every feature file has these sections (in order):

```rust
// 1. IMPORTS
use axum::...;

// 2. MODELS/DTOS
struct MyRequest { }

// 3. DATABASE
async fn fetch_data(...) { }

// 4. BUSINESS LOGIC
pub async fn do_something(...) { }

// 5. TEMPLATES (Maud)
fn render_page() -> Markup { html! { } }

// 6. HANDLERS
async fn my_handler(...) -> impl IntoResponse { }

// 7. ROUTES
pub fn routes() -> Router<AppState> { }
```

## 📦 Add a New Domain

Example: Add Finance domain

1. **Create structure**:
```bash
crates/finance/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── shared.rs
    └── create_invoice.rs
```

2. **Copy Cargo.toml** from `student/Cargo.toml`, rename to `finance`

3. **Copy lib.rs** from `student/src/lib.rs`

4. **Update workspace**: `Cargo.toml` (root)
```toml
[workspace]
members = [
    "crates/shared",
    "crates/student",
    "crates/finance",  # Add this
    "crates/server",
]
```

5. **Update server**: `crates/server/src/main.rs`
```rust
let app = Router::new()
    .merge(student::routes())
    .merge(finance::routes())  // Add this
```

## 🚀 Common Tasks

### View existing student
```
http://localhost:3000/student/1
```

### Get JSON
```
http://localhost:3000/api/student/1
```

### Add database migration

Edit: `crates/shared/src/db.rs` in the `run_migrations` function:

```rust
async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Add your CREATE TABLE statements here
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS new_table (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}
```

## 📚 Full Documentation

- **README.md** - Project overview
- **ARCHITECTURE.md** - Complete architecture guide  
- **EXAMPLE_NEW_FEATURE.md** - Detailed example with "List Students"
- **BEFORE_VS_AFTER.md** - Why this approach is better
- **AI_CODING_GUIDE.md** - Guide for AI agents
- **This file** - Quick reference

## 🎯 Key Principles

1. ✅ **One feature = One file** (unless >500 lines)
2. ✅ **Templates stay with handlers** (in same file)
3. ✅ **Organize by domain** (student, finance) not layers (model, controller)
4. ✅ **Share code sparingly** (only when 3+ features need it)

## ⚠️ Don't Do This

❌ Don't create separate folders for models/controllers/views
❌ Don't split one feature across multiple files
❌ Don't put templates in separate crate
❌ Don't create circular dependencies

## ✅ Do This

✅ Create one file per screen/workflow
✅ Keep everything for that feature in the file
✅ Use clear section comments
✅ Reuse shared models from `domain/shared.rs`

## 🆘 Help

**"Where do I add X?"**

- New student feature → `crates/student/src/feature_name.rs`
- New business domain → `crates/new_domain/`
- Shared Student code → `crates/student/src/shared.rs`
- Cross-domain code → `crates/shared/src/`

**"File is getting too big (>500 lines)"**

Split by **workflow** not by **layer**:
- ✅ Split into: `create_student.rs`, `edit_student.rs`
- ❌ Don't split into: `models.rs`, `handlers.rs`

**"Need to share code between features"**

- Same domain → `domain/src/shared.rs`
- Cross-domain → `crates/shared/src/`

---

**That's it! Simple, clean, AI-friendly. Now start coding! 🚀**
