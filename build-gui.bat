@echo off
REM QubeDB GUI Build Script
REM Script untuk build GUI dengan Tauri

echo 🖥️ QubeDB GUI Build Script
echo ==========================

REM Check if Node.js is installed
node --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Node.js is not installed!
    echo Please install Node.js from https://nodejs.org/
    echo.
    echo Or install via Chocolatey:
    echo choco install nodejs -y
    pause
    exit /b 1
)

echo ✅ Node.js found:
node --version

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Rust is not installed!
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo ✅ Rust found:
rustc --version

echo.
echo 🔨 Building QubeDB GUI...
cd qubedb-gui

REM Install npm dependencies
echo Installing npm dependencies...
npm install
if errorlevel 1 (
    echo ❌ Failed to install npm dependencies!
    pause
    exit /b 1
)

REM Build GUI
echo Building GUI...
cargo build --release
if errorlevel 1 (
    echo ❌ GUI build failed!
    pause
    exit /b 1
)

echo ✅ QubeDB GUI built successfully!

echo.
echo 🎉 GUI build completed!
echo ======================
echo.
echo ✅ QubeDB GUI: Ready
echo.
echo 🚀 Next steps:
echo 1. Run GUI: cargo run
echo 2. Or run: start-gui.bat
echo.
echo 📖 Documentation: SETUP-GUIDE.md
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
