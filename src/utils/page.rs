//! Shared "page chrome" — the data every authenticated page needs to render
//! the sidebar + navbar (current user, tenant id, live badge counts, and
//! permission flags used to hide inaccessible sidebar links).
//!
//! Every controller that renders a full page (i.e. anything extending
//! `base.html`) should embed a `PageChrome` field in its template struct:
//!
//! ```ignore
//! #[derive(Template)]
//! #[template(path = "settings/index.html")]
//! struct IndexTpl {
//!     chrome: PageChrome,
//!     ...
//! }
//! ```
//!
//! The templates then use `{{ chrome.tenant_id }}`, `{{ chrome.user_name }}`,
//! `{% if chrome.perms.can_manage_students %}` etc.
//!
//! This intentionally centralizes the "what is this authenticated user
//! allowed to see?" question so new modules cannot forget to filter the
//! sidebar.

use sqlx::SqlitePool;

use crate::auth_middleware::CurrentUser;
use crate::csrf_middleware::CSRF_TOKEN_VALUE;
use crate::errors::AppError;
use crate::models::user::{Role, User};

/// Coarse-grained navigation/action permissions computed from the user's
/// role. Deliberately expressive — one bool per menu group — so templates
/// can just check `{% if chrome.perms.can_manage_students %}` without
/// duplicating role logic.
#[derive(Debug, Clone, Default)]
pub struct NavPerms {
    pub can_view_dashboard: bool,
    pub can_manage_students: bool,
    pub can_manage_teachers: bool,
    pub can_manage_classes: bool,
    pub can_mark_attendance: bool,
    pub can_manage_exams: bool,
    pub can_manage_fees: bool,
    pub can_manage_library: bool,
    pub can_view_reports: bool,
    pub can_manage_settings: bool,
}

impl NavPerms {
    /// Compute permissions for a role. This is the single source of truth
    /// for "who can do what" at the navigation level. Route-level guards in
    /// `require_role` middleware provide the actual enforcement — this
    /// struct is only about what to *show*.
    pub fn for_role(role: Option<Role>) -> Self {
        match role {
            Some(Role::Admin) => Self {
                can_view_dashboard: true,
                can_manage_students: true,
                can_manage_teachers: true,
                can_manage_classes: true,
                can_mark_attendance: true,
                can_manage_exams: true,
                can_manage_fees: true,
                can_manage_library: true,
                can_view_reports: true,
                can_manage_settings: true,
            },
            Some(Role::Teacher) => Self {
                can_view_dashboard: true,
                can_manage_students: true,   // read + limited edit later
                can_mark_attendance: true,
                can_manage_exams: true,
                can_view_reports: true,
                ..Self::default()
            },
            Some(Role::Accountant) => Self {
                can_view_dashboard: true,
                can_manage_students: true,   // read-only for fee context
                can_manage_fees: true,
                can_view_reports: true,
                ..Self::default()
            },
            Some(Role::Librarian) => Self {
                can_view_dashboard: true,
                can_manage_library: true,
                ..Self::default()
            },
            Some(Role::Student) | Some(Role::Guardian) => Self {
                can_view_dashboard: true,
                ..Self::default()
            },
            None => Self::default(),
        }
    }
}

/// Everything the shared `base.html` + `_navbar.html` + `_sidebar.html`
/// templates need. Embedded as `chrome: PageChrome` in every page template
/// struct.
#[derive(Debug, Clone)]
pub struct PageChrome {
    /// Which top-level sidebar item is currently active (e.g. "students",
    /// "settings"). Used to highlight the nav link.
    pub active: &'static str,

    pub tenant_id: String,
    pub csrf_token: &'static str,

    // Current user summary (kept small — we don't pass the whole User to
    // templates so we can't accidentally leak the password_hash even if a
    // template were mis-authored).
    pub user_name: String,
    pub user_initials: String,
    pub user_role: String,          // for display, e.g. "admin"

    // Live sidebar badges.
    pub student_count: i64,

    // Nav-level permission flags.
    pub perms: NavPerms,
}

impl PageChrome {
    /// Load the chrome for the current authenticated user. Called once per
    /// full-page handler.
    ///
    /// If future badges (unread notifications, pending approvals, etc.) are
    /// added, they go here — one round-trip to the tenant DB.
    pub async fn load(
        pool: &SqlitePool,
        tenant_id: impl Into<String>,
        active: &'static str,
        current: &CurrentUser,
    ) -> Result<Self, AppError> {
        let CurrentUser(user) = current;
        let student_count: i64 =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM students")
                .fetch_one(pool)
                .await
                .unwrap_or(0);

        Ok(Self {
            active,
            tenant_id: tenant_id.into(),
            csrf_token: CSRF_TOKEN_VALUE,
            user_name: user.full_name.clone(),
            user_initials: user.initials(),
            user_role: user.role.clone(),
            student_count,
            perms: NavPerms::for_role(user.role_enum()),
        })
    }

    /// Escape hatch when a helper needs to reconstruct chrome from a raw
    /// `User` (e.g. `render_form_with_error` helpers that get the user by
    /// value, not via extractor).
    pub async fn load_with_user(
        pool: &SqlitePool,
        tenant_id: impl Into<String>,
        active: &'static str,
        user: &User,
    ) -> Result<Self, AppError> {
        Self::load(pool, tenant_id, active, &CurrentUser(user.clone())).await
    }
}
