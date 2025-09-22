@echo off
REM QubeDB Complete Installer
REM Script untuk install semua dependencies dan build QubeDB

echo 🚀 QubeDB Complete Installer
echo ===========================

echo.
echo 📦 Step 1: Installing Dependencies...
call install-dependencies.bat
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
call test-all.bat
if errorlevel 1 (
    echo ❌ Tests failed!
    pause
    exit /b 1
)

echo.
echo 🎉 QubeDB Installation Completed!
echo =================================
echo.
echo ✅ Dependencies: Installed
echo ✅ Core: Built
echo ✅ GUI: Built
echo ✅ Tests: Passing
echo.
echo 🚀 Ready to use!
echo.
echo 📖 Next steps:
echo 1. Run GUI: run-gui.bat
echo 2. Read docs: SETUP-GUIDE.md
echo 3. Start developing!
echo.
echo 📞 Support:
echo 📧 Email: support@qubedb.com
echo 💬 Discord: https://discord.gg/qubedb
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
