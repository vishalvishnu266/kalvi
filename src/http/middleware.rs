//! Tenant-scoping middleware.
//!
//! Extracts the tenant id from the request, consults the [`TenantRegistry`]
//! to get (or create) the tenant's pool, builds a per-request
//! [`crate::services::AppServices`], and inserts both into the request
//! extensions so downstream extractors can pull them out.
//!
//! Tenant id source is configurable via [`TenantSource`]. The application
//! router wires `TenantSource::PathParam { name: "tenant" }` and mounts the
//! per-tenant subtree under `/api/tenant/{tenant}/…` (API) and
//! `/{tenant}/…` (web UI), so the tenant id always travels in the URL —
//! no header or cookie plumbing is needed.

use std::collections::HashMap;

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header::HOST, HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};

use crate::services::{Actor, AppServices, RequestCtx};
use crate::tenancy::{TenantError, TenantId, TenantRegistry};

/// Header consulted for a correlation id. If absent, a fresh id is generated.
const REQUEST_ID_HEADER: &str = "x-request-id";
/// Header consulted for a W3C-style trace id, if any.
const TRACEPARENT_HEADER: &str = "traceparent";
/// Header consulted for the authenticated user id. In a real deployment the
/// user id would be derived from a verified JWT / session cookie by a
/// dedicated auth layer that runs *before* `tenant_scope`; keeping this as a
/// header keeps the middleware focused on wiring and avoids coupling it to
/// any particular auth scheme.
const USER_ID_HEADER: &str = "x-user-id";

/// Where in the request to read the tenant id from.
#[derive(Debug, Clone)]
pub enum TenantSource {
    /// Read from a request header (default: `x-tenant-id`).
    Header(String),
    /// Read the leftmost DNS label of the `Host` header (e.g. `acme.erp.com` → `acme`).
    Subdomain,
    /// Read the first path segment after an optional prefix, e.g. `/t/acme/foo`.
    /// The `prefix` should be like `"/t/"`.
    PathPrefix { prefix: String },
    /// Read from a named path parameter captured by a parent `.nest("…/{name}", …)`.
    ///
    /// This is the natural fit when the router mounts a subtree under
    /// `/api/tenant/{tenant}` or `/{tenant}`. The middleware pulls the
    /// value from the matched path params — no header, no cookie needed.
    PathParam { name: String },
}

impl TenantSource {
    pub fn header_default() -> Self { Self::Header("x-tenant-id".into()) }
    pub fn path_param(name: impl Into<String>) -> Self { Self::PathParam { name: name.into() } }

    async fn extract(&self, req: &mut Request<Body>) -> Result<TenantId, (StatusCode, String)> {
        let raw = match self {
            TenantSource::Header(name) => {
                let hs: &HeaderMap = req.headers();
                hs.get(name.as_str())
                    .and_then(|v| v.to_str().ok())
                    .map(str::to_string)
                    .ok_or((StatusCode::BAD_REQUEST, format!("missing header {name}")))?
            }
            TenantSource::Subdomain => {
                let host = req.headers().get(HOST)
                    .and_then(|v| v.to_str().ok())
                    .ok_or((StatusCode::BAD_REQUEST, "missing Host header".into()))?;
                let host = host.split(':').next().unwrap_or(host);
                let sub = host.split('.').next().unwrap_or("");
                if sub.is_empty() {
                    return Err((StatusCode::BAD_REQUEST, "no subdomain in Host".into()));
                }
                sub.to_string()
            }
            TenantSource::PathPrefix { prefix } => {
                let path = req.uri().path();
                let rest = path.strip_prefix(prefix.as_str())
                    .ok_or((StatusCode::BAD_REQUEST, "missing tenant path prefix".into()))?;
                rest.split('/').next().unwrap_or("").to_string()
            }
            TenantSource::PathParam { name } => {
                // Use axum's `Path` extractor to pull captured route params.
                // We split into parts, run the extractor, and reassemble so
                // the request is left intact for downstream handlers.
                let (mut parts, body) = std::mem::replace(
                    req,
                    Request::new(Body::empty()),
                ).into_parts();
                let params = parts.extract::<Path<HashMap<String, String>>>().await
                    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
                let value = params.0.get(name).cloned()
                    .ok_or((StatusCode::BAD_REQUEST, format!("missing path parameter {name}")))?;
                *req = Request::from_parts(parts, body);
                value
            }
        };
        TenantId::new(raw).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
    }
}

/// State that this middleware carries.
#[derive(Clone)]
pub struct TenantScopeState {
    pub registry: TenantRegistry,
    pub source: TenantSource,
}

impl TenantScopeState {
    pub fn new(registry: TenantRegistry) -> Self {
        Self { registry, source: TenantSource::header_default() }
    }

    pub fn with_source(mut self, source: TenantSource) -> Self {
        self.source = source;
        self
    }
}

/// The middleware function. Register with:
///
/// ```ignore
/// let state = TenantScopeState::new(registry);
/// let app = Router::new()
///     .route("/students", get(list_students))
///     .layer(axum::middleware::from_fn_with_state(state.clone(), tenant_scope))
///     .with_state(state);
/// ```
pub async fn tenant_scope(
    State(state): State<TenantScopeState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let tenant = state.source.extract(&mut req).await?;

    // Record the tenant id into the current tracing span so every log line
    // downstream is correlated to it.
    crate::tracing_layer::record_tenant(tenant.as_str());

    let services = state.registry
        .services_for(&tenant)
        .await
        .map_err(tenant_error_to_http)?;

    // Build the per-request context. Extracted first because we consume
    // header values before moving `tenant` into the extensions.
    let ctx = build_request_ctx(&req, tenant.clone());

    req.extensions_mut().insert::<TenantId>(tenant);
    req.extensions_mut().insert::<AppServices>(services);
    req.extensions_mut().insert::<RequestCtx>(ctx);
    Ok(next.run(req).await)
}

/// Build a [`RequestCtx`] from the incoming request headers.
///
/// * `x-request-id` is honored if present; otherwise a fresh id is minted so
///   every request is correlatable in logs.
/// * `traceparent` is captured verbatim when present.
/// * `x-user-id` (numeric) marks the caller as [`Actor::User`]; anything
///   else — missing, malformed, or empty — falls back to [`Actor::Anonymous`],
///   which lets unauthenticated endpoints (e.g. login, health) still get a
///   valid ctx.
///
/// Real authentication should replace the `x-user-id` shortcut with a
/// dedicated auth middleware that verifies a token and populates the ctx
/// (including `permissions`) before `tenant_scope`, or right after it.
fn build_request_ctx(req: &Request<Body>, tenant: TenantId) -> RequestCtx {
    let headers = req.headers();

    let request_id = headers.get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let trace_id = headers.get(TRACEPARENT_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);

    let actor = headers.get(USER_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<i64>().ok())
        .map(|user_id| Actor::User { user_id })
        .unwrap_or(Actor::Anonymous);

    RequestCtx {
        tenant,
        actor,
        request_id,
        trace_id,
        permissions: std::sync::Arc::new(Vec::new()),
        remote_ip: None,
    }
}

fn tenant_error_to_http(e: TenantError) -> (StatusCode, String) {
    match e {
        TenantError::InvalidId(_)     => (StatusCode::BAD_REQUEST,   e.to_string()),
        TenantError::NotFound(_)      => (StatusCode::NOT_FOUND,     e.to_string()),
        TenantError::Disabled(_)      => (StatusCode::FORBIDDEN,     e.to_string()),
        TenantError::Repo(_)          => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
