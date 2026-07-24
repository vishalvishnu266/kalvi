//! Per-request context: **who** is calling **what** and **from where**.
//!
//! [`RequestCtx`] is a plain, cheap-to-clone value that carries per-request
//! metadata *alongside* the tenant-scoped [`crate::services::AppServices`].
//! Service methods that need auditing, authorization, tracing or actor
//! attribution accept `&RequestCtx` as their first parameter.
//!
//! ## Why a separate value, not a field on `AppServices`?
//!
//! `AppServices` is cached per-tenant by [`crate::http::AppState`]
//! and shared across concurrent requests. Baking mutable per-request state
//! into it would either force us to reconstruct the whole services bundle
//! per request (defeating the cache) or introduce cross-request contamination.
//! Passing `&RequestCtx` keeps `AppServices` immutable and safe to share.
//!
//! ## Use-cases covered
//!
//! * **HTTP requests** — built by tenant-scope middleware from headers /
//!   authenticated user / trace ids; stored in request extensions and
//!   pulled out via the [`crate::http::TenantScope`] extractor.
//! * **Background jobs / cron** — construct with
//!   [`RequestCtx::system`] (`Actor::System { component }`).
//! * **CLI / admin scripts** — [`RequestCtx::system`] with a descriptive
//!   component name (e.g. `"cli:seed-data"`).
//! * **Impersonation / on-behalf-of flows** — [`RequestCtx::impersonated`].
//! * **Tests** — [`RequestCtx::test`] to get a fully populated ctx cheaply.
//!
//! ## Adoption guide (READ ME before adding `&RequestCtx` everywhere)
//!
//! `AppServices` is tenant-scoped and cached, and the request-context
//! plumbing (the [`crate::http::TenantScope`] extractor) is
//! already in place for **every** request. Individual service methods do
//! **not** need `&RequestCtx` unless they actually use it. We deliberately
//! adopt this parameter **incrementally**, not by a sweeping rewrite.
//!
//! ### When to add `&RequestCtx` to a service method
//!
//! Add it as the first parameter (before domain args) if the method:
//!
//! 1. **Writes an audit-worthy row** (`created_by`, `updated_by`, an
//!    `audit_log` entry, an admin action record, a financial mutation).
//! 2. **Performs authorization or row-level checks** that depend on
//!    *who* the caller is (e.g. "a guardian may only see their own
//!    child", "only the assigned counselor may resolve this incident").
//! 3. **Sends a notification / email / SMS** that should attribute the
//!    action to a specific person or system component.
//! 4. **Is invoked by an impersonation flow** and must record both
//!    `by_user_id` and `as_user_id`.
//! 5. **Emits domain events / webhooks** where actor + request-id must
//!    be part of the payload for downstream correlation.
//!
//! ### When *not* to bother
//!
//! Skip `&RequestCtx` for methods that are pure reads with no row-level
//! auth, pure calculations, or internal helpers. Adding the parameter
//! everywhere just for symmetry costs signature churn and buys nothing.
//!
//! ### Convention
//!
//! ```ignore
//! // Before
//! pub async fn charge_tuition(&self, student_id: i64, amount: Money)
//!     -> ServiceResult<FeeId>;
//!
//! // After — ctx always first, domain args unchanged
//! pub async fn charge_tuition(&self, ctx: &RequestCtx,
//!     student_id: i64, amount: Money) -> ServiceResult<FeeId>;
//! ```
//!
//! Inside the method, use:
//! * `ctx.user_id()` for `created_by` / `updated_by` columns,
//! * `ctx.actor` for structured audit rows (matches on `System` vs `User`
//!   vs `Impersonated`),
//! * `ctx.request_id` / `ctx.trace_id` in log fields for correlation,
//! * `ctx.has_permission("...")` for cheap in-memory permission gates
//!   (call [`crate::services::AuthService`] for the authoritative check).
//!
//! ### Recommended adoption order
//!
//! Retrofit audit-heavy domains first, then authorization-sensitive ones,
//! then leave the rest alone until they change for another reason:
//!
//! 1. **First wave** — `fees`, `payroll`, `discipline`, `documents`, `admin`.
//! 2. **Second wave** — `auth` (impersonation, grant/revoke),
//!    `enrollment` (admission actions), `people` (student/staff mutations).
//! 3. **Later / maybe never** — `library`, `inventory`, `transport`,
//!    `hostel`, `communication`, `timetable`, `health`, `academic`
//!    (mostly read paths or low-audit CRUD).
//!
//! See `DisciplineService::report` for a worked example.

use std::sync::Arc;

use crate::tenancy::TenantId;

/// Who is performing an operation.
///
/// Services can pattern-match on this to make auditing, rate-limiting and
/// authorization decisions without reaching into HTTP-layer types.
#[derive(Debug, Clone)]
pub enum Actor {
    /// An authenticated end-user, identified by `user_id` in the tenant DB.
    User { user_id: i64 },

    /// A user acting on behalf of another (support tools, admin
    /// impersonation). Audits should record both ids.
    Impersonated { by_user_id: i64, as_user_id: i64 },

    /// A non-human caller: cron job, worker, migration, CLI, etc.
    /// `component` is a short label used in logs and audit rows.
    System { component: &'static str },

    /// No known actor. Reserved for very early request handling (e.g. the
    /// `/healthz` endpoint) and for tests that don't care.
    Anonymous,
}

impl Actor {
    /// Convenience: the acting user id, if any. Impersonation returns the
    /// **subject** (the user being acted *as*), not the impersonator.
    pub fn user_id(&self) -> Option<i64> {
        match *self {
            Actor::User { user_id }             => Some(user_id),
            Actor::Impersonated { as_user_id, .. } => Some(as_user_id),
            Actor::System { .. } | Actor::Anonymous => None,
        }
    }

    /// The impersonator's id, if the actor is an impersonation.
    pub fn on_behalf_of(&self) -> Option<i64> {
        match *self {
            Actor::Impersonated { by_user_id, .. } => Some(by_user_id),
            _ => None,
        }
    }
}

/// Cheap-to-clone per-request context.
///
/// All heavy fields (permission list) live behind an `Arc` so cloning is
/// effectively an `Arc::clone` plus a few small `Copy`/`String` clones.
#[derive(Debug, Clone)]
pub struct RequestCtx {
    /// Tenant the request is scoped to. Redundant with the tenant-scoped
    /// `AppServices` the caller already holds, but handy for logging and
    /// for cross-tenant sanity checks in service code.
    pub tenant: TenantId,

    /// Who is calling.
    pub actor: Actor,

    /// Correlation id for logs / traces. Typically the incoming
    /// `x-request-id` header or a freshly generated ULID/UUID.
    pub request_id: String,

    /// W3C traceparent-style trace id, if the request came in with one.
    pub trace_id: Option<String>,

    /// Permission codes granted to the actor for **this tenant**.
    /// Wrapped in `Arc` so ctx clones don't re-allocate the vec.
    /// May be empty for `Actor::System` (system callers typically bypass
    /// permission checks explicitly).
    pub permissions: Arc<Vec<String>>,

    /// Client IP, if the HTTP layer resolved one. `None` for non-HTTP
    /// callers.
    pub remote_ip: Option<String>,
}

impl RequestCtx {
    /// Build a context for a system / background caller. Prefer this over
    /// hand-constructing an `Actor::System` variant so future fields get
    /// sensible defaults automatically.
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

    /// Build a context for an authenticated user.
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

    /// Build a context for an impersonation (support / admin acting as user).
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

    /// Convenience for tests. Uses a fixed tenant id and an anonymous actor.
    ///
    /// Gated on `cfg(test)` — this crate has no public `test-util` feature,
    /// so integration tests in `tests/` that need a `RequestCtx` should call
    /// [`RequestCtx::system`] or [`RequestCtx::for_user`] directly.
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            tenant: TenantId::new("test").expect("valid test tenant id"),
            actor: Actor::Anonymous,
            request_id: "test".into(),
            trace_id: None,
            permissions: Arc::new(Vec::new()),
            remote_ip: None,
        }
    }

    // ---------- fluent setters ----------

    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into()); self
    }

    pub fn with_remote_ip(mut self, ip: impl Into<String>) -> Self {
        self.remote_ip = Some(ip.into()); self
    }

    pub fn with_permissions(mut self, perms: Vec<String>) -> Self {
        self.permissions = Arc::new(perms); self
    }

    // ---------- ergonomic accessors ----------

    /// The acting user id, if any. See [`Actor::user_id`] for impersonation
    /// semantics.
    pub fn user_id(&self) -> Option<i64> { self.actor.user_id() }

    /// `true` if `code` is present in the ctx's cached permission list.
    /// Cheap; no DB roundtrip. If your service needs an authoritative check,
    /// call `AuthService::has_permission` instead.
    pub fn has_permission(&self, code: &str) -> bool {
        self.permissions.iter().any(|p| p == code)
    }
}
