@echo off
REM build.bat — thin wrapper that delegates to build.ps1
REM 真正的逻辑都在 build.ps1 里. 这样 .bat 不会因为路径里包含 (x86) 之类括号
REM 在 PowerShell 解析时炸掉, 也能在 PowerShell 环境下直接跑.
chcp 65001 >nul
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0build.ps1"
if errorlevel 1 (
    echo.
    echo [FAIL] build failed.
    pause
    exit /b 1
)
