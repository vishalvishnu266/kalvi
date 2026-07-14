# Kalvi ERP - Deep Implementation Guide (Native Rust)

This guide provides the low-level technical specifications for the "v2" rewrite. Use this alongside `REWRITE_BLUEPRINT.md` for exact logic implementation.

---

## Phase 1: Foundations

### Step 1.1: Scaffolding & Dependencies
- **Cargo.toml Configuration**: 
    - `axum`: Use features `["macros", "multipart"]` if needed later, but keep it minimal.
    - `sqlx`: Features `["sqlite", "runtime-tokio-rustls"]`. **DO NOT** enable `macros` or `chrono`.
    - `tracing-subscriber`: Use `EnvFilter` for dynamic log levels.
- **Directory Structure**: Create all `mod.rs` files immediately to establish the tree.
- **Verification**: Run `cargo check`. It must pass with no errors.

### Step 1.2: Identity & Correlation
- **id_util.rs**: 
    - `generate_uuid()`: Simple wrapper around `uuid::Uuid::new_v4().to_string()`.
    - `current_timestamp()`: Use `SystemTime::now().duration_since(UNIX_EPOCH)`.
- **request_id_middleware.rs**:
    - **Logic**: 
        1. Generate ID.
        2. `req.extensions_mut().insert(RequestId(id.clone()))`.
        3. Create `tracing::info_span!("request", id = %id)`.
        4. Wrap `next.run(req)` in `.instrument(span).await`.
        5. `response.headers_mut().insert("X-Request-ID", ...)`
- **Verification**: Logs should look like: `2026-07-14 ... INFO request{id=req_123}: Server listening...`

### Step 1.3: Dual-Layer Exception System
- **AppError Logic**:
    - **RuntimeException**: Used for everything that isn't a user mistake (SQL errors, IO, etc.).
    - **BusinessException**: 
        - `HashMap<String, String>` for field-specific errors.
        - `Option<String>` for a general top-level message.
- **IntoResponse Implementation**:
    - **Technical Path**: Generate a `service_id`. `println!` the raw error to console with that ID. Return the HTML error template to the user.
    - **Business Path**: Return `StatusCode::BAD_REQUEST`. **Crucial**: The controller should catch this and re-render the specific form fragment.
- **Verification**: Visit a non-existent URL. It should trigger a 404 (handled as a RuntimeException).

---

## Phase 2: Multi-Tenant Core

### Step 2.1: Manual Migrator
- **database_config.rs**:
    - `run_manual_migrations(pool, path)` logic:
        1. `CREATE TABLE IF NOT EXISTS _manual_migrations (version TEXT PRIMARY KEY)`.
        2. Loop through files in `path`.
        3. For each file: `SELECT EXISTS...`.
        4. If missing: `pool.begin()`, read file, split by `;`, execute each statement, `INSERT INTO _manual_migrations`, `commit()`.
- **Verification**: Check `data/master.db` with an external SQLite viewer to see the `_manual_migrations` table and institutional metadata.

### Step 2.2: Dynamic Pool Registry
- **Logic**:
    - `DatabaseConfig` holds `master_pool` and `Arc<RwLock<HashMap<String, SqlitePool>>>`.
    - `get_tenant_pool(name)`: Read-lock check first. If missing, write-lock, open connection, run tenant migrations, insert, return.
- **Verification**: Register two different tenants. Verify `data/tenant/` contains two distinct `.db` files.

---

## Phase 3: Presentation Layer

### Step 3.1: Flicker-Free Theme Engine
- **Head Script**:
    ```javascript
    (function() {
        const theme = localStorage.getItem('kalvi_theme') || 'light';
        const color = localStorage.getItem('kalvi_primary') || '#3b82f6';
        if (theme === 'dark') document.documentElement.classList.add('dark');
        document.documentElement.style.setProperty('--primary-color', color);
    })();
    ```
- **Verification**: Refresh the page while in Dark Mode. The background should be dark *instantly*, without any white flash.

### Step 3.2: Turbo-Ready Form Components
- **input.rs**:
    - Params: `label`, `name`, `type`, `error: Option<String>`.
    - If `error`, add `border-red-500` and a small text div below.
- **Verification**: Manually pass `Some("Invalid password")` to a component and verify it renders red.

---

## Phase 4: Lifecycle & Identity

### Step 4.1: Institutional Registration
- **Process**:
    1. Validate inputs (no empty fields, valid slug chars).
    2. Start Master DB Transaction.
    3. Save Tenant record.
    4. Call `db.get_tenant_pool(slug)`. This triggers the schema creation.
    5. Hash password.
    6. Insert Admin user into the *Tenant* pool.
    7. Commit Master Transaction.
- **Verification**: Submit registration. Browser should redirect to `/web/slug/login`.

### Step 4.2: Auth Middleware
- **Logic**:
    1. Middleware gets `TenantContext` (from previous tenant_middleware).
    2. Get `kalvi_session` cookie.
    3. `UserRepository::find_session(context.pool, cookie_val)`.
    4. If valid, `req.extensions_mut().insert(user)`, continue.
    5. Else, `Redirect::to("/web/{slug}/login")`.
- **Verification**: Try to access `/web/my-school/dashboard`. It must force a redirect to login.

---

## Phase 5: Verification (Add Student)

### Step 5.1: The Proof-of-Architecture Module
- **Controller**:
    - Returns `Html(StudentView::render_form(...))` for GET.
    - Post: `StudentService::add(...)`. 
    - **Catch BusinessException**: Return the *form fragment only* using `<turbo-frame id="student-form">`.
- **Verification**: Submit form with missing last name. Only the form section should update; the sidebar/nav must not flicker or reload.

---

## Technical Appendix: Schemas & Logic

### Initial SQL Schemas (Unix Timestamps)

#### Master Database (`master.db`)
```sql
CREATE TABLE IF NOT EXISTS tenants (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    database_name TEXT NOT NULL UNIQUE,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS saas_owners (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    full_name TEXT,
    created_at INTEGER NOT NULL
);
```

#### Tenant Database (`tenant_{slug}.db`)
```sql
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'staff',
    full_name TEXT,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY, 
    user_id INTEGER NOT NULL,
    user_agent TEXT,
    client_ip TEXT,
    expires_at INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS students (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    enrollment_number TEXT UNIQUE,
    created_at INTEGER NOT NULL
);
```

### Session Lifecycle Logic
- **Generation**: Use `id_util::generate_uuid()`. 
- **Duration**: Default to 7 days (`now + (7 * 24 * 60 * 60)`).
- **Storage**: Sessions are stored **locally** in the tenant's isolated database. This means a user in "School A" cannot use their session cookie to access "School B".
- **Validation**: Middleware queries `SELECT user_id FROM sessions WHERE id = ? AND expires_at > ?`.
- **Cleanup**: On login, optionally delete expired sessions for that `user_id`.

