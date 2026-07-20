//! HTTP tracing / request-id propagation.
//!
//! Wires together:
//! * `tower_http::request_id::SetRequestIdLayer` — generates a UUID v4 if the
//!   incoming request doesn't already carry `x-request-id`.
//! * `tower_http::request_id::PropagateRequestIdLayer` — echoes the same id
//!   on the response so downstream services / clients can correlate.
//! * `tower_http::trace::TraceLayer` — one span per request with
//!   `method`, `path`, `request_id`, and an empty `tenant_id` field that
//!   the tenant middleware records via `Span::current().record(...)`.
//!
//! Result: every log line emitted during a request carries `request_id`
//! and (for tenant-scoped routes) `tenant_id`.

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request};
use tower_http::request_id::{MakeRequestId, RequestId};
use tracing::{field, Span};

pub const X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

/// Generates a random UUID v4 for each request that arrives without an
/// `x-request-id` header.
#[derive(Clone, Default)]
pub struct UuidRequestId;

impl MakeRequestId for UuidRequestId {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let id = uuid::Uuid::new_v4().to_string();
        HeaderValue::from_str(&id).ok().map(RequestId::new)
    }
}

/// The span factory used by `TraceLayer::make_span_with`. Extracts the
/// request id from the request (populated by `SetRequestIdLayer`) and opens
/// a placeholder `tenant_id` field for the tenant middleware to fill in.
pub fn make_span_with_ids(req: &Request<Body>) -> Span {
    let request_id = req
        .headers()
        .get(X_REQUEST_ID)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-");

    tracing::info_span!(
        "http",
        method = %req.method(),
        path   = %req.uri().path(),
        request_id = %request_id,
        tenant_id  = field::Empty,
    )
}

/// Called from the tenant middleware to inject the resolved tenant id into
/// the current span so all downstream logs carry it.
pub fn record_tenant(tenant_id: &str) {
    Span::current().record("tenant_id", tenant_id);
}
