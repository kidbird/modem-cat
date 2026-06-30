@echo off
REM ─────────────────────────────────────────────────────────────────────────────
REM build-win.bat — Modem Cat Windows 构建脚本
REM
REM 产出物（构建完成后统一展示路径）:
REM   Portable : src-tauri\target\release\modem-cat.exe
REM   安装包   : src-tauri\target\release\bundle\msi\Modem Cat_*.msi
REM              src-tauri\target\release\bundle\nsis\Modem Cat_*.exe  (如有)
REM
REM 用法:
REM   build-win.bat          直接双击或命令行运行
REM   build-win.bat --help   显示帮助
REM ─────────────────────────────────────────────────────────────────────────────
setlocal EnableDelayedExpansion

if "%~1"=="--help" goto :show_help
if "%~1"=="-h"     goto :show_help
if not "%~1"==""   (
    echo [ERR] 未知参数: %~1  请使用 --help 查看用法
    exit /b 1
)
goto :start

:show_help
echo 用法: build-win.bat
echo.
echo   直接运行即可。同时产出 portable .exe 与 .msi 安装包。
echo   构建完成后脚本会打印两种产物的完整路径。
exit /b 0

:start
echo.
echo ═══════════════════════════════════════════════
echo   Modem Cat Windows Build
echo ═══════════════════════════════════════════════
echo.

REM ── 检查 Rust ─────────────────────────────────────────────────────────────────
where cargo >nul 2>&1
if errorlevel 1 (
    echo [ERR] 未找到 cargo，请安装 Rust: https://rustup.rs
    exit /b 1
)
for /f "tokens=*" %%v in ('rustc --version 2^>^&1') do echo [OK]   Rust: %%v

REM ── 检查 tauri-cli ────────────────────────────────────────────────────────────
cargo tauri --version >nul 2>&1
if errorlevel 1 (
    echo [WARN] 未安装 cargo-tauri，正在安装...
    cargo install tauri-cli --version "^2" --locked
    if errorlevel 1 (
        echo [ERR] cargo-tauri 安装失败
        exit /b 1
    )
)
for /f "tokens=*" %%v in ('cargo tauri --version 2^>^&1') do echo [OK]   tauri-cli: %%v

REM ── 自动定位 MSVC 工具链 ──────────────────────────────────────────────────────
set "VCVARSALL="

REM 1. 优先用 vswhere（VS 2017+ 自带，能找到任意版本）
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
if not exist "%VSWHERE%" set "VSWHERE=%ProgramFiles%\Microsoft Visual Studio\Installer\vswhere.exe"

if exist "%VSWHERE%" (
    for /f "usebackq delims=" %%p in (
        `"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -find VC\Auxiliary\Build\vcvarsall.bat 2^>nul`
    ) do set "VCVARSALL=%%p"
)

REM 2. 备用：逐一检查常见路径（覆盖 2019 / 2022 各版本）
if not defined VCVARSALL (
    for %%e in (
        "%ProgramFiles%\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
        "%ProgramFiles%\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat"
        "%ProgramFiles%\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvarsall.bat"
        "%ProgramFiles%\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvarsall.bat"
        "%ProgramFiles(x86)%\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
        "%ProgramFiles(x86)%\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat"
        "%ProgramFiles(x86)%\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
        "%ProgramFiles(x86)%\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvarsall.bat"
    ) do if exist %%e (
        if not defined VCVARSALL set "VCVARSALL=%%~e"
    )
)

if not defined VCVARSALL (
    echo.
    echo [ERR] 未找到 MSVC 工具链
    echo       请安装 Visual Studio 2019/2022 ^(含 C++ 工作负载^) 或 Build Tools:
    echo       https://visualstudio.microsoft.com/visual-cpp-build-tools/
    exit /b 1
)

echo [OK]   MSVC: %VCVARSALL%
call "%VCVARSALL%" x64 >nul 2>&1
if errorlevel 1 (
    echo [ERR] MSVC 环境初始化失败
    exit /b 1
)
echo [OK]   MSVC 环境已就绪 ^(x64^)

REM ── 切换到项目根目录并构建 ────────────────────────────────────────────────────
cd /d "%~dp0"
echo [INFO] 工作目录: %CD%
echo.
echo ───────────────────────────────────────────────
echo   开始构建...
echo ───────────────────────────────────────────────
echo.

cd src-tauri
set NO_PROXY=go.microsoft.com,*.microsoft.com
cargo tauri build
if errorlevel 1 (
    echo.
    echo [ERR] 构建失败，请检查上方错误信息。
    exit /b 1
)
cd ..

REM ── 展示产出物 ────────────────────────────────────────────────────────────────
echo.
echo ───────────────────────────────────────────────
echo   产出物
echo ───────────────────────────────────────────────
echo.

set "RELEASE=src-tauri\target\release"
set "BUNDLE=%RELEASE%\bundle"

REM Portable — 裸 .exe（需系统已安装 WebView2，Windows 10/11 默认自带）
if exist "%RELEASE%\modem-cat.exe" (
    echo [OK]   Portable  ^>  %RELEASE%\modem-cat.exe
) else (
    echo [WARN] 未找到 portable exe
)

REM 安装包 — MSI
for /r "%BUNDLE%\msi" %%f in (*.msi) do (
    echo [OK]   安装包    ^>  %%f
)

REM 安装包 — NSIS exe（如有）
for /r "%BUNDLE%\nsis" %%f in (*.exe) do (
    echo [OK]   安装包    ^>  %%f
)

echo.
echo ✓ 构建成功！
echo ═══════════════════════════════════════════════
echo.
endlocal
