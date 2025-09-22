@echo off
REM QubeDB GUI Launcher
REM Script untuk menjalankan QubeDB GUI

echo 🖥️ QubeDB GUI Launcher
echo ======================

REM Check if GUI is built
if not exist "qubedb-gui\target\release\qubedb-gui.exe" (
    echo ❌ GUI not built yet!
    echo Please run build-windows.bat first
    pause
    exit /b 1
)

echo ✅ GUI found, starting QubeDB Desktop...
echo.

REM Start GUI
cd qubedb-gui
cargo run

echo.
echo QubeDB GUI closed.
pause
