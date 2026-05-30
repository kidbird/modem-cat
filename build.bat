@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion
cd /d "%~dp0"
title Modem Cat Build

echo.
echo  ===================================================
echo   Modem Cat  -  Unified Build
echo   Portable   -^>  dist\portable\modem-cat.exe
echo   Installer  -^>  dist\installer\  (NSIS / MSI)
echo  ===================================================
echo.

REM ── 1. VS Build Tools ──────────────────────────────
set _vc=C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat
if not exist "%_vc%" set _vc=C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat
if not exist "%_vc%" set _vc=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat
if not exist "%_vc%" set _vc=C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvarsall.bat
if not exist "%_vc%" set _vc=C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvarsall.bat

if exist "%_vc%" (
    echo [1/4] VS env: %_vc%
    call "%_vc%" x64 >nul 2>&1
    echo       OK
) else (
    echo [1/4] VS env: vcvarsall.bat not found - proceeding without it
)
echo.

REM ── 2. Installer  (cargo tauri build) ──────────────
echo [2/4] Installer build...
cargo tauri --version >nul 2>&1
if %errorlevel% neq 0 (
    echo       SKIP - tauri-cli not installed
    echo       To install: cargo install tauri-cli --version "^2" --locked
    set _skip_installer=1
) else (
    REM tauri build manages its own RUSTFLAGS
    set RUSTFLAGS=
    cargo tauri build
    if !errorlevel! neq 0 (
        echo.
        echo [FAIL] Installer build failed. See errors above.
        pause
        exit /b 1
    )
    mkdir dist\installer 2>nul
    REM Bundle may land in workspace target or src-tauri\target depending on build origin
    set _copied=0
    if exist "target\release\bundle\nsis" (
        for /r "target\release\bundle\nsis" %%F in (*.exe) do (
            copy /Y "%%F" "dist\installer\" >nul
            echo       dist\installer\%%~nxF
            set _copied=1
        )
    )
    if exist "target\release\bundle\msi" (
        for /r "target\release\bundle\msi" %%F in (*.msi) do (
            copy /Y "%%F" "dist\installer\" >nul
            echo       dist\installer\%%~nxF
            set _copied=1
        )
    )
    if exist "src-tauri\target\release\bundle\nsis" (
        for /r "src-tauri\target\release\bundle\nsis" %%F in (*.exe) do (
            copy /Y "%%F" "dist\installer\" >nul
            echo       dist\installer\%%~nxF
            set _copied=1
        )
    )
    if exist "src-tauri\target\release\bundle\msi" (
        for /r "src-tauri\target\release\bundle\msi" %%F in (*.msi) do (
            copy /Y "%%F" "dist\installer\" >nul
            echo       dist\installer\%%~nxF
            set _copied=1
        )
    )
    if !_copied!==0 echo       WARNING: no bundle files found
)
echo.

REM ── 3. Portable  (static CRT, workspace root -> target\release) ──
echo [3/4] Portable build (static CRT)...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release -p modem-cat
if %errorlevel% neq 0 (
    echo.
    echo [FAIL] Portable build failed. See errors above.
    pause
    exit /b 1
)
mkdir dist\portable 2>nul
copy /Y "target\release\modem-cat.exe" "dist\portable\modem-cat.exe" >nul
echo       dist\portable\modem-cat.exe
echo.

REM ── 4. Summary ─────────────────────────────────────
echo [4/4] Output:
echo.
set _dist=%~dp0dist
powershell -NoProfile -Command "Get-ChildItem '%_dist%' -Recurse -File -ErrorAction SilentlyContinue | ForEach-Object { $s=if($_.Length -ge 1MB){'{0:N1} MB'-f($_.Length/1MB)}elseif($_.Length -ge 1KB){'{0:N0} KB'-f($_.Length/1KB)}else{'{0} B'-f$_.Length}; '  '+$_.FullName.Replace('%_dist%\','')+' ('+$s+')' }"
echo.
echo  ===================================================
echo   Build complete!
echo  ===================================================
echo.
pause
exit /b 0
