//! Tenant-scoping middleware.
//!
//! Extracts the tenant id from the request, consults the [`TenantRegistry`]
//! to get (or create) the tenant's pool, builds a per-request
//! [`crate::services::AppServices`], and inserts both into the request
//! extensions so downstream extractors can pull them out.
//!
//! Tenant id source is configurable via [`TenantSource`]. Default: header
//! `x-tenant-id`.

use axum::{
    body::Body,
    extract::State,
    http::{header::HOST, HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::services::AppServices;
use crate::tenancy::{TenantError, TenantId, TenantRegistry};

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
}

impl TenantSource {
    pub fn header_default() -> Self { Self::Header("x-tenant-id".into()) }

    fn extract(&self, req: &Request<Body>) -> Result<TenantId, (StatusCode, String)> {
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
    let tenant = state.source.extract(&req)?;

    // Record the tenant id into the current tracing span so every log line
    // downstream is correlated to it.
    crate::tracing_layer::record_tenant(tenant.as_str());

    let services = state.registry
        .services_for(&tenant)
        .await
        .map_err(tenant_error_to_http)?;

    req.extensions_mut().insert::<TenantId>(tenant);
    req.extensions_mut().insert::<AppServices>(services);
    Ok(next.run(req).await)
}

fn tenant_error_to_http(e: TenantError) -> (StatusCode, String) {
    match e {
        TenantError::InvalidId(_)     => (StatusCode::BAD_REQUEST,   e.to_string()),
        TenantError::NotFound(_)      => (StatusCode::NOT_FOUND,     e.to_string()),
        TenantError::Disabled(_)      => (StatusCode::FORBIDDEN,     e.to_string()),
        TenantError::Repo(_)          => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
