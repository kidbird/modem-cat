# setup-webview2.ps1 - prepare an app-local WebView2 Fixed Version package
# Usage:
#   powershell -NoProfile -ExecutionPolicy Bypass -File scripts\setup-webview2.ps1
#
# What it does:
#   1. Reads the official WebView2 download page and finds Fixed Version CAB links
#   2. Extracts the selected package into src-tauri\webview2-runtime\
#   3. Ensures tauri.conf.json uses app-local fixedRuntime mode
#
# Run it once on the build machine. Subsequent builds stay offline.

param(
    [string]$Version = "latest",
    [ValidateSet("x64", "x86", "arm64")]
    [string]$Architecture = "x64",
    [switch]$Force
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root

function Get-FixedVersionOptions {
    $downloadPage = "https://developer.microsoft.com/en-us/microsoft-edge/webview2/?form=MA13LH"
    $response = Invoke-WebRequest -Uri $downloadPage -UseBasicParsing
    $pattern = 'https:\\u002F\\u002F[^"]+Microsoft\.WebView2\.FixedVersionRuntime\.([0-9.]+)\.(x64|x86|arm64)\.cab'
    $matches = [regex]::Matches($response.Content, $pattern)

    if ($matches.Count -eq 0) {
        throw "failed to find Fixed Version package links on $downloadPage"
    }

    $seen = @{}
    $options = @()
    foreach ($match in $matches) {
        $version = $match.Groups[1].Value
        $arch = $match.Groups[2].Value
        $url = $match.Value -replace '\\u002F', '/' -replace '\\u0026', '&'
        $key = "$version|$arch"
        if (-not $seen.ContainsKey($key)) {
            $seen[$key] = $true
            $options += [PSCustomObject]@{
                Version = $version
                Architecture = $arch
                Url = $url
            }
        }
    }

    return $options
}

function Select-FixedVersionPackage([string]$RequestedVersion, [string]$RequestedArchitecture) {
    $options = Get-FixedVersionOptions | Where-Object { $_.Architecture -eq $RequestedArchitecture }
    if (-not $options) {
        throw "no Fixed Version package found for architecture $RequestedArchitecture"
    }

    if ($RequestedVersion -eq "latest") {
        return $options[0]
    }

    $exact = $options | Where-Object { $_.Version -eq $RequestedVersion } | Select-Object -First 1
    if ($exact) {
        return $exact
    }

    $available = ($options | Select-Object -ExpandProperty Version) -join ", "
    throw "fixed version $RequestedVersion for $RequestedArchitecture not found on the official WebView2 download page. Available versions: $available"
}

function Normalize-FixedRuntimeLayout([string]$RuntimeDir) {
    $rootRuntimeExe = Join-Path $RuntimeDir "msedgewebview2.exe"
    if (Test-Path $rootRuntimeExe) {
        return
    }

    $nestedRoots = Get-ChildItem -Path $RuntimeDir -Directory -ErrorAction SilentlyContinue | Where-Object {
        Test-Path (Join-Path $_.FullName "msedgewebview2.exe")
    }

    if ($nestedRoots.Count -ne 1) {
        return
    }

    $nestedRoot = $nestedRoots[0]
    Get-ChildItem -LiteralPath $nestedRoot.FullName -Force | ForEach-Object {
        Move-Item -LiteralPath $_.FullName -Destination $RuntimeDir -Force
    }
    Remove-Item -LiteralPath $nestedRoot.FullName -Recurse -Force
}

function Write-Utf8NoBomJson([string]$Path, [object]$Value) {
    $json = $Value | ConvertTo-Json -Depth 10
    $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($Path, $json, $utf8NoBom)
}

try {
    $webview2Dir = Join-Path $root "src-tauri\webview2-runtime"
    $legacyWebview2Dir = Join-Path $root "webview2-runtime"
    $configPath = Join-Path $root "src-tauri\tauri.conf.json"

    Write-Host ""
    Write-Host " ==================================================="
    Write-Host "  WebView2 Fixed Version Setup"
    Write-Host " ==================================================="
    Write-Host ""

    if ((Test-Path $webview2Dir) -and (-not $Force)) {
        $fileCount = (Get-ChildItem $webview2Dir -File -ErrorAction SilentlyContinue | Measure-Object).Count
        if ($fileCount -gt 0) {
            Write-Host "[INFO] WebView2 fixed runtime already exists at: $webview2Dir"
            Write-Host "[INFO] File count: $fileCount"
            Write-Host ""
            Write-Host "Use -Force to refresh the local runtime."
            Write-Host ""

            $updateConfig = Read-Host "Update tauri.conf.json to use this runtime? (y/n)"
            if ($updateConfig -eq 'y' -or $updateConfig -eq 'Y') {
                $cfg = Get-Content $configPath -Raw | ConvertFrom-Json
                $cfg.bundle.windows.webviewInstallMode = [PSCustomObject]@{
                    type = "fixedRuntime"
                    path = "webview2-runtime"
                }
                Write-Utf8NoBomJson -Path $configPath -Value $cfg
                Write-Host "[OK] tauri.conf.json updated"
            }
            exit 0
        }
    }

    Write-Host "[1/3] Download WebView2 Fixed Version package..."
    Write-Host ""

    $package = Select-FixedVersionPackage -RequestedVersion $Version -RequestedArchitecture $Architecture
    $packageName = "Microsoft.WebView2.FixedVersionRuntime.$($package.Version).$($package.Architecture).cab"
    $packagePath = Join-Path $root $packageName

    Write-Host "  Version: $($package.Version)"
    Write-Host "  Arch:    $($package.Architecture)"
    Write-Host "  URL:     $($package.Url)"
    Write-Host ""

    try {
        if (-not (Test-Path $packagePath)) {
            $ProgressPreference = 'SilentlyContinue'
            Invoke-WebRequest -Uri $package.Url -OutFile $packagePath -UseBasicParsing
            $ProgressPreference = 'Continue'
            Write-Host "  [OK] Download complete"
        } else {
            Write-Host "  [INFO] Package already exists, skipping download"
        }
    } catch {
        Write-Warning "  Download failed: $_"
        Write-Host ""
        Write-Host "  Manually download the official WebView2 Fixed Version package:"
        Write-Host "  https://developer.microsoft.com/en-us/microsoft-edge/webview2/"
        Write-Host "  Choose: Fixed Version -> $Architecture -> version $Version"
        Write-Host "  Then place it at: $packagePath"
        Write-Host ""
        exit 1
    }

    Write-Host ""
    Write-Host "[2/3] Extract runtime files..."

    if (Test-Path $webview2Dir) {
        Remove-Item $webview2Dir -Recurse -Force
    }
    New-Item -ItemType Directory -Path $webview2Dir -Force | Out-Null

    Write-Host "  Extract to: $webview2Dir"
    & expand.exe $packagePath -F:* $webview2Dir | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "expand.exe failed to extract $packageName (exit $LASTEXITCODE)"
    }

    Normalize-FixedRuntimeLayout -RuntimeDir $webview2Dir

    $extractedFiles = Get-ChildItem $webview2Dir -Recurse -File -ErrorAction SilentlyContinue
    if ($extractedFiles.Count -eq 0) {
        Write-Warning "  No files were extracted"
        Write-Host ""
        Write-Host "  Manually extract the Fixed Version package to: $webview2Dir"
        Write-Host ""
        exit 1
    }

    $runtimeExe = Join-Path $webview2Dir "msedgewebview2.exe"
    if (-not (Test-Path $runtimeExe)) {
        throw "fixed runtime extraction did not produce msedgewebview2.exe at $runtimeExe"
    }

    Write-Host "  [OK] Extraction complete, file count: $($extractedFiles.Count)"

    Write-Host ""
    Write-Host "[3/3] Update tauri.conf.json..."

    $cfg = Get-Content $configPath -Raw | ConvertFrom-Json
    $cfg.bundle.windows.webviewInstallMode = [PSCustomObject]@{
        type = "fixedRuntime"
        path = "webview2-runtime"
    }
    Write-Utf8NoBomJson -Path $configPath -Value $cfg

    Write-Host "  [OK] fixedRuntime mode enabled"

    Write-Host ""
    Write-Host "[Cleanup] Remove temporary package..."
    if (Test-Path $packagePath) {
        Remove-Item $packagePath -Force
    }
    if (Test-Path $legacyWebview2Dir) {
        Write-Host "[Cleanup] Keeping legacy repo-root webview2-runtime/ as a compatibility cache"
    }

    Write-Host ""
    Write-Host " ==================================================="
    Write-Host "  Setup complete"
    Write-Host " ==================================================="
    Write-Host ""
    Write-Host "WebView2 fixed runtime path: $webview2Dir"
    Write-Host "Fixed Version: $($package.Version) ($($package.Architecture))"
    Write-Host "Extracted files: $($extractedFiles.Count)"
    Write-Host ""
    Write-Host "Subsequent builds read fixed runtime from src-tauri\\webview2-runtime\\."
    Write-Host ""
    Write-Host "To switch modes later, edit: $configPath"
    Write-Host ""
} finally {
    Pop-Location
}
