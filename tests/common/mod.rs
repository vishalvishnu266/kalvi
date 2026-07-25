//! Shared test harness.
//!
//! Boots the full app against a per-test temp `data/` directory, provisions
//! a `demo` tenant, registers the standard demo personas, and hands back an
//! `axum-test` [`TestServer`] plus helpers for logging in and reading
//! cookie-authenticated responses.
//!
//! Every test gets its own tmp dir + fresh SQLite files, so tests are fully
//! isolated and can run in parallel.

#![allow(dead_code)]

use axum_test::TestServer;
use school_erp::health_probes::Readiness;
use school_erp::{build_router, connect_system, migrate_system, AppState, Config};
use tempfile::TempDir;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

pub struct TestApp {
    pub server: TestServer,
    _tmp: TempDir,
}

pub struct Persona {
    pub username: &'static str,
    pub password: &'static str,
    pub role: &'static str,
}

pub const ADMIN: Persona = Persona {
    username: "admin",
    password: "admin123",
    role: "admin",
};
pub const PRINCIPAL: Persona = Persona {
    username: "principal",
    password: "demo1234",
    role: "principal",
};
pub const TEACHER: Persona = Persona {
    username: "teacher",
    password: "demo1234",
    role: "teacher",
};
pub const ACCOUNTANT: Persona = Persona {
    username: "accountant",
    password: "demo1234",
    role: "accountant",
};
pub const LIBRARIAN: Persona = Persona {
    username: "librarian",
    password: "demo1234",
    role: "librarian",
};
pub const PARENT: Persona = Persona {
    username: "parent",
    password: "demo1234",
    role: "guardian",
};
pub const STUDENT: Persona = Persona {
    username: "student1",
    password: "demo1234",
    role: "student",
};

pub const TENANT: &str = "demo";

pub async fn boot_with_personas() -> TestApp {
    let tmp = TempDir::new().expect("tmpdir");

    // Point Config at the temp dir via env overrides so `Config::from_env`
    // yields our isolated paths.
    std::env::set_var("DB_DIR", tmp.path().to_str().unwrap());
    let config = Config::from_env();

    std::fs::create_dir_all(&config.db_dir).ok();
    std::fs::create_dir_all(config.tenant_db_root()).ok();

    let sys_pool = connect_system(&config.system_db_url())
        .await
        .expect("system db");
    migrate_system(&sys_pool).await.expect("system migrations");

    let sessions = school_erp::session::SessionStore::open(&config.session_db_url())
        .await
        .expect("session store");

    let state = AppState::new(sys_pool, sessions, config);
    let readiness = Readiness::new_ready();

    let router = build_router(state, readiness);
    let router = NormalizePathLayer::trim_trailing_slash().layer(router);

    let server = TestServer::builder()
        .save_cookies()
        .build(router.into_make_service())
        .expect("test server");

    create_tenant(&server).await;
    for p in [
        ADMIN, PRINCIPAL, TEACHER, ACCOUNTANT, LIBRARIAN, PARENT, STUDENT,
    ] {
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
    assert!(
        status == 200 || status == 201 || status == 409,
        "create tenant unexpected {status}: {}",
        resp.text()
    );
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
    assert!(
        status == 200 || status == 201 || status == 409,
        "register {} unexpected {status}: {}",
        p.username,
        resp.text()
    );
}

pub async fn login_as(app: &TestApp, p: &Persona) {
    let resp = app
        .server
        .post("/web/login")
        .form(&[
            ("tenant", TENANT),
            ("identifier", p.username),
            ("password", p.password),
        ])
        .await;
    let status = resp.status_code().as_u16();
    assert!(
        (200..=399).contains(&status),
        "login {} failed with {status}: {}",
        p.username,
        resp.text()
    );
}

pub async fn logout(app: &TestApp) {
    let _ = app.server.post("/web/logout").await;
}

pub async fn get(app: &TestApp, url: &str) -> (u16, String) {
    let resp = app.server.get(url).await;
    (resp.status_code().as_u16(), resp.text())
}
