# build-helper.ps1 — 构建一个安装包变体
# 用法:
#   powershell -NoProfile -File scripts\build-helper.ps1 -Variant webview
#   powershell -NoProfile -File scripts\build-helper.ps1 -Variant nowebview
#
# 行为:
#   - Variant=webview:  使用 tauri.conf.json 原始 (embedBootstrapper)
#   - Variant=nowebview: 在临时目录生成 tauri.nowebview.conf.json (skip)
#     因为 Tauri 2 schema 不支持 `extends`, 必须完整覆盖
#   - 预置 WebView2 bootstrapper 缓存：从 %LOCALAPPDATA%\tauri 复制已缓存文件，避免重复下载
#   - 运行 cargo tauri build
#   - 把生成的 msi / nsis exe 改名带 _<variant>_x64_ 后缀, 复制到 dist/
#   - 不再修改任何源码或配置, 纯产物处理

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("webview", "nowebview")]
    [string]$Variant
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root

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
    Write-Host "  staged x86 r26 runtime: $runtimeSrc"
    return $runtimeDst
}

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
        # 生成临时配置，改为 skip 模式：不自动下载/安装 WebView2
        # --config 参数会与默认配置合并, 所以需要先移除默认配置中的 path 字段
        $tempCfg = Join-Path $root "src-tauri\tauri.nowebview.conf.json"
        $cfg.bundle.windows.webviewInstallMode = [PSCustomObject]@{ type = "skip" }
        $cfg | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath $tempCfg -Encoding UTF8
        # 读取临时配置, 插入一个空 path 字段来覆盖默认配置
        $tempContent = Get-Content $tempCfg -Raw
        # 将 `"type": "skip"` 改为 `"type": "skip", "path": null` 以覆盖默认的 path
        $tempContent = $tempContent -replace '"type":\s*"skip"', '"type": "skip", "path": null'
        Set-Content -LiteralPath $tempCfg -Value $tempContent -Encoding UTF8
        $activeCfg = $tempCfg
        Write-Host "  generated temp config: $tempCfg  (webviewInstallMode=skip, path=null)"
    }
    $stagedR26Runtime = Sync-R26Runtime
    Write-Host "  using r26 runtime: $stagedR26Runtime"

    # 4. 预置 WebView2 bootstrapper 缓存（避免重复下载）
    $wixDir = Join-Path $root "target\release\wix\x64"
    if (-not (Test-Path $wixDir)) { New-Item -ItemType Directory -Path $wixDir -Force | Out-Null }
    
    if ($Variant -eq "webview") {
        $bootstrapperCache = Join-Path $wixDir "MicrosoftEdgeWebview2Setup.exe"
        if (-not (Test-Path $bootstrapperCache)) {
            # 尝试从 %LOCALAPPDATA%\tauri 查找
            $tauriCache = Join-Path $env:LOCALAPPDATA "tauri"
            $cachedBootstrapper = Get-ChildItem $tauriCache -Recurse -Filter "MicrosoftEdgeWebview2Setup.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
            if ($cachedBootstrapper) {
                Copy-Item -LiteralPath $cachedBootstrapper.FullName -Destination $bootstrapperCache -Force
                Write-Host "  pre-cached WebView2 bootstrapper"
            } else {
                Write-Warning "  no cached WebView2 bootstrapper found in $tauriCache - will allow tauri-cli to fetch it"
            }
        }
    }

    # 5. 运行 cargo tauri build
    Push-Location "src-tauri"
    try {
        if ($Variant -eq "nowebview") {
            Write-Host "  running: cargo tauri build --config $activeCfg"
            cargo tauri build --config $activeCfg
        } else {
            Write-Host "  running: cargo tauri build  (default = embedBootstrapper)"
            cargo tauri build
        }
        if ($LASTEXITCODE -ne 0) {
            throw "cargo tauri build failed (exit $LASTEXITCODE)"
        }
    } finally {
        Pop-Location
    }

    # 5. 复制并改名 MSI / NSIS 到 dist/ 根目录
    $destDir = Join-Path $root "dist"
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
    Write-Host "  done: $count file(s) staged in dist/"
}
finally {
    # 清理临时 nowebview config
    if ($tempCfg -and (Test-Path $tempCfg)) {
        Remove-Item -LiteralPath $tempCfg -Force
    }
    Pop-Location
}
