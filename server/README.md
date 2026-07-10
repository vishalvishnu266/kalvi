# ERP System - AI-Friendly Architecture

A modern ERP system built with Rust, Axum, and SQLite, using a **feature-based architecture** optimized for AI agent coding.

## 🎯 What Makes This AI-Friendly?

- **Single File Per Feature**: Each feature (screen/workflow) lives in one file with all layers
- **Templates Inline**: Maud templates stay with handlers for complete context
- **Domain-Driven**: Organized by business domain (student, finance, hr) not technical layers
- **Self-Contained**: Most changes affect only one file
- **Bootstrap UI**: Professional, responsive UI out of the box
- **Hotwire Ready**: Perfect architecture for Turbo + Stimulus integration

## 📁 Project Structure

```
crates/
├── shared/              # Cross-cutting concerns
│   ├── db.rs           # Database & tenant management
│   ├── error.rs        # Shared error types
│   ├── middleware.rs   # Tenant middleware & AppState
│   └── layout.rs       # Bootstrap layout templates
│
├── student/             # Student Management Domain
│   ├── shared.rs       # Student model & common queries
│   └── view_student.rs # Complete "view student" feature
│
└── server/
    └── main.rs         # Application entry point
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Cargo (comes with Rust)

### Build & Run

```bash
# Build the project
cargo build

# Run the server
cargo run -p server

# Server starts on http://localhost:3000
```

### Test Endpoints

```bash
# View student profile (HTML)
http://localhost:3000/student/1

# View student profile (JSON API)
http://localhost:3000/api/student/1
```

## 📚 Documentation

### Start Here
- **[README.md](README.md)** - This file (project overview)
- **[QUICK_START.md](QUICK_START.md)** - Quick reference and templates
- **[IMPROVEMENTS_SUMMARY.md](IMPROVEMENTS_SUMMARY.md)** - Latest improvements (Bootstrap, Hotwire, shared middleware)

### Architecture & Design
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Complete architecture guide and design philosophy
- **[ARCHITECTURE_DIAGRAM.md](ARCHITECTURE_DIAGRAM.md)** - Visual diagrams and flow charts
- **[BEFORE_VS_AFTER.md](BEFORE_VS_AFTER.md)** - Comparison with traditional layered architecture

### Guides & Examples
- **[EXAMPLE_NEW_FEATURE.md](EXAMPLE_NEW_FEATURE.md)** - Step-by-step guide to adding features
- **[HOTWIRE_GUIDE.md](HOTWIRE_GUIDE.md)** - Complete Hotwire/Turbo integration guide
- **[AI_CODING_GUIDE.md](AI_CODING_GUIDE.md)** - Guide for AI coding assistants

## 🎨 Architecture Highlights

### Feature File Structure

Each feature file contains everything in one place:

```rust
// student/src/view_student.rs

// 1. Database queries
async fn fetch_student(...) { ... }

// 2. Business logic
pub async fn get_student(...) { ... }

// 3. Templates (Maud)
fn render_student_profile(...) -> Markup {
    html! { ... }
}

// 4. HTTP handlers
async fn view_student_handler(...) { ... }

// 5. Routes
pub fn routes() -> Router { ... }
```

### Why This Works

✅ **AI sees complete feature context in one file**
✅ **No jumping between 6+ files to understand one feature**
✅ **Templates next to handlers for easy updates**
✅ **Clear boundaries between features**
✅ **Easy to add new features without touching existing code**

## 🔧 Adding New Features

### Example: Add "List Students"

1. Create `crates/student/src/list_students.rs`
2. Write complete feature (models, DB, logic, templates, handlers, routes)
3. Add to `crates/student/src/lib.rs`:
   ```rust
   mod list_students;
   
   pub fn routes() -> Router<AppState> {
       Router::new()
           .merge(view_student::routes())
           .merge(list_students::routes())  // Add this line
   }
   ```

That's it! See [EXAMPLE_NEW_FEATURE.md](EXAMPLE_NEW_FEATURE.md) for complete code.

## 🏗️ Adding New Domains

To add a new business domain (e.g., Finance, HR):

```bash
crates/
├── finance/              # New domain
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── shared.rs    # Invoice model, common queries
│       └── create_invoice.rs  # Feature file
```

Update `server/src/main.rs`:
```rust
let app = Router::new()
    .merge(student::routes())
    .merge(finance::routes())  // Add new domain
```

## 🛠️ Technology Stack

- **[Axum](https://github.com/tokio-rs/axum)** - Web framework
- **[SQLx](https://github.com/launchbadge/sqlx)** - Async SQL toolkit
- **[Maud](https://maud.lambda.xyz/)** - Compile-time HTML templates
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Bootstrap 5.3](https://getbootstrap.com/)** - UI framework
- **SQLite** - Database (with multi-tenant support)
- **Hotwire Ready** - Optional Turbo + Stimulus integration

## 📊 Comparison with Traditional Architecture

| Aspect | Traditional Layers | This Project |
|--------|-------------------|--------------|
| Files per feature | 6+ files | 1 file |
| Code organization | Technical (MVC) | Business domain |
| Template location | Separate crate | With feature |
| AI coordination | High complexity | Low complexity |
| Adding new feature | Update 6+ files | Create 1 file |

See [BEFORE_VS_AFTER.md](BEFORE_VS_AFTER.md) for detailed comparison.

## 🎯 Design Principles

1. **Feature = File**: One screen/workflow = One file (unless >500 lines)
2. **Templates Stay Close**: Keep Maud templates with handlers
3. **Domain Isolation**: Each domain crate is self-contained
4. **Shared is Minimal**: Only truly cross-domain code goes in `shared/`
5. **AI-First**: Optimize for AI agent understanding and coding

## 📝 Current Features

### Infrastructure (Shared)
- ✅ Multi-tenant database management
- ✅ Tenant middleware with header-based routing
- ✅ Bootstrap 5.3 layout templates
- ✅ Reusable page layouts and components
- ✅ Error handling

### Student Domain
- ✅ View student profile (HTML + JSON with Bootstrap UI)
- 🚧 List students (see EXAMPLE_NEW_FEATURE.md)
- 🚧 Create student
- 🚧 Edit student
- 🚧 Attendance tracking

### Future Domains
- 📋 Finance (invoices, payments)
- 👥 HR (employees, payroll)
- 📦 Inventory (products, stock)
- 🛒 Procurement

### Optional Enhancements
- 🎨 Hotwire/Turbo integration (see HOTWIRE_GUIDE.md)
- 🔄 Real-time updates with Turbo Streams
- ⚡ SPA-like navigation with Turbo Frames

## 🤝 Contributing

This project is designed to be AI-friendly. When adding features:

1. Follow the single-file-per-feature pattern
2. Keep templates inline with handlers
3. Use clear section comments
4. Provide both HTML and JSON endpoints
5. See [ARCHITECTURE.md](ARCHITECTURE.md) for guidelines

## 📄 License

MIT License - See LICENSE file for details

---

**Built with ❤️ for humans and AI agents alike** 🤖
