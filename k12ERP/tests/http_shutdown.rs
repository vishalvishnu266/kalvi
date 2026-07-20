//! Integration tests for the graceful-shutdown machinery:
//!
//! * `/api/live` is always 200.
//! * `/api/ready` returns 200 while ready, 503 once we flip the flag.
//! * A request that was in-flight when shutdown starts still completes
//!   successfully.

mod http_common;

use std::sync::Arc;
use std::time::Duration;

use http::StatusCode;
use http_common::{spawn, spawn_with_readiness};
use school_erp::health_probes::Readiness;
use school_erp::shutdown::wait_for_signal;
use tokio::sync::Notify;

#[tokio::test]
async fn live_endpoint_always_returns_ok() {
    let s = spawn().await;
    let r = s.get("/api/live").await;
    r.assert_status_ok();
}

#[tokio::test]
async fn ready_endpoint_flips_on_readiness_change() {
    let (s, readiness) = spawn_with_readiness().await;

    // Initially ready
    let ok = s.get("/api/ready").await;
    ok.assert_status_ok();
    assert_eq!(ok.json::<serde_json::Value>()["status"], "ready");

    // Flip to draining
    readiness.set_ready(false);

    let draining = s.get("/api/ready").await;
    draining.assert_status(StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(draining.json::<serde_json::Value>()["status"], "draining");
}

#[tokio::test]
async fn signal_helper_completes_when_notified() {
    // We can't actually raise a signal in a portable test, but we can prove
    // the *combined* shutdown future returns promptly when a Notify fires.
    // This is the pattern main.rs uses.
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();

    let shutdown = async move {
        tokio::select! {
            _ = wait_for_signal() => {},
            _ = notify_clone.notified() => {},
        }
    };

    // Fire the shortcut after 50ms so wait_for_signal doesn't block the test.
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(50)).await;
        notify.notify_one();
    });

    tokio::time::timeout(Duration::from_secs(2), shutdown)
        .await
        .expect("shutdown future must complete");
}

#[tokio::test]
async fn in_flight_request_still_finishes_after_readiness_flip() {
    // Simulates what happens in real shutdown: the LB observes /api/ready == 503
    // and stops sending new traffic, but a request already dispatched keeps
    // executing through to completion.
    let (s, readiness) = spawn_with_readiness().await;

    // Provision a tenant so /api/tenant/... has data to answer with.
    s.post("/api/admin/tenants")
        .json(&serde_json::json!({ "tenant_id":"acme", "name":"Acme" }))
        .await.assert_status_ok();

    // Fire the "in-flight" request in a background task.
    let s_bg = s.clone();
    let handle = tokio::spawn(async move {
        let r = s_bg.get("/api/tenant/auth/whoami")
            .add_header("x-tenant-id".parse().unwrap(), "acme".parse().unwrap())
            .await;
        (r.status_code(), r.json::<serde_json::Value>())
    });

    // Simulate the shutdown signal by flipping readiness immediately.
    readiness.set_ready(false);

    // The in-flight request must still complete successfully.
    let (status, body) = tokio::time::timeout(Duration::from_secs(5), handle)
        .await.expect("handler must finish").expect("task join");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["tenant"], "acme");

    // And a *new* request against /api/ready now reports 503 (the LB signal).
    s.get("/api/ready").await.assert_status(StatusCode::SERVICE_UNAVAILABLE);
}

// Silences unused-import warnings in this file since we use them selectively.
#[allow(dead_code)]
fn _readiness_type_check(_r: Readiness) {}
