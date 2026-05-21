@echo off
REM ─────────────────────────────────────────────────────────────────────────────
REM build-win.bat — Modem Cat Windows 构建脚本
REM
REM 用法:
REM   build-win.bat            构建当前架构的二进制 (x86_64)
REM   build-win.bat --bundle   构建 + 打包 .msi / .exe 安装包
REM   build-win.bat --help     显示帮助
REM ─────────────────────────────────────────────────────────────────────────────
setlocal EnableDelayedExpansion

REM ── 参数解析 ─────────────────────────────────────────────────────────────────
set "MODE=binary"
if "%~1"=="--bundle" set "MODE=bundle"
if "%~1"=="--help"   goto :show_help
if "%~1"=="-h"       goto :show_help
if not "%~1"=="" (
    echo [ERR] 未知参数: %~1
    echo 使用 --help 查看用法
    exit /b 1
)
goto :start

:show_help
echo 用法: build-win.bat [选项]
echo.
echo   (无参数)   仅构建 x86_64 二进制
echo   --bundle   构建 + 打包 .msi / .exe 安装包
echo   --help     显示此帮助
exit /b 0

:start
echo.
echo ═══════════════════════════════════════════
echo   Modem Cat — Windows 构建
echo ═══════════════════════════════════════════
echo.

REM ── 检查 Rust ─────────────────────────────────────────────────────────────────
where cargo >nul 2>&1
if errorlevel 1 (
    echo [ERR] 未找到 cargo，请安装 Rust: https://rustup.rs
    exit /b 1
)
for /f "tokens=*" %%v in ('rustc --version 2^>^&1') do set RUST_VER=%%v
echo [OK]  Rust: %RUST_VER%

REM ── 检查 tauri-cli（仅 --bundle 模式）────────────────────────────────────────
if "%MODE%"=="bundle" (
    cargo tauri --version >nul 2>&1
    if errorlevel 1 (
        echo [WARN] 未安装 cargo-tauri，正在安装...
        cargo install tauri-cli --version "^2" --locked
        if errorlevel 1 (
            echo [ERR] tauri-cli 安装失败
            exit /b 1
        )
    )
    for /f "tokens=*" %%v in ('cargo tauri --version 2^>^&1') do echo [OK]  tauri-cli: %%v
)

REM ── 自动定位 MSVC 工具链 ──────────────────────────────────────────────────────
REM 优先使用 vswhere（VS 2017+ 自带）定位最新版本的 vcvarsall.bat
set "VCVARSALL="

REM 尝试标准 vswhere 路径
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
if not exist "%VSWHERE%" set "VSWHERE=%ProgramFiles%\Microsoft Visual Studio\Installer\vswhere.exe"

if exist "%VSWHERE%" (
    for /f "usebackq delims=" %%p in (
        `"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -find VC\Auxiliary\Build\vcvarsall.bat`
    ) do set "VCVARSALL=%%p"
)

REM 备用：手动搜索常见路径
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
    ) do (
        if exist %%e set "VCVARSALL=%%~e"
    )
)

if not defined VCVARSALL (
    echo [ERR] 未找到 MSVC 工具链
    echo       请安装 Visual Studio 2019/2022 或 Build Tools:
    echo       https://visualstudio.microsoft.com/visual-cpp-build-tools/
    exit /b 1
)

echo [OK]  MSVC: %VCVARSALL%

REM 初始化 MSVC 环境（x64）
call "%VCVARSALL%" x64
if errorlevel 1 (
    echo [ERR] MSVC 环境初始化失败
    exit /b 1
)
echo [OK]  MSVC 环境已初始化 (x64)

REM ── 切换到项目目录 ────────────────────────────────────────────────────────────
cd /d "%~dp0"
echo [INFO] 工作目录: %CD%
echo.

REM ── 开始构建 ─────────────────────────────────────────────────────────────────
set "BUILD_START=%TIME%"

if "%MODE%"=="bundle" (
    echo [INFO] 模式: 构建 + 打包安装包
    cd src-tauri
    cargo tauri build
) else (
    echo [INFO] 模式: 构建二进制
    cd src-tauri
    cargo build --release
)

if errorlevel 1 (
    echo.
    echo [ERR] 构建失败！请检查上方错误信息。
    exit /b 1
)

REM ── 输出产物信息 ──────────────────────────────────────────────────────────────
echo.
if "%MODE%"=="bundle" (
    REM 查找安装包
    for /r "..\target\release\bundle" %%f in (*.msi *.exe) do (
        echo [OK]  安装包: %%f
    )
) else (
    set "BINARY=..\target\release\modem-cat.exe"
    if exist "!BINARY!" (
        echo [OK]  二进制: !BINARY!
    )
)

echo.
echo ✓ 构建成功！
echo.
endlocal
