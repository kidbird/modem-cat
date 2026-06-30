#requires -Version 5.1
<#
.SYNOPSIS
    Modem Cat — Complete release build script (Windows).

.DESCRIPTION
    Builds portable exe + MSI installer + NSIS installer, bundles r26-cli sidecar,
    license-gen, and WebView2 runtime. Outputs everything flat in dist/ root.

.PARAMETERS
    None. Version is read from tauri.conf.json.

.EXAMPLE
    powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-release.ps1
#>

param()

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$root = Split-Path -Parent $PSScriptRoot
$dist = Join-Path $root "dist"

# == Read version from tauri.conf.json ==
$cfg = Get-Content "$root\src-tauri\tauri.conf.json" -Raw | ConvertFrom-Json
$ver = $cfg.version
Write-Host ""
Write-Host "  Modem Cat v$ver -- Release Build"
Write-Host "  root: $root"
Write-Host ""

# == Step 1: Clean dist/ ==
Write-Host "[1/7] Cleaning dist/ ..."
if (Test-Path $dist) {
    # Dist files may be locked (e.g. portable exe running). Move-then-delete avoids
    # "access denied" on locked files.
    $trash = Join-Path $env:TEMP "modem-cat-dist-trash-$(Get-Random)"
    Move-Item $dist $trash -Force -ErrorAction SilentlyContinue
    if (Test-Path $trash) {
        Remove-Item $trash -Recurse -Force -ErrorAction SilentlyContinue
    }
    Start-Sleep -Milliseconds 500
}
New-Item -ItemType Directory -Path $dist -Force | Out-Null
Write-Host "  [OK] dist/ cleaned"

# == Step 2: Toolchain checks ==
Write-Host "[2/7] Toolchain checks ..."

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargo) { throw "cargo not found. Install Rust: https://rustup.rs" }
Write-Host "  [OK] cargo: $(cargo --version)"

$cargoTauri = Get-Command cargo-tauri -ErrorAction SilentlyContinue
if (-not $cargoTauri) {
    Write-Host "  [INFO] Installing cargo-tauri ..."
    cargo install tauri-cli --version "^2" --locked
}
Write-Host "  [OK] tauri-cli: $(cargo tauri --version 2>&1 | Select-Object -First 1)"

# MSVC
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $vswhere)) { $vswhere = "$env:ProgramFiles\Microsoft Visual Studio\Installer\vswhere.exe" }
if (-not (Test-Path $vswhere)) { throw "vswhere not found. Install Visual Studio" }

$vcvarsall = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -find "VC\Auxiliary\Build\vcvarsall.bat" 2>$null
if (-not $vcvarsall) { throw "MSVC C++ toolchain not found" }
Write-Host "  [OK] MSVC: $vcvarsall"

# WebView2 runtime (must exist — no download)
$webview2 = Join-Path $root "webview2-runtime"
if (-not (Test-Path $webview2)) {
    throw "webview2-runtime/ not found. Run: scripts\setup-webview2.ps1"
}
$wvCount = (Get-ChildItem $webview2 -Recurse -File -ErrorAction SilentlyContinue | Measure-Object).Count
Write-Host "  [OK] webview2-runtime: $wvCount files"

# == Step 3: Build Tauri ==
Write-Host "[3/7] Building Tauri (portable + installers) ..."

$vcvarsallPath = $vcvarsall -replace '"',''
cmd /c "`"$vcvarsallPath`" x64 && set" | ForEach-Object {
    if ($_ -match "^(.*?)=(.*)$") {
        Set-Item -Force "env:\$($matches[1])" $matches[2]
    }
}

$env:NO_PROXY = "go.microsoft.com,*.microsoft.com"
Push-Location "$root\src-tauri"
try {
    cargo tauri build
    if ($LASTEXITCODE -ne 0) { throw "cargo tauri build failed" }
} finally {
    Pop-Location
}
Write-Host "  [OK] Tauri build done"

# == Step 4: r26-cli sidecar (pre-built expected) ==
Write-Host "[4/7] Checking r26-cli sidecar ..."
$r26Src = "$root\src-tauri\binaries\r26-cli-x86_64-pc-windows-msvc.exe"
if (Test-Path $r26Src) {
    Write-Host "  [OK] sidecar found in binaries/"
} else {
    Write-Warning "r26-cli sidecar not found in binaries/ — firmware download will not be available"
}

# == Step 5: Build license-gen ==
Write-Host "[5/7] Building license-gen ..."
$licGenDir = Join-Path $root "tools\license-gen"
if (Test-Path $licGenDir) {
    Push-Location $licGenDir
    try {
        cargo build --release -p license-gen
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  [OK] license-gen built"
        } else {
            Write-Warning "license-gen build failed"
        }
    } finally { Pop-Location }
} else {
    Write-Warning "tools/license-gen not found"
}

# == Step 6: Copy artifacts to dist/ root ==
Write-Host "[6/7] Copying artifacts to dist/ ..."

# Portable exe
$exeSrc = "$root\target\release\modem-cat.exe"
if (Test-Path $exeSrc) {
    Copy-Item $exeSrc (Join-Path $dist "modem-cat.exe") -Force
    Write-Host "  [OK] modem-cat.exe"
}

# MSI installer
$msiDir = "$root\target\release\bundle\msi"
if (Test-Path $msiDir) {
    Get-ChildItem $msiDir -Filter "*.msi" -Recurse | ForEach-Object {
        Copy-Item $_.FullName (Join-Path $dist $_.Name) -Force
        Write-Host "  [OK] $($_.Name)"
    }
}

# NSIS installer
$nsisDir = "$root\target\release\bundle\nsis"
if (Test-Path $nsisDir) {
    Get-ChildItem $nsisDir -Filter "*.exe" -Recurse | ForEach-Object {
        Copy-Item $_.FullName (Join-Path $dist $_.Name) -Force
        Write-Host "  [OK] $($_.Name)"
    }
}

# r26-cli sidecar
if (Test-Path $r26Src) {
    Copy-Item $r26Src (Join-Path $dist "r26-cli-x86_64-pc-windows-msvc.exe") -Force
    Write-Host "  [OK] r26-cli-x86_64-pc-windows-msvc.exe"
    $r26Ver = "$root\src-tauri\binaries\r26-cli.version.txt"
    if (Test-Path $r26Ver) {
        Copy-Item $r26Ver (Join-Path $dist "r26-cli.version.txt") -Force
    }
}

# license-gen
$licGenPaths = @(
    "$root\target\release\license-gen.exe",
    "$root\tools\license-gen\target\release\license-gen.exe"
)
foreach ($lg in $licGenPaths) {
    if (Test-Path $lg) {
        Copy-Item $lg (Join-Path $dist "license-gen.exe") -Force
        Write-Host "  [OK] license-gen.exe"
        break
    }
}

# WebView2 runtime (entire directory — fixedRuntime, no download)
Write-Host "  [INFO] Copying webview2-runtime/ ($wvCount files) ..."
Copy-Item $webview2 (Join-Path $dist "webview2-runtime") -Recurse -Force
Write-Host "  [OK] webview2-runtime/"

# Create portable ZIP
Write-Host ""
Write-Host "[7/7] Creating portable ZIP ..."
$zipName = "ModemCat_v${ver}_portable.zip"
$zipPath = Join-Path $dist $zipName
$tempZip = Join-Path $env:TEMP "modem-cat-zip-temp"
if (Test-Path $tempZip) { Remove-Item $tempZip -Recurse -Force }
New-Item -ItemType Directory -Path $tempZip -Force | Out-Null

Get-ChildItem $dist -Force | ForEach-Object {
    if ($_.Name -ne $zipName) {
        if ($_.PSIsContainer) {
            Copy-Item $_.FullName (Join-Path $tempZip $_.Name) -Recurse -Force
        } else {
            Copy-Item $_.FullName (Join-Path $tempZip $_.Name) -Force
        }
    }
}

Compress-Archive -Path "$tempZip\*" -DestinationPath $zipPath -Force
Remove-Item $tempZip -Recurse -Force
Write-Host "  [OK] $zipName"

# == Final summary ==
Write-Host ""
Write-Host "  ==========================================="
Write-Host "  All artifacts (dist/ root — flat, no subdirs)"
Write-Host "  ==========================================="
Write-Host ""
Get-ChildItem $dist -Recurse -Force | Where-Object { -not $_.PSIsContainer } | ForEach-Object {
    $rel = $_.FullName.Substring($dist.Length).TrimStart('\')
    if ($_.Length -ge 1MB) { $s = "{0:N1} MB" -f ($_.Length/1MB) }
    elseif ($_.Length -ge 1KB) { $s = "{0:N0} KB" -f ($_.Length/1KB) }
    else { $s = "{0} B" -f $_.Length }
    Write-Host "    $rel  ($s)"
}
Write-Host ""
Write-Host "  BUILD COMPLETE"
Write-Host ""
