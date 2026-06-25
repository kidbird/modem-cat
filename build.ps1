# build.ps1 — Modem Cat 多变体构建脚本 (PowerShell)
# 入口脚本, 取代 build.bat 让脚本也能在 PowerShell 环境下直接运行
#
# 产出:
#   dist\installer\Modem Cat_<ver>_webview_*.msi/.exe
#   dist\installer\Modem Cat_<ver>_nowebview_*.msi/.exe
#   dist\portable\Modem Cat_<ver>_portable\  (modem-cat.exe + r26-cli sidecar)
#   dist\license-gen.exe

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
Push-Location $root

function Test-Vcvars {
    $candidates = @(
        "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
        "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
        "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat"
        "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvarsall.bat"
        "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvarsall.bat"
    )
    foreach ($p in $candidates) {
        if (Test-Path $p) { return $p }
    }
    return $null
}

Write-Host ""
Write-Host " ==================================================="
Write-Host "  Modem Cat  -  Multi-Variant Build"
Write-Host ""
Write-Host "  installer  ->  dist\installer\Modem Cat_*_webview_*.msi/.exe"
Write-Host "                 dist\installer\Modem Cat_*_nowebview_*.msi/.exe"
Write-Host "  portable   ->  dist\portable\Modem Cat_*_portable\"
Write-Host "                    (modem-cat.exe + r26-cli sidecar)"
Write-Host "  license    ->  dist\license-gen.exe"
Write-Host " ==================================================="
Write-Host ""

# ── 1. VS env ──────────────────────────────────────────
$vcvars = Test-Vcvars
if ($vcvars) {
    Write-Host "[1/6] VS env: $vcvars"
    cmd.exe /c "call `"$vcvars`" x64 >nul 2>&1 && set" | ForEach-Object {
        if ($_ -match '^([^=]+)=(.*)$') {
            $name = $matches[1]
            $value = $matches[2]
            [System.Environment]::SetEnvironmentVariable($name, $value, 'Process')
        }
    }
    Write-Host "       OK"
} else {
    Write-Host "[1/6] VS env: vcvarsall.bat not found - proceeding without it"
}
Write-Host ""

# ── 2. Webview installer ───────────────────────────────
Write-Host "[2/6] Webview installer (offline WebView2 bundle)..."
$tauriCli = $null
try { $tauriCli = (Get-Command "cargo-tauri" -ErrorAction SilentlyContinue) ; if (-not $tauriCli) { cargo tauri --version | Out-Null } } catch {}
$tauriOk = $LASTEXITCODE -eq 0
if (-not $tauriOk) {
    Write-Host "       SKIP - tauri-cli not installed"
    Write-Host "       To install: cargo install tauri-cli --version ^2 --locked"
} else {
    & powershell -NoProfile -ExecutionPolicy Bypass -File "$root\scripts\build-helper.ps1" -Variant webview
    if ($LASTEXITCODE -ne 0) { throw "webview installer build failed (exit $LASTEXITCODE)" }
}
Write-Host ""

# ── 3. No-webview installer ────────────────────────────
Write-Host "[3/6] No-webview installer (online WebView2 download)..."
if ($vcvars) {
    & powershell -NoProfile -ExecutionPolicy Bypass -File "$root\scripts\build-helper.ps1" -Variant nowebview
    if ($LASTEXITCODE -ne 0) { throw "no-webview installer build failed (exit $LASTEXITCODE)" }
} else {
    Write-Host "       SKIP - VS env missing, would fail to link"
}
Write-Host ""

# ── 4. license-gen ─────────────────────────────────────
Write-Host "[4/6] license-gen build..."
# license-gen is its own workspace; build from inside it
Push-Location "$root\tools\license-gen"
try {
    cargo build --release -p license-gen
    if ($LASTEXITCODE -ne 0) { throw "license-gen build failed (exit $LASTEXITCODE)" }
} finally {
    Pop-Location
}
$distDir = Join-Path $root "dist"
if (-not (Test-Path $distDir)) { New-Item -ItemType Directory -Path $distDir | Out-Null }
# license-gen 产物在 tools\license-gen\target\release\
Copy-Item -LiteralPath "$root\tools\license-gen\target\release\license-gen.exe" -Destination (Join-Path $distDir "license-gen.exe") -Force
Write-Host "       dist\license-gen.exe"
Write-Host ""

# ── 5. Portable + sidecar ──────────────────────────────
Write-Host "[5/6] Portable build (static CRT) + sidecar..."
$env:RUSTFLAGS = "-C target-feature=+crt-static"
cargo build --release -p modem-cat
if ($LASTEXITCODE -ne 0) { $env:RUSTFLAGS = "" ; throw "portable build failed (exit $LASTEXITCODE)" }
$env:RUSTFLAGS = ""

$cfg = Get-Content "$root\src-tauri\tauri.conf.json" -Raw | ConvertFrom-Json
$ver = $cfg.version
$pfDir = Join-Path $distDir "portable\Modem Cat_${ver}_portable"
if (-not (Test-Path $pfDir)) { New-Item -ItemType Directory -Path $pfDir -Force | Out-Null }

Copy-Item -LiteralPath "$root\target\release\modem-cat.exe" -Destination (Join-Path $pfDir "modem-cat.exe") -Force
Write-Host "       $pfDir\modem-cat.exe"

# Sidecar must keep its target-triple suffix
$sidecarSrc = Join-Path $root "src-tauri\binaries\r26-cli-x86_64-pc-windows-msvc.exe"
$sidecarDst = Join-Path $pfDir "r26-cli-x86_64-pc-windows-msvc.exe"
if (Test-Path $sidecarSrc) {
    Copy-Item -LiteralPath $sidecarSrc -Destination $sidecarDst -Force
    Write-Host "       $sidecarDst"
} else {
    Write-Warning "       sidecar not found at $sidecarSrc"
}
Write-Host ""

# ── 6. Summary ─────────────────────────────────────────
Write-Host "[6/6] Output:"
Write-Host ""
Get-ChildItem $distDir -Recurse -File -ErrorAction SilentlyContinue | Sort-Object FullName | ForEach-Object {
    $rel = $_.FullName.Substring($distDir.Length).TrimStart('\')
    if ($_.Length -ge 1MB) { $s = "{0:N1} MB" -f ($_.Length/1MB) }
    elseif ($_.Length -ge 1KB) { $s = "{0:N0} KB" -f ($_.Length/1KB) }
    else { $s = "{0} B" -f $_.Length }
    Write-Host ("  {0}  ({1})" -f $rel, $s)
}
Write-Host ""
Write-Host " ==================================================="
Write-Host "  Build complete!"
Write-Host " ==================================================="
Write-Host ""

Pop-Location
