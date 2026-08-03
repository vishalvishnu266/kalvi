# ERP System Architecture - AI-Friendly Design

## 🎯 Design Philosophy

This project uses **Single-File-Per-Feature** architecture, optimized for AI agent coding. Each feature is self-contained in one file with everything needed: models, database queries, business logic, templates, and HTTP handlers.

## 📁 Project Structure

```
crates/
├── shared/              # Cross-cutting concerns
│   ├── src/
│   │   ├── lib.rs      # Module exports
│   │   ├── db.rs       # Database connection & tenant management
│   │   └── error.rs    # Shared error types
│   └── Cargo.toml
│
├── student/             # Student Management Domain
│   ├── src/
│   │   ├── lib.rs              # Domain entry point & router
│   │   ├── shared.rs           # Shared Student models & queries
│   │   └── view_student.rs     # FEATURE: View student profile
│   │       ├── Models/DTOs
│   │       ├── Database queries
│   │       ├── Business logic
│   │       ├── Maud templates (HTML)
│   │       ├── HTTP handlers
│   │       └── Routes
│   └── Cargo.toml
│
└── server/              # Application entry point
    ├── src/
    │   └── main.rs     # Server setup & initialization
    └── Cargo.toml
```

## 🚀 Why This Structure is AI-Friendly

### ✅ Benefits for AI Agents

1. **Complete Context in One File**
   - AI sees entire feature flow: request → logic → database → template → response
   - No jumping between 4+ files to understand one feature
   - Reduced token usage and confusion

2. **Clear File Boundaries**
   - One feature = One file
   - AI knows exactly where to add/modify code
   - File name matches feature name

3. **Self-Contained Changes**
   - Most changes affect only one file
   - Less coordination needed
   - Fewer merge conflicts

4. **Predictable Patterns**
   - Every feature file follows same structure
   - AI learns pattern once, applies everywhere

5. **Easy Navigation**
   - Want to add "student attendance"? → Create `attendance.rs`
   - Want to modify "view student"? → Edit `view_student.rs`

## 📋 File Organization Pattern

Every feature file follows this structure:

```rust
// feature_name.rs

// 1. IMPORTS
use axum::...;
use maud::...;
use super::shared::{Student, db};

// 2. MODELS/DTOS (if feature-specific)
struct CreateStudentRequest { ... }

// 3. DATABASE LAYER
async fn fetch_student(...) -> Result<...> { ... }

// 4. BUSINESS LOGIC LAYER
pub async fn create_student(...) -> Result<...> { ... }

// 5. TEMPLATES (Maud HTML)
fn render_student_form() -> Markup { 
    html! { ... } 
}

// 6. HTTP HANDLERS
async fn create_student_handler(...) -> impl IntoResponse { ... }

// 7. ROUTES
pub fn routes() -> Router<AppState> { 
    Router::new().route("/students/create", post(...))
}
```

## 🎨 Template Strategy (Maud)

**Keep templates IN THE SAME FILE as the feature!**

### Why?

- AI sees the full picture: handler → business logic → template
- Templates are part of the feature, not separate
- Changes to feature often require template changes
- Better context for AI = better code generation

### Example:

```rust
// In view_student.rs

fn render_student_profile(student: &Student) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Student Profile - " (student.name) }
                style { "/* CSS here */" }
            }
            body {
                h1 { "Student Profile" }
                p { "Name: " (student.name) }
            }
        }
    }
}

async fn view_student_handler(...) -> Html<String> {
    let student = get_student(...).await?;
    Html(render_student_profile(&student).into_string())
}
```

### When to Extract Templates?

Only extract to separate template file if:
- Template is >200 lines
- Template is reused across 3+ features
- Template is purely presentational with no logic

For most cases, keep templates with the feature!

## 🔄 Adding New Features - AI Workflow

### Example: "Add student attendance tracking"

**AI thinks:**
1. Create `crates/student/src/attendance.rs`
2. Write complete feature in one file:
   - AttendanceRecord model
   - Database insert/query functions
   - Mark attendance business logic
   - HTML form template
   - Attendance list template
   - HTTP handlers (GET form, POST submission)
   - Routes
3. Add to `crates/student/src/lib.rs`:
   ```rust
   mod attendance;
   
   pub fn routes() -> Router<AppState> {
       Router::new()
           .merge(view_student::routes())
           .merge(attendance::routes())  // <- Add this
   }
   ```
4. Done! ✅

**Single file created, complete feature working.**

## 📦 Adding New Domains

When adding a new business domain (e.g., Finance, HR):

```bash
crates/
├── finance/              # NEW DOMAIN
│   ├── src/
│   │   ├── lib.rs
│   │   ├── shared.rs    # Invoice model, common queries
│   │   ├── create_invoice.rs
│   │   ├── invoice_list.rs
│   │   └── payment_workflow.rs
│   └── Cargo.toml
```

Then in `server/src/main.rs`:
```rust
let app = Router::new()
    .merge(student::routes())
    .merge(finance::routes())  // Add new domain
```

## 🔧 Shared Code Strategy

### What Goes in `shared/`?

- Database connection management
- Authentication/Authorization
- Error types used across domains
- Common middleware
- Truly cross-domain models (User, Tenant)

### What Goes in `domain/shared.rs`?

- Domain-specific models (Student, Invoice)
- Common database queries within the domain
- Domain utilities
- NOT cross-domain!

### Example:

```rust
// ✅ shared/src/db.rs
pub struct TenantDatabaseManager { ... }  // Used by all domains

// ✅ student/src/shared.rs
pub struct Student { ... }  // Only student domain needs this
pub async fn get_student_by_id(...) { ... }  // Reused by multiple student features

// ✅ finance/src/shared.rs
pub struct Invoice { ... }  // Only finance domain needs this
```

## 🎯 Key Principles

1. **Feature = File**: One screen/workflow = One file
2. **Templates Stay Close**: Keep Maud templates with the feature
3. **Domain Isolation**: Each domain is self-contained
4. **Shared is Minimal**: Only truly shared code goes in `shared/`
5. **AI-First**: Structure optimized for AI understanding and coding

## 📝 When to Split a File?

Split a feature file when:
- File exceeds 500-600 lines
- File handles multiple distinct workflows
- Split by workflow, not by layer

**Wrong way (technical split):**
```
student/models.rs
student/handlers.rs
student/templates.rs
```

**Right way (feature split):**
```
student/create_student.rs    # Everything for creating
student/edit_student.rs       # Everything for editing
student/view_student.rs       # Everything for viewing
```

## 🚀 Running the Project

```bash
# Build the project
cargo build

# Run the server
cargo run -p server

# Access the application
http://localhost:3000/student/1
```

## 📚 Next Steps

To add more features to the student domain:

1. **List Students**: Create `student/src/list_students.rs`
2. **Create Student**: Create `student/src/create_student.rs`
3. **Edit Student**: Create `student/src/edit_student.rs`
4. **Attendance**: Create `student/src/attendance.rs`
5. **Fees**: Create `student/src/fees.rs`

Each file will be self-contained with all layers!

---

**This architecture is designed for rapid development with AI assistance. Keep features self-contained and AI agents will thank you! 🤖**
