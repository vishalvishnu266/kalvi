# Kalvi ERP - Coding Standards & Conventions

This document outlines the architectural patterns and conventions used in this project to ensure a "token-efficient," clean, and maintainable codebase.

## 1. Backend Standards

### Repository Pattern (Lifetime Management)
To avoid verbose lifetime noise, always use the `impl Executor` pattern for repository methods.
- **Convention**: Use `impl Executor<'_, Database = Sqlite>` instead of generic `<'a, E>`.
- **Benefit**: Keeps method signatures clean while allowing them to work with both `Pool` and `Transaction`.

```rust
// GOOD
pub async fn find_by_slug(executor: impl Executor<'_, Database = Sqlite>, slug: &str) -> Result<Option<Tenant>, sqlx::Error> { ... }

// AVOID
pub async fn find_by_slug<'a, E>(executor: E, slug: &str) -> Result<Option<Tenant>, sqlx::Error> where E: Executor<'a, Database = Sqlite> { ... }
```

### Controller & View Rendering
We use a streamlined approach to handle HTML responses in Axum.
- **Convention**: Use the `IntoHtml` trait. Call `.into_html()` on rendered view strings.
- **Benefit**: Removes `Ok(Html(View::render(...)))` boilerplate.

```rust
pub async fn show_dashboard(...) -> Result<Html<String>, AppError> {
    Ok(DashboardView::render_dashboard(&ctx.tenant, &user).into_html())
}
```

### Middleware & Context Retrieval
Shared data like `TenantContext` is managed centrally.
- **Convention**: Use `TenantContext::from_req(&req)?` to retrieve tenant data in middleware or controllers.
- **Convention**: Use `ctx.login_url()` and `ctx.dashboard_url()` helpers instead of hardcoding paths.

### Session Management
Use `SessionUtil` helpers for all auth-related cookie and redirect logic.
- `finalize_login(session_id, slug)`: Sets cookies and redirects to dashboard.
- `finalize_logout(redirect_url)`: Clears cookies and redirects.
- `is_saas_session(headers)`: Quick check for control-plane sessions.

---

## 2. UI & Frontend Structure

### Pure Client-Side Personalization
We do **not** store theme (Dark Mode) or accent colors on the server.
- **Pattern**: `localStorage` + JavaScript IIFE in `layout_view.rs`.
- **Reason**: Eliminates server-side state, database columns, and network latency for UI preferences.
- **Implementation**: The layout script checks `localStorage` immediately upon load to prevent flicker.

### View & Component Architecture
Views are structured as static methods within structs, using a centralized `components.rs` for shared HTML.
- **Structure**: `src/view/layout_view.rs` is the base template.
- **Context**: Use `LayoutContext::for_tenant(tenant, "Title")` to build the UI context automatically.
- **Components**: Use `components::alert_error`, `components::card`, and `components::input` to maintain a consistent UI language.

### CSS Strategy
- **Tailwind CSS**: Used exclusively for styling.
- **Dynamic Themes**: Accent colors are handled via CSS Variables (`--primary-color`) updated by JavaScript, allowing real-time color changes without page reloads.

---

## 3. Error Handling
- **Pattern**: Use `AppError` and the `?` operator.
- **Convention**: We have implemented `From<sqlx::Error>` for `AppError`, so repository errors can be bubbled up seamlessly in service layers.
