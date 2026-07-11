# Authentication

## Users

Users live in the tenant DB (`users` table). Each user has:

- `username` (unique)
- `email` (optional)
- `password_hash` (bcrypt, cost 12 via `bcrypt::DEFAULT_COST`)
- `role` (currently just `admin` by default; extend as needed)
- `is_active` (booleanish integer)

## Login

- `GET /t/{slug}/login` renders `templates/auth/login.html`.
- `POST /t/{slug}/login` reads `username` + `password`, verifies against the tenant `users` table, creates a session and sets the cookie, then redirects to `/t/{slug}/dashboard`.

## Logout

`POST /t/{slug}/logout` deletes the session row and clears the cookie. The dashboard's Logout button is a form so Turbo can handle it as a Turbo Drive POST.

## Protecting a route

Add the `RequireAuth` extractor to a handler:

```rust
use auth::RequireAuth;
use axum::extract::Extension;
use shared::TenantContext;

async fn my_page(
    RequireAuth(user): RequireAuth,
    Extension(ctx): Extension<TenantContext>,
) -> Response {
    // user.username, user.role, ...
    // ctx.slug, ctx.database_name, ...
}
```

If the request has no valid session, the extractor returns `Redirect::to("/t/{slug}/login")`.

## Seeding an admin outside onboarding

Call `auth::create_admin_user(&tenant_pool, "username", "password")`. It uses `INSERT OR IGNORE`, so calling it twice with the same username is a no-op.
