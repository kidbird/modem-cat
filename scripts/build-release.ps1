#requires -Version 5.1
<#
.SYNOPSIS
    Modem Cat — Complete release build script (Windows).

.DESCRIPTION
    Builds portable exe + MSI installer + NSIS installer, bundles r26-cli sidecar,
    license-gen, and WebView2 runtime. Outputs everything flat in dist/ root.

.PARAMETERS
    -Quick
        Skip the portable ZIP stage. Useful for iteration when only exe / MSI / NSIS
        are needed.

.EXAMPLE
    powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-release.ps1
#>

param(
    [switch]$Quick
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"
Add-Type -AssemblyName System.IO.Compression
Add-Type -AssemblyName System.IO.Compression.FileSystem

$root = Split-Path -Parent $PSScriptRoot
$dist = Join-Path $root "dist"
$sdkDir = if (Test-Path (Join-Path $root "Sdk")) {
    Join-Path $root "Sdk"
} else {
    Join-Path $root "sdk"
}
$adbResourceDir = Join-Path $root "src-tauri\resources\adb"
$adbFiles = @(
    "adb.exe",
    "AdbWinApi.dll",
    "AdbWinUsbApi.dll"
)

function Stop-RunningDistApp {
    param(
        [string]$DistExePath
    )

    $normalized = [System.IO.Path]::GetFullPath($DistExePath)
    $running = Get-Process -ErrorAction SilentlyContinue | Where-Object {
        if (-not $_.Path) {
            return $false
        }
        try {
            return [System.IO.Path]::GetFullPath($_.Path) -eq $normalized
        } catch {
            return $false
        }
    }

    foreach ($proc in $running) {
        Write-Host "  [INFO] Stopping running dist app: PID $($proc.Id)"
        Stop-Process -Id $proc.Id -Force -ErrorAction Stop
        $proc.WaitForExit(5000) | Out-Null
    }
}

function Copy-FileWithRetry {
    param(
        [Parameter(Mandatory = $true)][string]$Source,
        [Parameter(Mandatory = $true)][string]$Destination,
        [int]$MaxAttempts = 5
    )

    for ($attempt = 1; $attempt -le $MaxAttempts; $attempt++) {
        try {
            Copy-Item $Source $Destination -Force
            return
        } catch {
            if ($attempt -eq $MaxAttempts) {
                throw
            }
            Start-Sleep -Milliseconds (500 * $attempt)
        }
    }
}

function Add-DirectoryToZip {
    param(
        [Parameter(Mandatory = $true)]
        [System.IO.Compression.ZipArchive]$Archive,
        [Parameter(Mandatory = $true)]
        [string]$SourceDir,
        [Parameter(Mandatory = $true)]
        [string]$EntryPrefix
    )

    $baseDir = [System.IO.Path]::GetFullPath($SourceDir)
    Get-ChildItem $baseDir -Recurse -File | ForEach-Object {
        $full = [System.IO.Path]::GetFullPath($_.FullName)
        $relative = $full.Substring($baseDir.Length).TrimStart('\', '/')
        $entryName = if ([string]::IsNullOrWhiteSpace($EntryPrefix)) {
            $relative
        } else {
            "$EntryPrefix/$relative".Replace('\', '/')
        }
        [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile(
            $Archive,
            $full,
            $entryName,
            [System.IO.Compression.CompressionLevel]::Optimal
        ) | Out-Null
    }
}

# == Read version from tauri.conf.json ==
$cfg = Get-Content "$root\src-tauri\tauri.conf.json" -Raw | ConvertFrom-Json
$ver = $cfg.version
Write-Host ""
Write-Host "  Modem Cat v$ver -- Release Build"
Write-Host "  root: $root"
Write-Host ""

# == Step 1: Clean dist/ ==
Write-Host "[1/8] Cleaning dist/ ..."
Stop-RunningDistApp -DistExePath (Join-Path $dist "modem-cat.exe")
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

# == Step 2: Sync ADB resources from Sdk/ ==
Write-Host "[2/8] Syncing ADB resources from Sdk/ ..."
if (-not (Test-Path $sdkDir)) {
    throw "Sdk directory not found: $sdkDir"
}

New-Item -ItemType Directory -Path $adbResourceDir -Force | Out-Null
foreach ($file in $adbFiles) {
    $src = Join-Path $sdkDir $file
    if (-not (Test-Path $src)) {
        throw "Required ADB runtime file not found in Sdk/: $file"
    }
    Copy-Item $src (Join-Path $adbResourceDir $file) -Force
    Write-Host "  [OK] $file -> src-tauri/resources/adb/"
}

# == Step 3: Toolchain checks ==
Write-Host "[3/8] Toolchain checks ..."

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

# == Step 4: Build Tauri ==
Write-Host "[4/8] Building Tauri (portable + installers) ..."

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

# == Step 5: r26-cli sidecar (pre-built expected) ==
Write-Host "[5/8] Checking r26-cli sidecar ..."
$r26Src = "$root\src-tauri\binaries\r26-cli-x86_64-pc-windows-msvc.exe"
if (Test-Path $r26Src) {
    Write-Host "  [OK] sidecar found in binaries/"
} else {
    Write-Warning "r26-cli sidecar not found in binaries/ — firmware download will not be available"
}

# == Step 6: Build license-gen ==
Write-Host "[6/8] Building license-gen ..."
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

# == Step 7: Copy artifacts to dist/ root ==
Write-Host "[7/8] Copying artifacts to dist/ ..."

# Portable exe
$exeSrc = "$root\target\release\modem-cat.exe"
if (Test-Path $exeSrc) {
    Stop-RunningDistApp -DistExePath (Join-Path $dist "modem-cat.exe")
    Copy-FileWithRetry -Source $exeSrc -Destination (Join-Path $dist "modem-cat.exe")
    Write-Host "  [OK] modem-cat.exe"
}

# ADB runtime (flat dist/ root for portable runs)
foreach ($file in $adbFiles) {
    $src = Join-Path $adbResourceDir $file
    if (Test-Path $src) {
        Stop-RunningDistApp -DistExePath (Join-Path $dist $file)
        Copy-FileWithRetry -Source $src -Destination (Join-Path $dist $file)
        Write-Host "  [OK] $file"
    }
}

# MSI installer
$msiDir = "$root\target\release\bundle\msi"
if (Test-Path $msiDir) {
    Get-ChildItem $msiDir -Filter "*.msi" -Recurse | Where-Object {
        $_.Name -like "*_${ver}_*"
    } | ForEach-Object {
        Copy-FileWithRetry -Source $_.FullName -Destination (Join-Path $dist $_.Name)
        Write-Host "  [OK] $($_.Name)"
    }
}

# NSIS installer
$nsisDir = "$root\target\release\bundle\nsis"
if (Test-Path $nsisDir) {
    Get-ChildItem $nsisDir -Filter "*.exe" -Recurse | Where-Object {
        $_.Name -like "*_${ver}_*"
    } | ForEach-Object {
        Copy-FileWithRetry -Source $_.FullName -Destination (Join-Path $dist $_.Name)
        Write-Host "  [OK] $($_.Name)"
    }
}

# r26-cli sidecar
if (Test-Path $r26Src) {
    Copy-FileWithRetry -Source $r26Src -Destination (Join-Path $dist "r26-cli-x86_64-pc-windows-msvc.exe")
    Write-Host "  [OK] r26-cli-x86_64-pc-windows-msvc.exe"
    $r26Ver = "$root\src-tauri\binaries\r26-cli.version.txt"
    if (Test-Path $r26Ver) {
        Copy-FileWithRetry -Source $r26Ver -Destination (Join-Path $dist "r26-cli.version.txt")
    }
}

# license-gen
$licGenPaths = @(
    "$root\target\release\license-gen.exe",
    "$root\tools\license-gen\target\release\license-gen.exe"
)
foreach ($lg in $licGenPaths) {
    if (Test-Path $lg) {
        Copy-FileWithRetry -Source $lg -Destination (Join-Path $dist "license-gen.exe")
        Write-Host "  [OK] license-gen.exe"
        break
    }
}

# Create portable ZIP
Write-Host ""
$zipName = "ModemCat_v${ver}_portable.zip"
$zipPath = Join-Path $dist $zipName
if ($Quick) {
    Write-Host "[8/8] Skipping portable ZIP (-Quick)"
} else {
    Write-Host "[8/8] Creating portable ZIP ..."
    if (Test-Path $zipPath) {
        Remove-Item $zipPath -Force
    }

    $archive = $null
    try {
        $archive = [System.IO.Compression.ZipFile]::Open(
            $zipPath,
            [System.IO.Compression.ZipArchiveMode]::Create
        )

        Get-ChildItem $dist -File | ForEach-Object {
            if ($_.Name -ne $zipName) {
                [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile(
                    $archive,
                    $_.FullName,
                    $_.Name,
                    [System.IO.Compression.CompressionLevel]::Optimal
                ) | Out-Null
            }
        }

        Write-Host "  [INFO] Adding webview2-runtime/ to portable ZIP payload ..."
        Add-DirectoryToZip -Archive $archive -SourceDir $webview2 -EntryPrefix "webview2-runtime"
    } finally {
        if ($archive) {
            $archive.Dispose()
        }
    }

    Write-Host "  [OK] $zipName"
}

# == Final summary ==
Write-Host ""
Write-Host "  ==========================================="
Write-Host "  All artifacts (dist/ root — flat, no subdirs)"
Write-Host "  ==========================================="
Write-Host ""
Get-ChildItem $dist -Force | Where-Object { -not $_.PSIsContainer } | ForEach-Object {
    $rel = $_.FullName.Substring($dist.Length).TrimStart('\')
    if ($_.Length -ge 1MB) { $s = "{0:N1} MB" -f ($_.Length/1MB) }
    elseif ($_.Length -ge 1KB) { $s = "{0:N0} KB" -f ($_.Length/1KB) }
    else { $s = "{0} B" -f $_.Length }
    Write-Host "    $rel  ($s)"
}
Write-Host ""
Write-Host "  BUILD COMPLETE"
Write-Host ""
