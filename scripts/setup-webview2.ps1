# setup-webview2.ps1 — 下载并配置 WebView2 固定版本
# 用法:
#   powershell -NoProfile -ExecutionPolicy Bypass -File scripts\setup-webview2.ps1
#
# 功能:
#   1. 下载 WebView2 固定版本（Evergreen Standalone Installer）
#   2. 解压到本地目录
#   3. 更新 tauri.conf.json 使用 fixedRuntime 模式
#
# 只需运行一次，后续构建完全离线

param(
    [string]$Version = "latest",
    [switch]$Force
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root

try {
    $webview2Dir = Join-Path $root "webview2-runtime"
    $configPath = Join-Path $root "src-tauri\tauri.conf.json"
    
    Write-Host ""
    Write-Host " ==================================================="
    Write-Host "  WebView2 固定版本设置"
    Write-Host " ==================================================="
    Write-Host ""
    
    # 检查是否已存在
    if ((Test-Path $webview2Dir) -and (-not $Force)) {
        $fileCount = (Get-ChildItem $webview2Dir -File -ErrorAction SilentlyContinue | Measure-Object).Count
        if ($fileCount -gt 0) {
            Write-Host "[INFO] WebView2 固定版本已存在于: $webview2Dir"
            Write-Host "[INFO] 文件数量: $fileCount"
            Write-Host ""
            Write-Host "如需重新下载，请使用: -Force 参数"
            Write-Host ""
            
            # 询问是否更新配置
            $updateConfig = Read-Host "是否更新 tauri.conf.json 使用此版本? (y/n)"
            if ($updateConfig -eq 'y' -or $updateConfig -eq 'Y') {
                # 更新配置
                $cfg = Get-Content $configPath -Raw | ConvertFrom-Json
                $cfg.bundle.windows.webviewInstallMode = [PSCustomObject]@{
                    type = "fixedRuntime"
                    path = $webview2Dir
                }
                $cfg | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath $configPath -Encoding UTF8
                Write-Host "[OK] 已更新 tauri.conf.json"
            }
            exit 0
        }
    }
    
    # 下载 WebView2
    Write-Host "[1/3] 下载 WebView2 固定版本..."
    Write-Host ""
    
    # Evergreen Bootstrapper URL
    $bootstrapperUrl = "https://go.microsoft.com/fwlink/p/?LinkId=2124703"
    $bootstrapperPath = Join-Path $root "MicrosoftEdgeWebview2Setup.exe"
    
    # Evergreen Standalone Installer URL (x64)
    $installerUrl = "https://go.microsoft.com/fwlink/?linkid=2124701"
    $installerPath = Join-Path $root "MicrosoftEdgeWebView2RuntimeInstallerX64.exe"
    
    # 尝试下载离线安装包
    Write-Host "  下载离线安装包 (约 200MB)..."
    Write-Host "  URL: $installerUrl"
    Write-Host ""
    
    try {
        if (-not (Test-Path $installerPath)) {
            # 使用 PowerShell 下载
            $ProgressPreference = 'SilentlyContinue'
            Invoke-WebRequest -Uri $installerUrl -OutFile $installerPath -UseBasicParsing
            $ProgressPreference = 'Continue'
            Write-Host "  [OK] 下载完成"
        } else {
            Write-Host "  [INFO] 安装包已存在，跳过下载"
        }
    } catch {
        Write-Warning "  下载失败: $_"
        Write-Host ""
        Write-Host "  请手动下载 WebView2 离线安装包:"
        Write-Host "  https://developer.microsoft.com/en-us/microsoft-edge/webview2/"
        Write-Host "  选择: Evergreen Standalone Installer (x64)"
        Write-Host "  下载后放到: $installerPath"
        Write-Host ""
        exit 1
    }
    
    # 解压安装包
    Write-Host ""
    Write-Host "[2/3] 解压 WebView2 运行时..."
    
    if (Test-Path $webview2Dir) {
        Remove-Item $webview2Dir -Recurse -Force
    }
    New-Item -ItemType Directory -Path $webview2Dir -Force | Out-Null
    
    # 离线安装包是自解压的，使用 /extract 参数
    Write-Host "  解压到: $webview2Dir"
    $process = Start-Process -FilePath $installerPath -ArgumentList "/extract:$webview2Dir /quiet" -Wait -PassThru -NoNewWindow
    
    if ($process.ExitCode -ne 0) {
        Write-Warning "  解压失败，尝试使用 cab 方式..."
        
        # 尝试查找 cab 文件
        $cabFiles = Get-ChildItem $installerPath -Recurse -Filter "*.cab" -ErrorAction SilentlyContinue
        if ($cabFiles) {
            foreach ($cab in $cabFiles) {
                Write-Host "  解压 CAB: $($cab.Name)"
                expand.exe -F:* $cab.FullName $webview2Dir
            }
        }
    }
    
    # 检查解压结果
    $extractedFiles = Get-ChildItem $webview2Dir -Recurse -File -ErrorAction SilentlyContinue
    if ($extractedFiles.Count -eq 0) {
        Write-Warning "  解压后未找到文件"
        Write-Host ""
        Write-Host "  请手动解压安装包并放到: $webview2Dir"
        exit 1
    }
    
    Write-Host "  [OK] 解压完成，文件数量: $($extractedFiles.Count)"
    
    # 更新配置
    Write-Host ""
    Write-Host "[3/3] 更新 tauri.conf.json..."
    
    $cfg = Get-Content $configPath -Raw | ConvertFrom-Json
    $cfg.bundle.windows.webviewInstallMode = [PSCustomObject]@{
        type = "fixedRuntime"
        path = $webview2Dir
    }
    $cfg | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath $configPath -Encoding UTF8
    
    Write-Host "  [OK] 已更新为 fixedRuntime 模式"
    
    # 清理临时文件
    Write-Host ""
    Write-Host "[清理] 删除临时安装包..."
    if (Test-Path $installerPath) {
        Remove-Item $installerPath -Force
    }
    if (Test-Path $bootstrapperPath) {
        Remove-Item $bootstrapperPath -Force
    }
    
    # 完成
    Write-Host ""
    Write-Host " ==================================================="
    Write-Host "  设置完成!"
    Write-Host " ==================================================="
    Write-Host ""
    Write-Host "WebView2 固定版本位置: $webview2Dir"
    Write-Host "文件数量: $($extractedFiles.Count)"
    Write-Host ""
    Write-Host "后续构建将使用此固定版本，完全离线。"
    Write-Host ""
    Write-Host "如需切换回其他模式，请手动编辑: $configPath"
    Write-Host ""
    
} finally {
    Pop-Location
}
