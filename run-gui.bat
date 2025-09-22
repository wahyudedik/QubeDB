@echo off
REM QubeDB GUI Runner - One Click Launch
REM Script untuk menjalankan GUI dengan sekali klik

echo 🖥️ QubeDB GUI - One Click Launch
echo ================================

REM Check if GUI is built
if not exist "qubedb-gui\target\release\qubedb-gui.exe" (
    echo ❌ GUI not built yet!
    echo.
    echo 🔨 Building GUI first...
    call build-gui.bat
    if errorlevel 1 (
        echo ❌ Failed to build GUI!
        pause
        exit /b 1
    )
)

echo ✅ GUI found, starting QubeDB Desktop...
echo.

REM Start GUI
cd qubedb-gui
cargo run

echo.
echo QubeDB GUI closed.
pause
