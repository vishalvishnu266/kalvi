use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
};
use std::collections::HashSet;
use std::sync::Arc;
use crate::http::AppState;
use crate::middleware::auth::{SessionUser, COOKIE_SESSION, read_cookie_from_headers};
use crate::tenancy::{TenantError, TenantId};
use crate::services::{AppServices, RequestCtx, Actor};

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
        tracing::debug!("TenantScope::from_request_parts: extracting tenant from path");
        let Path(tenant) =
            Path::<String>::from_request_parts(parts, state)
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        tracing::debug!("TenantScope::from_request_parts: validating tenant_id={}", tenant);
        let tid = TenantId::new(tenant)
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        tracing::debug!("TenantScope::from_request_parts: resolving services for tenant={}", tid);
        let services = state.services_for(&tid)
            .await
            .map_err(tenant_error_to_http)?;

        tracing::debug!("TenantScope::from_request_parts: resolving actor context");
        let ctx = resolve_ctx(&tid, &services, parts).await;

        tracing::debug!("TenantScope::from_request_parts: successfully resolved scope for tenant={}", tid);
        Ok(Self {
            tenant: tid,
            services,
            ctx,
        })
    }
}

/// Build a `RequestCtx` for the current request.
///
/// Priority:
/// 1. `SessionUser` extension — already inserted by the web/portal session middleware.
/// 2. `Authorization: Bearer <token>` header — used by the JSON API.
/// 3. Session cookie (`erp_session`) — fallback for browser-based API calls.
/// 4. Anonymous — if no credential is found.
async fn resolve_ctx(tid: &TenantId, services: &AppServices, parts: &Parts) -> RequestCtx {
    let request_id = uuid::Uuid::new_v4().to_string();

    // 1. SessionUser already set by web/portal middleware.
    if let Some(su) = parts.extensions.get::<SessionUser>() {
        let perms: Vec<String> = su.permissions.iter().cloned().collect();
        return RequestCtx {
            tenant: tid.clone(),
            actor: Actor::User { user_id: su.user_id },
            request_id,
            trace_id: None,
            permissions: Arc::new(perms),
            remote_ip: None,
        };
    }

    // 2. Authorization: Bearer <token> (JSON API).
    let bearer = parts.headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::to_string);

    // 3. Session cookie fallback.
    let token_opt = bearer.or_else(|| {
        read_cookie_from_headers(&parts.headers, COOKIE_SESSION).map(|s| s.to_string())
    });

    if let Some(token) = token_opt {
        if let Ok((_, user)) = services.auth.resolve_session(&token).await {
            let perms: HashSet<String> = services.repos.users
                .permissions_of(user.id).await
                .map(|ps| ps.into_iter().map(|p| p.code).collect())
                .unwrap_or_default();
            return RequestCtx {
                tenant: tid.clone(),
                actor: Actor::User { user_id: user.id },
                request_id,
                trace_id: None,
                permissions: Arc::new(perms.into_iter().collect()),
                remote_ip: None,
            };
        }
    }

    // 4. Anonymous.
    RequestCtx {
        tenant: tid.clone(),
        actor: Actor::Anonymous,
        request_id,
        trace_id: None,
        permissions: Arc::new(Vec::new()),
        remote_ip: None,
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
