use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
};
use sqlx::SqlitePool;
use std::collections::HashSet;
use std::sync::Arc;

use crate::application::AppState;
use crate::exception::tenant_error::TenantError;
use crate::security::security_context::{Actor, SecurityContext};
use crate::security::session_user::{read_cookie_from_headers, SessionUser, COOKIE_SESSION};
use crate::service::auth_service;
use crate::tenant::tenant_id::{validate_tenant_id, TenantId};

/// Per-request scope resolved from the `{tenant}` URL segment.
pub struct TenantScope {
    pub tenant: TenantId,
    pub pool: SqlitePool,
    pub sessions: SqlitePool,
    pub ctx: SecurityContext,
}

impl FromRequestParts<AppState> for TenantScope {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Path(tenant) = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        let tid =
            validate_tenant_id(tenant).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        let pool = state.pool_for(&tid).await.map_err(tenant_error_to_http)?;
        let sessions = state.sessions.clone();

        let ctx = resolve_ctx(&tid, &pool, &sessions, parts).await;

        Ok(Self {
            tenant: tid,
            pool,
            sessions,
            ctx,
        })
    }
}

async fn resolve_ctx(
    tid: &str,
    pool: &SqlitePool,
    sessions: &SqlitePool,
    parts: &Parts,
) -> SecurityContext {
    let request_id = uuid::Uuid::new_v4().to_string();

    if let Some(su) = parts.extensions.get::<SessionUser>() {
        let perms: Vec<String> = su.permissions.iter().cloned().collect();
        return SecurityContext {
            tenant: tid.to_string(),
            actor: Actor::User {
                user_id: su.user_id,
            },
            request_id,
            trace_id: None,
            permissions: Arc::new(perms),
            remote_ip: None,
        };
    }

    let bearer = parts
        .headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::to_string);

    let token_opt = bearer.or_else(|| {
        read_cookie_from_headers(&parts.headers, COOKIE_SESSION).map(|s| s.to_string())
    });

    if let Some(token) = token_opt {
        if let Ok((_, user)) = auth_service::resolve_session(pool, sessions, &token).await {
            let perms: HashSet<String> = auth_service::permissions_of(pool, user.id)
                .await
                .map(|ps| ps.into_iter().map(|p| p.code).collect())
                .unwrap_or_default();
            return SecurityContext {
                tenant: tid.to_string(),
                actor: Actor::User { user_id: user.id },
                request_id,
                trace_id: None,
                permissions: Arc::new(perms.into_iter().collect()),
                remote_ip: None,
            };
        }
    }

    SecurityContext {
        tenant: tid.to_string(),
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
        TenantError::NotFound(_) => (StatusCode::NOT_FOUND, e.to_string()),
        TenantError::Disabled(_) => (StatusCode::FORBIDDEN, e.to_string()),
        TenantError::Repo(_) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
