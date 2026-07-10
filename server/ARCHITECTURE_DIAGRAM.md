# Architecture Diagram - Visual Guide

## 🏗️ Complete System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         HTTP Request                                │
│                              ↓                                       │
│                    Server (localhost:3000)                           │
│                   crates/server/src/main.rs                         │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│                     Tenant Middleware                                │
│              shared/src/middleware.rs                               │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │ 1. Extract X-Tenant-ID header (optional)                    │    │
│  │ 2. Get database pool for tenant                             │    │
│  │ 3. Inject pool into request extensions                      │    │
│  └────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│                        Router                                        │
│                                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │   Student    │  │   Finance    │  │      HR      │             │
│  │    Routes    │  │    Routes    │  │    Routes    │             │
│  │              │  │              │  │              │             │
│  │ /student/:id │  │  /invoices   │  │  /employees  │             │
│  │ /students    │  │  /payments   │  │   /payroll   │             │
│  └──────────────┘  └──────────────┘  └──────────────┘             │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│                    Domain Crate (e.g., student/)                    │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │  Feature File: view_student.rs                              │    │
│  │                                                              │    │
│  │  ┌─────────────────────────────────────────────────────┐   │    │
│  │  │ 1. HTTP Handler                                       │   │    │
│  │  │    - Extracts path params (id)                        │   │    │
│  │  │    - Gets database pool from extensions               │   │    │
│  │  └─────────────────────────────────────────────────────┘   │    │
│  │                        ↓                                     │    │
│  │  ┌─────────────────────────────────────────────────────┐   │    │
│  │  │ 2. Business Logic                                     │   │    │
│  │  │    - Validates input                                  │   │    │
│  │  │    - Applies business rules                           │   │    │
│  │  └─────────────────────────────────────────────────────┘   │    │
│  │                        ↓                                     │    │
│  │  ┌─────────────────────────────────────────────────────┐   │    │
│  │  │ 3. Database Layer                                     │   │    │
│  │  │    - SQL queries (sqlx)                               │   │    │
│  │  │    - Fetch/insert/update data                         │   │    │
│  │  └─────────────────────────────────────────────────────┘   │    │
│  │                        ↓                                     │    │
│  │  ┌─────────────────────────────────────────────────────┐   │    │
│  │  │ 4. Template Rendering (Maud)                          │   │    │
│  │  │    - Bootstrap HTML                                   │   │    │
│  │  │    - Optional: Turbo Frames/Streams                   │   │    │
│  │  └─────────────────────────────────────────────────────┘   │    │
│  │                        ↓                                     │    │
│  │  ┌─────────────────────────────────────────────────────┐   │    │
│  │  │ 5. HTTP Response                                      │   │    │
│  │  │    - HTML (for browsers)                              │   │    │
│  │  │    - JSON (for APIs)                                  │   │    │
│  │  │    - Turbo Stream (for real-time updates)             │   │    │
│  │  └─────────────────────────────────────────────────────┘   │    │
│  │                                                              │    │
│  └────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────┘
```

## 📁 File Structure with Flow

```
project/
│
├── crates/
│   │
│   ├── shared/                          # ← Cross-domain utilities
│   │   ├── src/
│   │   │   ├── db.rs                    # ← Database pools & migrations
│   │   │   ├── error.rs                 # ← Error types
│   │   │   ├── middleware.rs            # ← ✨ Tenant middleware
│   │   │   └── layout.rs                # ← ✨ Bootstrap templates
│   │   └── Cargo.toml
│   │
│   ├── student/                         # ← Student domain
│   │   ├── src/
│   │   │   ├── lib.rs                   # ← Route aggregator
│   │   │   ├── shared.rs                # ← Student model
│   │   │   │
│   │   │   └── view_student.rs          # ← COMPLETE FEATURE
│   │   │       │
│   │   │       ├── Database queries     # ← fetch_student()
│   │   │       ├── Business logic       # ← get_student()
│   │   │       ├── Templates (Maud)     # ← render_student_profile()
│   │   │       ├── HTTP handlers        # ← view_student_handler()
│   │   │       └── Routes               # ← pub fn routes()
│   │   │
│   │   └── Cargo.toml
│   │
│   ├── finance/                         # ← Future: Finance domain
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── shared.rs                # ← Invoice model
│   │       └── create_invoice.rs        # ← Complete feature
│   │
│   └── server/                          # ← Application entry
│       ├── src/
│       │   └── main.rs                  # ← Server setup
│       └── Cargo.toml
│
└── Documentation/
    ├── README.md                        # ← Start here
    ├── QUICK_START.md                   # ← Quick reference
    ├── ARCHITECTURE.md                  # ← Design philosophy
    ├── HOTWIRE_GUIDE.md                 # ← ✨ Hotwire integration
    ├── IMPROVEMENTS_SUMMARY.md          # ← ✨ Latest changes
    └── ...
```

## 🔄 Request Flow Example

### Example: User visits `/student/1`

```
1. HTTP Request
   GET /student/1
   Header: X-Tenant-ID: acme-corp
        ↓

2. Server Entry Point
   server/src/main.rs
   - Receives request
        ↓

3. Tenant Middleware
   shared/src/middleware.rs
   - Extracts tenant: "acme-corp"
   - Gets database pool: acme-corp.db
   - Injects pool into request
        ↓

4. Router
   - Matches route: /student/:id
   - Routes to student::routes()
        ↓

5. Student Domain
   student/src/lib.rs
   - Routes to view_student::routes()
        ↓

6. Feature Handler
   student/src/view_student.rs
   
   a) Handler extracts:
      - Path param: id = 1
      - Extension: SqlitePool
   
   b) Calls business logic:
      get_student(&pool, 1)
   
   c) Business logic calls database:
      fetch_student(&pool, 1)
      → SELECT * FROM students WHERE id = 1
   
   d) Database returns:
      Student { id: 1, name: "Vishal Doe" }
   
   e) Handler calls template:
      render_student_profile(&student)
   
   f) Template renders:
      HTML with Bootstrap + student data
        ↓

7. HTTP Response
   200 OK
   Content-Type: text/html
   
   <!DOCTYPE html>
   <html>
     <head>
       <link href="bootstrap.min.css">
     </head>
     <body>
       <div class="container">
         <div class="card">
           <h2>Student Profile</h2>
           <p>Name: Vishal Doe</p>
         </div>
       </div>
     </body>
   </html>
```

## 🎨 Hotwire Request Flow

### Example: User clicks "Edit" with Hotwire enabled

```
1. User clicks link
   <a href="/student/1/edit" data-turbo-frame="student-profile">
        ↓

2. Turbo intercepts
   - Adds header: Turbo-Frame: student-profile
   - Makes AJAX request
        ↓

3. Server processes
   GET /student/1/edit
   Header: Turbo-Frame: student-profile
        ↓

4. Handler detects Turbo
   if headers.get("Turbo-Frame").is_some() {
       return Html(render_edit_form_partial(&student))
   }
        ↓

5. Returns partial HTML
   <turbo-frame id="student-profile">
     <form method="post" action="/student/1">
       <input name="name" value="Vishal Doe">
       <button>Save</button>
     </form>
   </turbo-frame>
        ↓

6. Turbo updates page
   - Replaces only the turbo-frame
   - No full page reload!
   - Instant, SPA-like experience
```

## 🗂️ Data Flow in Feature File

```
view_student.rs (Single File)

┌─────────────────────────────────────────────────┐
│  HTTP Handler Layer                              │
│  ┌──────────────────────────────────────────┐  │
│  │ view_student_html_handler()               │  │
│  │ - Extracts: Path(id), Extension(pool)     │  │
│  │ - Calls: get_student(&pool, id)           │  │
│  │ - Returns: Html<String>                   │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│  Business Logic Layer                            │
│  ┌──────────────────────────────────────────┐  │
│  │ get_student(pool, id)                     │  │
│  │ - Validates input                         │  │
│  │ - Applies business rules                  │  │
│  │ - Calls: fetch_student(pool, id)          │  │
│  │ - Returns: Result<Option<Student>>        │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│  Database Layer                                  │
│  ┌──────────────────────────────────────────┐  │
│  │ fetch_student(pool, id)                   │  │
│  │ - SQL query via sqlx                      │  │
│  │ - Returns: Result<Option<Student>>        │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│  Template Layer (Maud)                           │
│  ┌──────────────────────────────────────────┐  │
│  │ render_student_profile(student)           │  │
│  │ - Bootstrap HTML                          │  │
│  │ - Returns: Markup                         │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
                    ↓
                HTML Response
```

## 🔌 Shared Components Usage

```
┌──────────────────────────────────────────────────────────┐
│              Shared Crate (Infrastructure)                │
│                                                           │
│  ┌────────────────┐  ┌────────────────┐  ┌───────────┐ │
│  │   Database     │  │   Middleware   │  │  Layouts  │ │
│  │   (db.rs)      │  │(middleware.rs) │  │(layout.rs)│ │
│  │                │  │                │  │           │ │
│  │ • Pool manager │  │ • AppState     │  │• render_  │ │
│  │ • Migrations   │  │ • Tenant ID    │  │  head()   │ │
│  │ • Multi-tenant │  │   extraction   │  │• render_  │ │
│  │                │  │ • Pool inject  │  │  layout() │ │
│  └────────────────┘  └────────────────┘  └───────────┘ │
└──────────────────────────────────────────────────────────┘
            ↓                  ↓                  ↓
┌──────────────────────────────────────────────────────────┐
│                    Domain Crates                          │
│                                                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │   Student   │  │   Finance   │  │      HR     │     │
│  │             │  │             │  │             │     │
│  │ Uses all    │  │ Uses all    │  │ Uses all    │     │
│  │ shared      │  │ shared      │  │ shared      │     │
│  │ components  │  │ components  │  │ components  │     │
│  └─────────────┘  └─────────────┘  └─────────────┘     │
└──────────────────────────────────────────────────────────┘
```

## 🎯 Benefits Visualization

```
Traditional Layered Architecture          This Architecture
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━          ━━━━━━━━━━━━━━━━━━━━━

┌─────────────────────────┐              ┌─────────────────────┐
│   Models (All domains)  │              │  Student Feature    │
│  • Student              │              │  ┌───────────────┐  │
│  • Invoice              │              │  │ Models        │  │
│  • Employee             │              │  │ Database      │  │
└─────────────────────────┘              │  │ Logic         │  │
         ↓                                │  │ Templates     │  │
┌─────────────────────────┐              │  │ Handlers      │  │
│ Repositories (All)      │              │  └───────────────┘  │
│  • StudentRepo          │              └─────────────────────┘
│  • InvoiceRepo          │                       ↓
│  • EmployeeRepo         │              ┌─────────────────────┐
└─────────────────────────┘              │  Finance Feature    │
         ↓                                │  ┌───────────────┐  │
┌─────────────────────────┐              │  │ Models        │  │
│   Services (All)        │              │  │ Database      │  │
│  • StudentService       │              │  │ Logic         │  │
│  • InvoiceService       │              │  │ Templates     │  │
│  • EmployeeService      │              │  │ Handlers      │  │
└─────────────────────────┘              │  └───────────────┘  │
         ↓                                └─────────────────────┘
┌─────────────────────────┐
│  Controllers (All)      │              Clear boundaries
│  • StudentController    │              Self-contained
│  • InvoiceController    │              AI-friendly
│  • EmployeeController   │              Easy to navigate
└─────────────────────────┘

6 files per feature                      1 file per feature
Scattered context                        Complete context
Hard to find code                        Obvious location
Complex dependencies                     Clear dependencies
```

## 📊 Scaling Visualization

```
As your ERP grows to 50+ features...

Traditional Approach:                    This Approach:
━━━━━━━━━━━━━━━━━━━━━                    ━━━━━━━━━━━━━━━━━━

model/                                   student/
├── student.rs                           ├── view.rs
├── employee.rs                          ├── create.rs
├── invoice.rs                           ├── edit.rs
├── payment.rs                           ├── list.rs
├── product.rs                           └── attendance.rs
├── ... (50+ files!)                     
                                         finance/
service/                                 ├── invoices.rs
├── student_service.rs                   ├── payments.rs
├── employee_service.rs                  └── reports.rs
├── invoice_service.rs                   
├── ... (50+ files!)                     hr/
                                         ├── employees.rs
controller/                              ├── payroll.rs
├── student_controller.rs                └── attendance.rs
├── ... (50+ files!)                     
                                         Clear domains
Massive files                            Focused features
Hard to navigate                         Easy to find
AI gets lost                             AI succeeds
```

---

## 🚀 Summary

This architecture provides:

1. ✅ **Clear separation** - Shared infrastructure vs domain features
2. ✅ **Complete context** - One feature = One file
3. ✅ **AI-friendly** - Easy to understand and modify
4. ✅ **Scalable** - Add domains without complexity
5. ✅ **Modern stack** - Bootstrap + Hotwire ready
6. ✅ **Production-ready** - Multi-tenant, responsive, accessible

**Built for the future of AI-assisted development!** 🎯
