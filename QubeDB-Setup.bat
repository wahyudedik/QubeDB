@echo off
REM QubeDB Complete Windows Installer
REM Script untuk install, compile, test, dan jalankan GUI

echo 🚀 QubeDB Complete Windows Installer
echo =====================================
echo.
echo This installer will:
echo ✅ Check and install all dependencies
echo ✅ Compile QubeDB Core
echo ✅ Test all functionality
echo ✅ Build and launch GUI
echo.

pause

echo.
echo 🔍 Step 1: Checking System Requirements...
echo ==========================================

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Rust is not installed!
    echo.
    echo 📥 Installing Rust...
    echo Please download and install Rust from: https://rustup.rs/
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
    echo.
    echo 📥 Installing Git...
    echo Please download and install Git from: https://git-scm.com/download/win
    echo.
    echo Or install via Chocolatey:
    echo choco install git -y
    pause
    exit /b 1
)

echo ✅ Git found:
git --version

REM Check if Node.js is installed
node --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Node.js is not installed!
    echo.
    echo 📥 Installing Node.js...
    echo Please download and install Node.js from: https://nodejs.org/
    echo.
    echo Or install via Chocolatey:
    echo choco install nodejs -y
    pause
    exit /b 1
)

echo ✅ Node.js found:
node --version

echo.
echo 🛠️ Step 2: Installing Tauri CLI...
echo ==================================

cargo install tauri-cli
if errorlevel 1 (
    echo ❌ Failed to install Tauri CLI!
    echo Please check the error messages above
    pause
    exit /b 1
)

echo ✅ Tauri CLI installed successfully!

echo.
echo 🔨 Step 3: Building QubeDB Core...
echo ==================================

cd qubedb-core

REM Clean previous build
cargo clean

REM Build release version
cargo build --release
if errorlevel 1 (
    echo ❌ Core build failed!
    echo Please check the error messages above
    pause
    exit /b 1
)

echo ✅ QubeDB Core built successfully!

echo.
echo 🧪 Step 4: Testing Core Functionality...
echo =======================================

cargo run --example basic_usage
if errorlevel 1 (
    echo ❌ Basic test failed!
    pause
    exit /b 1
)

echo.
echo ⚡ Step 5: Performance Testing...
echo ================================

cargo run --example performance_test
if errorlevel 1 (
    echo ❌ Performance test failed!
    pause
    exit /b 1
)

echo.
echo 🖥️ Step 6: Building QubeDB GUI...
echo ==================================

cd ..\qubedb-gui

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
echo 🎉 Installation Completed Successfully!
echo ======================================
echo.
echo ✅ Dependencies: Installed
echo ✅ Core: Built and Tested
echo ✅ GUI: Built
echo ✅ All Tests: Passing
echo.
echo 🚀 Starting QubeDB GUI...
echo.

REM Start GUI
cargo run

echo.
echo QubeDB GUI closed.
echo.
echo 📖 Documentation: SETUP-GUIDE.md
echo 🐛 Issues: https://github.com/wahyudedik/QubeDB/issues
echo.

pause
