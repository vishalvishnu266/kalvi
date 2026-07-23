#!/usr/bin/env bash
# =============================================================================
# scripts/seed_demo.sh
# -----------------------------------------------------------------------------
# Populate a demo tenant with realistic sample data via the HTTP API.
#
# Nothing here is required by the application — the app itself ships zero mock
# data. This script exists purely so you can point a fresh dev / demo instance
# at populated screens without hand-typing rows.
#
# It is safe to re-run: every step is idempotent (existing tenant / users /
# staff / students are detected and skipped).
#
# ---------------------------------------------------------------------------
# Usage
# ---------------------------------------------------------------------------
#   # 1. Start the server (see scripts/README.md for env-var options):
#   cargo run --release
#
#   # 2. In another shell, run the seeder:
#   BASE_URL=http://127.0.0.1:3000 bash scripts/seed_demo.sh
#
# Overridable env vars:
#   BASE_URL     Server URL           (default: http://127.0.0.1:3000)
#   TENANT       Tenant id to seed    (default: demo)
#   TENANT_NAME  Tenant display name  (default: "Demo School")
#   ADMIN_USER   Admin username       (default: admin)
#   ADMIN_PASS   Admin password       (default: admin123)
# =============================================================================
set -euo pipefail

BASE_URL="${BASE_URL:-http://127.0.0.1:3000}"
TENANT="${TENANT:-demo}"
TENANT_NAME="${TENANT_NAME:-Demo School}"
ADMIN_USER="${ADMIN_USER:-admin}"
ADMIN_PASS="${ADMIN_PASS:-admin123}"

# ---------- pretty printing ----------
bold()  { printf '\033[1m%s\033[0m\n' "$*"; }
ok()    { printf '  \033[32m✔\033[0m %s\n' "$*"; }
warn()  { printf '  \033[33m•\033[0m %s\n' "$*"; }
fail()  { printf '  \033[31m✘\033[0m %s\n' "$*" >&2; exit 1; }
pp()    { if command -v jq >/dev/null 2>&1; then jq .; else cat; fi }

# ---------- HTTP helper ----------
# Usage: call METHOD PATH [JSON_BODY]
# Prints the response body and returns the HTTP status via the RETURN_CODE var.
call() {
  local method="$1" path="$2" body="${3:-}"
  local url="$BASE_URL$path" resp status
  if [[ -n "$body" ]]; then
    resp=$(curl -sS -o /tmp/seed_body.$$ -w '%{http_code}' \
                 -X "$method" "$url" \
                 -H 'content-type: application/json' \
                 --data "$body") || fail "network error calling $method $path"
  else
    resp=$(curl -sS -o /tmp/seed_body.$$ -w '%{http_code}' \
                 -X "$method" "$url") || fail "network error calling $method $path"
  fi
  status="$resp"
  RETURN_CODE="$status"
  RETURN_BODY="$(cat /tmp/seed_body.$$)"
  rm -f /tmp/seed_body.$$
}

# ---------- 0. Sanity ----------
bold "Seeding demo data on $BASE_URL (tenant: $TENANT)"
call GET /api/live
[[ "$RETURN_CODE" == "200" ]] || fail "server is not live at $BASE_URL (got $RETURN_CODE)"
ok "server is live"

# ---------- 1. Provision the tenant (idempotent) ----------
call POST /admin/api/tenants "{
  \"tenant_id\": \"$TENANT\",
  \"name\":      \"$TENANT_NAME\",
  \"plan\":      \"demo\",
  \"notes\":     \"Seeded by scripts/seed_demo.sh\"
}"
case "$RETURN_CODE" in
  200|201) ok "created tenant '$TENANT'" ;;
  409)     warn "tenant '$TENANT' already exists — reusing" ;;
  *)       fail "unexpected $RETURN_CODE from POST /admin/api/tenants: $RETURN_BODY" ;;
esac

# ---------- 2. Register logins (idempotent) ----------
# The tenant DB already ships (via migrations 017 + 019) with the standard
# role catalogue — admin/principal/teacher/accountant/librarian/student/
# guardian — and a baseline permission → role mapping. Assigning a role at
# registration time is therefore all we need to do here; RBAC filtering on
# the sidebar, dashboard tiles, and (soon) route guards picks it up.

register_user() {
  local user="$1" pass="$2" role="$3" email="$4"
  call POST "/api/$TENANT/auth/register" "{
    \"username\": \"$user\",
    \"email\":    \"$email\",
    \"password\": \"$pass\",
    \"roles\":    [\"$role\"]
  }"
  case "$RETURN_CODE" in
    200|201) ok "registered $role user '$user'" ;;
    409)     warn "user '$user' already exists — skipped" ;;
    *)       fail "register '$user' failed ($RETURN_CODE): $RETURN_BODY" ;;
  esac
}

# The primary admin account (name/pass driven by env vars — see top of file).
register_user "$ADMIN_USER" "$ADMIN_PASS" "admin" "$ADMIN_USER@$TENANT.example"

# A handful of role-scoped demo logins so you can eyeball the RBAC-filtered
# sidebar + dashboard tiles as any of these personas. All share the password
# `demo1234` for convenience — never do this outside dev / demo.
register_user "principal"  "demo1234" "principal"  "principal@$TENANT.example"
register_user "teacher"    "demo1234" "teacher"    "teacher@$TENANT.example"
register_user "accountant" "demo1234" "accountant" "accountant@$TENANT.example"
register_user "librarian"  "demo1234" "librarian"  "librarian@$TENANT.example"
register_user "parent"     "demo1234" "guardian"   "parent@$TENANT.example"
register_user "student1"   "demo1234" "student"    "student1@$TENANT.example"

# Quick login sanity check against the admin credentials.
call POST "/api/$TENANT/auth/login" "{
  \"identifier\": \"$ADMIN_USER\",
  \"password\":   \"$ADMIN_PASS\"
}"
[[ "$RETURN_CODE" == "200" ]] || fail "login failed for '$ADMIN_USER': $RETURN_BODY"
ok "verified admin login"

# ---------- 3. Hire staff ----------
bold "Hiring staff…"

hire_one() {
  local no="$1" first="$2" last="$3" designation="$4" etype="$5" phone="$6" email="$7"
  call POST "/api/$TENANT/people/staff" "{
    \"employee_no\":     \"$no\",
    \"first_name\":      \"$first\",
    \"last_name\":       \"$last\",
    \"designation\":     \"$designation\",
    \"employment_type\": \"$etype\",
    \"phone\":           \"$phone\",
    \"email\":           \"$email\",
    \"date_of_joining\": \"2024-04-01\"
  }"
  case "$RETURN_CODE" in
    200|201) ok "hired $first $last ($no)" ;;
    409)     warn "$no already exists — skipped" ;;
    *)       fail "hire $no failed ($RETURN_CODE): $RETURN_BODY" ;;
  esac
}

hire_one "EMP-DEMO-001" "Meera"  "Iyer"    "Principal"        "full-time" "+91-98100-00001" "meera.iyer@$TENANT.example"
hire_one "EMP-DEMO-002" "Rohit"  "Verma"   "Math Teacher"     "full-time" "+91-98100-00002" "rohit.verma@$TENANT.example"
hire_one "EMP-DEMO-003" "Sana"   "Ali"     "Science Teacher"  "full-time" "+91-98100-00003" "sana.ali@$TENANT.example"
hire_one "EMP-DEMO-004" "David"  "Thomas"  "Librarian"        "part-time" "+91-98100-00004" "david.thomas@$TENANT.example"
hire_one "EMP-DEMO-005" "Neha"   "Kapoor"  "Accountant"       "full-time" "+91-98100-00005" "neha.kapoor@$TENANT.example"
hire_one "EMP-DEMO-006" "Arjun"  "Nair"    "PE Teacher"       "full-time" "+91-98100-00006" "arjun.nair@$TENANT.example"
hire_one "EMP-DEMO-007" "Priya"  "Menon"   "English Teacher"  "full-time" "+91-98100-00007" "priya.menon@$TENANT.example"
hire_one "EMP-DEMO-008" "Amit"   "Gupta"   "IT Administrator" "full-time" "+91-98100-00008" "amit.gupta@$TENANT.example"

# ---------- 4. Admit students ----------
bold "Admitting students…"

admit_one() {
  local no="$1" first="$2" last="$3" dob="$4" gender="$5"
  call POST "/api/$TENANT/people/students/admit" "{
    \"student\": {
      \"admission_no\":  \"$no\",
      \"first_name\":    \"$first\",
      \"last_name\":     \"$last\",
      \"date_of_birth\": \"$dob\",
      \"gender\":        \"$gender\",
      \"admission_date\":\"2024-04-01\",
      \"nationality\":   \"Indian\",
      \"city\":          \"Mumbai\",
      \"state\":         \"MH\",
      \"country\":       \"India\"
    }
  }"
  case "$RETURN_CODE" in
    200|201) ok "admitted $first $last ($no)" ;;
    409)     warn "$no already exists — skipped" ;;
    *)       fail "admit $no failed ($RETURN_CODE): $RETURN_BODY" ;;
  esac
}

admit_one "ADM-DEMO-0001" "Aarav"   "Sharma"  "2012-03-15" "male"
admit_one "ADM-DEMO-0002" "Diya"    "Patel"   "2010-07-08" "female"
admit_one "ADM-DEMO-0003" "Kabir"   "Khan"    "2014-01-22" "male"
admit_one "ADM-DEMO-0004" "Ananya"  "Rao"     "2008-11-30" "female"
admit_one "ADM-DEMO-0005" "Vihaan"  "Mehta"   "2011-05-19" "male"
admit_one "ADM-DEMO-0006" "Isha"    "Bhatt"   "2013-09-04" "female"
admit_one "ADM-DEMO-0007" "Rohan"   "Desai"   "2009-02-14" "male"
admit_one "ADM-DEMO-0008" "Meera"   "Krishna" "2012-12-25" "female"
admit_one "ADM-DEMO-0009" "Yash"    "Joshi"   "2010-08-17" "male"
admit_one "ADM-DEMO-0010" "Sara"    "Fernandes" "2011-06-06" "female"

# ---------- 5. Done ----------
bold "Done."
ok "tenant '$TENANT' seeded"
echo
echo "  Sign in at:  $BASE_URL/web/login (or $BASE_URL/web/$TENANT/login)"
echo "  Tenant:      $TENANT"
echo
echo "  Role-scoped demo logins (all share password: demo1234)"
echo "    admin       — $ADMIN_USER / $ADMIN_PASS  (sees everything)"
echo "    principal   — principal   / demo1234     (most modules, no settings)"
echo "    teacher     — teacher     / demo1234     (students, attendance, marks)"
echo "    accountant  — accountant  / demo1234     (fees, payroll)"
echo "    librarian   — librarian   / demo1234     (library only)"
echo "    parent      — parent      / demo1234     (own child's views + pay fees)"
echo "    student1    — student1    / demo1234     (own attendance/fees/marks)"
echo
echo "  Sidebar and dashboard tiles will vary by role thanks to RBAC filtering."
echo
echo "  Tenant home: $BASE_URL/web/$TENANT/"
echo "  Students:    $BASE_URL/web/$TENANT/students"
echo "  Staff:       $BASE_URL/web/$TENANT/staff"
