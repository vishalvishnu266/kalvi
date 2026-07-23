use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
};
use serde::Deserialize;
use std::sync::Arc;

use crate::http::{AppState, ServiceHttpError};
use crate::tenancy::{TenantId, TenantError};
use crate::services::{AppServices, RequestCtx, Actor};

/// Helper for pulling the `{tenant}` segment out of the URL path.
#[derive(Deserialize)]
struct TenantPath {
    tenant: String,
}

/// Everything a tenant-scoped handler needs, obtained in one extractor call:
/// the validated tenant id, the per-tenant [`AppServices`] bundle, and a
/// fresh [`RequestCtx`].
///
/// The extractor:
/// 1. Pulls the `{tenant}` path segment via `axum::extract::Path`.
/// 2. Validates it with [`TenantId::new`] (charset + length).
/// 3. Asks the [`TenantRegistry`] for the cached `AppServices` (or lazily
///    builds the connection pool + runs migrations on first access).
pub struct TenantScope {
    pub tenant: TenantId,
    pub services: AppServices,
    pub ctx: RequestCtx,
}

impl FromRequestParts<AppState> for TenantScope {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Path(TenantPath { tenant }) =
            Path::<TenantPath>::from_request_parts(parts, state)
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        let tid = TenantId::new(tenant)
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        let services = state
            .tenants
            .services_for(&tid)
            .await
            .map_err(tenant_error_to_http)?;

        let ctx = RequestCtx {
            tenant: tid.clone(),
            actor: Actor::Anonymous,
            request_id: uuid::Uuid::new_v4().to_string(),
            trace_id: None,
            permissions: Arc::new(Vec::new()),
            remote_ip: None,
        };

        Ok(Self {
            tenant: tid,
            services,
            ctx,
        })
    }
}

fn tenant_error_to_http(e: TenantError) -> (StatusCode, String) {
    match e {
        TenantError::InvalidId(_) => (StatusCode::BAD_REQUEST, e.to_string()),
        TenantError::NotFound(_)  => (StatusCode::NOT_FOUND, e.to_string()),
        TenantError::Disabled(_)  => (StatusCode::FORBIDDEN, e.to_string()),
        TenantError::Repo(_)      => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
