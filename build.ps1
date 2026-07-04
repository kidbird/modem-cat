# build.ps1 — Modem Cat 多变体构建脚本 (PowerShell)
# 入口脚本, 取代 build.bat 让脚本也能在 PowerShell 环境下直接运行
#
# 产出 (全部在 dist/ 根目录):
#   dist\Modem Cat_<ver>_webview_*.msi/.exe        (Webview 安装包)
#   dist\Modem Cat_<ver>_nowebview_*.msi/.exe      (无 Webview 安装包)
#   dist\modem-cat.exe                              (便携版主程序)
#   dist\r26-cli-x86_64-pc-windows-msvc.exe         (固件下载 sidecar)

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
Write-Host "  所有产物统一输出到 dist/ 根目录:"
Write-Host "    MSI/NSIS 安装包 -> dist\\Modem Cat_*_[webview|nowebview]_x64_*.msi/.exe"
Write-Host "    便携版 ZIP      -> dist\\ModemCat_v*_portable_[webview|nowebview].zip"
Write-Host "    便携版主程序    -> dist\\modem-cat.exe"
Write-Host "    Sidecar         -> dist\\r26-cli-x86_64-pc-windows-msvc.exe"
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

# ── 4. Portable + sidecar ──────────────────────────────
Write-Host "[4/6] Portable build (static CRT) + sidecar..."
$env:RUSTFLAGS = "-C target-feature=+crt-static"
cargo build --release -p modem-cat
if ($LASTEXITCODE -ne 0) { $env:RUSTFLAGS = "" ; throw "portable build failed (exit $LASTEXITCODE)" }
$env:RUSTFLAGS = ""

$cfg = Get-Content "$root\src-tauri\tauri.conf.json" -Raw | ConvertFrom-Json
$ver = $cfg.version
$distDir = Join-Path $root "dist"
if (-not (Test-Path $distDir)) { New-Item -ItemType Directory -Path $distDir | Out-Null }

Copy-Item -LiteralPath "$root\target\release\modem-cat.exe" -Destination (Join-Path $distDir "modem-cat.exe") -Force
Write-Host "       dist\modem-cat.exe"

# Sidecar must keep its target-triple suffix
$sidecarSrc = Join-Path $root "src-tauri\binaries\r26-cli-x86_64-pc-windows-msvc.exe"
$sidecarDst = Join-Path $distDir "r26-cli-x86_64-pc-windows-msvc.exe"
if (Test-Path $sidecarSrc) {
    Copy-Item -LiteralPath $sidecarSrc -Destination $sidecarDst -Force
    Write-Host "       dist\r26-cli-x86_64-pc-windows-msvc.exe"
} else {
    Write-Warning "       sidecar not found at $sidecarSrc"
}

# ADB tools (from Sdk/ or resources/)
$adbFiles = @("adb.exe", "AdbWinApi.dll", "AdbWinUsbApi.dll")
foreach ($adb in $adbFiles) {
    $adbSrc = Join-Path $root "src-tauri\resources\adb\$adb"
    if (-not (Test-Path $adbSrc)) { $adbSrc = Join-Path $root "Sdk\$adb" }
    if (Test-Path $adbSrc) {
        Copy-Item -LiteralPath $adbSrc -Destination (Join-Path $distDir $adb) -Force
        Write-Host "       dist\$adb"
    }
}

# license-gen
$lg = Join-Path $root "target\release\license-gen.exe"
if (-not (Test-Path $lg)) { $lg = Join-Path $root "tools\license-gen\target\release\license-gen.exe" }
if (Test-Path $lg) {
    Copy-Item -LiteralPath $lg -Destination (Join-Path $distDir "license-gen.exe") -Force
    Write-Host "       dist\license-gen.exe"
}
Write-Host ""

# ── 5. Portable ZIPs (webview + nowebview) ────────────
Write-Host "[5/6] Portable ZIPs..."
$pWebview = Join-Path $env:TEMP "mc-pwv"
$pNowebview = Join-Path $env:TEMP "mc-pwnwv"
foreach ($d in @($pWebview, $pNowebview)) {
    if (Test-Path $d) { Remove-Item $d -Recurse -Force }
    New-Item -ItemType Directory -Path $d -Force | Out-Null
}
$portableFiles = @("modem-cat.exe", "r26-cli-x86_64-pc-windows-msvc.exe", "r26-cli.version.txt", "license-gen.exe", "adb.exe", "AdbWinApi.dll", "AdbWinUsbApi.dll")
foreach ($f in $portableFiles) {
    $src = Join-Path $distDir $f
    if (Test-Path $src) { Copy-Item $src $pWebview -Force; Copy-Item $src $pNowebview -Force }
}
$wvSrc = Join-Path $root "webview2-runtime"
if (Test-Path $wvSrc) { Copy-Item $wvSrc (Join-Path $pWebview "webview2-runtime") -Recurse -Force }

$zipW = Join-Path $distDir "ModemCat_v${ver}_portable_webview.zip"
$zipN = Join-Path $distDir "ModemCat_v${ver}_portable_nowebview.zip"
Get-ChildItem $distDir -Filter "ModemCat_v${ver}_portable*" | Remove-Item -Force -EA 0

Compress-Archive -Path "$pWebview\*" -DestinationPath $zipW -Force
$szW = [math]::Round((Get-Item $zipW).Length / 1MB, 1)
Write-Host "  $(Split-Path $zipW -Leaf) ($szW MB)"
Compress-Archive -Path "$pNowebview\*" -DestinationPath $zipN -Force
$szN = [math]::Round((Get-Item $zipN).Length / 1MB, 1)
Write-Host "  $(Split-Path $zipN -Leaf) ($szN MB)"
Remove-Item $pWebview -Recurse -Force
Remove-Item $pNowebview -Recurse -Force
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
