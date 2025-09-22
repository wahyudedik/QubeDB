@echo off
REM QubeDB Complete Builder
REM Script untuk build semua versi QubeDB

echo 🚀 QubeDB Complete Builder
echo ==========================

echo.
echo 🔨 Building QubeDB Core...
cd qubedb-core
cargo build --release
if errorlevel 1 (
    echo ❌ Core build failed!
    pause
    exit /b 1
)

echo ✅ Core built successfully!

echo.
echo 🧪 Testing Core Functionality...
cargo run --example basic_usage
if errorlevel 1 (
    echo ❌ Basic test failed!
    pause
    exit /b 1
)

echo.
echo ⚡ Testing Performance...
cargo run --example performance_test
if errorlevel 1 (
    echo ❌ Performance test failed!
    pause
    exit /b 1
)

echo.
echo 🖥️ Building QubeDB GUI...
cd ..\qubedb-gui
cargo build --release
if errorlevel 1 (
    echo ❌ GUI build failed!
    pause
    exit /b 1
)

echo ✅ GUI built successfully!

echo.
echo 📦 Building Portable Package...
cd ..
call build-single-exe.bat
if errorlevel 1 (
    echo ❌ Portable build failed!
    pause
    exit /b 1
)

echo.
echo 🏗️ Building Windows Installer...
call build-installer.bat
if errorlevel 1 (
    echo ❌ Installer build failed!
    pause
    exit /b 1
)

echo.
echo 🎉 All Builds Completed Successfully!
echo =====================================
echo.
echo ✅ QubeDB Core: Built and Tested
echo ✅ QubeDB GUI: Built
echo ✅ Portable Package: Ready
echo ✅ Windows Installer: Ready
echo.
echo 📦 Outputs:
echo   - Portable: QubeDB-Portable\
echo   - Installer: installer\windows\QubeDB-Setup.exe
echo.
echo 🚀 Next steps:
echo 1. Test portable: cd QubeDB-Portable && QubeDB-Setup.bat
echo 2. Test installer: installer\windows\QubeDB-Setup.exe
echo 3. Run GUI: start-gui.bat
echo.
echo 📖 Documentation: SETUP-GUIDE.md
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
