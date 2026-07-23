#!/usr/bin/env bash
# =============================================================================
# scripts/dev_reset.sh
# -----------------------------------------------------------------------------
# One command to blow away a dev environment and rebuild it from scratch:
#
#   1. Stop any running `school-erp` server on the target port.
#   2. Delete the system DB and all tenant DBs (i.e. wipe `data/`).
#   3. (Re)start the server in the background so it re-creates the system DB
#      and applies all migrations.
#   4. Poll `/api/live` until the server is ready.
#   5. Run `seed_demo.sh` to provision the demo tenant, register the role-
#      scoped demo users, and admit/hire sample students + staff.
#   6. Print login details and leave the server running.
#
# Everything is idempotent; safe to re-run at any time.
#
# ---------------------------------------------------------------------------
# Usage
# ---------------------------------------------------------------------------
#   bash scripts/dev_reset.sh                    # release build, port 3000
#   PROFILE=debug bash scripts/dev_reset.sh      # skip --release for faster
#                                                # incremental rebuilds
#   PORT=4000 bash scripts/dev_reset.sh          # different port
#   SKIP_BUILD=1 bash scripts/dev_reset.sh       # assume binary is up-to-date
#   FOREGROUND=1 bash scripts/dev_reset.sh       # leave the server attached
#                                                # to the current terminal
#
# ---------------------------------------------------------------------------
# Env vars
# ---------------------------------------------------------------------------
#   PROFILE=release|debug   Cargo profile to run   (default: release)
#   PORT=3000               Port the server binds  (default: 3000)
#   BASE_URL=…              Explicit URL for seeder
#                           (default: http://127.0.0.1:$PORT)
#   DATA_DIR=data           Dir to wipe            (default: data)
#   SKIP_BUILD=0|1          Skip `cargo build`     (default: 0)
#   FOREGROUND=0|1          Keep server in fg      (default: 0)
# =============================================================================
set -euo pipefail

# ---------- config ---------------------------------------------------------
PROFILE="${PROFILE:-release}"
PORT="${PORT:-3000}"
BASE_URL="${BASE_URL:-http://127.0.0.1:$PORT}"
DATA_DIR="${DATA_DIR:-data}"
SKIP_BUILD="${SKIP_BUILD:-0}"
FOREGROUND="${FOREGROUND:-0}"
LOG_FILE="${LOG_FILE:-.dev_server.log}"
PID_FILE="${PID_FILE:-.dev_server.pid}"

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

# ---------- pretty printing -----------------------------------------------
bold()  { printf '\n\033[1;36m»\033[0m \033[1m%s\033[0m\n' "$*"; }
ok()    { printf '  \033[32m✔\033[0m %s\n' "$*"; }
warn()  { printf '  \033[33m•\033[0m %s\n' "$*"; }
fail()  { printf '  \033[31m✘\033[0m %s\n' "$*" >&2; exit 1; }

# ---------- 1. Stop any running server ------------------------------------
bold "Stopping any running dev server"

kill_by_pidfile() {
  if [[ -f "$PID_FILE" ]]; then
    local pid
    pid="$(cat "$PID_FILE" || true)"
    if [[ -n "$pid" ]] && kill -0 "$pid" 2>/dev/null; then
      kill "$pid" 2>/dev/null || true
      sleep 1
      kill -9 "$pid" 2>/dev/null || true
      ok "killed previous dev server (pid $pid)"
    fi
    rm -f "$PID_FILE"
  fi
}
kill_by_port() {
  # Best-effort — different OSes ship different tools. We try lsof, then ss,
  # then fuser; if none of them are present we just warn.
  if command -v lsof >/dev/null 2>&1; then
    local pids
    pids="$(lsof -tiTCP:"$PORT" -sTCP:LISTEN 2>/dev/null || true)"
    for p in $pids; do
      kill "$p" 2>/dev/null || true
      sleep 1
      kill -9 "$p" 2>/dev/null || true
      ok "killed process holding port $PORT (pid $p)"
    done
  elif command -v fuser >/dev/null 2>&1; then
    fuser -k "$PORT"/tcp 2>/dev/null || true
  else
    warn "no lsof/fuser found — can't check if port $PORT is free"
  fi
}
kill_by_pidfile
kill_by_port

# ---------- 2. Nuke the data dir ------------------------------------------
bold "Wiping database files under '$DATA_DIR/'"

if [[ -d "$DATA_DIR" ]]; then
  # Remove SQLite files and any WAL / SHM sidecars, but keep the top-level
  # data/ directory itself (main.rs re-creates it anyway).
  find "$DATA_DIR" -type f \( -name '*.db' -o -name '*.db-wal' -o -name '*.db-shm' \) \
       -print -delete | sed 's/^/    deleted: /' || true
  # Also blast the tenants subdir wholesale (small and self-recreating).
  rm -rf "$DATA_DIR/tenants"
  ok "cleared $DATA_DIR/"
else
  warn "no existing $DATA_DIR/ — skipping wipe"
fi
mkdir -p "$DATA_DIR/tenants"

# Also remove the old background log if we're re-running.
rm -f "$LOG_FILE"

# ---------- 3. Build (unless skipped) -------------------------------------
if [[ "$SKIP_BUILD" != "1" ]]; then
  bold "Building server ($PROFILE profile)"
  if [[ "$PROFILE" == "release" ]]; then
    cargo build --release
  else
    cargo build
  fi
  ok "build complete"
else
  warn "SKIP_BUILD=1 — assuming existing binary is current"
fi

BIN="target/$PROFILE/school-erp"
[[ -x "$BIN" ]] || fail "server binary not found at $BIN — build failed?"

# ---------- 4. Start server -----------------------------------------------
bold "Starting server on $BASE_URL"

export BIND="0.0.0.0:$PORT"

if [[ "$FOREGROUND" == "1" ]]; then
  ok "FOREGROUND=1 — server will run in this terminal; open another shell to run the seeder"
  exec "$BIN"
fi

# Background mode: capture stdout+stderr and PID.
nohup "$BIN" >"$LOG_FILE" 2>&1 &
SERVER_PID=$!
echo "$SERVER_PID" > "$PID_FILE"
ok "started server (pid $SERVER_PID, log: $LOG_FILE)"

# ---------- 5. Wait for /api/live -----------------------------------------
bold "Waiting for server to become live"
ATTEMPTS=60
for i in $(seq 1 $ATTEMPTS); do
  if curl -sf "$BASE_URL/api/live" >/dev/null 2>&1; then
    ok "server is live (after ${i}s)"
    break
  fi
  # If the process died during startup, fail fast with the tail of the log.
  if ! kill -0 "$SERVER_PID" 2>/dev/null; then
    echo
    echo "--- $LOG_FILE (last 40 lines) ---"
    tail -n 40 "$LOG_FILE" || true
    fail "server exited before becoming live"
  fi
  sleep 1
  if [[ "$i" == "$ATTEMPTS" ]]; then
    fail "server didn't become live within ${ATTEMPTS}s (see $LOG_FILE)"
  fi
done

# ---------- 6. Seed demo tenant -------------------------------------------
bold "Seeding demo tenant"
BASE_URL="$BASE_URL" bash "$REPO_ROOT/scripts/seed_demo.sh"

# ---------- 7. Summary ----------------------------------------------------
bold "Done"
ok "Dev environment reset and seeded."
echo
echo "  Server:      $BASE_URL           (pid $SERVER_PID, log: $LOG_FILE)"
echo "  Tenant home: $BASE_URL/web/demo/"
echo "  Login page:  $BASE_URL/web/login"
echo
echo "  Stop the server later with:  kill \$(cat $PID_FILE)"
echo "  Tail logs with:              tail -f $LOG_FILE"
