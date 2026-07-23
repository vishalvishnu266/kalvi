# RBAC & row-scoping — how it works, how to extend it

> The **single reference** for adding a new module screen. If you follow the
> checklist at the bottom, RBAC + row scoping come along for the ride with
> no template surgery.

---

## 1. Design in one page

```
                          ┌────────────────────────┐
                          │  migration 019 seeds   │
                          │   permission + role_   │
                          │   permission rows      │
                          └───────────┬────────────┘
                                      │
                                      ▼
    ┌────────────────────────────────────────────────────────────────┐
    │   src/services/mod.rs :: pub mod perm { pub const X_VIEW…}     │
    │   ← single Rust source of truth for permission code strings.   │
    └───────────────┬────────────────────────────────────────────────┘
                    │
                    ▼
    ┌────────────────────────────────────────────────────────────────┐
    │  middleware/auth.rs :: require_session                         │
    │    · resolves opaque cookie → SessionUser                      │
    │    · hydrates roles + permissions ONCE per request             │
    └───────────────┬────────────────────────────────────────────────┘
                    │
   ┌────────────────┼───────────────────────────────┐
   ▼                ▼                               ▼
route guard    UI-visibility filter          row-scope in service layer
require_perm!  · visible_nav_items(session)  · Scope enum + from_session
               · visible_tiles(session)      · list_x_for(scope, …)
               · {% if session.has(...) %}   · can_view_x(scope, id)
```

Three layers, all driven by the **same permission strings**:

| Layer            | Where                                     | What it decides                           |
| ---------------- | ----------------------------------------- | ----------------------------------------- |
| **Route guard**  | `.route_layer(require_perm!(perm::…))`    | 403 for URL access without the right code.|
| **UI visibility**| `NavItem.perm`, `Tile.perm`, `{% if %}`   | Nav / tiles / buttons hidden from render. |
| **Row scope**    | `Scope` enum + `list_x_for(scope, …)`     | Which rows a list returns for this user.  |

---

## 2. Permission catalogue

- SQL side — `migrations/20260720120019_seed_rbac.sql`
- Rust side — `src/services/mod.rs :: pub mod perm { … }`

Every code follows `"<module>.<action>"`. Actions ending in `_own` mean
"row-scoped variant" (e.g. `students.view_own` = only your own children).

**These two lists must stay in lock-step.** Treat them as one unit; any PR
that adds a permission must touch both files.

---

## 3. Baseline role bindings (migration 019)

Shipped in the tenant DB on first run:

| Role         | Baseline permissions                                                                                                                                                                     |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `admin`      | All (cross join).                                                                                                                                                                        |
| `principal`  | All except `settings.manage`.                                                                                                                                                            |
| `teacher`    | `students.view`, `academic.view`, `attendance.view/mark`, `timetable.view`, `examinations.view/enter_marks`, `guardians.view`, `communication.view`, `library.view`, `documents.view`.  |
| `accountant` | `students.view`, `staff.view`, `fees.view/collect`, `payroll.view/run`, `documents.view`, `audit.view`.                                                                                  |
| `librarian`  | `students.view`, `library.view/manage`.                                                                                                                                                  |
| `student`    | `students.view_own`, `attendance.view_own`, `timetable.view`, `fees.view_own/pay`, `examinations.view_own`, `academic.view`, `library.view`, `communication.view`, `documents.view`.    |
| `guardian`   | `students.view_own`, `attendance.view_own`, `fees.view_own/pay`, `examinations.view_own`, `timetable.view`, `communication.view`, `documents.view`.                                     |

All are set via `INSERT OR IGNORE`, so tenant admins can freely add/remove
rows without conflicting with subsequent migrations.

---

## 4. Session hydration

`middleware/auth.rs :: require_session` extends `SessionUser` with:

```rust
pub struct SessionUser {
    pub user_id: i64,
    pub username: String,
    pub display: String,
    pub session_id: i64,
    pub roles: Vec<String>,
    pub permissions: HashSet<String>,
}
impl SessionUser {
    pub fn has(&self, code: &str) -> bool;
    pub fn any_of(&self, codes: &[&str]) -> bool;
    pub fn is_role(&self, r: &str) -> bool;
}
```

Cost: 2 tiny queries once per authenticated request. If it ever hurts,
cache by `session_id` in an in-memory LRU.

---

## 5. Route guard — `require_perm!`

Defined in `middleware/auth.rs`. Ergonomic wrapper around `check_perm`:

```rust
use crate::{middleware::auth::require_perm, services::perm};

// Single required code:
.route_layer(require_perm!(perm::STAFF_VIEW))

// Any-of (e.g. full view or row-scoped view):
.route_layer(require_perm!(perm::FEES_VIEW, perm::FEES_VIEW_OWN, perm::FEES_PAY))
```

Response format is content-aware:
- `/api/*` → JSON `{ "error": "forbidden", "missing_any_of": [...] }`.
- `/web/*` → small standalone HTML 403 page.

**Always attach this**. Nav/tile filtering hides links; only the middleware
actually returns 403 on manual URL edits.

---

## 6. UI-visibility filters

### 6a. Sidebar / bottom-bar — `NavItem.perm`
```rust
NavItem {
    key: "staff", label: "Staff", href: "staff", icon: "briefcase",
    mobile: false,
    perm: Some(&[perm::STAFF_VIEW]),     // hidden unless caller has this
}
```
Handlers call `visible_nav_items(&session)` (returns `Vec<&'static NavItem>`)
and pass the result to their template. Template loop is unchanged:
```html
{% for item in nav_items %} … {% endfor %}
```

### 6b. Dashboard launcher — `Tile.perm`
Same idea in `web/dashboard.rs`. `visible_tiles(&session)` filters the tile
grid so parents don't see admin tiles even for a moment.

### 6c. Inline buttons — `{% if can_x %}`
Handlers set a per-CTA bool and template gates the button:
```rust
let can_admit = session.has(perm::STUDENTS_ADMIT);
render(&StudentsListPage { …, can_admit });
```
```html
{% if can_admit %}
  <button>Admit student</button>
{% endif %}
```

---

## 7. Row scoping — `Scope`

Defined in `src/services/people.rs`. Decides **which rows** a list returns.

```rust
pub enum Scope {
    Global,                     // admin / staff-with-view
    GuardianOfUser(i64),        // parent portal
    SelfStudent(i64),           // student self-service portal
}

impl Scope {
    pub fn from_session(session: &SessionUser) -> Self;
}
```

Handlers:
```rust
let scope   = Scope::from_session(&session);
let people  = tscope.services.people.list_students_for(scope, 50).await?;
```

Ownership check for detail pages:
```rust
if !tscope.services.people.can_view_student(&scope, id).await? {
    return Err(WebError::forbidden("not permitted"));
}
```

Same URL (`/students`, `/students/42`), different data per caller. This is
what "parent only sees their own child" actually is.

---

## 8. Portal URL policy

**The role never appears in the URL.** URLs describe resources
(`/web/{tenant}/students/42` and `/portal/{tenant}/students/42`), not callers.
Staff/teacher routes live under `/web/{tenant}/…`; parent/student routes live
under `/portal/{tenant}/…`, and the global portal hub lives at `/portal` where
linked tenant accounts are aggregated.

---

## 9. **Recipe: add a new module screen with RBAC baked in**

Follow this checklist for any new screen (guardians, academic, attendance,
whatever). Every step is small.

### ① Define the permission codes

**Rust** — `src/services/mod.rs :: pub mod perm`:
```rust
pub const GUARDIANS_VIEW:   &str = "guardians.view";
pub const GUARDIANS_MANAGE: &str = "guardians.manage";
```

**SQL** — a new migration (or append to `019` if you're comfortable
re-running against dev; production tenants only run each migration once):
```sql
INSERT OR IGNORE INTO permission (code) VALUES
    ('guardians.view'),
    ('guardians.manage');

INSERT OR IGNORE INTO role_permission (role_id, permission_id)
SELECT r.id, p.id FROM role r, permission p
 WHERE r.name = 'principal' AND p.code IN ('guardians.view','guardians.manage');
-- (repeat for teacher / accountant / etc. as needed)
```

### ② Add the row-scope variant (if applicable)

If the screen shows rows a user might only partially own:

**Repo** — add a batch fetch and an ownership check to the relevant repo:
```rust
pub async fn list_by_ids(&self, ids: &[i64]) -> RepoResult<Vec<Guardian>> { … }
pub async fn is_guardian_of(&self, user_id: i64, sid: i64) -> RepoResult<bool> { … }
```

**Service** — extend or add a `Scope` variant and a `list_x_for(scope, …)`
method (see `people.rs` for the reference implementation).

### ③ Handler

```rust
pub async fn list(
    tscope: TenantScope,
    Extension(session): Extension<SessionUser>,
) -> Result<Response, WebError> {
    let scope   = Scope::from_session(&session);
    let rows    = tscope.services.people.list_guardians_for(scope).await?;
    let nav     = NavContext::new(session.display.clone(), tscope.tenant.as_str().into(),
                                  "guardians", "Guardians");
    let nav_items = visible_nav_items(&session);
    let can_manage = session.has(perm::GUARDIANS_MANAGE);
    render(&GuardiansListPage { nav: &nav, nav_items, rows, can_manage })
}
```

Detail handler: call `can_view_x(&scope, id)` before rendering.

### ④ Template

- Use `{% if can_manage %}` around admin CTAs (Add / Edit / Delete).
- Loop `nav_items` the same way every other template does.
- Show a friendly empty-state — the app has **no built-in mock data**.

### ⑤ Register nav + tile

`src/web/layout.rs`:
```rust
NavItem {
    key: "guardians", label: "Guardians", href: "guardians", icon: "users",
    mobile: false, perm: Some(&[perm::GUARDIANS_VIEW]),
},
```

`src/web/dashboard.rs`:
```rust
Tile {
    href: "guardians", label: "Guardians", description: "Parents & contacts",
    icon: "users", gradient: "from-rose-500 to-pink-600",
    perm: Some(&[perm::GUARDIANS_VIEW]),
},
```

If you're replacing an existing `modules.rs` stub, also delete the matching
`ModuleStub` and `stub_handler!(...)` from `web/modules.rs`.

### ⑥ Route + guard

`src/http/routes.rs :: web_tenant_shell`:
```rust
.route("/guardians", get(wgd::list)
    .route_layer(require_perm!(perm::GUARDIANS_VIEW)))
.route("/guardians/{id}", get(wgd::show)
    .route_layer(require_perm!(perm::GUARDIANS_VIEW)))
```

### ⑦ (Optional) Seed data

Add rows to `scripts/seed_demo.sh` (API-based) and/or `scripts/seed_demo.sql`
(SQL-based) so `dev_reset.sh` lights the screen up on a fresh instance.

### ⑧ Verify

```bash
bash scripts/dev_reset.sh
```
- Sign in as `admin` — the screen should render.
- Sign in as `parent` — the screen should be **absent from the sidebar and
  dashboard**, and typing the URL directly should return the 403 page.

---

## 10. Known gaps (backlog for future PRs)

Carried forward from the Step-3 rollout. Small, incremental — pick any one.

- [ ] **Teacher row scope** — currently a teacher without `students.view`
      falls through to `GuardianOfUser` (empty list). Add:
      - `Scope::Teacher { class_section_ids: Vec<i64> }`
      - `StudentRepo::list_in_sections(&[i64])`
      - Update `Scope::from_session` to hydrate a teacher's active sections.
- [ ] **API-side `require_perm!` sweep** — the macro already handles JSON
      responses; we just haven't attached `.route_layer(require_perm!(…))`
      to `tenant_api` routes yet. Mirror the web-tenant wiring.
- [ ] **Row scoping for staff / fees / attendance** — same `Scope` pattern
      needs porting to `staff`, `fees`, `attendance`, `payroll`,
      `examinations` service methods as those screens are built.
- [ ] **Staff self-service (`Scope::SelfStaff`)** — for the future employee
      portal (view own payslip, mark own leave). Needs
      `StaffRepo::find_by_user_id(uid)` + a `Scope::SelfStaff(uid)` variant.
- [ ] **Cross-tenant parent portal** — current shell is tenant-scoped at
      `/portal/{tenant}/…`. Add global portal identity + tenant membership
      mapping so one parent account can switch schools without separate
      tenant logins.
- [ ] **Roles & permissions admin screen** — CRUD over `role_permission`
      inside a new `/web/{tenant}/settings/roles` page. Would let tenant
      admins rebind without touching migrations.
- [ ] **Session cache for roles+permissions** — currently hydrated per
      request (2 small queries). Only optimize if profiling shows it in the
      top-N.
- [ ] **Notifications badge** — the topbar bell currently has no data
      source. When we implement the communication module, wire an unread
      count through `SessionUser`.
- [ ] **Audit trail for permission grants** — record who added/removed each
      `role_permission` row. Table exists in migration 016; hook up when the
      Roles & permissions admin screen lands.

---

## 11. Testing checklist for any RBAC-touching PR

Do these before requesting review:

1. `bash scripts/dev_reset.sh` — clean-slate reset must succeed.
2. Sign in as **each** persona (`admin`, `principal`, `teacher`,
   `accountant`, `librarian`, `parent`, `student1`) and verify:
   - **Sidebar** only shows expected items.
   - **Dashboard tiles** only show expected tiles.
   - **Manually typing** a forbidden URL returns the friendly 403 page.
   - **Row-scoped lists** show only the appropriate rows.
3. Grep to make sure any new permission code is present in **both** the
   Rust `perm::` module and the SQL migration.
