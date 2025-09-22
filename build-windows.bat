@echo off
REM QubeDB Windows Build Script - Fixed Version
REM Script untuk build QubeDB di Windows dengan GUI

echo 🚀 QubeDB Windows Build Script
echo ================================

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Rust is not installed!
    echo Please install Rust from https://rustup.rs/
    echo.
    echo Quick install:
    echo 1. Download: https://win.rustup.rs/x86_64
    echo 2. Run: rustup-init.exe
    echo 3. Restart PowerShell
    echo 4. Run this script again
    pause
    exit /b 1
)

echo ✅ Rust found:
rustc --version

REM Check if Git is installed
git --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Git is not installed!
    echo Please install Git from https://git-scm.com/download/win
    pause
    exit /b 1
)

echo ✅ Git found:
git --version

echo.
echo 🔨 Building QubeDB Core...
cd qubedb-core

REM Clean previous build
cargo clean

REM Build release version
cargo build --release
if errorlevel 1 (
    echo ❌ Build failed!
    echo Please check the error messages above
    pause
    exit /b 1
)

echo ✅ QubeDB Core built successfully!

echo.
echo 🧪 Testing Basic Functionality...
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
echo 🎉 All builds completed successfully!
echo ====================================
echo.
echo ✅ QubeDB Core: Ready
echo ✅ QubeDB GUI: Ready
echo.
echo 🚀 Next steps:
echo 1. Run GUI: cd qubedb-gui && cargo run
echo 2. Or run: start-gui.bat
echo.
echo 📖 Documentation: SETUP-GUIDE.md
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause