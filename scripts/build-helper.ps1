# build-helper.ps1 — 构建一个安装包变体
# 用法:
#   powershell -NoProfile -File scripts\build-helper.ps1 -Variant webview
#   powershell -NoProfile -File scripts\build-helper.ps1 -Variant nowebview
#
# 行为:
#   - Variant=webview:  使用 tauri.conf.json 原始 (offlineInstaller)
#   - Variant=nowebview: 在临时目录生成 tauri.nowebview.conf.json (downloadBootstrapper)
#     因为 Tauri 2 schema 不支持 `extends`, 必须完整覆盖
#   - 运行 cargo tauri build
#   - 把生成的 msi / nsis exe 改名带 _<variant>_x64_ 后缀, 复制到 dist\installer\
#   - 不再修改任何源码或配置, 纯产物处理

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("webview", "nowebview")]
    [string]$Variant
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    # 1. 读取 version
    $cfgPath = "src-tauri\tauri.conf.json"
    $cfg = Get-Content $cfgPath -Raw | ConvertFrom-Json
    $ver = $cfg.version
    $product = $cfg.productName
    Write-Host "  variant = $Variant  version = $ver  product = $product"

    # 2. Force-rebuild the modem-cat binary. cargo tauri build patches the
    #    binary with __TAURI_BUNDLE_TYPE; if a prior `cargo build --release`
    #    left a binary without that marker in the cache, the patch step
    #    fails with "io: unexpected end of file" on subsequent tauri builds.
    $modemExe = Join-Path $root "target\release\modem-cat.exe"
    if (Test-Path $modemExe) { Remove-Item -LiteralPath $modemExe -Force }

    # 3. 确定实际使用的 config path
    $activeCfg = $cfgPath
    $tempCfg = $null
    if ($Variant -eq "nowebview") {
        # Tauri 2 schema 不接受 `extends` 也不接受 `onlineInstaller`,
        # 改为完整复制后改 webviewInstallMode.type = downloadBootstrapper
        $tempCfg = Join-Path $root "src-tauri\tauri.nowebview.conf.json"
        $cfg.bundle.windows.webviewInstallMode = [PSCustomObject]@{ type = "downloadBootstrapper" }
        $cfg | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath $tempCfg -Encoding UTF8
        $activeCfg = $tempCfg
        Write-Host "  generated temp config: $tempCfg  (webviewInstallMode=downloadBootstrapper)"
    }

    # 4. 运行 cargo tauri build
    Push-Location "src-tauri"
    try {
        if ($Variant -eq "nowebview") {
            Write-Host "  running: cargo tauri build --config $activeCfg"
            cargo tauri build --config $activeCfg
        } else {
            Write-Host "  running: cargo tauri build  (default = with webview)"
            cargo tauri build
        }
        if ($LASTEXITCODE -ne 0) {
            throw "cargo tauri build failed (exit $LASTEXITCODE)"
        }
    } finally {
        Pop-Location
    }

    # 5. 复制并改名 MSI / NSIS
    $destDir = Join-Path $root "dist\installer"
    if (-not (Test-Path $destDir)) { New-Item -ItemType Directory -Path $destDir | Out-Null }

    # Bundle 可能落在工作区根或 src-tauri 下, 两处都查
    $bundleRoots = @(
        (Join-Path $root "target\release\bundle")
        (Join-Path $root "src-tauri\target\release\bundle")
    )
    $candidates = @()
    foreach ($bundleRoot in $bundleRoots) {
        if (Test-Path $bundleRoot) {
            foreach ($sub in @("msi", "nsis")) {
                $p = Join-Path $bundleRoot $sub
                if (Test-Path $p) {
                    # Use -Filter instead of -Include; -Include requires -Recurse
                    $candidates += Get-ChildItem -Path $p -File -Filter "*.msi" -ErrorAction SilentlyContinue
                    $candidates += Get-ChildItem -Path $p -File -Filter "*.exe" -ErrorAction SilentlyContinue
                }
            }
        }
    }

    $suffix = "_${Variant}_x64_"
    $count = 0
    foreach ($f in $candidates) {
        # 原始文件名: "Modem Cat_0.2.6_x64_zh-CN.msi"  /  "Modem Cat_0.2.6_x64-setup.exe"
        # 改名为:     "Modem Cat_0.2.6_webview_x64_zh-CN.msi"  /  "..._webview_x64-setup.exe"
        $bn = $f.BaseName
        # 匹配 _x64_ (后跟语言) 或 _x64- (后跟 setup).
        # 注意: 单引号下 $1 不会被 PowerShell 变量扩展, 会在 -replace 里被当成正则反向引用
        $replacement = '_' + $Variant + '_x64$1'
        $newBn = $bn -replace '_x64([_-])', $replacement
        if ($newBn -eq $bn) {
            Write-Warning "  skip (no _x64_ marker in name): $($f.Name)"
            continue
        }
        # 跳过版本号不匹配当前配置版本的旧产物 (例如上一次 0.1.0 en-US 残留)
        if ($bn -notmatch [regex]::Escape($ver)) {
            Write-Warning "  skip (version mismatch, expected $ver): $($f.Name)"
            continue
        }
        $newName = "$newBn$($f.Extension)"
        $dest = Join-Path $destDir $newName
        Copy-Item -LiteralPath $f.FullName -Destination $dest -Force
        $size = [math]::Round($f.Length / 1MB, 2)
        Write-Host "  -> $newName  ($size MB)"
        $count++
    }

    if ($count -eq 0) {
        throw "No MSI/NSIS files found. Searched: $($bundleRoots -join ', ')"
    }
    Write-Host "  done: $count file(s) staged in dist\installer\"
}
finally {
    # 清理临时 nowebview config
    if ($tempCfg -and (Test-Path $tempCfg)) {
        Remove-Item -LiteralPath $tempCfg -Force
    }
    Pop-Location
}
