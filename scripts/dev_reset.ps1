# =============================================================================
# scripts/dev_reset.ps1  (PowerShell equivalent of dev_reset.sh)
# -----------------------------------------------------------------------------
# One command to blow away a Windows dev environment and rebuild it:
#
#   1. Stop any running `school-erp` server on the target port.
#   2. Delete the system DB and all tenant DBs (wipe `data/`).
#   3. (Re)start the server in the background.
#   4. Poll /api/live until ready.
#   5. Run seed_demo.sh (needs bash — Git Bash or WSL).
#   6. Print login details.
#
# Usage:
#   pwsh scripts/dev_reset.ps1
#   $env:PORT="4000"; pwsh scripts/dev_reset.ps1
#   $env:PROFILE="debug"; pwsh scripts/dev_reset.ps1
#   $env:SKIP_BUILD="1"; pwsh scripts/dev_reset.ps1
# =============================================================================
$ErrorActionPreference = 'Stop'

# ---------- config ---------------------------------------------------------
$Profile      = $(if ($env:PROFILE)     { $env:PROFILE }    else { 'release' })
$Port         = $(if ($env:PORT)        { $env:PORT }       else { '3000' })
$BaseUrl      = $(if ($env:BASE_URL)    { $env:BASE_URL }   else { "http://127.0.0.1:$Port" })
$DataDir      = $(if ($env:DATA_DIR)    { $env:DATA_DIR }   else { 'data' })
$SkipBuild    = ($env:SKIP_BUILD -eq '1')
$LogFile      = $(if ($env:LOG_FILE)    { $env:LOG_FILE }   else { '.dev_server.log' })
$PidFile      = $(if ($env:PID_FILE)    { $env:PID_FILE }   else { '.dev_server.pid' })

$RepoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $RepoRoot

function Bold($msg) { Write-Host "`n> $msg" -ForegroundColor Cyan }
function Ok($msg)   { Write-Host "  [ok] $msg" -ForegroundColor Green }
function Warn($msg) { Write-Host "  [!] $msg" -ForegroundColor Yellow }
function Fail($msg) { Write-Host "  [x] $msg" -ForegroundColor Red; exit 1 }

# ---------- 1. Stop any running server ------------------------------------
Bold "Stopping any running dev server"

# Kill by PID file.
if (Test-Path $PidFile) {
    $pidVal = Get-Content $PidFile -ErrorAction SilentlyContinue
    if ($pidVal) {
        try {
            Stop-Process -Id $pidVal -Force -ErrorAction Stop
            Ok "killed previous dev server (pid $pidVal)"
        } catch { }
    }
    Remove-Item $PidFile -Force -ErrorAction SilentlyContinue
}
# Kill by port.
$conns = Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue
foreach ($c in $conns) {
    try {
        Stop-Process -Id $c.OwningProcess -Force -ErrorAction Stop
        Ok "killed process holding port $Port (pid $($c.OwningProcess))"
    } catch { }
}

# ---------- 2. Nuke the data dir ------------------------------------------
Bold "Wiping database files under '$DataDir/'"
if (Test-Path $DataDir) {
    Get-ChildItem -Path $DataDir -Recurse -File -Include *.db,*.db-wal,*.db-shm |
        ForEach-Object {
            Write-Host "    deleted: $($_.FullName)"
            Remove-Item -Force $_.FullName
        }
    if (Test-Path (Join-Path $DataDir 'tenants')) {
        Remove-Item -Recurse -Force (Join-Path $DataDir 'tenants')
    }
    Ok "cleared $DataDir/"
} else {
    Warn "no existing $DataDir/ — skipping wipe"
}
New-Item -ItemType Directory -Force -Path (Join-Path $DataDir 'tenants') | Out-Null
if (Test-Path $LogFile) { Remove-Item -Force $LogFile }

# ---------- 3. Build (unless skipped) -------------------------------------
if (-not $SkipBuild) {
    Bold "Building server ($Profile profile)"
    if ($Profile -eq 'release') { cargo build --release }
    else                         { cargo build }
    if ($LASTEXITCODE -ne 0) { Fail "cargo build failed" }
    Ok "build complete"
} else {
    Warn "SKIP_BUILD=1 — assuming existing binary is current"
}

$Bin = Join-Path 'target' (Join-Path $Profile 'school_erp.exe')
if (-not (Test-Path $Bin)) { Fail "server binary not found at $Bin" }

# ---------- 4. Start server -----------------------------------------------
Bold "Starting server on $BaseUrl"
$env:BIND = "0.0.0.0:$Port"
$proc = Start-Process -FilePath $Bin `
                      -RedirectStandardOutput $LogFile `
                      -RedirectStandardError  $LogFile `
                      -PassThru -WindowStyle Hidden
$proc.Id | Out-File -FilePath $PidFile -Encoding ascii
Ok "started server (pid $($proc.Id), log: $LogFile)"

# ---------- 5. Wait for /api/live -----------------------------------------
Bold "Waiting for server to become live"
$attempts = 60
for ($i=1; $i -le $attempts; $i++) {
    try {
        Invoke-RestMethod "$BaseUrl/api/live" -TimeoutSec 2 | Out-Null
        Ok "server is live (after ${i}s)"
        break
    } catch { }
    if ($proc.HasExited) {
        Write-Host "`n--- $LogFile (last 40 lines) ---"
        Get-Content $LogFile -Tail 40
        Fail "server exited before becoming live"
    }
    Start-Sleep -Seconds 1
    if ($i -eq $attempts) { Fail "server didn't become live within ${attempts}s (see $LogFile)" }
}

# ---------- 6. Seed demo tenant -------------------------------------------
Bold "Seeding demo tenant"
$env:BASE_URL = $BaseUrl
bash "scripts/seed_demo.sh"
if ($LASTEXITCODE -ne 0) { Fail "seed_demo.sh failed" }

# ---------- 7. Summary ----------------------------------------------------
Bold "Done"
Ok "Dev environment reset and seeded."
Write-Host ""
Write-Host "  Server:      $BaseUrl           (pid $($proc.Id), log: $LogFile)"
Write-Host "  Tenant home: $BaseUrl/web/demo/"
Write-Host "  Login page:  $BaseUrl/web/login"
Write-Host ""
Write-Host "  Stop the server later with:  Stop-Process -Id (Get-Content $PidFile)"
Write-Host "  Tail logs with:              Get-Content -Tail 40 -Wait $LogFile"
