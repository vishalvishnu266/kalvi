//! Shared test harness.
//!
//! Boots the full app against a per-test temp `data/` directory, provisions
//! a `demo` tenant, registers the standard demo personas, and hands back an
//! `axum-test` [`TestServer`] plus helpers for logging in and reading
//! cookie-authenticated responses.
//!
//! Every test gets its own tmp dir + fresh SQLite files, so tests are fully
//! isolated and can run in parallel.

#![allow(dead_code)] // helpers here are used selectively by each test file

use axum_test::TestServer;
use school_erp::health_probes::Readiness;
use school_erp::system::{connect_system, migrate_system};
use school_erp::tenancy::{new_tenant_registry, TenantAdmissionMode};
use school_erp::{build_router, AppState, SystemRegistry};
use tempfile::TempDir;
use tower_http::normalize_path::NormalizePathLayer;
use tower::Layer;

/// One test app + the temp dir backing its DBs. Drop this and the DBs vanish.
pub struct TestApp {
    /// Cookie-aware `axum-test` server for the built router.
    pub server: TestServer,
    /// Kept alive so the temp dir is not cleaned up before the pools close.
    _tmp: TempDir,
}

/// Standard demo persona used by the seeded-user helpers.
pub struct Persona {
    pub username: &'static str,
    pub password: &'static str,
    pub role:     &'static str,
}

pub const ADMIN:      Persona = Persona { username: "admin",      password: "admin123", role: "admin"      };
pub const PRINCIPAL:  Persona = Persona { username: "principal",  password: "demo1234", role: "principal"  };
pub const TEACHER:    Persona = Persona { username: "teacher",    password: "demo1234", role: "teacher"    };
pub const ACCOUNTANT: Persona = Persona { username: "accountant", password: "demo1234", role: "accountant" };
pub const LIBRARIAN:  Persona = Persona { username: "librarian",  password: "demo1234", role: "librarian"  };
pub const PARENT:     Persona = Persona { username: "parent",     password: "demo1234", role: "guardian"   };
pub const STUDENT:    Persona = Persona { username: "student1",   password: "demo1234", role: "student"    };

pub const TENANT: &str = "demo";

/// Boot the full app against a temp `data/` dir, provision the `demo`
/// tenant, and register the seven standard personas. The server is ready
/// for HTTP calls when this returns.
pub async fn boot_with_personas() -> TestApp {
    let tmp = TempDir::new().expect("tmpdir");
    let sys_url = format!(
        "sqlite://{}/system.db?mode=rwc",
        tmp.path().display()
    );
    let tenant_root = tmp.path().join("tenants");
    std::fs::create_dir_all(&tenant_root).unwrap();

    let sys_pool = connect_system(&sys_url).await.expect("system db");
    migrate_system(&sys_pool).await.expect("system migrations");
    let system = SystemRegistry::new(sys_pool);

    let tenants = new_tenant_registry(
        tenant_root.clone(),
        TenantAdmissionMode::SystemDb(system.clone()),
        true,
        None,
    );
    let state = AppState { system, tenants };
    let readiness = Readiness::new_ready();

    let router = build_router(state, readiness);
    let router = NormalizePathLayer::trim_trailing_slash().layer(router);

    let server = TestServer::builder()
        .save_cookies() // sticky session cookies across requests
        .build(router.into_make_service())
        .expect("test server");

    // Provision tenant + personas via the admin & auth APIs. Doing this
    // through the real HTTP surface is deliberate: it verifies the same
    // paths our real `dev_reset.sh` script exercises.
    create_tenant(&server).await;
    for p in [ADMIN, PRINCIPAL, TEACHER, ACCOUNTANT, LIBRARIAN, PARENT, STUDENT] {
        register(&server, &p).await;
    }

    TestApp { server, _tmp: tmp }
}

async fn create_tenant(server: &TestServer) {
    let resp = server
        .post("/admin/api/tenants")
        .json(&serde_json::json!({
            "tenant_id": TENANT,
            "name":      "Test School",
            "plan":      "demo",
        }))
        .await;
    let status = resp.status_code().as_u16();
    assert!(status == 200 || status == 201 || status == 409,
        "create tenant unexpected {status}: {}", resp.text());
}

async fn register(server: &TestServer, p: &Persona) {
    let resp = server
        .post(&format!("/api/{TENANT}/auth/register"))
        .json(&serde_json::json!({
            "username": p.username,
            "email":    format!("{}@{}.example", p.username, TENANT),
            "password": p.password,
            "roles":    [p.role],
        }))
        .await;
    let status = resp.status_code().as_u16();
    assert!(status == 200 || status == 201 || status == 409,
        "register {} unexpected {status}: {}", p.username, resp.text());
}

/// Log the given persona in via the web login form. Session cookie is
/// stored in the server's cookie jar and applied automatically to
/// subsequent requests on the same [`TestApp`].
pub async fn login_as(app: &TestApp, p: &Persona) {
    // The web login form posts as `application/x-www-form-urlencoded`.
    let resp = app
        .server
        .post("/web/login")
        .form(&[
            ("tenant",     TENANT),
            ("identifier", p.username),
            ("password",   p.password),
        ])
        .await;
    // Successful login redirects — accept anything in the 2xx/3xx range.
    let status = resp.status_code().as_u16();
    assert!(
        (200..=399).contains(&status),
        "login {} failed with {status}: {}", p.username, resp.text()
    );
}

/// Log the current session out (best-effort — 200/204/303 all fine).
pub async fn logout(app: &TestApp) {
    let _ = app.server.post("/web/logout").await;
}

/// Fetch a URL and return `(status, body)` for the *current* session.
pub async fn get(app: &TestApp, url: &str) -> (u16, String) {
    let resp = app.server.get(url).await;
    (resp.status_code().as_u16(), resp.text())
}
