@echo off
REM QubeDB Windows Installer Build Script
REM Script untuk build Windows installer

echo 🏗️ QubeDB Windows Installer Builder
echo ===================================

REM Check if NSIS is installed
makensis --version >nul 2>&1
if errorlevel 1 (
    echo ❌ NSIS is not installed!
    echo.
    echo 📥 Installing NSIS...
    echo Please download and install NSIS from: https://nsis.sourceforge.io/Download
    echo.
    echo Or install via Chocolatey:
    echo choco install nsis -y
    pause
    exit /b 1
)

echo ✅ NSIS found:
makensis --version

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
echo 📦 Building Windows Installer...
cd ..\installer\windows
makensis qubedb-installer.nsi
if errorlevel 1 (
    echo ❌ Installer build failed!
    pause
    exit /b 1
)

echo ✅ Windows Installer built successfully!

echo.
echo 🎉 Installer build completed!
echo =============================
echo.
echo ✅ QubeDB Core: Ready
echo ✅ QubeDB GUI: Ready
echo ✅ Windows Installer: Ready
echo.
echo 📦 Installer location: installer\windows\QubeDB-Setup.exe
echo.
echo 🚀 Next steps:
echo 1. Run installer: QubeDB-Setup.exe
echo 2. Or run GUI directly: start-gui.bat
echo.
echo 📖 Documentation: SETUP-GUIDE.md
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
