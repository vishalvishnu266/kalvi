# Multi-Tenancy Architecture Decision for Educational ERP

## 🎯 Your Situation

- **Use Case**: Educational ERP system for schools/institutions
- **Tech Stack**: Rust + Axum + SQLite
- **Developer**: Solo developer
- **AI-First**: Code should be easy for AI agents to work with

## 🤔 The Question

Should you support:
1. **One DB per school** (simple, current approach)
2. **One DB per tenant, supporting multiple schools** (complex, more flexible)

---

## 📊 Scenario Analysis

### Scenario A: One DB per School (Recommended ✅)

**Model:**
```
Tenant "acme-edu" → school1.db
Tenant "acme-edu-campus2" → school2.db
Tenant "rainbow-school" → rainbow.db
```

**Structure:**
```sql
-- Each database has same schema
CREATE TABLE students (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    grade TEXT,
    -- No school_id needed!
);

CREATE TABLE teachers (...);
CREATE TABLE classes (...);
```

**Code:**
```rust
// X-Tenant-ID: school1
// Automatically routes to school1.db

async fn get_student(Extension(pool): Extension<SqlitePool>, Path(id): Path<i64>) {
    // This pool is already scoped to the right school!
    let student = sqlx::query_as!(Student, "SELECT * FROM students WHERE id = ?", id)
        .fetch_one(&pool).await?;
}
```

### Scenario B: Multiple Schools per Tenant (Complex ❌)

**Model:**
```
Tenant "acme-edu" → acme.db
  ├─ school_id: 1 (Main Campus)
  ├─ school_id: 2 (Branch Campus)
  └─ school_id: 3 (Online Campus)
```

**Structure:**
```sql
-- Single database with school_id everywhere
CREATE TABLE students (
    id INTEGER PRIMARY KEY,
    school_id INTEGER NOT NULL,  -- Required in EVERY table!
    name TEXT NOT NULL,
    grade TEXT,
    FOREIGN KEY (school_id) REFERENCES schools(id)
);

CREATE TABLE teachers (
    id INTEGER PRIMARY KEY,
    school_id INTEGER NOT NULL,  -- Again!
    -- ...
);
```

**Code:**
```rust
// Much more complex!
async fn get_student(
    Extension(pool): Extension<SqlitePool>,
    Extension(school_context): Extension<SchoolContext>,  // Need this!
    Path(id): Path<i64>
) {
    // Must filter by school_id in EVERY query
    let student = sqlx::query_as!(
        Student, 
        "SELECT * FROM students WHERE id = ? AND school_id = ?", 
        id, 
        school_context.school_id  // Easy to forget!
    ).fetch_one(&pool).await?;
}

// EVERY route needs school context
Router::new()
    .route("/students/{id}", get(handler))
    .layer(school_context_middleware)  // Extra layer
```

---

## 🎯 Decision Matrix

| Factor | One DB per School ✅ | Multi-School per Tenant ❌ |
|--------|---------------------|---------------------------|
| **Simplicity** | ⭐⭐⭐⭐⭐ Very simple | ⭐⭐ Complex |
| **AI Coding** | ⭐⭐⭐⭐⭐ Easy | ⭐⭐ Harder (more context) |
| **Data Isolation** | ⭐⭐⭐⭐⭐ Perfect isolation | ⭐⭐⭐ Good (if coded correctly) |
| **Security** | ⭐⭐⭐⭐⭐ Can't access wrong school | ⭐⭐⭐ Risk of query bugs |
| **Query Speed** | ⭐⭐⭐⭐⭐ No filtering needed | ⭐⭐⭐ Must filter everything |
| **Schema Changes** | ⭐⭐⭐⭐ Migrate all DBs | ⭐⭐⭐⭐⭐ One migration |
| **Backup/Restore** | ⭐⭐⭐⭐⭐ Per-school granular | ⭐⭐⭐ All or nothing |
| **Development Speed** | ⭐⭐⭐⭐⭐ Fast | ⭐⭐ Slower (more boilerplate) |

---

## 💡 Recommendation: **One DB per School** ✅

### Why This is Best for You:

#### 1. **Solo Developer Reality**
You'll spend 80% less time on:
- ❌ Debugging "why did student from school A appear in school B?"
- ❌ Adding `school_id` to every single table
- ❌ Remembering to filter by `school_id` in every query
- ❌ Complex RBAC across schools

#### 2. **AI Agent Friendliness** 🤖
```rust
// AI can write this confidently - pool is already scoped!
async fn get_student(Extension(pool): Extension<SqlitePool>, Path(id): Path<i64>) {
    sqlx::query_as!(Student, "SELECT * FROM students WHERE id = ?", id)
        .fetch_one(&pool).await
}

// vs AI must remember context everywhere
async fn get_student(
    Extension(pool): Extension<SqlitePool>,
    Extension(school): Extension<SchoolContext>,  // Extra complexity
    Path(id): Path<i64>
) {
    sqlx::query_as!(
        Student, 
        "SELECT * FROM students WHERE id = ? AND school_id = ?",  // Easy to forget
        id, school.id
    )
    .fetch_one(&pool).await
}
```

**AI agents work better** when:
- ✅ Less context to track
- ✅ Fewer "magic" dependencies
- ✅ Simpler error modes

#### 3. **Perfect Data Isolation** 🔒
```
school1.db ← Physically separate
school2.db ← No way to mix data
```
- No risk of bugs leaking data between schools
- Easier compliance (FERPA, GDPR)
- Each school can be backed up/restored independently

#### 4. **Real-World Flexibility**
```
X-Tenant-ID: springfield-high     → springfield-high.db
X-Tenant-ID: springfield-middle   → springfield-middle.db
```

**If owner has 3 schools:**
- They get 3 tenant IDs
- You give them a dashboard aggregating all 3
- Still simple, still isolated

---

## 🚫 "But What About Aggregate Queries?"

### Option 1: Application-Level Aggregation (Recommended)

```rust
// Dashboard for owner with multiple schools
async fn owner_dashboard(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Html<String> {
    let school_ids = get_user_schools(&user).await; // ["school1", "school2", "school3"]
    
    let mut total_students = 0;
    let mut total_revenue = 0.0;
    
    for school_id in school_ids {
        let pool = state.db_manager.get_pool(Some(school_id)).await?;
        
        // Query each DB
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM students")
            .fetch_one(&pool).await?;
        total_students += count;
        
        let revenue: f64 = sqlx::query_scalar("SELECT SUM(amount) FROM payments")
            .fetch_one(&pool).await?;
        total_revenue += revenue;
    }
    
    Html(render_dashboard(total_students, total_revenue).into_string())
}
```

**Pros:**
- ✅ Simple to understand
- ✅ Works with existing architecture
- ✅ Each school still isolated
- ✅ Easy for AI to generate

**Cons:**
- ⚠️ Slower for many schools (but fine for 2-10 schools)
- ⚠️ Not suitable for complex joins across schools

### Option 2: SQLite ATTACH (Advanced)

```rust
async fn aggregate_report() -> Result<Report, Error> {
    let pool = state.db_manager.get_pool(Some("school1")).await?;
    
    sqlx::query(r#"
        ATTACH DATABASE 'school2.db' AS school2;
        ATTACH DATABASE 'school3.db' AS school3;
        
        SELECT COUNT(*) FROM (
            SELECT * FROM students
            UNION ALL
            SELECT * FROM school2.students
            UNION ALL
            SELECT * FROM school3.students
        )
    "#).fetch_one(&pool).await?;
}
```

**Pros:**
- ✅ SQLite native
- ✅ Can do complex joins

**Cons:**
- ⚠️ More complex
- ⚠️ Harder for AI to generate correctly

### Option 3: Read Replica / Analytics DB (Overkill)

Skip this - too complex for solo dev + AI workflow.

---

## 🎯 Your Architecture Should Be:

```
Educational ERP
├── Tenant Model: One DB per School
├── Tenant ID: School identifier (e.g., "springfield-high")
├── Multi-School Owners: Get multiple tenant IDs
└── Aggregation: Application-level (query each DB, sum in code)
```

### Implementation:

**Current middleware (already perfect!):**
```rust
// crates/shared/src/middleware.rs
pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let tenant_id = req.headers().get("X-Tenant-ID")
        .and_then(|h| h.to_str().ok());
    
    match state.db_manager.get_pool(tenant_id).await {
        Ok(pool) => {
            req.extensions_mut().insert(pool);  // Pool is scoped to ONE school!
            next.run(req).await
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
```

**Owner with multiple schools:**
```rust
// Add a simple owner context
#[derive(Clone)]
pub struct OwnerContext {
    pub owner_id: String,
    pub school_ids: Vec<String>,  // ["school1", "school2", "school3"]
}

// Separate middleware for owner dashboards
pub async fn owner_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let owner_id = req.headers().get("X-Owner-ID")
        .and_then(|h| h.to_str().ok());
    
    if let Some(owner_id) = owner_id {
        let school_ids = get_owner_schools(owner_id).await;
        req.extensions_mut().insert(OwnerContext { 
            owner_id: owner_id.to_string(), 
            school_ids 
        });
    }
    
    next.run(req).await
}
```

---

## 🏗️ RBAC (Role-Based Access Control)

### Simple Approach (Recommended for Solo Dev + AI)

**One table per school DB:**
```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    role TEXT NOT NULL  -- 'admin', 'teacher', 'student', 'parent'
);

CREATE TABLE permissions (
    user_id INTEGER,
    resource TEXT,  -- 'students', 'grades', 'attendance'
    action TEXT,    -- 'read', 'write', 'delete'
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

**Simple middleware:**
```rust
async fn require_role(role: &str) -> Result<(), StatusCode> {
    if user.role == role {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

// Usage
async fn view_grades(
    Extension(pool): Extension<SqlitePool>,
    Extension(user): Extension<User>,
) -> Result<Html<String>, StatusCode> {
    require_role("teacher").await?;  // Only teachers can view
    // ... rest of handler
}
```

**AI-Friendly Pattern:**
```rust
// Each handler declares what it needs
#[derive(Debug)]
struct RequireRole(&'static str);

async fn check_permission(
    Extension(user): Extension<User>,
    required: RequireRole,
) -> Result<(), StatusCode> {
    if user.role == required.0 {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

// Routes
Router::new()
    .route("/grades", get(view_grades))
    .layer(axum::middleware::from_fn(check_permission(RequireRole("teacher"))))
```

### Advanced RBAC (Only if Needed Later)

Use a library like `casbin` or `oso`, but **start simple**.

---

## 📋 Implementation Checklist

### ✅ Phase 1: MVP (Start Here)

- [x] One SQLite DB per school
- [x] Tenant middleware (you already have this!)
- [ ] Simple user table with role column
- [ ] Basic role checks in handlers
- [ ] Login/logout with JWT

### ✅ Phase 2: Multi-School Support

- [ ] Owner table linking owners to multiple schools
- [ ] Owner dashboard endpoint
- [ ] Application-level aggregation (loop through schools)
- [ ] Owner middleware for aggregate queries

### ✅ Phase 3: Advanced (Optional)

- [ ] SQLite ATTACH for complex cross-school queries
- [ ] Advanced RBAC with permissions table
- [ ] Audit logging per school

---

## 🎓 Example: Student Management

### Simple (One DB per School) ✅

```rust
// crates/student/src/view_student.rs

async fn get_student(
    Extension(pool): Extension<SqlitePool>,  // Already scoped to school!
    Path(id): Path<i64>,
) -> Result<Json<Student>, StatusCode> {
    let student = sqlx::query_as!(
        Student,
        "SELECT * FROM students WHERE id = ?",  // No school_id needed!
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(student))
}
```

### Complex (Multi-School per Tenant) ❌

```rust
// Much harder for AI and you!

async fn get_student(
    Extension(pool): Extension<SqlitePool>,
    Extension(school_ctx): Extension<SchoolContext>,  // Extra dependency
    Extension(user): Extension<User>,  // For RBAC
    Path(id): Path<i64>,
) -> Result<Json<Student>, StatusCode> {
    // Check if user has access to this school
    if !user.can_access_school(school_ctx.school_id) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Must remember to filter by school_id!
    let student = sqlx::query_as!(
        Student,
        "SELECT * FROM students WHERE id = ? AND school_id = ?",  // Easy to forget!
        id,
        school_ctx.school_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(student))
}
```

**More code to write, more bugs to fix, harder for AI.**

---

## 🎯 Final Recommendation

### ✅ DO THIS: One DB per School

**Reasons:**
1. ✅ **Simpler code** - Less mental overhead
2. ✅ **AI-friendly** - Clear, simple patterns
3. ✅ **Secure by default** - Physical data isolation
4. ✅ **Fast queries** - No `school_id` filtering
5. ✅ **Easier debugging** - Isolated databases
6. ✅ **Flexible** - Can still support multi-school owners

**Implementation:**
- Each school = one tenant ID = one SQLite DB
- Owner with 3 schools = 3 tenant IDs
- Aggregate queries = loop through DBs in code (simple!)

### ❌ DON'T DO: Multi-School per Tenant

**Unless:**
- You have complex cross-school queries (unlikely for schools)
- You need real-time aggregation across 100+ schools (overkill for now)
- You have a team of developers (you're solo)

**You can always migrate later if needed, but you won't need to.**

---

## 🚀 Next Steps

1. **Stick with current architecture** (one DB per tenant)
2. **Build core features** (students, teachers, grades, attendance)
3. **Add simple RBAC** (user role column)
4. **If needed later**, add owner dashboard with app-level aggregation
5. **Profit** 💰

---

**Bottom Line:** One DB per school is the right choice for solo dev + AI workflow. It's simpler, safer, and easier to maintain. You can handle multi-school owners with application-level aggregation when needed.
