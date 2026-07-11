# Authentication

## Users

Users live in the tenant DB (`users` table). Each user has:

- `username` (unique)
- `email` (optional)
- `password_hash` (bcrypt, cost 12 via `bcrypt::DEFAULT_COST`)
- `role` (currently just `admin` by default; extend as needed)
- `is_active` (booleanish integer)

## Login

- `GET /t/{slug}/login` renders the login view.
- `POST /t/{slug}/login` reads `username` + `password`, verifies against the tenant `users` table, creates a session and sets the cookie, then redirects to `/t/{slug}/dashboard`.

## Logout

`POST /t/{slug}/logout` deletes the session row and clears the cookie. The dashboard's Logout button is a form so Turbo can handle it as a Turbo Drive POST.

## Protecting a route

Add the `AuthenticationMiddleware` or use the `Session` extension. For simple protection, we use `AuthenticationMiddleware::require_auth_middleware`.

## Seeding an admin outside onboarding

Call `AuthService::create_admin_user(&tenant_pool, "username", "password")`. It uses `INSERT OR IGNORE`, so calling it twice with the same username is a no-op.
