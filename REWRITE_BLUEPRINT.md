# Kalvi ERP - Rewrite Blueprint (v2)

This document serves as the master guide for rebuilding Kalvi ERP from scratch. It follows a "Native Rust" philosophy: minimal dependencies, zero macros where possible, and explicit, transparent logic.

---

## Technical Constraints & Standards
- **Dependencies**: `axum`, `tokio`, `sqlx` (core only), `serde`, `uuid`, `bcrypt`, `cookie`, `tracing`.
- **Database**: SQLite. Master: `data/master.db`. Tenants: `data/tenant/{name}.db`.
- **UI**: Tailwind CSS (CDN) + Pure JS (No build step).
- **Interactivity**: Hotwire (Turbo) for zero-reload navigation and form feedback.
- **Validation**: Manual Rust logic (no `validator` crate).
- **Identity**: UUID-based Correlation ID for every request.
- **Errors**: 
    - `BusinessException`: Inline validation/logic feedback.
    - `RuntimeException`: Logged to console with ID; generic "Oops" page for users.

---

## Step 1: Scaffolding & Identity Tracking
**Goal**: Core environment and request traceability.

1.  **Cargo.toml**: Set up minimal dependencies. Avoid `macros` and `chrono` features in SQLx.
2.  **ID Utility (`util/id_util.rs`)**: Implement `generate_uuid()` and `current_timestamp() -> i64`.
3.  **Request ID Middleware**: 
    - Generate a UUID for every incoming request.
    - Insert into a `tracing::info_span!`.
    - Attach to `Extensions` and `X-Request-ID` response header.
4.  **Verification**: Log a message in a test route; verify the console output includes the unique Request ID.

## Step 2: The Modern Exception Framework
**Goal**: Robust error handling that distinguishes system failure from user error.

1.  **AppError Enum (`util/errors.rs`)**:
    - `RuntimeException(String)`: Log full details + Correlation ID to console. Show generic page to user with "Service ID".
    - `BusinessException(HashMap<String, String>, Option<String>)`: Structured field errors for the UI.
2.  **IntoResponse**: Logic to map these to status codes and HTML/Turbo fragments.
3.  **Verification**: Trigger a DB error. Verify the console shows the technical root cause while the user sees a clean, non-technical error page.

## Step 3: Dynamic Multi-Tenant Engine
**Goal**: Automatic database isolation and manual migrations.

1.  **Manual Migrator (`config/database_config.rs`)**: 
    - Custom logic to read `.sql` files from `./resources/migrations/master` and `.../tenant`.
    - Sequential execution using a `_manual_migrations` table.
2.  **Dynamic Registry**:
    - `DatabaseConfig` struct with a cached pool registry: `Arc<RwLock<HashMap<String, SqlitePool>>>`.
    - Logic to ensure `data/tenant/` exists.
3.  **Verification**: Startup the app. Verify `data/master.db` is created and migrated automatically.

## Step 4: Multi-Tenant Middleware
**Goal**: Link URLs to isolated databases.

1.  **Middleware (`middleware/tenant_middleware.rs`)**:
    - Pattern match URL: `/web/{tenant}/...`.
    - Extract `{tenant}` (institution name).
    - Resolve the pool via `DatabaseConfig`.
    - Inject `TenantContext { tenant_metadata, pool }` into extensions.
2.  **Verification**: Access `/web/demo/health`. It should only work if "demo" exists in the master database.

## Step 5: Frontend Layout & Theme Engine
**Goal**: Responsive Tailwind UI with no-flicker personalization.

1.  **Base Layout (`view/layout_view.rs`)**: 
    - Standard HTML5 shell with Turbo and Tailwind.
    - **Header Script**: IIFE to apply Dark Mode class and `--primary-color` from `localStorage` instantly.
2.  **Shared Components (`view/components.rs`)**:
    - `input(..., error: Option<String>)`: Support for `is-invalid` styling and inline messages.
    - `card`, `button`, `alert`.
3.  **Verification**: Change colors in browser console `localStorage`. Refresh; colors must apply before the page body appears.

## Step 6: SaaS & Institutional Lifecycle
**Goal**: Self-registration and automated database provisioning.

1.  **SaaS Onboarding**: `/saas/onboard` to create the platform owner.
2.  **Institution Registration**: 
    - `/registration` form.
    - Manual Rust validation.
    - Logic: Save to Master -> Initialize `data/tenant/{name}.db` -> Run Tenant Migrations -> Create Tenant Admin.
3.  **Verification**: Register a school named "Acme". Verify `data/tenant/acme.db` appears on disk.

## Step 7: Session Management & Auth
**Goal**: Secure, institutional-specific login.

1.  **Session Utility**: Secure, HTTP-only cookie management for `kalvi_session`.
2.  **Auth Middleware**: 
    - Reads pool from `TenantContext`.
    - Verifies session UUID against the institution's private `sessions` table.
    - Redirects to `/web/{tenant}/login` on failure.
3.  **Verification**: Log into Acme. Verify you cannot access `/web/other-school/dashboard` even with a valid Acme session.

## Step 8: Verification - "Add Student" Feature
**Goal**: Prove the architecture with a real module.

1.  **Logic**: Implement Student Model, Repo, and Service in the tenant layer.
2.  **UI**: 
    - Dashboard showing "Total Students" count.
    - "Enroll Student" form inside a `<turbo-frame>`.
    - Inline validation for missing fields.
3.  **Verification**: Submit an empty student form. Verify the fields turn red with specific messages without a full page reload.
