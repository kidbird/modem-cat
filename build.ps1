# build.ps1 — Modem Cat 多变体构建脚本 (PowerShell)
# 入口脚本, 取代 build.bat 让脚本也能在 PowerShell 环境下直接运行
#
# 产出 (全部在 dist/ 根目录):
#   dist\Modem Cat_<ver>_webview_*.msi/.exe        (Webview 安装包)
#   dist\Modem Cat_<ver>_nowebview_*.msi/.exe      (无 Webview 安装包)
#   dist\ModemCat_v<ver>_portable.zip               (完整便携包, 含 fixed WebView2)
#   dist\ModemCat_v<ver>_portable-lite.zip          (轻量便携包, 依赖系统 WebView2)
#   dist\modem-cat.exe                              (便携版主程序)
#   dist\r26-cli-x86_64-pc-windows-msvc.exe         (固件下载 sidecar)
#   dist\vcruntime140.dll                           (r26 sidecar 的 x86 VC 运行库)

param(
    [switch]$Quick
)

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
Push-Location $root

function Get-NonEmptyDirectory([string]$Path) {
    if (-not (Test-Path $Path)) {
        return $false
    }
    return ((Get-ChildItem $Path -Recurse -File -ErrorAction SilentlyContinue | Measure-Object).Count -gt 0)
}

function Sync-Webview2Runtime {
    $stagedRuntime = Join-Path $root "src-tauri\webview2-runtime"
    $legacyRuntime = Join-Path $root "webview2-runtime"

    if (Get-NonEmptyDirectory $stagedRuntime) {
        return $stagedRuntime
    }

    if (Get-NonEmptyDirectory $legacyRuntime) {
        if (Test-Path $stagedRuntime) {
            Remove-Item -LiteralPath $stagedRuntime -Recurse -Force
        }
        New-Item -ItemType Directory -Path $stagedRuntime -Force | Out-Null
        Copy-Item -Path (Join-Path $legacyRuntime "*") -Destination $stagedRuntime -Recurse -Force
        Write-Host "       staged fixed WebView2 runtime from legacy root cache"
        return $stagedRuntime
    }

    throw "fixed WebView2 runtime not found. Expected src-tauri\\webview2-runtime or legacy webview2-runtime. Run scripts\\setup-webview2.ps1 first."
}

function Find-R26RuntimeDll {
    $exactCandidates = @(
        "C:\Windows\SysWOW64\vcruntime140.dll"
    )
    foreach ($candidate in $exactCandidates) {
        if (Test-Path $candidate) {
            return $candidate
        }
    }

    $globCandidates = @(
        "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Redist\MSVC\*\x86\Microsoft.VC143.CRT\vcruntime140.dll",
        "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Redist\MSVC\*\x86\Microsoft.VC143.CRT\vcruntime140.dll",
        "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Redist\MSVC\*\x86\Microsoft.VC143.CRT\vcruntime140.dll",
        "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Redist\MSVC\*\x86\Microsoft.VC143.CRT\vcruntime140.dll",
        "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Redist\MSVC\*\x86\Microsoft.VC143.CRT\vcruntime140.dll"
    )
    foreach ($pattern in $globCandidates) {
        $match = Get-ChildItem -Path $pattern -File -ErrorAction SilentlyContinue |
            Sort-Object FullName -Descending |
            Select-Object -First 1
        if ($match) {
            return $match.FullName
        }
    }

    throw "x86 vcruntime140.dll not found. Install the VC++ x86 runtime or Visual Studio x86 redist before packaging the r26 sidecar."
}

function Sync-R26Runtime {
    $runtimeDir = Join-Path $root "src-tauri\resources\r26-runtime"
    $runtimeSrc = Find-R26RuntimeDll
    $runtimeDst = Join-Path $runtimeDir "vcruntime140.dll"
    if (-not (Test-Path $runtimeDir)) {
        New-Item -ItemType Directory -Path $runtimeDir -Force | Out-Null
    }
    Copy-Item -LiteralPath $runtimeSrc -Destination $runtimeDst -Force
    Write-Host "       staged x86 r26 runtime: $runtimeSrc"
    return $runtimeDst
}

function Remove-StaleDistArtifacts([string]$DistDir, [string]$Version) {
    if (-not (Test-Path $DistDir)) {
        return
    }

    $cleanupPatterns = @(
        "modem-cat.zip",
        "ModemCat_v*_portable_webview.zip",
        "ModemCat_v*_portable_nowebview.zip",
        "ModemCat_v*_portable.zip",
        "ModemCat_v*_portable-lite.zip",
        "Modem Cat_*_webview_x64_*.msi",
        "Modem Cat_*_webview_x64-setup.exe",
        "Modem Cat_*_nowebview_x64_*.msi",
        "Modem Cat_*_nowebview_x64-setup.exe"
    )

    foreach ($pattern in $cleanupPatterns) {
        Get-ChildItem $DistDir -Filter $pattern -File -ErrorAction SilentlyContinue | ForEach-Object {
            if ($_.Name -notmatch [regex]::Escape($Version)) {
                Remove-Item -LiteralPath $_.FullName -Force -ErrorAction SilentlyContinue
            }
        }
    }

    $legacyPortable = Join-Path $DistDir "modem-cat.zip"
    if (Test-Path $legacyPortable) {
        Remove-Item -LiteralPath $legacyPortable -Force -ErrorAction SilentlyContinue
    }

    $distRuntimeDir = Join-Path $DistDir "webview2-runtime"
    if (Test-Path $distRuntimeDir) {
        Remove-Item -LiteralPath $distRuntimeDir -Recurse -Force -ErrorAction SilentlyContinue
    }

    $distR26Runtime = Join-Path $DistDir "vcruntime140.dll"
    if (Test-Path $distR26Runtime) {
        Remove-Item -LiteralPath $distR26Runtime -Force -ErrorAction SilentlyContinue
    }
}

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
Write-Host "    便携版 ZIP      -> dist\\ModemCat_v*_portable.zip / portable-lite.zip"
Write-Host "    便携版主程序    -> dist\\modem-cat.exe"
Write-Host "    Sidecar         -> dist\\r26-cli-x86_64-pc-windows-msvc.exe"
Write-Host "    Sidecar runtime -> dist\\vcruntime140.dll"
if ($Quick) {
    Write-Host "    Quick 模式      -> 跳过 portable ZIP 打包"
}
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
$stagedWebview2 = $null
$stagedR26Runtime = $null
try { $tauriCli = (Get-Command "cargo-tauri" -ErrorAction SilentlyContinue) ; if (-not $tauriCli) { cargo tauri --version | Out-Null } } catch {}
$tauriOk = $LASTEXITCODE -eq 0
if (-not $tauriOk) {
    Write-Host "       SKIP - tauri-cli not installed"
    Write-Host "       To install: cargo install tauri-cli --version ^2 --locked"
} else {
    $stagedWebview2 = Sync-Webview2Runtime
    $stagedR26Runtime = Sync-R26Runtime
    Write-Host "       using fixed runtime: $stagedWebview2"
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
Remove-StaleDistArtifacts -DistDir $distDir -Version $ver
if (-not $stagedWebview2) {
    $stagedWebview2 = Sync-Webview2Runtime
}
if (-not $stagedR26Runtime) {
    $stagedR26Runtime = Sync-R26Runtime
}

Copy-Item -LiteralPath "$root\target\release\modem-cat.exe" -Destination (Join-Path $distDir "modem-cat.exe") -Force
Write-Host "       dist\modem-cat.exe"
$distRuntimeDir = Join-Path $distDir "webview2-runtime"
Copy-Item -LiteralPath $stagedWebview2 -Destination $distRuntimeDir -Recurse -Force
Write-Host "       dist\webview2-runtime\"

# Sidecar must keep its target-triple suffix
$sidecarSrc = Join-Path $root "src-tauri\binaries\r26-cli-x86_64-pc-windows-msvc.exe"
$sidecarDst = Join-Path $distDir "r26-cli-x86_64-pc-windows-msvc.exe"
if (Test-Path $sidecarSrc) {
    Copy-Item -LiteralPath $sidecarSrc -Destination $sidecarDst -Force
    Write-Host "       dist\r26-cli-x86_64-pc-windows-msvc.exe"
} else {
    Write-Warning "       sidecar not found at $sidecarSrc"
}
$sidecarRuntimeDst = Join-Path $distDir "vcruntime140.dll"
Copy-Item -LiteralPath $stagedR26Runtime -Destination $sidecarRuntimeDst -Force
Write-Host "       dist\vcruntime140.dll"

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

Write-Host ""

# ── 5. Portable ZIPs (webview + nowebview) ────────────
Write-Host "[5/6] Portable ZIPs..."
if ($Quick) {
    Write-Host "       SKIP - Quick mode"
} else {
    if (-not $stagedWebview2) {
        $stagedWebview2 = Sync-Webview2Runtime
    }
    $pFull = Join-Path $env:TEMP "mc-pfull"
    $pLite = Join-Path $env:TEMP "mc-plite"
    foreach ($d in @($pFull, $pLite)) {
        if (Test-Path $d) { Remove-Item $d -Recurse -Force }
        New-Item -ItemType Directory -Path $d -Force | Out-Null
    }
    $portableFiles = @("modem-cat.exe", "r26-cli-x86_64-pc-windows-msvc.exe", "r26-cli.version.txt", "vcruntime140.dll", "adb.exe", "AdbWinApi.dll", "AdbWinUsbApi.dll")
    foreach ($f in $portableFiles) {
        $src = Join-Path $distDir $f
        if (Test-Path $src) {
            Copy-Item $src $pFull -Force
            Copy-Item $src $pLite -Force
        }
    }
    Copy-Item -LiteralPath $stagedWebview2 -Destination (Join-Path $pFull "webview2-runtime") -Recurse -Force

    $zipFull = Join-Path $distDir "ModemCat_v${ver}_portable.zip"
    $zipLite = Join-Path $distDir "ModemCat_v${ver}_portable-lite.zip"
    Get-ChildItem $distDir -Filter "ModemCat_v${ver}_portable*" | Remove-Item -Force -EA 0

    Compress-Archive -Path "$pFull\*" -DestinationPath $zipFull -Force
    $szFull = [math]::Round((Get-Item $zipFull).Length / 1MB, 1)
    Write-Host "  $(Split-Path $zipFull -Leaf) ($szFull MB)"
    Compress-Archive -Path "$pLite\*" -DestinationPath $zipLite -Force
    $szLite = [math]::Round((Get-Item $zipLite).Length / 1MB, 1)
    Write-Host "  $(Split-Path $zipLite -Leaf) ($szLite MB)"
    Remove-Item $pFull -Recurse -Force
    Remove-Item $pLite -Recurse -Force
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
