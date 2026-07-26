use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
};
use sqlx::SqlitePool;
use std::collections::HashSet;
use std::sync::Arc;

use crate::http::AppState;
use crate::middleware::auth::{read_cookie_from_headers, SessionUser, COOKIE_SESSION};
use crate::services::{auth as auth_svc, Actor, RequestCtx};
use crate::tenancy::{validate_tenant_id, TenantError, TenantId};

/// Per-request scope resolved from the `{tenant}` URL segment.
/// Holds the tenant pool + session store + request context. Handlers
/// call free-fn services directly, e.g.
/// `services::demo::create_message(&scope.pool, &scope.ctx, body).await?`.
pub struct TenantScope {
    pub tenant: TenantId,
    pub pool: SqlitePool,
    pub sessions: SqlitePool,
    pub ctx: RequestCtx,
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
) -> RequestCtx {
    let request_id = uuid::Uuid::new_v4().to_string();

    if let Some(su) = parts.extensions.get::<SessionUser>() {
        let perms: Vec<String> = su.permissions.iter().cloned().collect();
        return RequestCtx {
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
        if let Ok((_, user)) = auth_svc::resolve_session(pool, sessions, &token).await {
            let perms: HashSet<String> = auth_svc::permissions_of(pool, user.id)
                .await
                .map(|ps| ps.into_iter().map(|p| p.code).collect())
                .unwrap_or_default();
            return RequestCtx {
                tenant: tid.to_string(),
                actor: Actor::User { user_id: user.id },
                request_id,
                trace_id: None,
                permissions: Arc::new(perms.into_iter().collect()),
                remote_ip: None,
            };
        }
    }

    RequestCtx {
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
