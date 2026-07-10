# AI Coding Guide - How AI Agents Should Work With This Codebase

This guide is optimized for AI coding assistants (like me!) to understand how to efficiently work with this codebase.

## 🎯 Core Principle: One Feature = One File

When a human asks to add/modify a feature, AI should work with **ONE primary file** containing everything.

## 🤖 AI Workflow for Common Tasks

### Task: "Add a new feature to view student attendance"

#### ✅ CORRECT AI Approach:

```
Step 1: Create crates/student/src/attendance.rs
Step 2: Write EVERYTHING in that file:
  - AttendanceRecord model
  - Database queries (SELECT, INSERT)
  - Business logic (mark_present, mark_absent)
  - Maud templates (attendance form, attendance list)
  - HTTP handlers (GET form, POST submit, GET list)
  - Routes (pub fn routes())
  
Step 3: Update crates/student/src/lib.rs:
  - Add: mod attendance;
  - Add: .merge(attendance::routes())

DONE! Feature complete in ONE new file + 2 line changes.
```

#### ❌ WRONG AI Approach (Don't Do This):

```
Step 1: Add Attendance model to crates/model/attendance.rs
Step 2: Add repository to crates/repository/attendance_repository.rs
Step 3: Add service to crates/service/attendance_service.rs
Step 4: Add template to crates/web/attendance_template.rs
Step 5: Add controller to crates/controller/attendance_controller.rs
Step 6: Add router to crates/server/router/attendance_router.rs
Step 7: Update 6 different mod.rs files...

This is WRONG for this project! Too scattered!
```

## 📋 Feature File Template

When creating a new feature file, AI should use this template:

```rust
// Feature: [Feature Name]
// Description: [What this feature does]

use axum::{/* imports */};
use maud::{html, Markup, DOCTYPE};
use sqlx::SqlitePool;
use super::shared::{db, Student}; // Reuse shared models
use super::AppState;

// ============================================================================
// MODELS / DTOs (if feature-specific)
// ============================================================================

#[derive(Deserialize)]
pub struct CreateStudentRequest {
    pub name: String,
}

// ============================================================================
// DATABASE LAYER
// ============================================================================

async fn insert_student(pool: &SqlitePool, name: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO students (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}

// ============================================================================
// BUSINESS LOGIC LAYER
// ============================================================================

pub async fn create_student(
    pool: &SqlitePool, 
    req: CreateStudentRequest
) -> Result<Student, sqlx::Error> {
    // Validation
    if req.name.trim().is_empty() {
        // Handle error
    }
    
    // Insert
    let id = insert_student(pool, &req.name).await?;
    
    // Return
    Ok(Student { id, name: req.name })
}

// ============================================================================
// TEMPLATES (Maud HTML)
// ============================================================================

fn render_create_form() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Create Student" }
                style { "/* CSS here */" }
            }
            body {
                h1 { "Create Student" }
                form method="post" action="/students" {
                    input type="text" name="name" placeholder="Student name" required;
                    button type="submit" { "Create" }
                }
            }
        }
    }
}

fn render_success(student: &Student) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head { title { "Success" } }
            body {
                h1 { "Student Created!" }
                p { "Created student: " (student.name) }
                a href={"/student/" (student.id)} { "View Student" }
            }
        }
    }
}

// ============================================================================
// HTTP HANDLERS
// ============================================================================

async fn create_form_handler() -> Html<String> {
    Html(render_create_form().into_string())
}

async fn create_submit_handler(
    Extension(pool): Extension<SqlitePool>,
    Form(req): Form<CreateStudentRequest>,
) -> impl IntoResponse {
    match create_student(&pool, req).await {
        Ok(student) => Html(render_success(&student).into_string()),
        Err(_) => Html("<h1>Error creating student</h1>".to_string()),
    }
}

// ============================================================================
// ROUTES
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/students/new", get(create_form_handler))
        .route("/students", post(create_submit_handler))
}
```

## 🎨 Template Guidelines for AI

### ✅ DO: Keep templates in the feature file

```rust
// In create_student.rs

fn render_form() -> Markup {
    html! {
        form { /* ... */ }
    }
}

async fn handler() -> Html<String> {
    Html(render_form().into_string())
}
```

**Why?** AI sees template + handler together = better context

### ❌ DON'T: Split templates into separate files (unless >200 lines)

```rust
// DON'T create separate template files for small templates
// templates/student_form.html  ← NO!
```

## 🔄 Reusing Code Across Features

### When multiple features need the same model/query:

Put it in `domain/shared.rs`:

```rust
// student/src/shared.rs

pub struct Student {
    pub id: i64,
    pub name: String,
}

pub mod db {
    pub async fn get_student_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Student>> {
        // Shared query used by multiple features
    }
}
```

Then import in feature files:

```rust
// student/src/view_student.rs
use super::shared::{Student, db};

async fn handler() {
    let student = db::get_student_by_id(&pool, id).await?;
}
```

## 📦 Adding New Domains

When asked to add a new business domain (Finance, HR, etc.):

### Step 1: Create crate structure

```
crates/finance/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── shared.rs
    └── create_invoice.rs  // First feature
```

### Step 2: Cargo.toml

```toml
[package]
name = "finance"
version = "0.1.0"
edition = "2024"

[dependencies]
shared = { path = "../shared" }
axum = "0.8"
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
maud = { version = "0.26", features = ["axum"] }
```

### Step 3: lib.rs

```rust
mod shared;
mod create_invoice;

use axum::Router;
use std::sync::Arc;
use ::shared::TenantDatabaseManager;

#[derive(Clone)]
pub struct AppState {
    pub db_manager: Arc<TenantDatabaseManager>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(create_invoice::routes())
}

pub mod middleware {
    // Copy from student/src/lib.rs
}
```

### Step 4: Update workspace Cargo.toml

```toml
[workspace]
members = [
    "crates/shared",
    "crates/student",
    "crates/finance",  # Add this
    "crates/server",
]
```

### Step 5: Update server/main.rs

```rust
let app = Router::new()
    .merge(student::routes())
    .merge(finance::routes())  // Add this
```

## 🚨 Common AI Mistakes to Avoid

### ❌ Mistake 1: Creating layered structure

```
student/
├── models/      ← NO!
├── services/    ← NO!
├── handlers/    ← NO!
```

✅ **Correct:**
```
student/
├── shared.rs
├── view_student.rs      ← Complete feature
├── create_student.rs    ← Complete feature
```

### ❌ Mistake 2: Separating templates from handlers

```
student/
├── handlers/
│   └── view_student.rs
└── templates/
    └── student_view.rs  ← NO!
```

✅ **Correct:**
```
student/
└── view_student.rs      ← Templates AND handlers together
```

### ❌ Mistake 3: Creating circular dependencies

```rust
// student/src/lib.rs depends on view_student
use view_student::Student;  ← NO!

// view_student.rs depends on lib
use crate::AppState;  ← This creates circular dependency!
```

✅ **Correct:**
```rust
// Put shared types in shared.rs
// student/src/shared.rs
pub struct Student { ... }

// student/src/lib.rs
pub struct AppState { ... }

// student/src/view_student.rs
use super::shared::Student;
use super::AppState;
```

## 🎯 Decision Tree for AI

```
Human asks: "Add [feature]"
    │
    ├─ Is it in existing domain (student, finance)?
    │  │
    │  ├─ YES: Create feature file in that domain
    │  │       Example: student/src/attendance.rs
    │  │
    │  └─ NO: Is it a new domain?
    │         │
    │         └─ YES: Create new domain crate
    │                 Example: crates/hr/
    │
    ├─ Does it need shared models/queries?
    │  │
    │  ├─ YES: Add to domain/src/shared.rs
    │  │
    │  └─ NO: Keep in feature file
    │
    └─ Does template exceed 200 lines?
       │
       ├─ YES: Consider extracting (but ask first)
       │
       └─ NO: Keep template in feature file
```

## 📊 File Size Guidelines

- **Feature file**: 50-500 lines is ideal
- **If >500 lines**: Consider if it's really ONE feature or multiple workflows
- **If >800 lines**: Split by workflow, not by layer
- **Templates**: Keep inline unless >200 lines

## ✅ AI Success Checklist

When completing a feature, verify:

- [ ] Feature is in ONE file (or shared code in shared.rs)
- [ ] Templates are in the same file as handlers
- [ ] Routes are exported via `pub fn routes()`
- [ ] Feature is added to domain's lib.rs
- [ ] Both HTML and JSON endpoints (if applicable)
- [ ] Error handling is present
- [ ] Clear section comments (DATABASE, TEMPLATES, HANDLERS, etc.)

## 🚀 Quick Reference

**Creating new feature:**
1. Create `domain/src/feature_name.rs`
2. Write everything in that file
3. Add `mod feature_name;` to `domain/src/lib.rs`
4. Add `.merge(feature_name::routes())` to routes

**Creating new domain:**
1. Create `crates/new_domain/`
2. Copy structure from `student/`
3. Add to workspace Cargo.toml
4. Add to server/main.rs

**Sharing code:**
- Within domain: Use `domain/src/shared.rs`
- Cross-domain: Use `crates/shared/`

---

**Remember: The goal is to make AI coding as simple and error-free as possible. One feature = One file = Success!** 🎯
