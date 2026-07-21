#!/usr/bin/env bash
# End-to-end smoke test for the multi-tenant school-erp server.
#
# Usage:
#   BASE_URL=http://127.0.0.1:3000 bash scripts/e2e_smoke.sh
#
# Requires: curl. Uses `jq` if available for prettier output, but doesn't
# depend on it.
#
# What it does — see scripts/README.md for the full table.

set -Eeuo pipefail

BASE_URL="${BASE_URL:-http://127.0.0.1:3000}"
ACME="${ACME:-acme}"
GLOBEX="${GLOBEX:-globex}"

# Default login credentials created for both tenants. Overridable via env so
# CI can inject stronger credentials without editing the script.
ADMIN_USER="${ADMIN_USER:-admin}"
ADMIN_PASS="${ADMIN_PASS:-admin123}"

# ---------- pretty printing ----------
bold()  { printf '\033[1m%s\033[0m\n' "$*"; }
ok()    { printf '  \033[32m✔\033[0m %s\n' "$*"; }
fail()  { printf '  \033[31m✘\033[0m %s\n' "$*" >&2; exit 1; }
step()  { printf '\n\033[36m▶ %s\033[0m\n' "$*"; }

pp() {
  # Pretty-print JSON if jq is on PATH, otherwise passthrough.
  if command -v jq >/dev/null 2>&1; then jq . ; else cat ; fi
}

# ---------- HTTP helper ----------
# call METHOD PATH [--data JSON] [--header 'K: V' ...]
# Prints status + body, returns non-zero if HTTP status >= 400.
call() {
  local method="$1"; shift
  local path="$1";   shift
  local url="$BASE_URL$path"
  local tmp; tmp="$(mktemp)"
  local status
  # -o body, -w status; -sS = silent-with-errors.
  status="$(curl -sS -o "$tmp" -w '%{http_code}' -X "$method" "$@" "$url")"
  printf '  %s %s -> %s\n' "$method" "$path" "$status"
  if [[ -s "$tmp" ]]; then
    sed 's/^/    /' "$tmp" | pp | sed 's/^/    /'
  fi
  # Expose body/status to caller.
  RESP_BODY="$(cat "$tmp")"
  RESP_STATUS="$status"
  rm -f "$tmp"
  [[ "$status" -lt 400 ]]
}

# ------------------------------------------------------------------------
bold "E2E smoke test against $BASE_URL"
bold "tenants: $ACME, $GLOBEX"

# 1. Health / readiness
step "1) Liveness + readiness probes"
call GET /api/live  || fail "server not live"
call GET /api/ready || fail "server not ready"
ok "server is up"

# 2. Provision tenant #1
step "2) Provision tenant '$ACME'"
call POST /api/admin/tenants \
  -H 'content-type: application/json' \
  --data "$(cat <<JSON
{ "tenant_id": "$ACME",
  "name":      "Acme Public School",
  "plan":      "standard",
  "notes":     "smoke-test tenant" }
JSON
)" || fail "could not create tenant $ACME (already exists? try DELETE first)"
ok "tenant '$ACME' provisioned (data/tenants/$ACME.db created + migrated)"

# 3. Provision tenant #2
step "3) Provision tenant '$GLOBEX'"
call POST /api/admin/tenants \
  -H 'content-type: application/json' \
  --data "$(cat <<JSON
{ "tenant_id": "$GLOBEX",
  "name":      "Globex International School",
  "plan":      "premium" }
JSON
)" || fail "could not create tenant $GLOBEX"
ok "tenant '$GLOBEX' provisioned"

# 4. List tenants
step "4) List tenants (control plane)"
call GET /api/admin/tenants || fail "could not list tenants"
grep -q "$ACME"   <<<"$RESP_BODY" || fail "'$ACME' missing from tenant list"
grep -q "$GLOBEX" <<<"$RESP_BODY" || fail "'$GLOBEX' missing from tenant list"
ok "both tenants present in system DB"

# 4b. Register a default admin **login user** in each tenant.
#
# The `people/staff` and `people/students` records created below are domain
# records only — they carry `user_id: null` and can't sign in. To exercise
# the web UI (or /api/tenant/auth/login) we need real auth users. We create
# `admin/admin123` in both tenants. If the user already exists (re-run of
# the smoke test), the server returns 409 — we treat that as success.
register_user() {
  local tenant="$1"; local email="$2"
  call POST "/api/tenant/$tenant/auth/register" \
    -H 'content-type: application/json' \
    --data "$(cat <<JSON
{ "username": "$ADMIN_USER",
  "email":    "$email",
  "password": "$ADMIN_PASS",
  "roles":    ["admin"] }
JSON
)" && return 0
  # 409 = user already registered → idempotent success.
  [[ "$RESP_STATUS" == "409" ]] && { ok "user already existed in $tenant"; return 0; }
  return 1
}

verify_login() {
  local tenant="$1"
  call POST "/api/tenant/$tenant/auth/login" \
    -H 'content-type: application/json' \
    --data "$(cat <<JSON
{ "identifier": "$ADMIN_USER",
  "password":   "$ADMIN_PASS" }
JSON
)" || fail "login as $ADMIN_USER failed in $tenant"
}

step "4b) Register default admin user in each tenant"
register_user "$ACME"   "admin@acme.example"   || fail "register in $ACME failed"
register_user "$GLOBEX" "admin@globex.example" || fail "register in $GLOBEX failed"
verify_login  "$ACME"
verify_login  "$GLOBEX"
ok "admin login users ready in both tenants"
ok "you can now sign in at $BASE_URL/login with:"
printf '      tenant   = %s   or   %s\n' "$ACME" "$GLOBEX"
printf '      username = %s\n' "$ADMIN_USER"
printf '      password = %s\n' "$ADMIN_PASS"

# 5. Hire staff in ACME
step "5) Hire staff member 'Alice' in tenant '$ACME'"
call POST "/api/tenant/$ACME/people/staff" \
  -H 'content-type: application/json' \
  --data '{
    "employee_no":     "EMP-ACME-001",
    "user_id":         null,
    "department_id":   null,
    "first_name":      "Alice",
    "last_name":       "Acme",
    "date_of_birth":   "1990-01-01",
    "gender":          "female",
    "phone":           null,
    "email":           "alice@acme.example",
    "designation":     "Teacher",
    "employment_type": "full_time",
    "date_of_joining": "2025-06-01",
    "photo_path":      null
  }' || fail "hire failed in $ACME"
ok "Alice hired in $ACME"

# 6. Hire staff in GLOBEX
step "6) Hire staff member 'Grace' in tenant '$GLOBEX'"
call POST "/api/tenant/$GLOBEX/people/staff" \
  -H 'content-type: application/json' \
  --data '{
    "employee_no":     "EMP-GLOBEX-001",
    "user_id":         null,
    "department_id":   null,
    "first_name":      "Grace",
    "last_name":       "Globex",
    "date_of_birth":   "1992-02-02",
    "gender":          "female",
    "phone":           null,
    "email":           "grace@globex.example",
    "designation":     "Principal",
    "employment_type": "full_time",
    "date_of_joining": "2025-06-01",
    "photo_path":      null
  }' || fail "hire failed in $GLOBEX"
ok "Grace hired in $GLOBEX"

# 7. Tenant isolation check
step "7) Tenant isolation check — staff lists MUST be different"
call GET "/api/tenant/$ACME/people/staff?limit=50&offset=0" \
  || fail "staff list failed for $ACME"
ACME_LIST="$RESP_BODY"
call GET "/api/tenant/$GLOBEX/people/staff?limit=50&offset=0" \
  || fail "staff list failed for $GLOBEX"
GLOBEX_LIST="$RESP_BODY"

grep -q 'EMP-ACME-001'   <<<"$ACME_LIST"   || fail "$ACME's staff list missing Alice"
grep -q 'EMP-GLOBEX-001' <<<"$GLOBEX_LIST" || fail "$GLOBEX's staff list missing Grace"
if grep -q 'EMP-GLOBEX-001' <<<"$ACME_LIST"; then
  fail "TENANT LEAK: $ACME can see $GLOBEX's staff"
fi
if grep -q 'EMP-ACME-001' <<<"$GLOBEX_LIST"; then
  fail "TENANT LEAK: $GLOBEX can see $ACME's staff"
fi
ok "each tenant only sees its own staff (DB isolation verified)"

# 8. Admit a student in ACME (needed for the discipline demo)
step "8) Admit student 'Bob' in tenant '$ACME'"
# We rely on the migrations having seeded the reference data
# (grades, sections, roles). Admission requires an academic year + a
# class_section, but the plain admit endpoint accepts a student without
# an enrollment target — we use that simpler shape here to keep the
# smoke test independent of seed IDs.
call POST "/api/tenant/$ACME/people/students/admit" \
  -H 'content-type: application/json' \
  --data '{
    "student": {
      "admission_no":   "ADM-ACME-001",
      "user_id":        null,
      "first_name":     "Bob",
      "middle_name":    null,
      "last_name":      "Student",
      "date_of_birth":  "2015-01-01",
      "gender":         "male",
      "blood_group":    null,
      "nationality":    null,
      "religion":       null,
      "photo_path":     null,
      "admission_date": "2025-06-01",
      "address_line1":  null,
      "address_line2":  null,
      "city":           null,
      "state":          null,
      "postal_code":    null,
      "country":        null
    },
    "guardian":                    null,
    "enroll_into_class_section":   null,
    "roll_no":                     null,
    "enrolled_on":                 null
  }' || fail "student admission failed"
STUDENT_ID="$(echo "$RESP_BODY" | sed -n 's/.*"student":{"id":\([0-9]*\).*/\1/p')"
if [[ -z "${STUDENT_ID:-}" ]]; then
  fail "could not parse student id from admission response"
fi
ok "student admitted (id=$STUDENT_ID)"

# 9. Exercise the RequestCtx-aware discipline endpoint
step "9) Report a discipline incident as user #42 (RequestCtx demo)"
call POST "/api/tenant/$ACME/discipline/" \
  -H 'content-type: application/json' \
  -H 'x-user-id: 42' \
  -H 'x-request-id: smoke-test-req-1' \
  --data "$(cat <<JSON
{ "student_id":          $STUDENT_ID,
  "date":                "2025-06-15",
  "description":         "Late to class",
  "severity":            "low",
  "action_taken":        "Warning",
  "reported_by_staff_id": null,
  "notify_guardians":    false }
JSON
)" || fail "discipline report failed"
ok "discipline incident recorded; check the server logs — you should see:"
printf '    tracing: actor=User { user_id: 42 } request=smoke-test-req-1\n'

# 10. Unknown tenant → 404
step "10) Unknown tenant should be rejected (404 expected)"
if call GET "/api/tenant/nope-does-not-exist/people/staff"; then
  fail "unknown tenant was accepted (expected 404)"
fi
[[ "$RESP_STATUS" == "404" ]] || fail "expected 404, got $RESP_STATUS"
ok "unknown tenant correctly rejected with 404"

# 11. Disabled tenant → 403
step "11) Disable '$GLOBEX' then try to call it (403 expected)"
call POST /api/admin/tenants/$GLOBEX/disable || fail "could not disable $GLOBEX"
if call GET "/api/tenant/$GLOBEX/people/staff"; then
  fail "disabled tenant was accepted (expected 403)"
fi
[[ "$RESP_STATUS" == "403" ]] || fail "expected 403, got $RESP_STATUS"
ok "disabled tenant correctly rejected with 403"

# 12. Re-enable
step "12) Re-enable '$GLOBEX'"
call POST /api/admin/tenants/$GLOBEX/enable || fail "could not re-enable $GLOBEX"
ok "$GLOBEX re-enabled"

printf '\n\033[1;32mALL CHECKS PASSED ✔\033[0m\n'
