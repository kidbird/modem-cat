@echo off
setlocal

if /I "%~1"=="--help" goto :show_help
if /I "%~1"=="-h" goto :show_help

set "PS_ARGS="

:parse_args
if "%~1"=="" goto :run
if /I "%~1"=="--quick" (
    set "PS_ARGS=%PS_ARGS% -Quick"
    shift
    goto :parse_args
)
if /I "%~1"=="-Quick" (
    set "PS_ARGS=%PS_ARGS% -Quick"
    shift
    goto :parse_args
)

echo [ERR] 未知参数: %~1
echo       请使用 --help 查看用法
exit /b 1

:show_help
echo 用法: build-win.bat [--quick]
echo.
echo   build-win.bat           通过 build.ps1 生成完整 Windows 产物
echo   build-win.bat --quick   跳过 portable ZIP, 仅做快速正式构建
echo.
echo   所有产物统一输出到 dist\ 根目录。
echo   WebView2 规则:
echo     - 安装包: 默认走 downloadBootstrapper
echo     - dist\modem-cat.exe / portable ZIP: 依赖系统 WebView2
exit /b 0

:run
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0build.ps1"%PS_ARGS%
exit /b %ERRORLEVEL%
