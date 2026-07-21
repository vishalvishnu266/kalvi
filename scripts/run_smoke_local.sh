#!/usr/bin/env bash
# Convenience runner for the E2E smoke test against a locally running server.
#
# Usage:
#   bash scripts/run_smoke_local.sh
#
# This just sets sensible defaults for a local dev machine and delegates to
# `scripts/e2e_smoke.sh`. Any env var set before invoking this script wins,
# so you can still override individual settings:
#
#   BASE_URL=http://127.0.0.1:4000 bash scripts/run_smoke_local.sh
#   ADMIN_PASS='s3cret!'           bash scripts/run_smoke_local.sh

set -Eeuo pipefail

# Resolve script directory so this works no matter where you run it from.
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# ---- Defaults (only applied if the caller hasn't already set them) ----
: "${BASE_URL:=http://127.0.0.1:3000}"
: "${ACME:=acme}"
: "${GLOBEX:=globex}"
: "${ADMIN_USER:=admin}"
: "${ADMIN_PASS:=admin123}"

export BASE_URL ACME GLOBEX ADMIN_USER ADMIN_PASS

printf '\033[1mRunning smoke test against %s\033[0m\n' "$BASE_URL"
printf '  tenants : %s, %s\n' "$ACME" "$GLOBEX"
printf '  admin   : %s / %s\n\n' "$ADMIN_USER" "$ADMIN_PASS"

# Quick pre-flight so we don't hand a cryptic curl error to the user.
if ! curl -sSf "$BASE_URL/api/live" >/dev/null 2>&1; then
  printf '\033[31m✘ Server not reachable at %s\033[0m\n' "$BASE_URL" >&2
  printf '  Start it first with:  cargo run\n' >&2
  exit 1
fi

bash "$SCRIPT_DIR/e2e_smoke.sh"
