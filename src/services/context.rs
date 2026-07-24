use std::sync::Arc;

use crate::tenancy::TenantId;

#[derive(Debug, Clone)]
pub enum Actor {

    User { user_id: i64 },

Impersonated { by_user_id: i64, as_user_id: i64 },

System { component: &'static str },

Anonymous,
}

impl Actor {

pub fn user_id(&self) -> Option<i64> {
        match *self {
            Actor::User { user_id }             => Some(user_id),
            Actor::Impersonated { as_user_id, .. } => Some(as_user_id),
            Actor::System { .. } | Actor::Anonymous => None,
        }
    }

pub fn on_behalf_of(&self) -> Option<i64> {
        match *self {
            Actor::Impersonated { by_user_id, .. } => Some(by_user_id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestCtx {

pub tenant: TenantId,

pub actor: Actor,

pub request_id: String,

pub trace_id: Option<String>,

pub permissions: Arc<Vec<String>>,

pub remote_ip: Option<String>,
}

impl RequestCtx {

pub fn system(tenant: TenantId, component: &'static str) -> Self {
        Self {
            tenant,
            actor: Actor::System { component },
            request_id: format!("system:{component}"),
            trace_id: None,
            permissions: Arc::new(Vec::new()),
            remote_ip: None,
        }
    }

pub fn for_user(
        tenant: TenantId,
        user_id: i64,
        request_id: impl Into<String>,
    ) -> Self {
        Self {
            tenant,
            actor: Actor::User { user_id },
            request_id: request_id.into(),
            trace_id: None,
            permissions: Arc::new(Vec::new()),
            remote_ip: None,
        }
    }

pub fn impersonated(
        tenant: TenantId,
        by_user_id: i64,
        as_user_id: i64,
        request_id: impl Into<String>,
    ) -> Self {
        Self {
            tenant,
            actor: Actor::Impersonated { by_user_id, as_user_id },
            request_id: request_id.into(),
            trace_id: None,
            permissions: Arc::new(Vec::new()),
            remote_ip: None,
        }
    }

pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into()); self
    }

    pub fn with_remote_ip(mut self, ip: impl Into<String>) -> Self {
        self.remote_ip = Some(ip.into()); self
    }

    pub fn with_permissions(mut self, perms: Vec<String>) -> Self {
        self.permissions = Arc::new(perms); self
    }

pub fn user_id(&self) -> Option<i64> { self.actor.user_id() }

pub fn has_permission(&self, code: &str) -> bool {
        self.permissions.iter().any(|p| p == code)
    }
}
