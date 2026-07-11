# Getting Started

## Prerequisites

- Rust (stable, recent — 1.75+ for native `async fn` in traits)
- SQLite libraries are not needed separately; `sqlx` bundles the SQLite C library.

## Run

```bash
cargo run
```

- Master DB (`db/master.db`) is created and master migrations are applied on startup.
- Server listens on `0.0.0.0:3000`.

## First demo

1. Open http://localhost:3000/ — click **Onboard New Institution**.
2. Fill in the form:
   - Slug: `demo`
   - Name: `Demo School`
   - Contact email/phone/address: anything
   - Admin username: `admin`
   - Admin password: `admin123`
3. Submit. You'll land on a success page.
4. Click **Go to login** → http://localhost:3000/t/demo/login.
5. Log in with `admin` / `admin123` → you're on the dashboard.
6. Click **Logout** to end the session.

## What was created on disk

```
db/
├── master.db              # tenant registry
└── demo.db                # your first tenant DB (users/sessions/students)
```

## Adding another tenant

Just onboard again with a different slug. Each tenant is fully isolated.
