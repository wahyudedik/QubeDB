@echo off
REM QubeDB Complete Runner
REM Script untuk install, build, test, dan jalankan GUI

echo 🚀 QubeDB Complete Runner
echo =========================

echo.
echo 📦 Step 1: Installing Dependencies...
call install-everything.bat
if errorlevel 1 (
    echo ❌ Failed to install dependencies!
    pause
    exit /b 1
)

echo.
echo 🔨 Step 2: Building QubeDB Core...
call quick-install.bat
if errorlevel 1 (
    echo ❌ Failed to build core!
    pause
    exit /b 1
)

echo.
echo 🖥️ Step 3: Building QubeDB GUI...
call build-gui.bat
if errorlevel 1 (
    echo ❌ Failed to build GUI!
    pause
    exit /b 1
)

echo.
echo 🧪 Step 4: Running Tests...
call test-complete.bat
if errorlevel 1 (
    echo ❌ Tests failed!
    pause
    exit /b 1
)

echo.
echo 🎉 QubeDB Setup Completed Successfully!
echo ======================================
echo.
echo ✅ Dependencies: Installed
echo ✅ Core: Built and Tested
echo ✅ GUI: Built
echo ✅ Tests: Passing
echo.
echo 🚀 Starting QubeDB GUI...
echo.

REM Start GUI
call run-gui-now.bat

echo.
echo QubeDB setup and testing completed!
echo.
echo 📖 Documentation: SETUP-GUIDE.md
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
