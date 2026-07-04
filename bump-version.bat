@echo off
REM ─────────────────────────────────────────────────────────────────────────────
REM bump-version.bat — 统一更新 Modem Cat 版本号
REM
REM 用法:
REM   bump-version.bat 0.2.0
REM
REM 会同步更新以下文件：
REM   src-tauri\tauri.conf.json   → "version"
REM   src-tauri\Cargo.toml        → version
REM   package.json                → "version"
REM   modem-hal\Cargo.toml        → version
REM ─────────────────────────────────────────────────────────────────────────────
setlocal EnableDelayedExpansion

if "%~1"=="" (
    echo 用法: bump-version.bat ^<版本号^>   示例: bump-version.bat 0.2.0
    exit /b 1
)

set "NEW=%~1"

REM 格式校验（PowerShell 做，batch 正则太弱）
powershell -NoProfile -Command ^
  "if ('%NEW%' -notmatch '^\d+\.\d+\.\d+$') { Write-Host '[ERR]   格式无效: %NEW%  应为 X.Y.Z' -ForegroundColor Red; exit 1 }"
if errorlevel 1 exit /b 1

cd /d "%~dp0"

REM 读取当前版本
for /f "tokens=*" %%v in ('powershell -NoProfile -Command ^
  "(Get-Content src-tauri\tauri.conf.json | ConvertFrom-Json).version"') do set "OLD=%%v"

if "%OLD%"=="%NEW%" (
    echo 已是 v%NEW%，无需更新。
    exit /b 0
)

echo.
echo   版本升级: v%OLD% ^-^> v%NEW%
echo.

REM ── 用 PowerShell 统一处理 JSON + TOML ──────────────────────────────────────
powershell -NoProfile -Command ^
"$new = '%NEW%'; " ^
"$ErrorActionPreference = 'Stop'; " ^
^
"# 1. tauri.conf.json " ^
"$p = 'src-tauri\tauri.conf.json'; " ^
"$d = Get-Content $p -Raw | ConvertFrom-Json; " ^
"$d.version = $new; " ^
"$d | ConvertTo-Json -Depth 10 | Set-Content $p -Encoding UTF8; " ^
"Write-Host '[OK]    src-tauri\tauri.conf.json  ->  version: ' + $new -ForegroundColor Green; " ^
^
"# 2. package.json " ^
"$p = 'package.json'; " ^
"$d = Get-Content $p -Raw | ConvertFrom-Json; " ^
"$d.version = $new; " ^
"$d | ConvertTo-Json -Depth 10 | Set-Content $p -Encoding UTF8; " ^
"Write-Host '[OK]    package.json               ->  version: ' + $new -ForegroundColor Green; " ^
^
"# 3. src-tauri\Cargo.toml  (替换第一个 version = '...' ) " ^
"$p = 'src-tauri\Cargo.toml'; " ^
"$content = Get-Content $p -Raw; " ^
"$content = $content -replace '(?m)^(version\s*=\s*)""[^""]*""', ('${1}""' + $new + '""'); " ^
"Set-Content $p $content -Encoding UTF8 -NoNewline; " ^
"Write-Host '[OK]    src-tauri\Cargo.toml        ->  version = ' + $new -ForegroundColor Green; " ^
^
"# 4. modem-hal\Cargo.toml " ^
"$p = 'modem-hal\Cargo.toml'; " ^
"$content = Get-Content $p -Raw; " ^
"$content = $content -replace '(?m)^(version\s*=\s*)""[^""]*""', ('${1}""' + $new + '""'); " ^
"Set-Content $p $content -Encoding UTF8 -NoNewline; " ^
"Write-Host '[OK]    modem-hal\Cargo.toml        ->  version = ' + $new -ForegroundColor Green"

if errorlevel 1 (
    echo [ERR] 更新失败，请检查上方错误信息。
    exit /b 1
)

echo.
echo [OK]    版本已更新为 v%NEW%
echo.
echo   下一步:
echo     git add -A ^&^& git commit -m "chore: bump version to v%NEW%"
echo     build-win.bat
echo.
endlocal
