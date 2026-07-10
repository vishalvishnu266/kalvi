# ERP System - AI-Friendly Rust + Axum + Hotwire

A modern ERP system built with **Rust**, **Axum**, **Hotwire Turbo**, and **SQLite**, using a **feature-based architecture** optimized for AI agent development.

---

## 🎯 Quick Start

```bash
cargo run -p server
```

**Test Endpoints:**
- HTML: http://localhost:3000/student/1
- JSON API: http://localhost:3000/api/student/1
- **Turbo Demo**: http://localhost:3000/turbo-demo/student/1 ← Try this!

---

## 📚 Documentation

### → **[AI_AGENT_GUIDE.md](AI_AGENT_GUIDE.md)** ← Complete Development Guide

That single file contains everything you need:
- ✅ Adding features & domains
- ✅ Hotwire Turbo integration (Frames + Streams)
- ✅ Database & routing patterns
- ✅ Layout & template helpers
- ✅ Best practices & working examples
- ✅ Troubleshooting guide

---

## 📁 Project Structure

```
crates/
├── server/          # Main binary (Axum server)
├── shared/          # Cross-domain utilities (DB, layout, middleware)
└── student/         # Example domain
    ├── lib.rs                    # Domain routes
    ├── shared.rs                 # Domain models & queries
    ├── view_student.rs           # Feature: View student profile
    └── edit_student_turbo.rs     # Feature: Edit with Turbo Frames/Streams
```

**Key Principle: One feature = One file** (models, queries, templates, handlers, routes all together)

---

## 🛠️ Technology Stack

- **Web Framework**: Axum (async Rust)
- **Database**: SQLite + SQLx (compile-time checked queries)
- **Templates**: Maud (type-safe HTML in Rust)
- **Frontend**: Hotwire Turbo + Stimulus (minimal JavaScript)
- **CSS**: Bootstrap 5
- **Architecture**: Multi-tenant with header-based routing

---

## 🎯 Why AI-Friendly Architecture?

### Traditional (AI unfriendly):
```
controllers/student_controller.rs    # Handler logic
models/student.rs                     # Data structures  
views/student/show.html               # Templates
services/student_service.rs           # Business logic
```
❌ AI must read 4+ files to understand one feature

### Our Approach (AI friendly):
```rust
student/src/view_student.rs  // Everything in ONE file:
  ├─ Models (Student struct)
  ├─ Database queries
  ├─ Templates (Maud HTML)
  ├─ HTTP handlers
  └─ Routes
```
✅ AI sees complete context in one place

---

## 🚀 Feature File Pattern

```rust
// crates/student/src/view_student.rs - Complete feature in ONE file

use axum::{Router, extract::{Extension, Path}, response::Html, routing::get};
use maud::{html, Markup};
use sqlx::SqlitePool;
use serde::{Serialize, Deserialize};

// 1. Models
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Student {
    pub id: i64,
    pub name: String,
}

// 2. Database
async fn fetch_student(pool: &SqlitePool, id: i64) -> Result<Option<Student>, sqlx::Error> {
    sqlx::query_as!(Student, "SELECT * FROM students WHERE id = ?", id)
        .fetch_optional(pool)
        .await
}

// 3. Templates (Maud - type-safe HTML)
fn render_student_profile(student: &Student) -> Markup {
    html! {
        div.card {
            div.card-body {
                h2 { "Student: " (student.name) }
                p { "ID: " (student.id) }
            }
        }
    }
}

// 4. HTTP Handlers
async fn view_student_handler(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> Html<String> {
    match fetch_student(&pool, id).await {
        Ok(Some(student)) => Html(render_student_profile(&student).into_string()),
        _ => Html("<h1>Not Found</h1>".to_string()),
    }
}

// 5. Routes (Note: Axum uses {id} not :id)
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/student/{id}", get(view_student_handler))
}
```

**Everything together = Maximum AI context**

---

## 🎨 Hotwire Turbo Integration

### Turbo Frames (Partial Updates)

```rust
// View mode
turbo-frame id="student-profile" {
    div.card {
        p { "Name: " (student.name) }
        a href={"/student/" (student.id) "/edit"} 
          data-turbo-frame="student-profile" { "Edit" }
    }
}

// Edit mode (same frame ID)
turbo-frame id="student-profile" {
    form method="post" action={"/student/" (student.id)} {
        input type="text" name="name" value=(student.name);
        button type="submit" { "Save" }
    }
}
```

**Result**: Clicking "Edit" updates only the frame, no page reload!

### Turbo Streams (Multiple Updates)

```rust
use axum::response::{Response, IntoResponse};

async fn update_student(/*...*/) -> Response {
    let student = /* update database */;
    
    let html = html! {
        // Update multiple elements at once!
        turbo-stream action="replace" target="student-profile" {
            template { (render_profile(&student)) }
        }
        turbo-stream action="update" target="timestamp" {
            template { "Updated just now" }
        }
    };
    
    (
        [("Content-Type", "text/vnd.turbo-stream.html")],
        Html(html.into_string())
    ).into_response()
}
```

**See `crates/student/src/edit_student_turbo.rs` for complete working example!**

---

## 🏗️ Adding a New Feature

**See [AI_AGENT_GUIDE.md](AI_AGENT_GUIDE.md) for detailed steps.**

Quick version:

1. **Create file**: `crates/student/src/create_student.rs`
2. **Add models, queries, templates, handlers, routes** (all in that one file)
3. **Register in `lib.rs`**:
   ```rust
   mod create_student;
   
   pub fn routes() -> Router<AppState> {
       Router::new()
           .merge(create_student::routes())
   }
   ```

Done! ✅

---

## 🧪 Working Examples

### 1. View Student Profile
- **File**: `crates/student/src/view_student.rs`
- **Routes**: `/student/{id}` (HTML), `/api/student/{id}` (JSON)
- **Features**: Database query, Maud templates, dual endpoints

### 2. Edit with Turbo
- **File**: `crates/student/src/edit_student_turbo.rs`
- **Route**: `/turbo-demo/student/{id}`
- **Features**: Turbo Frames, Turbo Streams, multi-element updates
- **Try it**: http://localhost:3000/turbo-demo/student/1

---

## 🎯 Design Principles

1. ✅ **One feature = One file** (unless >500 lines)
2. ✅ **Templates next to logic** (Maud, not separate files)
3. ✅ **Type safety everywhere** (Rust + Maud + SQLx macros)
4. ✅ **Clear boundaries** (Domain crates, not layers)
5. ✅ **AI-friendly** (Complete context in one place)

---

## 📊 Multi-Tenancy

Built-in header-based tenant routing:

```bash
# Request for tenant "company1"
curl -H "X-Tenant-ID: company1" http://localhost:3000/student/1

# Different database pool automatically used
```

Middleware in `shared/src/middleware.rs` handles everything. Each tenant gets isolated SQLite database.

---

## 🔧 Common Patterns

### HTML + JSON Endpoints

```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/students", get(list_html))
        .route("/api/students", get(list_json))
}
```

### Error Handling

```rust
match get_student(&pool, id).await {
    Ok(Some(student)) => Ok(Html(render(&student).into_string())),
    Ok(None) => Err(StatusCode::NOT_FOUND),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
}
```

### Redirects

```rust
use axum::response::Redirect;
Redirect::to("/students")
```

---

## 🆘 Troubleshooting

**Turbo not working?**
- Check CDN: Use `https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/+esm` (NOT skypack)
- Check import: `import * as Turbo` (NOT default import)
- Check Content-Type: `text/vnd.turbo-stream.html` for Turbo Streams

**Routes not matching?**
- Use curly braces: `/student/{id}` ✅ not `/student/:id` ❌

**Database errors?**
- Check table exists
- Use `sqlx::query_as!` for compile-time checked queries

**More help**: See [AI_AGENT_GUIDE.md](AI_AGENT_GUIDE.md) troubleshooting section

---

## 📄 License

MIT

---

**Built for AI agents to understand and modify efficiently. Every feature is self-contained, type-safe, and easy to reason about.**
