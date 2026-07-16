# setup-webview2.ps1 — WebView2 mode sanity helper
#
# The project now uses Tauri's `downloadBootstrapper` mode by default:
#   - Windows 10/11 with system WebView2: app starts directly
#   - machines without WebView2: installer bootstrapper downloads/installs it
#
# This script no longer stages an app-local fixed runtime. It only keeps
# `src-tauri/tauri.conf.json` aligned with the supported default and can
# optionally clean legacy `webview2-runtime/` directories left by older builds.

param(
    [switch]$CleanLegacy
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$configPath = Join-Path $root "src-tauri\tauri.conf.json"
$legacyDirs = @(
    (Join-Path $root "webview2-runtime"),
    (Join-Path $root "src-tauri\webview2-runtime")
)

function Write-Utf8NoBomJson([string]$Path, $Value) {
    $json = $Value | ConvertTo-Json -Depth 10
    $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($Path, $json, $utf8NoBom)
}

$cfg = Get-Content $configPath -Raw | ConvertFrom-Json
$cfg.bundle.windows.webviewInstallMode = [PSCustomObject]@{
    type = "downloadBootstrapper"
}
Write-Utf8NoBomJson -Path $configPath -Value $cfg

Write-Host ""
Write-Host "WebView2 mode normalized:"
Write-Host "  tauri.conf.json -> bundle.windows.webviewInstallMode = downloadBootstrapper"
Write-Host "  portable builds  -> rely on system WebView2"
Write-Host "  installer builds -> download bootstrapper when needed"

if ($CleanLegacy) {
    foreach ($dir in $legacyDirs) {
        if (Test-Path $dir) {
            Remove-Item -LiteralPath $dir -Recurse -Force
            Write-Host "  removed legacy fixed runtime cache: $dir"
        }
    }
} else {
    foreach ($dir in $legacyDirs) {
        if (Test-Path $dir) {
            Write-Host "  legacy fixed runtime cache still present (optional cleanup): $dir"
        }
    }
}

Write-Host ""
Write-Host "No app-local WebView2 runtime is staged by this script anymore."
